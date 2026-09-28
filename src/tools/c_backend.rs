//! Standalone scalar C code generator;
use alloc::{boxed::Box, format, string::{String, ToString}, vec::Vec};
use hashbrown::HashMap;

fn err_str(msg: &str) -> String {
    crate::vdebug_autoprefix!(12, "C backend error: {}", msg);
    msg.to_string()
}

#[derive(Clone)]
enum E {
    N(i64),
    V(String),
    Str(String),
    Addr(String),
    Deref(Box<E>),
    Index(Box<E>, Box<E>),
    Member(Box<E>, String),
    PtrMember(Box<E>, String),
    Call(Box<E>, Vec<E>),
    Bin(Box<E>, String, Box<E>),
    Neg(Box<E>),
    Not(Box<E>),
    BitNot(Box<E>),
    Ternary(Box<E>, Box<E>, Box<E>),
}

#[derive(Clone)]
enum S {
    Let(String, E),
    Set(String, E),
    Array(String, usize),
    SetDeref(E, E),
    SetIndex(E, E, E),
    SetMember(E, String, E),
    SetPtrMember(E, String, E),
    Expr(E),
    Ret(Option<E>),
    If(E, Vec<S>, Vec<S>),
    Loop(Vec<S>, Vec<S>),
    Break,
    Continue,
}

struct F {
    name: String,
    args: Vec<String>,
    body: Vec<S>,
}

struct P {
    t: Vec<String>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<&str> { self.t.get(self.i).map(String::as_str) }
    fn pop(&mut self) -> String { let x = self.t.get(self.i).cloned().unwrap_or_default(); self.i += 1; x }
    fn eat(&mut self, s: &str) -> bool { if self.peek() == Some(s) { self.i += 1; true } else { false } }
    fn need(&mut self, s: &str) -> Result<(), String> { if self.eat(s) { Ok(()) } else { Err(err_str("invalid normalized C syntax")) } }
    fn names(&mut self, end: &str) -> Result<Vec<String>, String> {
        let mut v = Vec::new();
        while self.peek() != Some(end) {
            let n = self.pop();
            if n.is_empty() { return Err(err_str("unterminated C parameter list")); }
            v.push(n);
            if !self.eat(",") && self.peek() != Some(end) { return Err(err_str("invalid C parameter list")); }
        }
        Ok(v)
    }
    fn block(&mut self) -> Result<Vec<S>, String> {
        self.need("{")?;
        let mut v = Vec::new();
        while self.peek() != Some("}") {
            if self.peek().is_none() { return Err(err_str("unterminated C body")); }
            v.push(self.stmt()?);
        }
        self.need("}")?;
        Ok(v)
    }
    fn block_or_stmt(&mut self) -> Result<Vec<S>, String> {
        if self.peek() == Some("{") { self.block() } else { Ok(alloc::vec![self.stmt()?]) }
    }
    fn stmt(&mut self) -> Result<S, String> {
        if self.eat("let") {
            let n = self.pop();
            self.need("=")?;
            let e = self.expr(0)?;
            self.need(";")?;
            return Ok(S::Let(n, e));
        }
        if self.eat("let_array") {
            let n = self.pop();
            let length = number(&self.pop()).ok_or_else(|| err_str("invalid local array length"))?;
            if length <= 0 || length > 4096 { return Err(err_str("invalid local array length")); }
            self.need(";")?;
            return Ok(S::Array(n, length as usize));
        }
        if self.eat("return") {
            let e = if self.peek() == Some(";") { None } else { Some(self.expr(0)?) };
            self.need(";")?;
            return Ok(S::Ret(e));
        }
        if self.eat("break") {
            self.need(";")?;
            return Ok(S::Break);
        }
        if self.eat("continue") {
            self.need(";")?;
            return Ok(S::Continue);
        }
        if self.eat("if") {
            self.need("(")?;
            let e = self.expr(0)?;
            self.need(")")?;
            let a = self.block_or_stmt()?;
            let b = if self.eat("else") { self.block_or_stmt()? } else { Vec::new() };
            return Ok(S::If(e, a, b));
        }
        if self.eat("loop_update") {
            let body = self.block()?;
            let update = self.block()?;
            return Ok(S::Loop(body, update));
        }
        if self.eat("loop") {
            return Ok(S::Loop(self.block()?, Vec::new()));
        }

        let start = self.i;
        let lhs_res = self.expr(0);
        if let Ok(lhs) = lhs_res {
            if matches!(self.peek(), Some("=" | "+=" | "-=" | "*=" | "/=" | "%=" | "&=" | "|" | "^=" | "<<=" | ">>=")) {
                let op = self.pop();
                let rhs = self.expr(0)?;
                self.need(";")?;
                let rhs_expr = if op == "=" { rhs } else { E::Bin(Box::new(lhs.clone()), String::from(&op[..op.len() - 1]), Box::new(rhs)) };
                return match lhs {
                    E::V(n) => Ok(S::Set(n, rhs_expr)),
                    E::Deref(target) => Ok(S::SetDeref(*target, rhs_expr)),
                    E::Index(base, index) => Ok(S::SetIndex(*base, *index, rhs_expr)),
                    E::PtrMember(base, field) => Ok(S::SetPtrMember(*base, field, rhs_expr)),
                    E::Member(base, field) => Ok(S::SetMember(*base, field, rhs_expr)),
                    _ => Err(err_str("invalid assignment target")),
                };
            }
        }
        self.i = start;

        let e = self.expr(0)?;
        self.need(";")?;
        Ok(S::Expr(e))
    }

    fn expr(&mut self, min: u8) -> Result<E, String> {
        let t = self.pop();
        let mut a = if t == "(" {
            if super::c_compiler::is_cast_type(self.peek().unwrap_or("")) {
                while self.peek().map(super::c_compiler::is_cast_type).unwrap_or(false) || self.peek() == Some("*") { self.i += 1; }
                self.need(")")?;
                self.expr(7)?
            } else {
                let e = self.expr(0)?;
                self.need(")")?;
                e
            }
        } else if t == "-" {
            E::Neg(Box::new(self.expr(11)?))
        } else if t == "!" {
            E::Not(Box::new(self.expr(7)?))
        } else if t == "~" {
            E::BitNot(Box::new(self.expr(7)?))
        } else if t == "*" {
            E::Deref(Box::new(self.expr(11)?))
        } else if t == "&" {
            let n = self.pop();
            if !ident(&n) { return Err(err_str("address-of requires a local variable")); }
            E::Addr(n)
        } else if t == "+" {
            self.expr(7)?
        } else if let Some(n) = number(&t) {
            E::N(n)
        } else if t.starts_with('"') {
            E::Str(t)
        } else if self.eat("(") {
            let mut args = Vec::new();
            while self.peek() != Some(")") {
                args.push(self.expr(0)?);
                if !self.eat(",") && self.peek() != Some(")") { return Err(err_str("invalid call args")); }
            }
            self.need(")")?;
            E::Call(Box::new(E::V(t)), args)
        } else if ident(&t) {
            E::V(t)
        } else {
            return Err(err_str(&format!("unsupported C expression {t}")));
        };

        loop {
            if self.eat("[") {
                let index = self.expr(0)?;
                self.need("]")?;
                a = E::Index(Box::new(a), Box::new(index));
                continue;
            }
            if self.eat(".") {
                let field = self.pop();
                if !ident(&field) { return Err(err_str("expected field name after .")); }
                a = E::Member(Box::new(a), field);
                continue;
            }
            if self.eat("->") {
                let field = self.pop();
                if !ident(&field) { return Err(err_str("expected field name after ->")); }
                a = E::PtrMember(Box::new(a), field);
                continue;
            }
            let Some(op) = self.peek() else { break; };
            let p = prec(op);
            if p == 0 || p < min { break; }
            if op == "?" {
                self.pop();
                let yes = self.expr(0)?;
                self.need(":")?;
                let no = self.expr(p)?;
                a = E::Ternary(Box::new(a), Box::new(yes), Box::new(no));
                continue;
            }
            let op = self.pop();
            let b = self.expr(p + 1)?;
            a = E::Bin(Box::new(a), op, Box::new(b));
        }
        Ok(a)
    }

    fn program(mut self) -> Result<(Vec<F>, Vec<String>), String> {
        let mut fs = Vec::new();
        let mut ex = Vec::new();
        while self.peek().is_some() {
            let external = self.eat("extern");
            if !external { self.eat("export"); }
            self.need("fn")?;
            let name = self.pop();
            self.need("(")?;
            let args = self.names(")")?;
            self.need(")")?;
            if external {
                self.need(";")?;
                ex.push(name);
            } else {
                fs.push(F { name, args, body: self.block()? });
            }
        }
        Ok((fs, ex))
    }
}

fn number(s: &str) -> Option<i64> {
    let s = s.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        i64::from_str_radix(h, 16).ok()
    } else if let Some(b) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        i64::from_str_radix(b, 2).ok()
    } else if s.len() > 1 && s.starts_with('0') {
        i64::from_str_radix(&s[1..], 8).ok()
    } else {
        s.parse().ok()
    }
}

fn ident(s: &str) -> bool {
    let mut c = s.chars();
    matches!(c.next(), Some(x) if x.is_ascii_alphabetic() || x == '_') && c.all(|x| x.is_ascii_alphanumeric() || x == '_')
}

fn prec(s: &str) -> u8 {
    match s {
        "?" => 2,
        "||" => 1,
        "&&" => 2,
        "|" => 3,
        "^" => 4,
        "&" => 5,
        "==" | "!=" => 6,
        "<" | "<=" | ">" | ">=" => 7,
        "<<" | ">>" => 8,
        "+" | "-" => 9,
        "*" | "/" | "%" => 10,
        _ => 0,
    }
}

fn field_offset(field: &str) -> i32 {
    match field {
        "frames" => 0,
        "color" => 8,
        "notice" => 12,
        "abi_version" => 0,
        "draw_text" => 8,
        "fill_rect" => 16,
        "read_file" => 24,
        "write_file" => 32,
        "allocate" => 40,
        "deallocate" => 48,
        "file_make_dir" => 56,
        "file_remove" => 64,
        "file_rename" => 72,
        "current_directory" => 80,
        "set_global_variable" => 88,
        "get_global_variable" => 96,
        "cpu_info" => 104,
        "pci_device_count" => 112,
        "pci_get_device" => 120,
        "pci_read_u32" => 128,
        "pci_write_u32" => 136,
        "network_initialize" => 144,
        "network_link_up" => 152,
        "network_transmit" => 160,
        "network_receive" => 168,
        "beep" => 176,
        "play_tone" => 184,
        "mute" => 192,
        "sleep_ms" => 200,
        "vendor" => 0,
        "brand" => 13,
        "cores" => 62,
        "threads" => 66,
        "ap_count" => 70,
        "feature_flags" => 74,
        _ => 0,
    }
}

fn is_32bit_field(field: &str) -> bool {
    matches!(field, "color" | "notice" | "abi_version" | "cores" | "threads" | "ap_count" | "feature_flags")
}

fn collect(s: &[S], v: &mut Vec<String>, arrays: &mut HashMap<String, usize>) {
    for x in s {
        match x {
            S::Let(n, _) => if !v.contains(n) { v.push(n.clone()); },
            S::Array(n, len) => { if !v.contains(n) { v.push(n.clone()); arrays.insert(n.clone(), *len); } },
            S::If(_, a, b) => { collect(a, v, arrays); collect(b, v, arrays); },
            S::Loop(a, u) => { collect(a, v, arrays); collect(u, v, arrays); },
            _ => (),
        }
    }
}

struct G {
    a: String,
    slots: HashMap<String, i32>,
    arrays: HashMap<String, usize>,
    label: u32,
    loops: Vec<(String, String)>,
    strings: Vec<(String, String)>,
}

impl G {
    fn lab(&mut self) -> String { let s = format!("CL{}", self.label); self.label += 1; s }

    fn add_string(&mut self, s: &str) -> String {
        for (lbl, val) in &self.strings {
            if val == s { return lbl.clone(); }
        }
        let lbl = format!("__str_{}", self.strings.len());
        self.strings.push((lbl.clone(), s.to_string()));
        lbl
    }

    fn address(&mut self, e: &E) -> Result<(), String> {
        match e {
            E::V(n) if self.arrays.contains_key(n) => {
                let o = *self.slots.get(n).ok_or_else(|| err_str("unknown local array"))?;
                self.a.push_str(&format!("    mov rax, rbp\n    sub rax, {}\n", o));
            }
            E::Index(base, index) => {
                self.e(base)?;
                self.a.push_str("    push rax\n");
                self.e(index)?;
                self.a.push_str("    mov rcx, 8\n    imul rax, rcx\n    pop rcx\n    add rax, rcx\n");
            }
            E::PtrMember(base, field) => {
                self.e(base)?;
                let off = field_offset(field);
                if off != 0 {
                    self.a.push_str(&format!("    add rax, {}\n", off));
                }
            }
            E::Member(base, field) => {
                self.address(base)?;
                let off = field_offset(field);
                if off != 0 {
                    self.a.push_str(&format!("    add rax, {}\n", off));
                }
            }
            E::Deref(ptr) => self.e(ptr)?,
            E::V(n) => {
                let o = *self.slots.get(n).ok_or_else(|| err_str("unknown local"))?;
                self.a.push_str(&format!("    mov rax, rbp\n    sub rax, {}\n", o));
            }
            _ => return Err(err_str("expression is not an assignable C object")),
        }
        Ok(())
    }

    fn e(&mut self, e: &E) -> Result<(), String> {
        match e {
            E::N(n) => self.a.push_str(&format!("    mov rax, {}\n", n)),
            E::Str(s) => {
                let lbl = self.add_string(s);
                self.a.push_str(&format!("    lea rax, [{}]\n", lbl));
            }
            E::V(n) if self.arrays.contains_key(n) => self.address(e)?,
            E::V(n) => {
                let o = *self.slots.get(n).ok_or("unknown local")?;
                self.a.push_str(&format!("    mov rax, [rbp-{}]\n", o));
            }
            E::Addr(n) => {
                let o = *self.slots.get(n).ok_or("unknown local")?;
                self.a.push_str(&format!("    mov rax, rbp\n    sub rax, {}\n", o));
            }
            E::Deref(x) => {
                self.e(x)?;
                self.a.push_str("    mov rax, [rax]\n");
            }
            E::Index(_, _) | E::Member(_, _) | E::PtrMember(_, _) => {
                let is_32 = match e {
                    E::Member(_, f) | E::PtrMember(_, f) => is_32bit_field(f),
                    _ => false,
                };
                self.address(e)?;
                if is_32 {
                    self.a.push_str("    movsxd rax, dword [rax]\n");
                } else {
                    self.a.push_str("    mov rax, [rax]\n");
                }
            }
            E::Neg(x) => {
                self.e(x)?;
                self.a.push_str("    mov rcx, 0\n    sub rcx, rax\n    mov rax, rcx\n");
            }
            E::Not(x) => {
                self.e(x)?;
                self.a.push_str("    cmp rax, 0\n    sete al\n    movzx rax, al\n");
            }
            E::BitNot(x) => {
                self.e(x)?;
                self.a.push_str("    not rax\n");
            }
            E::Ternary(c, y, n) => {
                let els = self.lab();
                let end = self.lab();
                self.e(c)?;
                self.a.push_str(&format!("    cmp rax, 0\n    je {}\n", els));
                self.e(y)?;
                self.a.push_str(&format!("    jmp {}\n{}:\n", end, els));
                self.e(n)?;
                self.a.push_str(&format!("{}:\n", end));
            }
            E::Bin(l, op, r) if op == "&&" || op == "||" => {
                let short = self.lab();
                let end = self.lab();
                self.e(l)?;
                self.a.push_str("    cmp rax, 0\n");
                self.a.push_str(&format!("    {} {}\n", if op == "&&" { "je" } else { "jne" }, short));
                self.e(r)?;
                self.a.push_str("    cmp rax, 0\n    setne al\n    movzx rax, al\n");
                self.a.push_str(&format!("    jmp {}\n{}:\n    mov rax, {}\n{}:\n", end, short, if op == "&&" { 0 } else { 1 }, end));
            }
            E::Bin(l, op, r) => {
                self.e(l)?;
                self.a.push_str("    push rax\n");
                self.e(r)?;
                self.a.push_str("    mov rcx, rax\n    pop rax\n");
                match op.as_str() {
                    "+" => self.a.push_str("    add rax, rcx\n"),
                    "-" => self.a.push_str("    sub rax, rcx\n"),
                    "*" => self.a.push_str("    imul rax, rcx\n"),
                    "/" => self.a.push_str("    cqo\n    idiv rcx\n"),
                    "%" => self.a.push_str("    cqo\n    idiv rcx\n    mov rax, rdx\n"),
                    "&" => self.a.push_str("    and rax, rcx\n"),
                    "|" => self.a.push_str("    or rax, rcx\n"),
                    "^" => self.a.push_str("    xor rax, rcx\n"),
                    "<<" => self.a.push_str("    shl rax, cl\n"),
                    ">>" => self.a.push_str("    sar rax, cl\n"),
                    "==" | "!=" | "<" | "<=" | ">" | ">=" => {
                        self.a.push_str("    cmp rax, rcx\n");
                        let c = match op.as_str() {
                            "==" => "sete",
                            "!=" => "setne",
                            "<" => "setl",
                            "<=" => "setle",
                            ">" => "setg",
                            _ => "setge",
                        };
                        self.a.push_str(&format!("    {} al\n    movzx rax, al\n", c));
                    }
                    _ => return Err(err_str(&format!("unsupported C operator {op}"))),
                }
            }
            E::Call(callee, args) => {
                if args.len() > 4 { return Err(err_str("calls support up to four arguments")); }
                for x in args {
                    self.e(x)?;
                    self.a.push_str("    push rax\n");
                }
                let regs = ["rcx", "rdx", "r8", "r9"];
                for i in (0..args.len()).rev() {
                    self.a.push_str(&format!("    pop {}\n", regs[i]));
                }
                match callee.as_ref() {
                    E::V(n) => self.a.push_str(&format!("    call {}\n", n)),
                    _ => {
                        self.e(callee)?;
                        self.a.push_str("    call rax\n");
                    }
                }
            }
        }
        Ok(())
    }

    fn ss(&mut self, s: &[S]) -> Result<(), String> {
        for x in s {
            match x {
                S::Let(n, e) | S::Set(n, e) => {
                    self.e(e)?;
                    let o = *self.slots.get(n).ok_or_else(|| err_str("unknown C local"))?;
                    self.a.push_str(&format!("    mov [rbp-{}], rax\n", o));
                }
                S::Array(_, _) => {}
                S::SetDeref(p, e) => {
                    self.e(p)?;
                    self.a.push_str("    push rax\n");
                    self.e(e)?;
                    self.a.push_str("    mov rcx, rax\n    pop rax\n    mov [rax], rcx\n");
                }
                S::SetIndex(base, index, e) => {
                    self.address(&E::Index(Box::new(base.clone()), Box::new(index.clone())))?;
                    self.a.push_str("    push rax\n");
                    self.e(e)?;
                    self.a.push_str("    mov rcx, rax\n    pop rax\n    mov [rax], rcx\n");
                }
                S::SetPtrMember(base, field, e) => {
                    self.address(&E::PtrMember(Box::new(base.clone()), field.clone()))?;
                    self.a.push_str("    push rax\n");
                    self.e(e)?;
                    self.a.push_str("    mov rcx, rax\n    pop rax\n");
                    if is_32bit_field(field) {
                        self.a.push_str("    mov dword [rax], ecx\n");
                    } else {
                        self.a.push_str("    mov [rax], rcx\n");
                    }
                }
                S::SetMember(base, field, e) => {
                    self.address(&E::Member(Box::new(base.clone()), field.clone()))?;
                    self.a.push_str("    push rax\n");
                    self.e(e)?;
                    self.a.push_str("    mov rcx, rax\n    pop rax\n");
                    if is_32bit_field(field) {
                        self.a.push_str("    mov dword [rax], ecx\n");
                    } else {
                        self.a.push_str("    mov [rax], rcx\n");
                    }
                }
                S::Expr(e) => self.e(e)?,
                S::Ret(e) => {
                    if let Some(e) = e { self.e(e)?; }
                    self.a.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n");
                }
                S::If(c, y, n) => {
                    let els = self.lab();
                    let end = self.lab();
                    self.e(c)?;
                    self.a.push_str(&format!("    cmp rax, 0\n    je {}\n", els));
                    self.ss(y)?;
                    self.a.push_str(&format!("    jmp {}\n{}:\n", end, els));
                    self.ss(n)?;
                    self.a.push_str(&format!("{}:\n", end));
                }
                S::Loop(b, u) => {
                    let st = self.lab();
                    let cont = self.lab();
                    let en = self.lab();
                    self.loops.push((cont.clone(), en.clone()));
                    self.a.push_str(&format!("{}:\n", st));
                    self.ss(b)?;
                    self.a.push_str(&format!("{}:\n", cont));
                    self.ss(u)?;
                    self.a.push_str(&format!("    jmp {}\n{}:\n", st, en));
                    self.loops.pop();
                }
                S::Break => {
                    let e = self.loops.last().ok_or_else(|| err_str("break outside loop"))?.1.clone();
                    self.a.push_str(&format!("    jmp {}\n", e));
                }
                S::Continue => {
                    let st = self.loops.last().ok_or_else(|| err_str("continue outside loop"))?.0.clone();
                    self.a.push_str(&format!("    jmp {}\n", st));
                }
            }
        }
        Ok(())
    }
}

pub fn compile(source: &str) -> Result<String, String> {
    let t = super::c_compiler::tokenize(source)?;
    let (fs, ex) = (P { t, i: 0 }).program()?;
    if fs.is_empty() { return Err(err_str("no C functions found")); }
    let mut asm = String::from("; standalone HPVMx C backend\nBITS 64\nORG 0x100000\n");
    for n in ex { asm.push_str(&format!("extern {}\n", n)); }
    let mut next_label = 0;
    let mut all_strings = Vec::new();
    for f in fs {
        let mut ns = f.args.clone();
        let mut arrays = HashMap::new();
        collect(&f.body, &mut ns, &mut arrays);
        let mut slots = HashMap::new();
        let mut used = 0i32;
        for n in &f.args { used += 8; slots.insert(n.clone(), used); }
        for n in ns.iter().skip(f.args.len()) {
            let size = arrays.get(n).copied().unwrap_or(1).min(4096) as i32 * 8;
            used += size;
            slots.insert(n.clone(), used);
        }
        let frame = (((used + 15) / 16) * 16 + 16);
        asm.push_str(&format!("{}:\n    push rbp\n    mov rbp, rsp\n    sub rsp, {}\n", f.name, frame));
        let regs = ["rcx", "rdx", "r8", "r9"];
        for (i, n) in f.args.iter().enumerate() {
            if i >= 4 { return Err(err_str("functions support up to four arguments")); }
            asm.push_str(&format!("    mov [rbp-{}], {}\n", slots[n], regs[i]));
        }
        let mut g = G { a: asm, slots, arrays, label: next_label, loops: Vec::new(), strings: Vec::new() };
        g.ss(&f.body)?;
        next_label = g.label;
        g.a.push_str("    mov rsp, rbp\n    pop rbp\n    ret\n\n");
        asm = g.a;
        for s in g.strings {
            if !all_strings.contains(&s) { all_strings.push(s); }
        }
    }
    if !all_strings.is_empty() {
        asm.push_str("\nsection .data\n");
        for (lbl, val) in all_strings {
            asm.push_str(&format!("{}: db {}, 0\n", lbl, val));
        }
    }
    Ok(asm)
}
