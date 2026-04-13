use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::ast::*;

#[derive(Debug)]
enum Control {
    None,
    Return(i64),
    Break,
    Continue,
}

pub struct Interpreter {
    scopes: Vec<HashMap<String, i64>>,
    types: HashMap<String, Type>,

    memory: HashMap<i64, i64>,
    functions: HashMap<String, Stmt>,
    structs: HashMap<String, Vec<(String, Type)>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            types: HashMap::new(),
            memory: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
        }
    }

    fn set(&mut self, name: String, val: i64, ty: Type) {
        self.scopes.last_mut().unwrap().insert(name.clone(), val);
        self.types.insert(name, ty);
    }

    fn get(&self, name: &str) -> i64 {
        for s in self.scopes.iter().rev() {
            if let Some(v) = s.get(name) {
                return *v;
            }
        }
        panic!("Undefined {}", name);
    }

    fn field_offset(&self, struct_name: &str, field: &str) -> i64 {
        let fields = self.structs.get(struct_name).unwrap();

        for (i, (f, _)) in fields.iter().enumerate() {
            if f == field {
                return i as i64;
            }
        }

        panic!("Unknown field {}", field);
    }

    fn get_struct_type(&self, expr: &Expr) -> String {
        if let Expr::Variable(name) = expr {
            if let Type::Ptr(inner) = self.types.get(name).unwrap() {
                if let Type::Struct(s) = &**inner {
                    return s.clone();
                }
            }
        }
        panic!("Not a struct pointer");
    }

    fn eval(&mut self, expr: Expr) -> i64 {
        match expr {
            Expr::Number(n) => n,

            Expr::Variable(name) => self.get(&name),

            Expr::Binary(left, op, right) => {
                let l = self.eval(*left);
                let r = self.eval(*right);

                match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,

                    Op::Eq => (l == r) as i64,
                    Op::Neq => (l != r) as i64,
                    Op::Lt => (l < r) as i64,
                    Op::Gt => (l > r) as i64,
                    Op::LtEq => (l <= r) as i64,
                    Op::GtEq => (l >= r) as i64,
                }
            }

            Expr::Call(name, args) => {
                if name == "alloc_struct" {
                    if let Expr::Variable(struct_name) = &args[0] {
                        let fields = self.structs.get(struct_name).unwrap();

                        let base = self.memory.len() as i64 + 1000;

                        for i in 0..fields.len() {
                            self.memory.insert(base + i as i64, 0);
                        }

                        return base;
                    } else {
                        panic!("alloc_struct expects struct name");
                    }
                }

                let vals: Vec<i64> = args.into_iter().map(|a| self.eval(a)).collect();
                self.call(&name, vals)
            }

            Expr::Peek(e) => {
                let addr = self.eval(*e);
                *self.memory.get(&addr).unwrap_or(&0)
            }

            Expr::Index(base, index) => {
                let b = self.eval(*base);
                let i = self.eval(*index);
                *self.memory.get(&(b + i)).unwrap_or(&0)
            }

            Expr::Field(base, field) => {
                let addr = self.eval(*base.clone());
                let struct_name = self.get_struct_type(&base);
                let offset = self.field_offset(&struct_name, &field);

                *self.memory.get(&(addr + offset)).unwrap_or(&0)
            }
        }
    }

    fn call(&mut self, name: &str, args: Vec<i64>) -> i64 {
        // println!("Calling function: {}", name);
        let f = self.functions.get(name)
            .unwrap_or_else(|| panic!("Function {} not found", name))
            .clone();

        if let Stmt::Function { params, body, .. } = f {
            self.scopes.push(HashMap::new());

            // bind params
            for (i, p) in params.iter().enumerate() {
                self.set(p.clone(), args.get(i).copied().unwrap_or(0), Type::I64);
            }

            let result = self.exec_block(&body);

            self.scopes.pop();

            match result {
                Control::Return(v) => v,
                _ => 0,
            }
        } else {
            unreachable!()
        }
    }

    fn exec_block(&mut self, stmts: &[Stmt]) -> Control {
        self.scopes.push(HashMap::new());

        for stmt in stmts {
            let ctrl = self.exec(stmt.clone());

            match ctrl {
                Control::None => {}
                _ => {
                    self.scopes.pop();
                    return ctrl;
                }
            }
        }

        self.scopes.pop();
        Control::None
    }

    fn assign(&mut self, name: String, val: i64) {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(&name) {
                scope.insert(name.clone(), val);
                return;
            }
        }
        panic!("Undefined variable {}", name);
    }

    fn exec(&mut self, stmt: Stmt) -> Control {
        match stmt {
            Stmt::Let { name, ty, value } => {
                let val = self.eval(value.clone());

                let inferred_ty = match value {
                    Expr::Call(ref name_call, ref args) if name_call == "alloc_struct" => {
                        if let Expr::Variable(struct_name) = &args[0] {
                            Type::Ptr(Box::new(Type::Struct(struct_name.clone())))
                        } else {
                            Type::Ptr(Box::new(Type::I64))
                        }
                    }
                    _ => ty.unwrap_or(Type::I64),
                };

                self.set(name, val, inferred_ty);
                Control::None
            }

            Stmt::Assign(name, expr) => {
                let val = self.eval(expr);
                self.assign(name, val);
                Control::None
            }

            Stmt::AssignIndex { base, index, value } => {
                let b = self.eval(base);
                let i = self.eval(index);
                let v = self.eval(value);
                self.memory.insert(b + i, v);
                Control::None
            }

            Stmt::AssignField { base, field, value } => {
                let addr = self.eval(base.clone());
                let val = self.eval(value);

                let struct_name = self.get_struct_type(&base);
                let offset = self.field_offset(&struct_name, &field);

                self.memory.insert(addr + offset, val);

                Control::None
            }

            Stmt::Struct { name, fields } => {
                self.structs.insert(name, fields);
                Control::None
            }

            Stmt::Return(expr) => {
                let val = self.eval(expr);
                Control::Return(val)
            }

            Stmt::Expr(expr) => {
                self.eval(expr);
                Control::None
            }

            Stmt::Poke(addr, val) => {
                let a = self.eval(addr);
                let v = self.eval(val);
                self.memory.insert(a, v);
                Control::None
            }

            Stmt::If { cond, then_branch, elif, else_branch } => {
                if self.eval(cond) != 0 {
                    return self.exec_block(&then_branch);
                }

                // for (c, b) in elif {
                //     if self.eval(c) != 0 {
                //         return self.exec_block(&b);
                //     }
                // }

                if let Some(b) = else_branch {
                    return self.exec_block(&b);
                }

                Control::None
            }

            Stmt::Loop(body) => {
                loop {
                    match self.exec_block(&body) {
                        Control::None => {}
                        Control::Break => break,
                        Control::Continue => continue,
                        Control::Return(v) => return Control::Return(v),
                    }
                }
                Control::None
            }

            Stmt::Break => Control::Break,
            Stmt::Continue => Control::Continue,

            Stmt::Function { .. } => Control::None,
            _ => Control::None,
        }
    }

    pub fn run(&mut self, stmts: &[Stmt]) -> i64 {
        for s in stmts {
            match s {
                Stmt::Function { name, .. } => {
                    self.functions.insert(name.clone(), s.clone());
                }
                Stmt::Struct { name, fields } => {
                    self.structs.insert(name.clone(), fields.clone());
                }
                _ => {}
            }
        }

        self.call("main", vec![])
    }
}