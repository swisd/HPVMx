//! Intermediate Representation (IR) generation.
//!
//! This module contains the [`IRGenerator`] which lowers the AST into
//! a flat, three-address-style [`IRInst`] sequence.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::ast::*;
use crate::error::error;
use crate::ir::*;

/// State for generating Intermediate Representation from the AST.
pub struct IRGenerator {
    temp_count: usize,
    label_count: usize,
    /// The generated list of IR instructions.
    pub code: Vec<IRInst>,
    /// Metadata about function parameters, used for backend code generation.
    pub function_params: HashMap<String, Vec<String>>,
    structs: HashMap<String, Vec<(String, Type)>>,
    var_types: HashMap<String, Type>,
    loop_stack: Vec<(String, String)>,
    position: u64,
}

impl IRGenerator {
    /// Creates a new IRGenerator.
    pub fn new() -> Self {
        Self {
            temp_count: 0,
            label_count: 0,
            code: vec![],
            function_params: HashMap::new(),
            structs: HashMap::new(),
            var_types: HashMap::new(),
            loop_stack: vec![],
            position: 0
        }
    }

    fn temp(&mut self) -> String {
        let t = format!("t{}", self.temp_count);
        self.temp_count += 1;
        t
    }

    fn label(&mut self) -> String {
        let l = format!("L{}", self.label_count);
        self.label_count += 1;
        l
    }

    /// Generates IR for a complete program (list of statements).
    pub fn gen_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.gen_stmt(stmt.clone());
        }
    }

    pub fn gen_stmt(&mut self, stmt: Stmt) {
        self.position += 1;
        match stmt {
            Stmt::Let { name, value, .. } => {
                let inferred_ty = self.infer_expr_type(&value);
                let v = self.gen_expr(value);
                self.code.push(IRInst::StoreVar(name.clone(), v));
                if let Some(ty) = inferred_ty {
                    self.var_types.insert(name, ty);
                }
            }

            Stmt::Assign(name, value) => {
                let v = self.gen_expr(value);
                self.code.push(IRInst::StoreVar(name, v));
            }

            Stmt::Return(expr) => {
                let v = self.gen_expr(expr);
                self.code.push(IRInst::Return(v));
            }

            Stmt::Expr(expr) => {
                self.gen_expr(expr);
            }

            Stmt::Poke(addr, value) => {
                let addr = self.gen_expr(addr);
                let value = self.gen_expr(value);
                self.code.push(IRInst::StoreMem(addr, value));
            }

            Stmt::AssignIndex { base, index, value } => {
                let addr = self.gen_index_addr(base, index);
                let value = self.gen_expr(value);
                self.code.push(IRInst::StoreMem(addr, value));
            }

            Stmt::AssignField { base, field, value } => {
                let addr = self.gen_field_addr(base, field);
                let value = self.gen_expr(value);
                self.code.push(IRInst::StoreMem(addr, value));
            }

            Stmt::Struct { name, fields } => {
                self.structs.insert(name, fields);
            }

            Stmt::If {
                cond,
                then_branch,
                elif,
                else_branch,
            } => {
                let cond_val = self.gen_expr(cond);
                let else_label = self.label();
                let end_label = self.label();

                self.code.push(IRInst::JumpIfZero(cond_val, else_label.clone()));

                for s in then_branch {
                    self.gen_stmt(s);
                }

                self.code.push(IRInst::Jump(end_label.clone()));
                self.code.push(IRInst::Label(else_label));

                for (elif_cond, elif_branch) in elif {
                    let next_label = self.label();
                    let elif_val = self.gen_expr(elif_cond);
                    self.code.push(IRInst::JumpIfZero(elif_val, next_label.clone()));

                    for s in elif_branch {
                        self.gen_stmt(s);
                    }

                    self.code.push(IRInst::Jump(end_label.clone()));
                    self.code.push(IRInst::Label(next_label));
                }

                if let Some(branch) = else_branch {
                    for s in branch {
                        self.gen_stmt(s);
                    }
                }

                self.code.push(IRInst::Label(end_label));
            }

            Stmt::Loop(body) => {
                let start = self.label();
                let end = self.label();

                self.loop_stack.push((start.clone(), end.clone()));
                self.code.push(IRInst::Label(start.clone()));

                for s in body {
                    self.gen_stmt(s);
                }

                self.code.push(IRInst::Jump(start));
                self.code.push(IRInst::Label(end));
                self.loop_stack.pop();
            }

            Stmt::Break => {
                if let Some((_, end)) = self.loop_stack.last() {
                    self.code.push(IRInst::Jump(end.clone()));
                } else {
                    error("break outside loop");
                }
            }

            Stmt::Continue => {
                if let Some((start, _)) = self.loop_stack.last() {
                    self.code.push(IRInst::Jump(start.clone()));
                } else {
                    error("continue outside loop");
                }
            }

            Stmt::Function { name, params, body, .. } => {
                // println!("INSERT PARAMS {} => {:?}", name, params);
                self.function_params.insert(name.clone(), params.clone());
                // println!("FPARAMS ==> {:?}", self.function_params);

                self.code.push(IRInst::Label(name.clone()));

                for stmt in body {
                    self.gen_stmt(stmt);
                }
            }

            Stmt::ExternFunction { name, params } => {
                self.function_params.insert(name.clone(), params);
                self.code.push(IRInst::Extern(name));
            }

            Stmt::Import { .. } => {}
            Stmt::None => {}
        }
    }

    pub fn gen_expr(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Number(n) => {
                let t = self.temp();
                self.code.push(IRInst::LoadConst(t.clone(), n));
                t
            }

            Expr::Variable(name) => {
                let t = self.temp();
                self.code.push(IRInst::LoadVar(t.clone(), name));
                t
            }

            Expr::Binary(left, op, right) => {
                let l = self.gen_expr(*left);
                let r = self.gen_expr(*right);
                let out = self.temp();

                match op {
                    Op::Add => self.code.push(IRInst::Add(out.clone(), l, r)),
                    Op::Sub => self.code.push(IRInst::Sub(out.clone(), l, r)),
                    Op::Mul => self.code.push(IRInst::Mul(out.clone(), l, r)),
                    Op::Div => self.code.push(IRInst::Div(out.clone(), l, r)),

                    Op::Eq => self.code.push(IRInst::Eq(out.clone(), l, r)),
                    Op::Neq => self.code.push(IRInst::Neq(out.clone(), l, r)),
                    Op::Lt => self.code.push(IRInst::Lt(out.clone(), l, r)),
                    Op::Gt => self.code.push(IRInst::Gt(out.clone(), l, r)),
                    Op::LtEq => self.code.push(IRInst::LtEq(out.clone(), l, r)),
                    Op::GtEq => self.code.push(IRInst::GtEq(out.clone(), l, r)),
                }

                out
            }

            Expr::Call(name, args) => {
                if name == "alloc_struct" {
                    let out = self.temp();
                    let size = self.struct_size(&args);
                    self.code.push(IRInst::StackAlloc(out.clone(), size));
                    return out;
                }

                let mut vals = vec![];

                for arg in args {
                    vals.push(self.gen_expr(arg));
                }

                let out = self.temp();
                self.code.push(IRInst::Call(out.clone(), name, vals));
                out
            }

            Expr::Peek(addr) => {
                let addr = self.gen_expr(*addr);
                let out = self.temp();
                self.code.push(IRInst::LoadMem(out.clone(), addr));
                out
            }

            Expr::Index(base, index) => {
                let addr = self.gen_index_addr(*base, *index);
                let out = self.temp();
                self.code.push(IRInst::LoadMem(out.clone(), addr));
                out
            }

            Expr::Field(base, field) => {
                let addr = self.gen_field_addr(*base, field);
                let out = self.temp();
                self.code.push(IRInst::LoadMem(out.clone(), addr));
                out
            }
        }
    }

    fn infer_expr_type(&self, expr: &Expr) -> Option<Type> {
        match expr {
            Expr::Call(name, args) if name == "alloc_struct" => {
                if let Some(Expr::Variable(struct_name)) = args.first() {
                    Some(Type::Ptr(Box::new(Type::Struct(struct_name.clone()))))
                } else {
                    Some(Type::Ptr(Box::new(Type::I64)))
                }
            }
            _ => None,
        }
    }

    fn struct_size(&self, args: &[Expr]) -> i64 {
        if let Some(Expr::Variable(struct_name)) = args.first() {
            if let Some(fields) = self.structs.get(struct_name) {
                return fields.len() as i64 * 8;
            }
        }

        error("alloc_struct expects a known struct name");
        8
    }

    fn gen_index_addr(&mut self, base: Expr, index: Expr) -> String {
        let base = self.gen_expr(base);
        let index = self.gen_expr(index);
        let scale = self.temp();
        let addr = self.temp();

        self.code.push(IRInst::LoadConst(scale.clone(), 8));
        self.code.push(IRInst::Mul(scale.clone(), index, scale.clone()));
        self.code.push(IRInst::Add(addr.clone(), base, scale));

        addr
    }

    fn gen_field_addr(&mut self, base: Expr, field: String) -> String {
        let offset = self.field_offset(&base, &field);
        let base = self.gen_expr(base);

        if offset == 0 {
            return base;
        }

        let offset_tmp = self.temp();
        let addr = self.temp();
        self.code.push(IRInst::LoadConst(offset_tmp.clone(), offset));
        self.code.push(IRInst::Add(addr.clone(), base, offset_tmp));
        addr
    }

    fn field_offset(&self, base: &Expr, field: &str) -> i64 {
        if let Expr::Variable(name) = base {
            if let Some(Type::Ptr(inner)) = self.var_types.get(name) {
                if let Type::Struct(struct_name) = &**inner {
                    if let Some(fields) = self.structs.get(struct_name) {
                        for (i, (candidate, _)) in fields.iter().enumerate() {
                            if candidate == field {
                                return i as i64 * 8;
                            }
                        }
                    }
                }
            }
        }

        error(&format!("unknown field {}", field));
        0
    }
}
