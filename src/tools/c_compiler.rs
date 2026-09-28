//! Standalone freestanding C compiler frontend translating C source files
//! into relocatable HPVMx object files (`.o`) via the standalone C backend.

use alloc::{format, string::{String, ToString}, vec::Vec};
use hashbrown::HashMap;
use crate::vdebug_autoprefix;

fn err_str(msg: &str) -> String {
    vdebug_autoprefix!(12, "C compiler error: {}", msg);
    msg.to_string()
}

const TYPES: &[&str] = &[
    "void", "char", "short", "int", "long", "signed", "unsigned",
    "float", "double", "_Bool", "size_t", "uint8_t", "int8_t",
    "uint16_t", "int16_t", "uint32_t", "int32_t", "uint64_t", "int64_t",
    "HpxHostApi", "CounterState", "HpxCpuInfo", "HpxPciDeviceInfo",
];

pub fn is_type(token: &str) -> bool {
    TYPES.contains(&token) || token.ends_with_ext("_t") || token.starts_with("Hpx")
}

pub fn is_cast_type(token: &str) -> bool {
    is_type(token) || matches!(token, "const" | "volatile" | "restrict" | "struct" | "*")
}

trait Ext {
    fn ends_with_ext(&self, s: &str) -> bool;
}
impl Ext for str {
    fn ends_with_ext(&self, s: &str) -> bool { self.ends_with(s) }
}

pub fn tokenize(source: &str) -> Result<Vec<String>, String> {
    let bytes = source.as_bytes();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
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
            if i + 1 >= bytes.len() { return Err("unterminated C block comment".parse().unwrap()); }
            i += 2;
            continue;
        }
        if c == b'"' {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i] != b'"' {
                if bytes[i] == b'\\' && i + 1 < bytes.len() { i += 2; } else { i += 1; }
            }
            if i >= bytes.len() { return Err("unterminated string literal".parse().unwrap()); }
            i += 1;
            let s = core::str::from_utf8(&bytes[start..i]).map_err(|_| "string literal must be UTF-8")?;
            tokens.push(String::from(s));
            continue;
        }
        if c == b'\'' {
            i += 1;
            if i >= bytes.len() { return Err("unterminated character constant".parse().unwrap()); }
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
                        if start == i { return Err("hex character escape requires digits".parse().unwrap()); }
                        u32::from_str_radix(core::str::from_utf8(&bytes[start..i]).map_err(|_| "invalid character escape")?, 16).map_err(|_| "character escape is out of range")?
                    }
                    b'0'..=b'7' => {
                        let start = i - 1;
                        let mut count = 1;
                        while count < 3 && i < bytes.len() && (b'0'..=b'7').contains(&bytes[i]) { i += 1; count += 1; }
                        u32::from_str_radix(core::str::from_utf8(&bytes[start..i]).unwrap(), 8).map_err(|_| "invalid octal character escape")?
                    }
                    _ => return Err("unknown C character escape".parse().unwrap()),
                }
            } else {
                let value = bytes[i] as u32;
                if !bytes[i].is_ascii() { return Err("non-ASCII character constants are not supported yet".parse().unwrap()); }
                i += 1;
                value
            };
            if bytes.get(i) != Some(&b'\'') { return Err("C character constant must contain one character".parse().unwrap()); }
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
            if matches!(two, b"==" | b"!=" | b"<=" | b">=" | b"++" | b"--" | b"&&" | b"||" | b"<<" | b">>" | b"+=" | b"-=" | b"*=" | b"/=" | b"%=" | b"&=" | b"|=" | b"^=" | b"->") {
                tokens.push(String::from(core::str::from_utf8(two).unwrap()));
                i += 2;
                continue;
            }
        }
        if b"{}(),;:?~^:+-*/%=<>!&|[].".contains(&c) {
            tokens.push(String::from(c as char));
            i += 1;
            continue;
        }
        return Err(format!("unsupported token in C source {c}"));
    }
    Ok(tokens)
}

enum Macro { Object(Vec<String>), Function(Vec<String>, Vec<String>) }

fn expand_tokens(tokens: &[String], macros: &HashMap<String, Macro>, depth: u8, out: &mut Vec<String>) -> Result<(), &'static str> {
    if depth >= 32 { return Err("C macro expansion exceeded the maximum nesting depth"); }
    let mut i = 0;
    while i < tokens.len() {
        let Some(definition) = macros.get(tokens[i].as_str()) else { out.push(tokens[i].clone()); i += 1; continue; };
        match definition {
            Macro::Object(replacement) => { expand_tokens(replacement, macros, depth + 1, out)?; i += 1; }
            Macro::Function(params, replacement) if tokens.get(i + 1).map(String::as_str) == Some("(") => {
                let mut args: Vec<Vec<String>> = Vec::new();
                let mut arg = Vec::new();
                let mut nesting = 0usize;
                let mut j = i + 2;
                let mut closed = false;
                while j < tokens.len() {
                    match tokens[j].as_str() {
                        "(" | "[" | "{" => { nesting += 1; arg.push(tokens[j].clone()); }
                        ")" if nesting == 0 => { if !arg.is_empty() || !args.is_empty() || !params.is_empty() { args.push(arg); } closed = true; j += 1; break; }
                        ")" | "]" | "}" => { nesting = nesting.saturating_sub(1); arg.push(tokens[j].clone()); }
                        "," if nesting == 0 => { args.push(core::mem::take(&mut arg)); }
                        _ => arg.push(tokens[j].clone()),
                    }
                    j += 1;
                }
                if !closed { return Err("unterminated function-like macro invocation"); }
                if args.len() != params.len() { return Err("wrong number of arguments to C macro"); }
                let mut substituted = Vec::new();
                for item in replacement {
                    if let Some(index) = params.iter().position(|p| p == item) { substituted.extend_from_slice(&args[index]); }
                    else { substituted.push(item.clone()); }
                }
                expand_tokens(&substituted, macros, depth + 1, out)?;
                i = j;
            }
            Macro::Function(_, _) => { out.push(tokens[i].clone()); i += 1; }
        }
    }
    Ok(())
}

fn preprocess(source: &str) -> Result<Vec<String>, String> {
    let mut macros: HashMap<String, Macro> = HashMap::new();
    let mut output = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with('#') {
            expand_tokens(&tokenize(line)?, &macros, 0, &mut output)?;
            continue;
        }
        let directive = trimmed.trim();
        if directive.starts_with("#include") { continue; }
        if let Some(rest) = directive.strip_prefix("#define") {
            let rest = rest.trim_start();
            let name_end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(rest.len());
            if name_end == 0 { return Err("#define requires a macro name".parse().unwrap()); }
            let name = &rest[..name_end];
            if !is_ident(name) { return Err(format!("invalid C macro name \"{name}\"")); }
            if macros.contains_key(name) { return Err(format!("duplicate C macro definition \"{name}\"")); }
            if rest[name_end..].starts_with('(') {
                let close = rest[name_end..].find(')').ok_or("unterminated function-like macro parameters")? + name_end;
                let params_text = &rest[name_end + 1..close];
                let mut params = Vec::new();
                if !params_text.trim().is_empty() {
                    for param in params_text.split(',') {
                        let param = param.trim();
                        if !is_ident(param) || params.iter().any(|p| p == param) { return Err(format!("invalid function-like macro parameter \"{param}\"")); }
                        params.push(String::from(param));
                    }
                }
                let body = tokenize(rest[close + 1..].trim())?;
                if body.iter().any(|t| t == "#") { return Err("macro stringification and token pasting are not supported yet".parse().unwrap()); }
                macros.insert(String::from(name), Macro::Function(params, body));
            } else {
                macros.insert(String::from(name), Macro::Object(tokenize(rest[name_end..].trim())?));
            }
            continue;
        }
        if let Some(rest) = directive.strip_prefix("#undef") {
            let name = rest.trim();
            if !is_ident(name) { return Err("#undef requires a macro name".parse().unwrap()); }
            macros.remove(name);
            continue;
        }
        // ignore other preprocessor directives like #ifndef, #endif
        continue;
    }
    Ok(output)
}

fn is_ident(token: &str) -> bool {
    let mut chars = token.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

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

pub fn compile_to_assembly(source: &str) -> Result<String, String> {
    let tokens = preprocess(source)?;
    if tokens.is_empty() { return Err("C source is empty".parse().unwrap()); }
    let mut out = String::new();
    if source.lines().any(|line| line.trim() == "#include <HPVMx>") {
        out.push_str("extern fn hpx_ui_resolution_x();\nextern fn hpx_ui_resolution_y();\nextern fn hpx_ui_clear(color);\nextern fn hpx_ui_fill_rect(x, y, dimensions, color);\nextern fn hpx_ui_draw_pixel(x, y, color);\nextern fn hpx_ui_draw_text(x, y, text, color);\nextern fn hpx_fs_read_file(path, out, capacity);\nextern fn hpx_fs_write_file(path, data, length);\nextern fn hpx_fs_make_dir(path);\nextern fn hpx_fs_remove(path);\nextern fn hpx_fs_rename(from, to);\nextern fn hpx_cpu_core_count();\nextern fn hpx_cpu_thread_count();\nextern fn hpx_pci_device_count();\nextern fn hpx_pci_read_u32(bdf, offset);\nextern fn hpx_pci_write_u32(bdf, offset, value);\nextern fn hpx_network_initialize();\nextern fn hpx_network_link_up();\nextern fn hpx_network_transmit(frame, length);\nextern fn hpx_network_receive(out, capacity);\nextern fn hpx_beep(frequency_hz);\nextern fn hpx_mute();\nextern fn hpx_sleep_ms(milliseconds);\nextern fn hpx_alloc(size);\nextern fn hpx_free(ptr, size);\nextern fn hpx_set_global_variable(key, value);\nextern fn hpx_get_global_variable(key, out, capacity);\n");
        out.push_str("extern fn hpx_pack_dimensions(width, height);\n");
    }
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "typedef" {
            i += 1;
            while i < tokens.len() && tokens[i] != ";" { i += 1; }
            if i < tokens.len() { i += 1; }
            continue;
        }
        if tokens[i] == "struct" {
            i += 1;
            if i < tokens.len() && is_ident(&tokens[i]) { i += 1; }
            if tokens.get(i).map(String::as_str) == Some("{") {
                let mut depth = 1usize;
                i += 1;
                while i < tokens.len() && depth > 0 {
                    if tokens[i] == "{" { depth += 1; }
                    else if tokens[i] == "}" { depth -= 1; }
                    i += 1;
                }
            }
            while i < tokens.len() && tokens[i] != ";" { i += 1; }
            if i < tokens.len() { i += 1; }
            continue;
        }

        let mut is_extern = false;
        while matches!(tokens.get(i).map(String::as_str), Some("static" | "inline" | "const" | "volatile")) { i += 1; }
        if tokens.get(i).map(String::as_str) == Some("extern") { is_extern = true; i += 1; }
        while i < tokens.len() && (is_type(&tokens[i]) || tokens[i] == "struct" || tokens[i] == "*") { i += 1; }
        let Some(name) = tokens.get(i) else { break; };
        if !is_ident(name) {
            i += 1;
            while i < tokens.len() && tokens[i] != ";" { i += 1; }
            if i < tokens.len() { i += 1; }
            continue;
        }
        let name = name.clone();
        i += 1;
        if tokens.get(i).map(String::as_str) != Some("(") {
            while i < tokens.len() && tokens[i] != ";" { i += 1; }
            if i < tokens.len() { i += 1; }
            continue;
        }
        i += 1;
        let mut params = Vec::new();
        let mut param: Vec<String> = Vec::new();
        while i < tokens.len() && tokens[i] != ")" {
            if tokens[i] == "," {
                if let Some(p) = param.iter().rev().find(|p| is_ident(p) && !is_type(p) && *p != "struct") { params.push(p.clone()); }
                param.clear();
            } else if tokens[i] == "[" || tokens[i] == "]" {
                return Err("array parameters are not supported by this C subset".parse().unwrap());
            } else if tokens[i] != "const" {
                param.push(tokens[i].clone());
            }
            i += 1;
        }
        if i >= tokens.len() { return Err("unterminated C function parameter list".parse().unwrap()); }
        if let Some(p) = param.iter().rev().find(|p| is_ident(p) && !is_type(p) && *p != "struct") { params.push(p.clone()); }
        i += 1;
        if tokens.get(i).map(String::as_str) == Some(";") {
            if is_extern {
                out.push_str(&format!("extern fn {}({});\n", name, params.join(", ")));
            }
            i += 1;
            continue;
        }
        if tokens.get(i).map(String::as_str) != Some("{") { return Err("expected a function body or extern prototype".parse().unwrap()); }
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
        if depth != 0 { return Err("unterminated C function body".parse().unwrap()); }
        emit_body(&body, &mut out)?;
        out.push_str("}\n\n");
    }
    if out.is_empty() { return Err("no C function definitions found".parse().unwrap()); }
    crate::tools::c_backend::compile(&out)
}

pub fn compile_to_object(source: &str) -> Result<Vec<u8>, String> {
    let assembly = compile_to_assembly(source)?;
    crate::tools::c_object::encode(&assembly)
}

fn emit_body(tokens: &[String], out: &mut String) -> Result<(), String> {
    let mut norm = normalize_body(tokens)?;
    let mut i = 0;
    while i < norm.len() {
        if is_type(&norm[i]) || norm[i] == "struct" || matches!(norm[i].as_str(), "const" | "volatile") {
            while i < norm.len() && (is_type(&norm[i]) || norm[i] == "struct" || matches!(norm[i].as_str(), "const" | "volatile")) { i += 1; }
            while norm.get(i).map(String::as_str) == Some("*") { i += 1; }
            let Some(name) = norm.get(i) else { return Err("expected a local variable name".parse().unwrap()); };
            if !is_ident(name) { return Err("expected a local variable name".parse().unwrap()); }
            let name = name.clone();
            i += 1;
            if norm.get(i).map(String::as_str) == Some("[") {
                let size_token = norm.get(i + 1).ok_or("array declaration requires a fixed size")?;
                let length = integer_literal(size_token).ok_or("local array size must be an integer constant")?;
                if length == 0 || length > 4096 { return Err("local array size must be between 1 and 4096 elements".parse().unwrap()); }
                if norm.get(i + 2).map(String::as_str) != Some("]") { return Err("only one-dimensional fixed-size arrays are supported yet".parse().unwrap()); }
                out.push_str(&format!("let_array {} {};\n", name, length));
                i += 3;
                if norm.get(i).map(String::as_str) == Some("=") { return Err("array initializers are not supported yet".parse().unwrap()); }
                if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
                continue;
            }
            if norm.get(i).map(String::as_str) == Some("=") {
                i += 1;
                let (expr_tokens, next_idx) = parse_statement_expr(&norm, i)?;
                out.push_str(&format!("let {} = {};\n", name, translate_expr(&expr_tokens)?));
                i = next_idx;
                if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
                continue;
            }
            if norm.get(i).map(String::as_str) == Some(";") {
                out.push_str(&format!("let {} = 0;\n", name));
                i += 1;
                continue;
            }
        }
        if norm[i] == "return" {
            i += 1;
            if norm.get(i).map(String::as_str) == Some(";") {
                out.push_str("return;\n");
                i += 1;
            } else {
                let (expr_tokens, next_idx) = parse_statement_expr(&norm, i)?;
                out.push_str(&format!("return {};\n", translate_expr(&expr_tokens)?));
                i = next_idx;
                if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
            }
            continue;
        }
        if norm[i] == "if" {
            i += 1;
            if norm.get(i).map(String::as_str) != Some("(") { return Err("expected '(' after if".parse().unwrap()); }
            let cond_end = matching_paren(&norm, i)?;
            let cond_tokens = &norm[i + 1..cond_end];
            out.push_str(&format!("if ({}) ", translate_expr(cond_tokens)?));
            i = cond_end + 1;
            let (then_body, next_idx) = parse_sub_statement(&norm, i)?;
            out.push_str("{\n");
            emit_body(&then_body, out)?;
            out.push_str("}\n");
            i = next_idx;
            if norm.get(i).map(String::as_str) == Some("else") {
                i += 1;
                let (else_body, next_idx) = parse_sub_statement(&norm, i)?;
                out.push_str("else {\n");
                emit_body(&else_body, out)?;
                out.push_str("}\n");
                i = next_idx;
            }
            continue;
        }
        if norm[i] == "while" {
            i += 1;
            if norm.get(i).map(String::as_str) != Some("(") { return Err("expected '(' after while".parse().unwrap()); }
            let cond_end = matching_paren(&norm, i)?;
            let cond_tokens = &norm[i + 1..cond_end];
            out.push_str(&format!("if (!({})) break;\nloop {{\n", translate_expr(cond_tokens)?));
            i = cond_end + 1;
            let (loop_body, next_idx) = parse_sub_statement(&norm, i)?;
            emit_body(&loop_body, out)?;
            out.push_str(&format!("if (!({})) break;\n}}\n", translate_expr(cond_tokens)?));
            i = next_idx;
            continue;
        }
        if norm[i] == "do" {
            i += 1;
            let (loop_body, next_idx) = parse_sub_statement(&norm, i)?;
            i = next_idx;
            if norm.get(i).map(String::as_str) != Some("while") { return Err("expected while after do loop".parse().unwrap()); }
            i += 1;
            if norm.get(i).map(String::as_str) != Some("(") { return Err("expected '(' after do-while".parse().unwrap()); }
            let cond_end = matching_paren(&norm, i)?;
            let cond_tokens = &norm[i + 1..cond_end];
            out.push_str("loop {{\n");
            emit_body(&loop_body, out)?;
            out.push_str(&format!("if (!({})) break;\n}}\n", translate_expr(cond_tokens)?));
            i = cond_end + 1;
            if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
            continue;
        }
        if norm[i] == "for" {
            i += 1;
            if norm.get(i).map(String::as_str) != Some("(") { return Err("expected '(' after for".parse().unwrap()); }
            let paren_end = matching_paren(&norm, i)?;
            let header = &norm[i + 1..paren_end];
            let mut semi_indices = Vec::new();
            for (idx, token) in header.iter().enumerate() {
                if token == ";" { semi_indices.push(idx); }
            }
            if semi_indices.len() != 2 { return Err("invalid for loop header".parse().unwrap()); }
            let init = &header[..semi_indices[0]];
            let cond = &header[semi_indices[0] + 1..semi_indices[1]];
            let update = &header[semi_indices[1] + 1..];
            if !init.is_empty() {
                let mut init_out = String::new();
                emit_body(init, &mut init_out)?;
                out.push_str(&init_out);
            }
            out.push_str("loop {\n");
            if !cond.is_empty() {
                out.push_str(&format!("if (!({})) break;\n", translate_expr(cond)?));
            }
            i = paren_end + 1;
            let (loop_body, next_idx) = parse_sub_statement(&norm, i)?;
            emit_body(&loop_body, out)?;
            if !update.is_empty() {
                let mut update_out = String::new();
                emit_body(update, &mut update_out)?;
                out.push_str(&update_out);
            }
            out.push_str("}\n");
            i = next_idx;
            continue;
        }
        if norm[i] == "break" {
            out.push_str("break;\n");
            i += 1;
            if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
            continue;
        }
        if norm[i] == "continue" {
            out.push_str("continue;\n");
            i += 1;
            if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
            continue;
        }
        if norm[i] == ";" {
            i += 1;
            continue;
        }

        let (expr_tokens, next_idx) = parse_statement_expr(&norm, i)?;
        out.push_str(&format!("{};\n", translate_expr(&expr_tokens)?));
        i = next_idx;
        if norm.get(i).map(String::as_str) == Some(";") { i += 1; }
    }
    Ok(())
}

fn normalize_body(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "switch" {
            i += 1;
            if tokens.get(i).map(String::as_str) != Some("(") { return Err("expected '(' after switch".parse().unwrap()); }
            let cond_end = matching_paren(tokens, i)?;
            let cond = &tokens[i + 1..cond_end];
            i = cond_end + 1;
            if tokens.get(i).map(String::as_str) != Some("{") { return Err("expected '{' after switch condition".parse().unwrap()); }
            let body_end = matching_brace(tokens, i)?;
            let body_tokens = &tokens[i + 1..body_end];
            i = body_end + 1;

            let mut cases: Vec<(Vec<String>, Vec<String>)> = Vec::new();
            let mut current_vals: Vec<String> = Vec::new();
            let mut current_body: Vec<String> = Vec::new();
            let mut j = 0;
            while j < body_tokens.len() {
                if body_tokens[j] == "case" {
                    if !current_body.is_empty() && current_vals.is_empty() {
                        // default or loose statements before case
                    }
                    if !current_vals.is_empty() && !current_body.is_empty() {
                        cases.push((core::mem::take(&mut current_vals), core::mem::take(&mut current_body)));
                    }
                    j += 1;
                    let val_start = j;
                    while j < body_tokens.len() && body_tokens[j] != ":" { j += 1; }
                    if j >= body_tokens.len() { return Err("unterminated case statement".parse().unwrap()); }
                    current_vals.push(body_tokens[val_start..j].join(" "));
                    j += 1; // skip ':'
                } else if body_tokens[j] == "default" {
                    if !current_vals.is_empty() && !current_body.is_empty() {
                        cases.push((core::mem::take(&mut current_vals), core::mem::take(&mut current_body)));
                    }
                    j += 1;
                    if body_tokens.get(j).map(String::as_str) == Some(":") { j += 1; }
                    current_vals.push(String::from("default"));
                } else {
                    current_body.push(body_tokens[j].clone());
                    j += 1;
                }
            }
            if !current_vals.is_empty() || !current_body.is_empty() {
                cases.push((current_vals, current_body));
            }

            // Build if-else chain
            let mut has_default = false;
            for (idx, (vals, body)) in cases.iter().enumerate() {
                if vals.contains(&String::from("default")) {
                    has_default = true;
                    out.extend(body.clone());
                    continue;
                }
                let cond_expr = vals.iter().map(|v| format!("({}) == ({})", cond.join(" "), v)).collect::<Vec<_>>().join(" || ");
                if idx > 0 {
                    out.push(String::from("else"));
                }
                out.push(String::from("if"));
                out.push(String::from("("));
                out.extend(tokenize(&cond_expr)?);
                out.push(String::from(")"));
                out.push(String::from("{"));
                out.extend(body.clone());
                out.push(String::from("}"));
            }
            if !has_default {
                // trailing empty else if needed or just skip
            }
            continue;
        }
        out.push(tokens[i].clone());
        i += 1;
    }
    Ok(out)
}

fn matching_brace(tokens: &[String], open: usize) -> Result<usize, &'static str> {
    if tokens.get(open).map(String::as_str) != Some("{") { return Err("expected opening brace"); }
    let mut depth = 0usize;
    for (i, token) in tokens.iter().enumerate().skip(open) {
        if token == "{" { depth += 1; }
        if token == "}" {
            depth -= 1;
            if depth == 0 { return Ok(i); }
        }
    }
    Err("unterminated brace block")
}

fn parse_statement_expr(tokens: &[String], start: usize) -> Result<(Vec<String>, usize), &'static str> {
    let mut i = start;
    let mut depth = 0usize;
    while i < tokens.len() {
        let t = tokens[i].as_str();
        if t == ";" && depth == 0 { break; }
        if t == "(" || t == "[" || t == "{" { depth += 1; }
        if t == ")" || t == "]" || t == "}" { depth = depth.saturating_sub(1); }
        i += 1;
    }
    Ok((tokens[start..i].to_vec(), i))
}

fn parse_sub_statement(tokens: &[String], start: usize) -> Result<(Vec<String>, usize), &'static str> {
    if tokens.get(start).map(String::as_str) == Some("{") {
        let end = matching_brace(tokens, start)?;
        Ok((tokens[start + 1..end].to_vec(), end + 1))
    } else {
        let mut i = start;
        let mut depth = 0usize;
        while i < tokens.len() {
            let t = tokens[i].as_str();
            if t == ";" && depth == 0 {
                i += 1;
                break;
            }
            if t == "(" || t == "[" || t == "{" { depth += 1; }
            if t == ")" || t == "]" || t == "}" { depth = depth.saturating_sub(1); }
            i += 1;
        }
        Ok((tokens[start..i].to_vec(), i))
    }
}

fn integer_literal(token: &str) -> Option<usize> {
    let token = token.trim_end_matches(['u', 'U', 'l', 'L']);
    if let Some(h) = token.strip_prefix("0x").or_else(|| token.strip_prefix("0X")) {
        usize::from_str_radix(h, 16).ok()
    } else if let Some(b) = token.strip_prefix("0b").or_else(|| token.strip_prefix("0B")) {
        usize::from_str_radix(b, 2).ok()
    } else {
        token.parse().ok()
    }
}

fn translate_expr(tokens: &[String]) -> Result<String, &'static str> {
    if tokens.is_empty() { return Err("empty expression"); }
    let mut tokens = tokens.to_vec();
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "sizeof" {
            if i + 1 < tokens.len() && tokens[i + 1] == "(" {
                let close = matching_paren(&tokens, i + 1)?;
                let inner = &tokens[i + 2..close];
                let size = evaluate_sizeof(inner)?;
                let replacement = format!("{}", size);
                tokens.splice(i..=close, core::iter::once(replacement));
            } else if i + 1 < tokens.len() {
                let inner = &tokens[i + 1..i + 2];
                let size = evaluate_sizeof(inner)?;
                let replacement = format!("{}", size);
                tokens.splice(i..=i + 1, core::iter::once(replacement));
            } else {
                return Err("invalid sizeof expression");
            }
        }
        i += 1;
    }

    let mut out = String::new();
    let mut i = 0;
    while i < tokens.len() {
        let t = &tokens[i];
        out.push_str(t);
        if i + 1 < tokens.len() {
            let next = &tokens[i + 1];
            if !(t == "(" || t == "[" || t == "." || t == "->" || next == ")" || next == "]" || next == "," || next == ";" || next == "." || next == "->" || next == ":" || next == "?")
                && !(next == "(" && is_ident(t))
                && !(t == "*" && is_ident(next)) {
                out.push(' ');
            }
        }
        i += 1;
    }
    Ok(out)
}

fn evaluate_sizeof(inner: &[String]) -> Result<usize, &'static str> {
    if inner.is_empty() {
        return Err("empty sizeof expression");
    }

    if inner.iter().any(|t| t == "*") {
        return Ok(8);
    }

    let cleaned: Vec<String> = inner.iter()
        .filter(|t| *t != "const" && *t != "volatile" && *t != "signed" && *t != "struct")
        .cloned()
        .collect();

    if cleaned.is_empty() {
        return Ok(8);
    }

    if cleaned.len() == 1 && cleaned[0].starts_with('"') {
        let unquoted = cleaned[0].trim_matches('"');
        return Ok(unquoted.len() + 1);
    }

    if let Some(pos) = inner.iter().position(|t| t == "[") {
        if pos + 2 < inner.len() && inner.last().map(String::as_str) == Some("]") {
            if let Some(size_tok) = inner.get(pos + 1) {
                if let Some(len) = integer_literal(size_tok) {
                    let base_tokens = &inner[..pos];
                    let base_size = evaluate_sizeof(base_tokens)?;
                    return Ok(base_size * len);
                }
            }
        }
    }

    if inner[0] == "struct" || matches!(inner[0].as_str(), "CounterState" | "HpxHostApi" | "HpxCpuInfo" | "HpxPciDeviceInfo") {
        let name = if inner[0] == "struct" { inner.get(1).map(String::as_str).unwrap_or("") } else { inner[0].as_str() };
        return match name {
            "CounterState" => Ok(16),
            "HpxHostApi" => Ok(208),
            "HpxCpuInfo" => Ok(78),
            "HpxPciDeviceInfo" => Ok(16),
            _ => Ok(16),
        };
    }

    if cleaned.len() == 1 {
        let ty = cleaned[0].as_str();
        return match ty {
            "char" | "uint8_t" | "int8_t" => Ok(1),
            "short" | "uint16_t" | "int16_t" => Ok(2),
            "int" | "uint32_t" | "int32_t" | "float" => Ok(4),
            "long" | "unsigned" | "size_t" | "uint64_t" | "int64_t" | "double" => Ok(8),
            "texture_t" | "patch_t" | "mobj_t" => Ok(32),
            _ => {
                if ty.starts_with('"') {
                    let unquoted = ty.trim_matches('"');
                    Ok(unquoted.len() + 1)
                } else {
                    Ok(4)
                }
            }
        };
    }

    let joined = cleaned.join(" ");
    match joined.as_str() {
        "unsigned char" | "signed char" => Ok(1),
        "unsigned short" | "short int" => Ok(2),
        "unsigned int" | "long int" | "unsigned long int" => Ok(4),
        "unsigned long long" | "long long" | "unsigned long" => Ok(8),
        _ => Ok(8),
    }
}
