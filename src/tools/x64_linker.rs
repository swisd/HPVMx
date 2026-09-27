//! Flat-image linker/assembler for the instruction subset emitted by the
//! embedded x86-64 backend. External HPVMx imports are recorded as relocations
//! and resolved by the disk loader at application load time.

use alloc::{format, string::String, vec::Vec};

const MAX_IMAGE: usize = 64 * 1024 * 1024;

#[derive(Clone)]
enum Op {
    Push(String), Pop(String), Mov(String, String), Add(String, String), Sub(String, String),
    Imul(String, String), Idiv(String), Cmp(String, String), Setcc(u8), Movzx(String, String),
    And(String, String), Or(String, String), Xor(String, String), Not(String), Shift(String, u8),
    Cqo, Call(String), Je(String), Jmp(String), Ret,
}

#[derive(Clone)]
enum Node { Label(String), Instruction(Op) }

pub use crate::tools::hpx_pack::ImportRelocation;

#[derive(Debug)]
pub struct LinkedImage {
    pub image: Vec<u8>,
    pub step_offset: u32,
    pub draw_offset: u32,
    pub input_offset: u32,
    pub imports: Vec<ImportRelocation>,
}

fn split_operands(text: &str) -> Vec<String> {
    text.split(',').map(|part| String::from(part.trim())).collect()
}

fn parse(source: &str) -> Result<Vec<Node>, &'static str> {
    let mut nodes = Vec::new();
    let mut skip_start = false;
    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with("bits ") || line.starts_with("org ") || line.starts_with("extern ") { continue; }
        if let Some(label) = line.strip_suffix(':') {
            if label == "_start" { skip_start = true; continue; }
            if skip_start && label.starts_with('.') { continue; }
            if skip_start { skip_start = false; }
            if label.starts_with('.') { return Err("local labels are not supported by the flat linker"); }
            nodes.push(Node::Label(String::from(label)));
            continue;
        }
        if skip_start { continue; }
        let mut pieces = line.splitn(2, char::is_whitespace);
        let mnemonic = pieces.next().unwrap_or("").to_ascii_lowercase();
        let operands = split_operands(pieces.next().unwrap_or(""));
        let op = match mnemonic.as_str() {
            "push" if operands.len() == 1 => Op::Push(operands[0].clone()),
            "pop" if operands.len() == 1 => Op::Pop(operands[0].clone()),
            "mov" if operands.len() == 2 => Op::Mov(operands[0].clone(), operands[1].clone()),
            "add" if operands.len() == 2 => Op::Add(operands[0].clone(), operands[1].clone()),
            "sub" if operands.len() == 2 => Op::Sub(operands[0].clone(), operands[1].clone()),
            "imul" if operands.len() == 2 => Op::Imul(operands[0].clone(), operands[1].clone()),
            "idiv" if operands.len() == 1 => Op::Idiv(operands[0].clone()),
            "cmp" if operands.len() == 2 => Op::Cmp(operands[0].clone(), operands[1].clone()),
            "and" if operands.len() == 2 => Op::And(operands[0].clone(), operands[1].clone()),
            "or" if operands.len() == 2 => Op::Or(operands[0].clone(), operands[1].clone()),
            "xor" if operands.len() == 2 => Op::Xor(operands[0].clone(), operands[1].clone()),
            "not" if operands.len() == 1 => Op::Not(operands[0].clone()),
            "shl" if operands == ["rax", "cl"] => Op::Shift(String::from("rax"), 4),
            "shr" if operands == ["rax", "cl"] => Op::Shift(String::from("rax"), 5),
            "sar" if operands == ["rax", "cl"] => Op::Shift(String::from("rax"), 7),
            "sete" => Op::Setcc(0x94), "setne" => Op::Setcc(0x95),
            "setl" => Op::Setcc(0x9c), "setle" => Op::Setcc(0x9e),
            "setg" => Op::Setcc(0x9f), "setge" => Op::Setcc(0x9d),
            "movzx" if operands.len() == 2 => Op::Movzx(operands[0].clone(), operands[1].clone()),
            "cqo" => Op::Cqo,
            "call" if operands.len() == 1 => Op::Call(operands[0].clone()),
            "je" if operands.len() == 1 => Op::Je(operands[0].clone()),
            "jmp" if operands.len() == 1 => Op::Jmp(operands[0].clone()),
            "ret" => Op::Ret,
            _ => return Err("assembly contains an instruction outside the built-in linker subset"),
        };
        nodes.push(Node::Instruction(op));
    }
    if nodes.is_empty() { return Err("assembly contains no linkable functions"); }
    Ok(nodes)
}

fn instruction_size(op: &Op) -> usize {
    match op {
        Op::Push(r) | Op::Pop(r) => if reg(r).map(|r| r >= 8).unwrap_or(false) { 2 } else { 1 },
        Op::Cqo | Op::Ret => 1,
        Op::Setcc(_) => 3,
        Op::Movzx(_, _) => 4,
        Op::Call(_) => 12,
        Op::Je(_) => 6,
        Op::Jmp(_) => 5,
        Op::Mov(dst, src) => {
            if parse_imm(src).is_some() { 10 }
            else if let Some((base, disp)) = parse_mem(dst).or_else(|| parse_mem(src)) {
                let low = base & 7;
                3 + usize::from(low == 4) + if disp == 0 && low != 5 { 0 } else if (-128..=127).contains(&disp) { 1 } else { 4 }
            }
            else { 3 }
        }
        Op::Add(_, src) | Op::Sub(_, src) | Op::Cmp(_, src) => if parse_imm(src).is_some() { 7 } else { 3 },
        Op::Imul(_, _) | Op::Idiv(_) | Op::And(_, _) | Op::Or(_, _) | Op::Xor(_, _) | Op::Not(_) | Op::Shift(_, _) => 3,
    }
}

fn reg(name: &str) -> Option<u8> {
    Some(match name.trim() {
        "rax" => 0, "rcx" => 1, "rdx" => 2, "rbx" => 3,
        "rsp" => 4, "rbp" => 5, "rsi" => 6, "rdi" => 7,
        "r8" => 8, "r9" => 9, "r10" => 10, "r11" => 11,
        "r12" => 12, "r13" => 13, "r14" => 14, "r15" => 15,
        _ => return None,
    })
}

fn parse_imm(text: &str) -> Option<i64> {
    let text = text.trim();
    if let Some(hex) = text.strip_prefix("0x") { i64::from_str_radix(hex, 16).ok() }
    else { text.parse().ok() }
}

fn parse_mem(text: &str) -> Option<(u8, i32)> {
    let inner = text.trim().strip_prefix('[')?.strip_suffix(']')?.trim();
    for (idx, c) in inner.char_indices().skip(1) {
        if c == '+' || c == '-' {
            let base = reg(inner[..idx].trim())?;
            let amount = parse_imm(&inner[idx + 1..])? as i32;
            return Some((base, if c == '-' { -amount } else { amount }));
        }
    }
    Some((reg(inner)?, 0))
}

fn rex(out: &mut Vec<u8>, reg_field: u8, base_field: u8) {
    let value = 0x48 | ((reg_field >> 3) << 2) | (base_field >> 3);
    out.push(value);
}

fn modrm_reg(out: &mut Vec<u8>, reg_field: u8, rm: u8) { out.push(0xc0 | ((reg_field & 7) << 3) | (rm & 7)); }

fn modrm_mem(out: &mut Vec<u8>, reg_field: u8, base: u8, disp: i32) {
    let low = base & 7;
    let mode = if disp == 0 && low != 5 { 0 } else if (-128..=127).contains(&disp) { 1 } else { 2 };
    out.push((mode << 6) | ((reg_field & 7) << 3) | if low == 4 { 4 } else { low });
    if low == 4 { out.push(0x24 | low); }
    if mode == 1 { out.push(disp as i8 as u8); }
    else if mode == 2 { out.extend_from_slice(&disp.to_le_bytes()); }
}

fn encode_rm_reg(out: &mut Vec<u8>, opcode: &[u8], rm: &str, source: u8) -> Result<(), &'static str> {
    let (base, disp, is_mem, dest_reg) = if let Some((base, disp)) = parse_mem(rm) { (base, disp, true, 0) }
        else { let r = reg(rm).ok_or("invalid x86 register")?; (r, 0, false, r) };
    rex(out, source, base);
    out.extend_from_slice(opcode);
    if is_mem { modrm_mem(out, source, base, disp); }
    else { modrm_reg(out, source, dest_reg); }
    Ok(())
}

fn encode_instruction(op: &Op, pc: usize, labels: &hashbrown::HashMap<String, usize>, out: &mut Vec<u8>, imports: &mut Vec<ImportRelocation>) -> Result<(), &'static str> {
    match op {
        Op::Push(r) => { let r = reg(r).ok_or("invalid push register")?; if r >= 8 { out.push(0x41); } out.push(0x50 + (r & 7)); }
        Op::Pop(r) => { let r = reg(r).ok_or("invalid pop register")?; if r >= 8 { out.push(0x41); } out.push(0x58 + (r & 7)); }
        Op::Mov(dst, src) => {
            if let Some(value) = parse_imm(src) {
                let r = reg(dst).ok_or("immediate mov destination must be a register")?;
                rex(out, 0, r); out.push(0xb8 + (r & 7)); out.extend_from_slice(&value.to_le_bytes());
            } else if let Some((base, disp)) = parse_mem(dst) {
                let src = reg(src).ok_or("memory mov source must be a register")?;
                rex(out, src, base); out.push(0x89); modrm_mem(out, src, base, disp);
            } else if let Some((base, disp)) = parse_mem(src) {
                let dst = reg(dst).ok_or("memory mov destination must be a register")?;
                rex(out, dst, base); out.push(0x8b); modrm_mem(out, dst, base, disp);
            } else {
                let dst = reg(dst).ok_or("invalid mov destination")?;
                let src = reg(src).ok_or("invalid mov source")?;
                encode_rm_reg(out, &[0x89], &format!("{}", reg_name(dst)?), src)?;
            }
        }
        Op::Add(dst, src) | Op::Sub(dst, src) | Op::Cmp(dst, src) => {
            let group = match op { Op::Add(_, _) => 0, Op::Sub(_, _) => 5, _ => 7 };
            let opcode = match op { Op::Add(_, _) => 0x01, Op::Sub(_, _) => 0x29, _ => 0x39 };
            if let Some(value) = parse_imm(src) {
                let dst = reg(dst).ok_or("immediate arithmetic destination must be a register")?;
                rex(out, 0, dst); out.push(0x81); modrm_reg(out, group, dst); out.extend_from_slice(&(value as i32).to_le_bytes());
            } else {
                let src = reg(src).ok_or("arithmetic source must be a register")?;
                encode_rm_reg(out, &[opcode], dst, src)?;
            }
        }
        Op::Imul(dst, src) => { let dst = reg(dst).ok_or("invalid imul destination")?; let src = reg(src).ok_or("invalid imul source")?; rex(out, dst, src); out.extend_from_slice(&[0x0f, 0xaf]); modrm_reg(out, dst, src); }
        Op::Idiv(src) => { let src = reg(src).ok_or("idiv source must be a register")?; rex(out, 7, src); out.push(0xf7); modrm_reg(out, 7, src); }
        Op::And(dst, src) | Op::Or(dst, src) | Op::Xor(dst, src) => {
            let dst = reg(dst).ok_or("invalid bitwise destination")?;
            let src = reg(src).ok_or("invalid bitwise source")?;
            let opcode = match op { Op::And(_, _) => 0x21, Op::Or(_, _) => 0x09, _ => 0x31 };
            rex(out, src, dst); out.push(opcode); modrm_reg(out, src, dst);
        }
        Op::Not(dst) => { let dst = reg(dst).ok_or("invalid not destination")?; rex(out, 0, dst); out.push(0xf7); modrm_reg(out, 2, dst); }
        Op::Shift(dst, ext) => { let dst = reg(dst).ok_or("invalid shift destination")?; rex(out, 0, dst); out.push(0xd3); modrm_reg(out, *ext, dst); }
        Op::Setcc(code) => { out.extend_from_slice(&[0x0f, *code, 0xc0]); }
        Op::Movzx(dst, src) => {
            if dst != "rax" || src != "al" { return Err("movzx linker subset currently supports movzx rax, al only"); }
            out.extend_from_slice(&[0x48, 0x0f, 0xb6, 0xc0]);
        }
        Op::Cqo => out.extend_from_slice(&[0x48, 0x99]),
        Op::Call(symbol) => {
            if let Some(target) = labels.get(symbol) {
                out.push(0xe8);
                let disp = *target as i64 - (pc as i64 + 5);
                out.extend_from_slice(&(disp as i32).to_le_bytes());
                out.extend_from_slice(&[0x90; 7]);
            } else {
                if !known_import(symbol) { return Err("unresolved symbol in call; only HPVMx hpx_* imports are linkable"); }
                out.extend_from_slice(&[0x48, 0xb8]);
                let patch_offset = out.len() as u32;
                out.extend_from_slice(&0u64.to_le_bytes());
                out.extend_from_slice(&[0xff, 0xd0]);
                imports.push(ImportRelocation { patch_offset, symbol: symbol.clone() });
            }
        }
        Op::Je(label) | Op::Jmp(label) => {
            let target = *labels.get(label).ok_or("branch target label is undefined")?;
            match op {
                Op::Je(_) => { out.extend_from_slice(&[0x0f, 0x84]); let disp = target as i64 - (pc as i64 + 6); out.extend_from_slice(&(disp as i32).to_le_bytes()); }
                _ => { out.push(0xe9); let disp = target as i64 - (pc as i64 + 5); out.extend_from_slice(&(disp as i32).to_le_bytes()); }
            }
        }
        Op::Ret => out.push(0xc3),
    }
    Ok(())
}

fn reg_name(id: u8) -> Result<&'static str, &'static str> {
    Ok(match id { 0 => "rax", 1 => "rcx", 2 => "rdx", 3 => "rbx", 4 => "rsp", 5 => "rbp", 6 => "rsi", 7 => "rdi", 8 => "r8", 9 => "r9", _ => return Err("invalid register") })
}

fn known_import(name: &str) -> bool {
    matches!(name,
        "hpx_ui_resolution_x" | "hpx_ui_resolution_y" | "hpx_ui_clear" |
        "hpx_ui_fill_rect" | "hpx_ui_draw_pixel" | "hpx_ui_draw_text" |
        "hpx_fs_read_file" | "hpx_fs_write_file" | "hpx_fs_make_dir" |
        "hpx_fs_remove" | "hpx_fs_rename" | "hpx_cpu_core_count" |
        "hpx_cpu_thread_count" | "hpx_pci_device_count" | "hpx_pci_read_u32" |
        "hpx_pci_write_u32" | "hpx_network_initialize" | "hpx_network_link_up" |
        "hpx_network_transmit" | "hpx_network_receive" | "hpx_beep" | "hpx_mute" |
        "hpx_sleep_ms" | "hpx_alloc" | "hpx_free" | "hpx_set_global_variable" |
        "hpx_get_global_variable" | "hpx_pack_dimensions")
}

/// Assemble/link the backend's flat Win64 instruction stream and export HPX
/// callbacks by their conventional names. Kernel imports remain relocatable.
pub fn link(assembly: &str) -> Result<LinkedImage, &'static str> {
    let nodes = parse(assembly)?;
    let mut labels = hashbrown::HashMap::new();
    let mut pc = 0usize;
    for node in &nodes {
        match node {
            Node::Label(name) => {
                if labels.insert(name.clone(), pc).is_some() { return Err("duplicate symbol in linked modules"); }
            }
            Node::Instruction(op) => { pc = pc.checked_add(instruction_size(op)).ok_or("linked image size overflow")?; }
        }
    }
    if pc == 0 || pc > MAX_IMAGE { return Err("linked image is empty or exceeds 64 MiB"); }
    let mut image = Vec::with_capacity(pc);
    let mut imports = Vec::new();
    for node in &nodes {
        if let Node::Instruction(op) = node {
            let here = image.len();
            encode_instruction(op, here, &labels, &mut image, &mut imports)?;
        }
    }
    if image.len() != pc { return Err("internal linker size mismatch"); }
    let callback = |name: &str| -> Result<u32, &'static str> {
        labels.get(name).copied().and_then(|v| u32::try_from(v).ok()).ok_or("required hpx_step callback is missing")
    };
    Ok(LinkedImage {
        image,
        step_offset: callback("hpx_step")?,
        draw_offset: labels.get("hpx_draw").and_then(|v| u32::try_from(*v).ok()).unwrap_or(0),
        input_offset: labels.get("hpx_input").and_then(|v| u32::try_from(*v).ok()).unwrap_or(0),
        imports,
    })
}

/// Link relocatable assembly sections from C objects and Micro-C `.asm` files.
pub fn link_modules(modules: &[&str]) -> Result<LinkedImage, &'static str> {
    if modules.is_empty() { return Err("link requires at least one input module"); }
    let mut combined = String::new();
    for (index, module) in modules.iter().enumerate() {
        let mut local_labels = hashbrown::HashMap::<String, String>::new();
        for line in module.lines() {
            if let Some(label) = line.trim().strip_suffix(':') {
                let digits = label.strip_prefix('L').unwrap_or("");
                let is_local_l = !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit());
                let c_digits = label.strip_prefix("CL").unwrap_or("");
                let is_local_cl = !c_digits.is_empty() && c_digits.bytes().all(|b| b.is_ascii_digit());
                if is_local_l || is_local_cl {
                    local_labels.insert(String::from(label), format!("M{}_{}", index, label));
                }
            }
        }
        for line in module.lines() {
            let trimmed = line.trim();
            if let Some(label) = trimmed.strip_suffix(':') {
                if let Some(mapped) = local_labels.get(label) { combined.push_str(mapped); combined.push(':'); }
                else { combined.push_str(line); }
            } else if trimmed.starts_with("je ") || trimmed.starts_with("jmp ") {
                let (mnemonic, target) = trimmed.split_once(' ').ok_or("invalid branch instruction")?;
                combined.push_str("    "); combined.push_str(mnemonic); combined.push(' ');
                combined.push_str(local_labels.get(target).map(String::as_str).unwrap_or(target));
            } else { combined.push_str(line); }
            combined.push('\n');
        }
    }
    link(&combined)
}
