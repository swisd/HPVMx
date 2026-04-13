use alloc::string::String;
use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::ast::*;
use crate::error::error;
use crate::ir::*;

pub struct IRGenerator {
    temp_count: usize,
    label_count: usize,
    pub code: Vec<IRInst>,
    pub function_params: HashMap<String, Vec<String>>,
    position: u64,
}

impl IRGenerator {
    pub fn new() -> Self {
        Self {
            temp_count: 0,
            label_count: 0,
            code: vec![],
            function_params: HashMap::new(),
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

    pub fn gen_program(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            self.gen_stmt(stmt.clone());
        }
    }

    pub fn gen_stmt(&mut self, stmt: Stmt) {
        self.position += 1;
        match stmt {
            Stmt::Let { name, value, .. } => {
                let v = self.gen_expr(value);
                self.code.push(IRInst::StoreVar(name, v));
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

            Stmt::If {
                cond,
                then_branch,
                else_branch,
                ..
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

                self.code.push(IRInst::Label(start.clone()));

                for s in body {
                    self.gen_stmt(s);
                }

                self.code.push(IRInst::Jump(start));
                self.code.push(IRInst::Label(end));
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

            _ => {}
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
                let mut vals = vec![];

                for arg in args {
                    vals.push(self.gen_expr(arg));
                }

                let out = self.temp();
                self.code.push(IRInst::Call(out.clone(), name, vals));
                out
            }

            _ => {error(&format!("{:#X} {:?}", self.position, expr)); String::new()}
        }
    }
}