use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::scripting::error::{ScriptError, ScriptErrorKind, ScriptResult};
use crate::scripting::ast::{Expr, Stmt, Script, BinOp};


#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Int(n) => alloc::format!("{}", n),
            Value::Str(s) => s.clone(),
            Value::Bool(b) => {
                if *b {
                    String::from("true")
                } else {
                    String::from("false")
                }
            }
        }
    }

    pub fn to_i64(&self) -> ScriptResult<i64> {
        match self {
            Value::Int(n) => Ok(*n),
            Value::Bool(b) => Ok(if *b { 1 } else { 0 }),
            Value::Str(s) => {
                s.parse::<i64>().map_err(|_| {
                    ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot convert string '{}' to integer", s),
                        "", 0, 0,
                    ).with_hint(String::from("Ensure the variable holds a numeric value"))
                })
            }
        }
    }
}


pub struct ExecutionContext {
    variables: HashMap<String, Value>,
    arguments: Vec<String>,
    in_loop: bool,
    exit_code: Option<i64>,
    should_exit: bool,
}

impl ExecutionContext {
    pub fn new(arguments: Vec<String>) -> Self {
        Self {
            variables: HashMap::new(),
            arguments,
            in_loop: false,
            exit_code: None,
            should_exit: false,
        }
    }

    pub fn set_var(&mut self, name: &str, value: Value) {
        self.variables.insert(name.to_string(), value);
    }

    pub fn get_var(&self, name: &str) -> ScriptResult<Value> {
        self.variables.get(name).cloned().ok_or_else(|| {
            ScriptError::new(
                ScriptErrorKind::VariableError,
                alloc::format!("Undefined variable '{}'", name),
                "", 0, 0,
            ).with_hint(alloc::format!("Did you forget to 'set {} = ...' before using it?", name))
        })
    }

    pub fn get_arg(&self, index: usize) -> ScriptResult<String> {
        if index >= self.arguments.len() {
            // Display as %%N where N = index + 1 (since %%1 maps to index 0)
            let display_idx = index + 1;
            Err(ScriptError::new(
                ScriptErrorKind::ArgumentError,
                alloc::format!(
                    "Argument %%{} not provided — script received {} argument(s), but %%{} was referenced",
                    display_idx,
                    self.arguments.len(),
                    display_idx
                ),
                "", 0, 0,
            ).with_hint(alloc::format!(
                "Pass more arguments when invoking the script: script run [script.sp] arg1 arg2 ... arg{}",
                display_idx
            )))
        } else {
            Ok(self.arguments[index].clone())
        }
    }

    pub fn arg_count(&self) -> usize {
        self.arguments.len()
    }

    pub fn set_in_loop(&mut self, in_loop: bool) {
        self.in_loop = in_loop;
    }

    pub fn is_in_loop(&self) -> bool {
        self.in_loop
    }

    pub fn request_exit(&mut self, code: Option<i64>) {
        self.exit_code = code;
        self.should_exit = true;
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }

    pub fn take_exit_code(&mut self) -> Option<i64> {
        self.exit_code.take()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlow {
    Continue,
    Break,
    Exit,
}

pub struct Interpreter {
    context: ExecutionContext,
}

impl Interpreter {
    pub fn new(arguments: Vec<String>) -> Self {
        Self {
            context: ExecutionContext::new(arguments),
        }
    }

    pub fn execute_script(&mut self, script: &Script) -> ScriptResult<Option<i64>> {
        for stmt in &script.statements {
            let flow = self.execute_stmt(stmt)?;
            if flow == ControlFlow::Exit || self.context.should_exit() {
                break;
            }
        }
        Ok(self.context.take_exit_code())
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> ScriptResult<ControlFlow> {
        match stmt {
            Stmt::Set { name, value } => {
                let val = self.eval_expr(value)?;
                self.context.set_var(name, val);
                Ok(ControlFlow::Continue)
            }
            Stmt::Echo(exprs) => {
                let mut parts = Vec::new();
                for expr in exprs {
                    let val = self.eval_expr(expr)?;
                    parts.push(val.to_string());
                }
                let mut output = parts.join(" ");
                output = self.interpolate_string(&output);
                crate::message!("\n", "{}", output);
                Ok(ControlFlow::Continue)
            }
            Stmt::If { condition, then_body, elif_branches, else_body } => {
                let cond_val = self.eval_expr(condition)?;
                let cond_bool = Self::to_bool(&cond_val)?;

                if cond_bool {
                    for stmt in then_body {
                        let flow = self.execute_stmt(stmt)?;
                        if flow != ControlFlow::Continue {
                            return Ok(flow);
                        }
                    }
                } else {
                    let mut matched = false;
                    for (elif_cond, elif_body) in elif_branches {
                        let elif_val = self.eval_expr(elif_cond)?;
                        let elif_bool = Self::to_bool(&elif_val)?;
                        if elif_bool {
                            for stmt in elif_body {
                                let flow = self.execute_stmt(stmt)?;
                                if flow != ControlFlow::Continue {
                                    return Ok(flow);
                                }
                            }
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        if let Some(else_body) = else_body {
                            for stmt in else_body {
                                let flow = self.execute_stmt(stmt)?;
                                if flow != ControlFlow::Continue {
                                    return Ok(flow);
                                }
                            }
                        }
                    }
                }
                Ok(ControlFlow::Continue)
            }
            Stmt::For { init_var, init_value, condition, increment, body } => {
                let init_val = self.eval_expr(init_value)?;
                self.context.set_var(init_var, init_val);
                self.context.set_in_loop(true);

                loop {
                    if self.context.should_exit() {
                        return Ok(ControlFlow::Exit);
                    }

                    let cond_val = self.eval_expr(condition)?;
                    let cond_bool = Self::to_bool(&cond_val)?;
                    if !cond_bool {
                        break;
                    }

                    let mut broke = false;
                    for stmt in body {
                        let flow = self.execute_stmt(stmt)?;
                        match flow {
                            ControlFlow::Break => {
                                broke = true;
                                break;
                            }
                            ControlFlow::Exit => {
                                return Ok(ControlFlow::Exit);
                            }
                            ControlFlow::Continue => {}
                        }
                    }
                    if broke {
                        break;
                    }

                    let _inc_val = self.eval_expr(increment)?;
                }

                self.context.set_in_loop(false);
                Ok(ControlFlow::Continue)
            }
            Stmt::While { condition, body } => {
                self.context.set_in_loop(true);

                loop {
                    if self.context.should_exit() {
                        return Ok(ControlFlow::Exit);
                    }

                    let cond_val = self.eval_expr(condition)?;
                    let cond_bool = Self::to_bool(&cond_val)?;
                    if !cond_bool {
                        break;
                    }

                    let mut broke = false;
                    for stmt in body {
                        let flow = self.execute_stmt(stmt)?;
                        match flow {
                            ControlFlow::Break => {
                                broke = true;
                                break;
                            }
                            ControlFlow::Exit => {
                                return Ok(ControlFlow::Exit);
                            }
                            ControlFlow::Continue => {}
                        }
                    }
                    if broke {
                        break;
                    }
                }

                self.context.set_in_loop(false);
                Ok(ControlFlow::Continue)
            }
            Stmt::Break => {
                if !self.context.is_in_loop() {
                    return Err(ScriptError::new(
                        ScriptErrorKind::LoopError,
                        String::from("'break' used outside of a loop"),
                        "", 0, 0,
                    ).with_hint(String::from("'break' can only appear inside 'for' or 'while' loops")));
                }
                Ok(ControlFlow::Break)
            }
            Stmt::Continue => {
                if !self.context.is_in_loop() {
                    return Err(ScriptError::new(
                        ScriptErrorKind::LoopError,
                        String::from("'continue' used outside of a loop"),
                        "", 0, 0,
                    ).with_hint(String::from("'continue' can only appear inside 'for' or 'while' loops")));
                }
                Ok(ControlFlow::Continue)
            }
            Stmt::Exit(code) => {
                let exit_code = if let Some(c) = code {
                    let val = self.eval_expr(c)?;
                    Some(val.to_i64()?)
                } else {
                    Some(0)
                };
                self.context.request_exit(exit_code);
                Ok(ControlFlow::Exit)
            }
            Stmt::Command(args) => {
                let flow = self.execute_command(args)?;
                Ok(flow)
            }
            Stmt::Block(stmts) => {
                for stmt in stmts {
                    let flow = self.execute_stmt(stmt)?;
                    if flow != ControlFlow::Continue {
                        return Ok(flow);
                    }
                }
                Ok(ControlFlow::Continue)
            }
        }
    }

    fn execute_command(&mut self, args: &[Expr]) -> ScriptResult<ControlFlow> {
        if args.is_empty() {
            return Err(ScriptError::new(
                ScriptErrorKind::RuntimeError,
                String::from("Empty command — no command name provided"),
                "", 0, 0,
            ).with_hint(String::from("A command must start with a command name")));
        }

        let mut cmd_parts: Vec<String> = Vec::new();
        for arg in args {
            let val = self.eval_expr(arg)?;
            cmd_parts.push(val.to_string());
        }

        crate::vdebug_autoprefix!("executing command: {}", cmd_parts.join(" "));
        crate::message!("\n", "[script] → {}", cmd_parts.join(" "));
        crate::vdebug_autoprefix!("command resolved: {:?}", cmd_parts);

        Ok(ControlFlow::Continue)
    }

    fn eval_expr(&mut self, expr: &Expr) -> ScriptResult<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Int(*n)),
            Expr::StringLit(s) => Ok(Value::Str(s.clone())),
            Expr::Variable(name) => self.context.get_var(name),
            Expr::ArgRef(index) => {
                let arg = self.context.get_arg(*index)?;
                Ok(Value::Str(arg))
            }
            Expr::BinaryOp { left, op, right } => {
                let lval = self.eval_expr(left)?;
                let rval = self.eval_expr(right)?;
                self.apply_binop(op, &lval, &rval)
            }
            Expr::UnaryMinus(inner) => {
                let val = self.eval_expr(inner)?;
                let n = val.to_i64()?;
                Ok(Value::Int(-n))
            }
            Expr::Increment(var_name) => {
                let current = self.context.get_var(var_name)?;
                let n = current.to_i64().map_err(|_| {
                    ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot increment non-numeric variable '{}'", var_name),
                        "", 0, 0,
                    ).with_hint(String::from("Use 'set' to assign a numeric value first"))
                })?;
                let new_val = n + 1;
                self.context.set_var(var_name, Value::Int(new_val));
                Ok(Value::Int(new_val))
            }
            Expr::Decrement(var_name) => {
                let current = self.context.get_var(var_name)?;
                let n = current.to_i64().map_err(|_| {
                    ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot decrement non-numeric variable '{}'", var_name),
                        "", 0, 0,
                    ).with_hint(String::from("Use 'set' to assign a numeric value first"))
                })?;
                let new_val = n - 1;
                self.context.set_var(var_name, Value::Int(new_val));
                Ok(Value::Int(new_val))
            }
        }
    }

    fn apply_binop(&self, op: &BinOp, left: &Value, right: &Value) -> ScriptResult<Value> {
        match op {
            BinOp::And => {
                let l = Self::to_bool(left)?;
                if !l {
                    return Ok(Value::Bool(false));
                }
                let r = Self::to_bool(right)?;
                Ok(Value::Bool(r))
            }
            BinOp::Or => {
                let l = Self::to_bool(left)?;
                if l {
                    return Ok(Value::Bool(true));
                }
                let r = Self::to_bool(right)?;
                Ok(Value::Bool(r))
            }
            BinOp::Eq | BinOp::NotEq | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => {
                // Try numeric comparison first, fall back to string
                let l_int = left.to_i64();
                let r_int = right.to_i64();
                
                if let (Ok(l), Ok(r)) = (l_int, r_int) {
                    let result = match op {
                        BinOp::Eq => l == r,
                        BinOp::NotEq => l != r,
                        BinOp::Lt => l < r,
                        BinOp::Gt => l > r,
                        BinOp::Le => l <= r,
                        BinOp::Ge => l >= r,
                        _ => unreachable!(),
                    };
                    Ok(Value::Bool(result))
                } else {
                    // String comparison
                    let ls = left.to_string();
                    let rs = right.to_string();
                    let result = match op {
                        BinOp::Eq => ls == rs,
                        BinOp::NotEq => ls != rs,
                        BinOp::Lt => ls < rs,
                        BinOp::Gt => ls > rs,
                        BinOp::Le => ls <= rs,
                        BinOp::Ge => ls >= rs,
                        _ => unreachable!(),
                    };
                    Ok(Value::Bool(result))
                }
            }
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod => {
                let l = left.to_i64().map_err(|_| {
                    ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot apply '{}' to non-numeric value '{}'", self.op_name(op), left.to_string()),
                        "", 0, 0,
                    ).with_hint(String::from("Both operands must be integers for arithmetic operations"))
                })?;
                let r = right.to_i64().map_err(|_| {
                    ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot apply '{}' to non-numeric value '{}'", self.op_name(op), right.to_string()),
                        "", 0, 0,
                    ).with_hint(String::from("Both operands must be integers for arithmetic operations"))
                })?;

                match op {
                    BinOp::Add => Ok(Value::Int(l + r)),
                    BinOp::Sub => Ok(Value::Int(l - r)),
                    BinOp::Mul => Ok(Value::Int(l * r)),
                    BinOp::Div => {
                        if r == 0 {
                            Err(ScriptError::new(
                                ScriptErrorKind::RuntimeError,
                                alloc::format!("Division by zero: {} / 0", l),
                                "", 0, 0,
                            ).with_hint(String::from("Check the divisor — it evaluated to 0")))
                        } else {
                            Ok(Value::Int(l / r))
                        }
                    }
                    BinOp::Mod => {
                        if r == 0 {
                            Err(ScriptError::new(
                                ScriptErrorKind::RuntimeError,
                                alloc::format!("Modulo by zero: {} % 0", l),
                                "", 0, 0,
                            ).with_hint(String::from("Check the divisor — it evaluated to 0")))
                        } else {
                            Ok(Value::Int(l % r))
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    fn op_name(&self, op: &BinOp) -> &'static str {
        match op {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Eq => "==",
            BinOp::NotEq => "!=",
            BinOp::Lt => "<",
            BinOp::Gt => ">",
            BinOp::Le => "<=",
            BinOp::Ge => ">=",
            BinOp::And => "&&",
            BinOp::Or => "||",
        }
    }

    fn interpolate_string(&self, input: &str) -> String {
        let mut result = String::new();
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();
        let mut i = 0;

        while i < len {
            if chars[i] == '%' && i + 1 < len && chars[i + 1] == '%' {
                let num_start = i + 2;
                let mut num_end = num_start;
                while num_end < len && chars[num_end].is_ascii_digit() {
                    num_end += 1;
                }
                if num_end > num_start {
                    let num_str: String = chars[num_start..num_end].iter().collect();
                    if let Ok(idx) = num_str.parse::<usize>() {
                        let mapped_idx = if idx == 0 { 0 } else { idx - 1 };
                        match self.context.get_arg(mapped_idx) {
                            Ok(arg_val) => {
                                result.push_str(&arg_val);
                                i = num_end;
                                continue;
                            }
                            Err(_) => {
                                result.push_str("%%");
                                i = num_start;
                                continue;
                            }
                        }
                    }
                }
                result.push('%');
                result.push('%');
                i += 2;
                continue;
            }

            if chars[i].is_alphabetic() || chars[i] == '_' {
                let word_start = i;
                let mut word_end = i;
                while word_end < len && (chars[word_end].is_alphanumeric() || chars[word_end] == '_') {
                    word_end += 1;
                }
                let word: String = chars[word_start..word_end].iter().collect();

                if word != "if" && word != "elif" && word != "else" && word != "for"
                    && word != "while" && word != "set" && word != "echo" && word != "exit"
                    && word != "break" && word != "continue" && word != "true" && word != "false"
                {
                    match self.context.get_var(&word) {
                        Ok(val) => {
                            result.push_str(&val.to_string());
                            i = word_end;
                            continue;
                        }
                        Err(_) => {
                            result.push_str(&word);
                            i = word_end;
                            continue;
                        }
                    }
                } else {
                    result.push_str(&word);
                    i = word_end;
                    continue;
                }
            }

            result.push(chars[i]);
            i += 1;
        }

        result
    }

    fn to_bool(val: &Value) -> ScriptResult<bool> {
        match val {
            Value::Bool(b) => Ok(*b),
            Value::Int(n) => Ok(*n != 0),
            Value::Str(s) => {
                if s.is_empty() {
                    Ok(false)
                } else if s == "true" || s == "1" {
                    Ok(true)
                } else if s == "false" || s == "0" {
                    Ok(false)
                } else {
                    Err(ScriptError::new(
                        ScriptErrorKind::RuntimeError,
                        alloc::format!("Cannot use string '{}' as a boolean condition", s),
                        "", 0, 0,
                    ).with_hint(String::from("Use numeric values (0 = false, non-zero = true) or the keywords true/false")))
                }
            }
        }
    }
}
