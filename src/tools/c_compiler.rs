//! Freestanding C frontend. It normalizes the supported C statements and passes
//! them to the standalone C backend, never to Micro-C's compiler pipeline.

use alloc::{format, string::String, vec::Vec};

const TYPES: &[&str] = &[
    "void", "char", "signed", "unsigned", "short", "int", "long", "bool",
    "size_t", "intptr_t", "uintptr_t", "int8_t", "uint8_t", "int16_t", "uint16_t",
    "int32_t", "uint32_t", "int64_t", "uint64_t",
];

pub(super) fn tokenize(source: &str) -> Result<Vec<String>, &'static str> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() { i += 1; continue; }
        if c == b'#' {
            let start = i;
            while i < bytes.len() && bytes[i] != b'\n' { i += 1; }
            let directive = core::str::from_utf8(&bytes[start..i]).map_err(|_| "source must be UTF-8")?.trim();
            if !directive.starts_with("#include") { return Err("only #include directives are supported; macros and conditionals are not"); }
            continue;
        }
        if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' { i += 1; }
            continue;
        }
        if c == b'/' && bytes.get(i + 1) == Some(&b'*') {
            i += 2;
            while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') { i += 1; }
            if i + 1 >= bytes.len() { return Err("unterminated C block comment"); }
            i += 2;
            continue;
        }
        if c == b'"' { return Err("string literals are not supported by this C frontend yet"); }
        if c == b'\'' {
            i += 1;
            if i >= bytes.len() { return Err("unterminated character constant"); }
            let value = if bytes[i] == b'\\' {
                i += 1;
                let escape = *bytes.get(i).ok_or("unterminated character escape")?;
                i += 1;
                match escape {
                    b'\\' => b'\\' as u32, b'\'' => b'\'' as u32, b'"' => b'"' as u32,
                    b'a' => 7, b'b' => 8, b'f' => 12, b'n' => 10, b'r' => 13, b't' => 9, b'v' => 11, b'?' => 63,
                    b'x' => {
                        let start = i;
                        while i < bytes.len() && bytes[i].is_ascii_hexdigit() { i += 1; }
                        if start == i { return Err("hex character escape requires digits"); }
                        u32::from_str_radix(core::str::from_utf8(&bytes[start..i]).map_err(|_| "invalid character escape")?, 16).map_err(|_| "character escape is out of range")?
                    }
                    b'0'..=b'7' => {
                        let start = i - 1;
                        let mut count = 1;
                        while count < 3 && i < bytes.len() && (b'0'..=b'7').contains(&bytes[i]) { i += 1; count += 1; }
                        u32::from_str_radix(core::str::from_utf8(&bytes[start..i]).unwrap(), 8).map_err(|_| "invalid octal character escape")?
                    }
                    _ => return Err("unknown C character escape"),
                }
            } else {
                let value = bytes[i] as u32;
                if !bytes[i].is_ascii() { return Err("non-ASCII character constants are not supported yet"); }
                i += 1;
                value
            };
            if bytes.get(i) != Some(&b'\'') { return Err("C character constant must contain one character"); }
            i += 1;
            tokens.push(format!("{}", value));
            continue;
        }
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            tokens.push(String::from(core::str::from_utf8(&bytes[start..i]).map_err(|_| "source must be UTF-8")?));
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            i += 1;
            while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') { i += 1; }
            tokens.push(String::from(core::str::from_utf8(&bytes[start..i]).map_err(|_| "source must be UTF-8")?));
            continue;
        }
        if let Some(three) = bytes.get(i..i + 3) {
            if matches!(three, b"<<=" | b">>=" ) {
                tokens.push(String::from(core::str::from_utf8(three).unwrap()));
                i += 3;
                continue;
            }
        }
        if let Some(two) = bytes.get(i..i + 2) {
            if matches!(two, b"==" | b"!=" | b"<=" | b">=" | b"++" | b"--" | b"&&" | b"||" | b"<<" | b">>" | b"+=" | b"-=" | b"*=" | b"/=" | b"%=" | b"&=" | b"|=" | b"^=") {
                tokens.push(String::from(core::str::from_utf8(two).unwrap()));
                i += 2;
                continue;
            }
        }
        if b"{}(),;:?~^:+-*/%=<>!&|[]".contains(&c) {
            tokens.push(String::from(c as char));
            i += 1;
            continue;
        }
        return Err("unsupported token in C source");
    }
    Ok(tokens)
}

fn is_ident(token: &str) -> bool {
    let mut chars = token.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

pub(super) fn is_type(token: &str) -> bool { TYPES.contains(&token) }

fn matching_paren(tokens: &[String], open: usize) -> Result<usize, &'static str> {
    if tokens.get(open).map(String::as_str) != Some("(") { return Err("expected opening parenthesis"); }
    let mut depth = 0usize;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        if token == "(" { depth += 1; }
        if token == ")" {
            depth -= 1;
            if depth == 0 { return Ok(i); }
        }
    }
    Err("unterminated control-flow condition")
}

fn block_body(tokens: &[String], open: usize) -> Result<(&[String], usize), &'static str> {
    if tokens.get(open).map(String::as_str) != Some("{") { return Err("C loop bodies must use braces"); }
    let mut depth = 1usize;
    for i in open + 1..tokens.len() {
        if tokens[i] == "{" { depth += 1; }
        if tokens[i] == "}" {
            depth -= 1;
            if depth == 0 { return Ok((&tokens[open + 1..i], i + 1)); }
        }
    }
    Err("unterminated C loop body")
}

/// Compile the currently supported freestanding C translation unit to
/// Win64-style x86-64 assembly using the standalone C backend.
pub fn compile_to_assembly(source: &str) -> Result<String, &'static str> {
    let tokens = tokenize(source)?;
    if tokens.is_empty() { return Err("C source is empty"); }
    let mut out = String::new();
    if source.lines().any(|line| line.trim() == "#include <HPVMx>") {
        out.push_str("extern fn hpx_ui_resolution_x();\nextern fn hpx_ui_resolution_y();\nextern fn hpx_ui_clear(color);\nextern fn hpx_ui_fill_rect(x, y, dimensions, color);\nextern fn hpx_ui_draw_pixel(x, y, color);\nextern fn hpx_ui_draw_text(x, y, text, color);\nextern fn hpx_fs_read_file(path, out, capacity);\nextern fn hpx_fs_write_file(path, data, length);\nextern fn hpx_fs_make_dir(path);\nextern fn hpx_fs_remove(path);\nextern fn hpx_fs_rename(from, to);\nextern fn hpx_cpu_core_count();\nextern fn hpx_cpu_thread_count();\nextern fn hpx_pci_device_count();\nextern fn hpx_pci_read_u32(bdf, offset);\nextern fn hpx_pci_write_u32(bdf, offset, value);\nextern fn hpx_network_initialize();\nextern fn hpx_network_link_up();\nextern fn hpx_network_transmit(frame, length);\nextern fn hpx_network_receive(out, capacity);\nextern fn hpx_beep(frequency_hz);\nextern fn hpx_mute();\nextern fn hpx_sleep_ms(milliseconds);\nextern fn hpx_alloc(size);\nextern fn hpx_free(ptr, size);\nextern fn hpx_set_global_variable(key, value);\nextern fn hpx_get_global_variable(key, out, capacity);\n");
        out.push_str("extern fn hpx_pack_dimensions(width, height);\n");
    }
    let mut i = 0;
    while i < tokens.len() {
        let mut is_extern = false;
        while matches!(tokens.get(i).map(String::as_str), Some("static" | "inline" | "const" | "volatile")) { i += 1; }
        if tokens.get(i).map(String::as_str) == Some("extern") { is_extern = true; i += 1; }
        while i < tokens.len() && is_type(&tokens[i]) { i += 1; }
        let Some(name) = tokens.get(i) else { break; };
        if !is_ident(name) { return Err("expected a C function declaration"); }
        let name = name.clone();
        i += 1;
        if tokens.get(i).map(String::as_str) != Some("(") { return Err("global variables and typedefs are not supported"); }
        i += 1;
        let mut params = Vec::new();
        let mut param: Vec<String> = Vec::new();
        while i < tokens.len() && tokens[i] != ")" {
            if tokens[i] == "," {
                if let Some(p) = param.iter().rev().find(|p| is_ident(p) && !is_type(p)) { params.push(p.clone()); }
                param.clear();
            } else if tokens[i] == "[" || tokens[i] == "]" {
                return Err("array parameters are not supported by this C subset");
            } else if tokens[i] != "const" {
                param.push(tokens[i].clone());
            }
            i += 1;
        }
        if i >= tokens.len() { return Err("unterminated C function parameter list"); }
        if let Some(p) = param.iter().rev().find(|p| is_ident(p) && !is_type(p)) { params.push(p.clone()); }
        i += 1;
        if tokens.get(i).map(String::as_str) == Some(";") {
            if !is_extern { return Err("function prototypes must be declared extern"); }
            out.push_str(&format!("extern fn {}({});\n", name, params.join(", ")));
            i += 1;
            continue;
        }
        if tokens.get(i).map(String::as_str) != Some("{") { return Err("expected a function body or extern prototype"); }
        i += 1;
        out.push_str(&format!("{} fn {}({}) {{\n", if is_extern { "extern" } else { "export" }, name, params.join(", ")));
        let mut depth = 1usize;
        let mut body = Vec::new();
        while i < tokens.len() && depth > 0 {
            match tokens[i].as_str() {
                "{" => { depth += 1; body.push(tokens[i].clone()); }
                "}" => { depth -= 1; if depth > 0 { body.push(tokens[i].clone()); } }
                _ => body.push(tokens[i].clone()),
            }
            i += 1;
        }
        if depth != 0 { return Err("unterminated C function body"); }
        emit_body(&body, &mut out)?;
        out.push_str("}\n\n");
    }
    if out.is_empty() { return Err("no C function definitions found"); }
    crate::tools::c_backend::compile(&out)
}

/// Emit the C frontend's relocatable object artifact for the HPVMx linker.
pub fn compile_to_object(source: &str) -> Result<Vec<u8>, &'static str> {
    let assembly = compile_to_assembly(source)?;
    crate::tools::c_object::encode(&assembly)
}

fn emit_body(tokens: &[String], out: &mut String) -> Result<(), &'static str> {
    let mut i = 0;
    while i < tokens.len() {
        if is_type(&tokens[i]) || matches!(tokens[i].as_str(), "const" | "volatile") {
            while i < tokens.len() && (is_type(&tokens[i]) || matches!(tokens[i].as_str(), "const" | "volatile")) { i += 1; }
            if tokens.get(i) == Some(&String::from("*")) { return Err("pointer locals are not supported by this C subset"); }
            let Some(name) = tokens.get(i) else { return Err("expected a local variable name"); };
            if !is_ident(name) { return Err("expected a local variable name"); }
            out.push_str(&format!("let {}", name));
            i += 1;
            continue;
        }
        if tokens[i] == "for" {
            let close = matching_paren(tokens, i + 1)?;
            let mut clauses = Vec::new();
            let mut clause_start = i + 2;
            let mut parens = 0usize;
            for j in i + 2..close {
                match tokens[j].as_str() {
                    "(" => parens += 1,
                    ")" => parens -= 1,
                    ";" if parens == 0 => { clauses.push(&tokens[clause_start..j]); clause_start = j + 1; }
                    _ => {}
                }
            }
            clauses.push(&tokens[clause_start..close]);
            if clauses.len() != 3 { return Err("invalid C for-loop header"); }
            let (loop_body, next) = block_body(tokens, close + 1)?;
            if loop_body.iter().any(|token| token == "continue") { return Err("continue inside for-loops is not supported yet"); }
            if !clauses[0].is_empty() {
                let mut init = clauses[0].to_vec();
                init.push(String::from(";"));
                emit_body(&init, out)?;
            }
            let condition = if clauses[1].is_empty() { String::from("1") } else { clauses[1].join(" ") };
            out.push_str(&format!("loop {{ if (({}) == 0) {{ break; }}\n", condition));
            emit_body(loop_body, out)?;
            if !clauses[2].is_empty() {
                let mut update = clauses[2].to_vec();
                update.push(String::from(";"));
                emit_body(&update, out)?;
            }
            out.push_str("}\n");
            i = next;
            continue;
        }
        if tokens[i] == "do" {
            let (loop_body, after_body) = block_body(tokens, i + 1)?;
            if loop_body.iter().any(|token| token == "continue") { return Err("continue inside do-while loops is not supported yet"); }
            if tokens.get(after_body).map(String::as_str) != Some("while") { return Err("expected while after do-loop body"); }
            let open = after_body + 1;
            let close = matching_paren(tokens, open)?;
            if tokens.get(close + 1).map(String::as_str) != Some(";") { return Err("expected semicolon after do-while condition"); }
            let condition = tokens[open + 1..close].join(" ");
            out.push_str("loop {\n");
            emit_body(loop_body, out)?;
            out.push_str(&format!("if (({}) == 0) {{ break; }} }}\n", condition));
            i = close + 2;
            continue;
        }
        if tokens[i] == "while" {
            i += 1;
            if tokens.get(i).map(String::as_str) != Some("(") { return Err("expected condition after while"); }
            i += 1;
            let start = i;
            let mut depth = 1usize;
            while i < tokens.len() && depth > 0 {
                if tokens[i] == "(" { depth += 1; }
                if tokens[i] == ")" { depth -= 1; }
                if depth > 0 { i += 1; }
            }
            if depth != 0 { return Err("unterminated while condition"); }
            let cond = tokens[start..i].join(" ");
            i += 1;
            if tokens.get(i).map(String::as_str) != Some("{") { return Err("while loops require a braced body"); }
            i += 1;
            let body_start = i;
            let mut braces = 1usize;
            while i < tokens.len() && braces > 0 {
                if tokens[i] == "{" { braces += 1; }
                if tokens[i] == "}" { braces -= 1; }
                if braces > 0 { i += 1; }
            }
            if braces != 0 { return Err("unterminated while body"); }
            out.push_str(&format!("loop {{ if ({}) {{\n", cond));
            emit_body(&tokens[body_start..i], out)?;
            out.push_str("} else { break; } }\n");
            i += 1;
            continue;
        }
        if matches!(tokens[i].as_str(), "for" | "do" | "switch" | "case" | "struct" | "typedef") {
            return Err("C for/do loops and aggregates are not yet supported");
        }
        if is_ident(&tokens[i]) && i + 1 < tokens.len() && matches!(tokens[i + 1].as_str(), "++" | "--") {
            let name = tokens[i].clone();
            let op = if tokens[i + 1] == "++" { "+" } else { "-" };
            out.push_str(&format!("{} = {} {} 1 ", name, name, op));
            i += 2;
            continue;
        }
        if (tokens[i] == "++" || tokens[i] == "--") && i + 1 < tokens.len() && is_ident(&tokens[i + 1]) {
            let name = tokens[i + 1].clone();
            let op = if tokens[i] == "++" { "+" } else { "-" };
            out.push_str(&format!("{} = {} {} 1 ", name, name, op));
            i += 2;
            continue;
        }
        out.push_str(&tokens[i]);
        if tokens[i] == ";" { out.push('\n'); } else if tokens[i] == "{" || tokens[i] == "}" { out.push('\n'); } else { out.push(' '); }
        i += 1;
    }
    Ok(())
}
