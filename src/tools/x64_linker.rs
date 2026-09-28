//! Flat-image linker/assembler for the instruction subset emitted by the
//! embedded x86-64 backend. External HPVMx imports are recorded as relocations
//! and resolved by the disk loader at application load time.

use alloc::{format, string::{String, ToString}, vec::Vec};
use crate::vdebug_autoprefix;

fn err_str(msg: &str) -> String {
    vdebug_autoprefix!(12, "x64 linker error [Advanced Debug]: {}", msg);
    msg.to_string()
}

const MAX_IMAGE: usize = 64 * 1024 * 1024;

#[derive(Clone)]
enum Op {
    Push(String), Pop(String), Mov(String, String), Add(String, String), Sub(String, String),
    Imul(String, String), Idiv(String), Cmp(String, String), Setcc(u8), Movzx(String, String),
    And(String, String), Or(String, String), Xor(String, String), Not(String), Shift(String, u8),
    Cqo, Call(String), Je(String), Jmp(String), LeaRip(String), Data(u64), Ret,
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

fn parse(source: &str) -> Result<Vec<Node>, String> {
    let mut nodes = Vec::new();
    let mut skip_start = false;
    for raw in source.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with("bits ") || line.starts_with("org ") || line.starts_with("extern ") { continue; }
        if let Some(label) = line.strip_suffix(':') {
            if label == "_start" { skip_start = true; continue; }
            if skip_start && label.starts_with('.') { continue; }
            if skip_start { skip_start = false; }
            if label.starts_with('.') { return Err(err_str("local labels are not supported by the flat linker")); }
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
            "lea" if operands.len() == 2 && operands[0] == "rax" && operands[1].starts_with("[rel ") && operands[1].ends_with(']') => Op::LeaRip(String::from(operands[1][5..operands[1].len()-1].trim())),
            "dq" if operands.len() == 1 => Op::Data(parse_imm(&operands[0]).ok_or_else(|| err_str("invalid data value"))? as u64),
            "cqo" => Op::Cqo,
            "call" if operands.len() == 1 => Op::Call(operands[0].clone()),
            "je" if operands.len() == 1 => Op::Je(operands[0].clone()),
            "jmp" if operands.len() == 1 => Op::Jmp(operands[0].clone()),
            "ret" => Op::Ret,
            _ => return Err(err_str("assembly contains an instruction outside the built-in linker subset")),
        };
        nodes.push(Node::Instruction(op));
    }
    if nodes.is_empty() { return Err(err_str("assembly contains no linkable functions")); }
    Ok(nodes)
}

fn instruction_size(op: &Op) -> usize {
    match op {
        Op::Push(r) | Op::Pop(r) => if reg(r).map(|r| r >= 8).unwrap_or(false) { 2 } else { 1 },
        Op::Cqo | Op::Ret => 1,
        Op::LeaRip(_) => 7,
        Op::Data(_) => 8,
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

fn reg(r: &str) -> Option<u8> {
    match r {
        "rax" | "al" => Some(0), "rcx" | "cl" => Some(1), "rdx" | "dl" => Some(2), "rbx" | "bl" => Some(3),
        "rsp" | "spl" => Some(4), "rbp" | "bpl" => Some(5), "rsi" | "sil" => Some(6), "rdi" | "dil" => Some(7),
        "r8" | "r8b" => Some(8), "r9" | "r9b" => Some(9), "r10" | "r10b" => Some(10), "r11" | "r11b" => Some(11),
        "r12" | "r12b" => Some(12), "r13" | "r13b" => Some(13), "r14" | "r14b" => Some(14), "r15" | "r15b" => Some(15),
        _ => None,
    }
}

fn parse_imm(text: &str) -> Option<i64> {
    if let Some(hex) = text.strip_prefix("0x") { i64::from_str_radix(hex, 16).ok() }
    else { text.parse().ok() }
}

fn parse_mem(text: &str) -> Option<(u8, i32)> {
    if !text.starts_with('[') || !text.ends_with(']') { return None; }
    let inner = text[1..text.len() - 1].trim();
    if let Some(reg_idx) = reg(inner) {
        return Some((reg_idx, 0));
    }
    if let Some((lhs, rhs)) = inner.split_once('+') {
        let base = reg(lhs.trim())?;
        let disp = parse_imm(rhs.trim())? as i32;
        return Some((base, disp));
    }
    if let Some((lhs, rhs)) = inner.split_once('-') {
        let base = reg(lhs.trim())?;
        let disp = -(parse_imm(rhs.trim())? as i32);
        return Some((base, disp));
    }
    None
}

fn encode_instruction(op: &Op, pc: usize, labels: &hashbrown::HashMap<String, usize>, out: &mut Vec<u8>, imports: &mut Vec<ImportRelocation>) -> Result<(), String> {
    match op {
        Op::Push(r) => {
            let id = reg(r).ok_or_else(|| err_str("invalid push register"))?;
            if id >= 8 { out.push(0x41); }
            out.push(0x50 + (id & 7));
        }
        Op::Pop(r) => {
            let id = reg(r).ok_or_else(|| err_str("invalid pop register"))?;
            if id >= 8 { out.push(0x41); }
            out.push(0x58 + (id & 7));
        }
        Op::Mov(dst, src) => {
            if let Some(imm) = parse_imm(src) {
                let id = reg(dst).ok_or_else(|| err_str("invalid mov destination register"))?;
                out.extend_from_slice(&[0x48 + ((id >> 3) & 1), 0xb8 + (id & 7)]);
                out.extend_from_slice(&imm.to_le_bytes());
            } else if let Some((base, disp)) = parse_mem(dst) {
                let src_id = reg(src).ok_or_else(|| err_str("invalid mov source register"))?;
                let low = base & 7;
                out.extend_from_slice(&[0x48 | ((src_id >> 3) & 1), 0x89 | ((src_id & 7) << 3)]);
                if low == 4 {
                    out.extend_from_slice(&[0x24]);
                } else if low == 5 && disp == 0 {
                    out.extend_from_slice(&[0x45, 0x00]);
                    return Ok(());
                }
                out.push(base);
                if disp != 0 || low == 5 {
                    if (-128..=127).contains(&disp) {
                        out.push(disp as u8);
                    } else {
                        out.extend_from_slice(&disp.to_le_bytes());
                    }
                }
            } else if let Some((base, disp)) = parse_mem(src) {
                let dst_id = reg(dst).ok_or_else(|| err_str("invalid mov destination register"))?;
                let low = base & 7;
                out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0x8b | ((dst_id & 7) << 3)]);
                if low == 4 {
                    out.extend_from_slice(&[0x24]);
                } else if low == 5 && disp == 0 {
                    out.extend_from_slice(&[0x45, 0x00]);
                    return Ok(());
                }
                out.push(base);
                if disp != 0 || low == 5 {
                    if (-128..=127).contains(&disp) {
                        out.push(disp as u8);
                    } else {
                        out.extend_from_slice(&disp.to_le_bytes());
                    }
                }
            } else {
                let src_id = reg(src).ok_or_else(|| err_str("invalid mov source register"))?;
                let dst_id = reg(dst).ok_or_else(|| err_str("invalid mov destination register"))?;
                out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x89, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
            }
        }
        Op::Add(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid add destination register"))?;
            if let Some(imm) = parse_imm(src) {
                out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0x81, 0xc0 | (dst_id & 7)]);
                out.extend_from_slice(&(imm as i32).to_le_bytes());
            } else {
                let src_id = reg(src).ok_or_else(|| err_str("invalid add source register"))?;
                out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x01, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
            }
        }
        Op::Sub(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid sub destination register"))?;
            if let Some(imm) = parse_imm(src) {
                out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0x81, 0xe8 | (dst_id & 7)]);
                out.extend_from_slice(&(imm as i32).to_le_bytes());
            } else {
                let src_id = reg(src).ok_or_else(|| err_str("invalid sub source register"))?;
                out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x29, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
            }
        }
        Op::Imul(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid imul destination register"))?;
            let src_id = reg(src).ok_or_else(|| err_str("invalid imul source register"))?;
            out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x0f, 0xaf, 0xc0 | ((dst_id & 7) << 3) | (src_id & 7)]);
        }
        Op::Idiv(src) => {
            let src_id = reg(src).ok_or_else(|| err_str("invalid idiv source register"))?;
            out.extend_from_slice(&[0x48 | ((src_id >> 3) & 1), 0xf7, 0xf8 | (src_id & 7)]);
        }
        Op::Cmp(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid cmp destination register"))?;
            if let Some(imm) = parse_imm(src) {
                out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0x81, 0xf8 | (dst_id & 7)]);
                out.extend_from_slice(&(imm as i32).to_le_bytes());
            } else {
                let src_id = reg(src).ok_or_else(|| err_str("invalid cmp source register"))?;
                out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x39, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
            }
        }
        Op::And(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid and destination register"))?;
            let src_id = reg(src).ok_or_else(|| err_str("invalid and source register"))?;
            out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x21, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
        }
        Op::Or(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid or destination register"))?;
            let src_id = reg(src).ok_or_else(|| err_str("invalid or source register"))?;
            out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x09, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
        }
        Op::Xor(dst, src) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid xor destination register"))?;
            let src_id = reg(src).ok_or_else(|| err_str("invalid xor source register"))?;
            out.extend_from_slice(&[0x48 | ((src_id >> 3) << 2) | ((dst_id >> 3) & 1), 0x31, 0xc0 | ((src_id & 7) << 3) | (dst_id & 7)]);
        }
        Op::Not(dst) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid not destination register"))?;
            out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0xf7, 0xd0 | (dst_id & 7)]);
        }
        Op::Shift(dst, amt) => {
            let dst_id = reg(dst).ok_or_else(|| err_str("invalid shift destination register"))?;
            let subop = match amt { 4 => 0xe0, 5 => 0xe8, 7 => 0xf8, _ => return Err(err_str("invalid shift amount")) };
            out.extend_from_slice(&[0x48 | ((dst_id >> 3) & 1), 0xd3, subop | (dst_id & 7)]);
        }
        Op::Setcc(code) => {
            out.extend_from_slice(&[0x0f, *code, 0xc0]);
        }
        Op::Movzx(dst, src) => {
            if dst != "rax" || src != "al" { return Err(err_str("movzx linker subset currently supports movzx rax, al only")); }
            out.extend_from_slice(&[0x48, 0x0f, 0xb6, 0xc0]);
        }
        Op::Cqo => out.extend_from_slice(&[0x48, 0x99]),
        Op::LeaRip(label) => {
            let target = *labels.get(label).ok_or_else(|| err_str("address target label is undefined"))?;
            out.extend_from_slice(&[0x48, 0x8d, 0x05]);
            out.extend_from_slice(&((target as i64 - (pc as i64 + 7)) as i32).to_le_bytes());
        }
        Op::Data(value) => out.extend_from_slice(&value.to_le_bytes()),
        Op::Call(symbol) => {
            if let Some(target) = labels.get(symbol) {
                out.push(0xe8);
                let disp = *target as i64 - (pc as i64 + 5);
                out.extend_from_slice(&(disp as i32).to_le_bytes());
                out.extend_from_slice(&[0x90; 7]);
            } else {
                if !known_import(symbol) { return Err(err_str("unresolved symbol in call; only HPVMx hpx_* imports are linkable")); }
                out.extend_from_slice(&[0x48, 0xb8]);
                let patch_offset = out.len() as u32;
                out.extend_from_slice(&0u64.to_le_bytes());
                out.extend_from_slice(&[0xff, 0xd0]);
                imports.push(ImportRelocation { patch_offset, symbol: symbol.clone() });
            }
        }
        Op::Je(label) | Op::Jmp(label) => {
            let target = *labels.get(label).ok_or_else(|| err_str("branch target label is undefined"))?;
            match op {
                Op::Je(_) => { out.extend_from_slice(&[0x0f, 0x84]); let disp = target as i64 - (pc as i64 + 6); out.extend_from_slice(&(disp as i32).to_le_bytes()); }
                _ => { out.push(0xe9); let disp = target as i64 - (pc as i64 + 5); out.extend_from_slice(&(disp as i32).to_le_bytes()); }
            }
        }
        Op::Ret => out.push(0xc3),
    }
    Ok(())
}

fn reg_name(id: u8) -> Result<&'static str, String> {
    Ok(match id { 0 => "rax", 1 => "rcx", 2 => "rdx", 3 => "rbx", 4 => "rsp", 5 => "rbp", 6 => "rsi", 7 => "rdi", 8 => "r8", 9 => "r9", _ => return Err(err_str("invalid register")) })
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
pub fn link(assembly: &str) -> Result<LinkedImage, String> {
    let nodes = parse(assembly)?;
    let mut labels = hashbrown::HashMap::new();
    let mut pc = 0usize;
    for node in &nodes {
        match node {
            Node::Label(name) => {
                if labels.insert(name.clone(), pc).is_some() { return Err(err_str("duplicate symbol in linked modules")); }
            }
            Node::Instruction(op) => { pc = pc.checked_add(instruction_size(op)).ok_or_else(|| err_str("linked image size overflow"))?; }
        }
    }
    if pc == 0 || pc > MAX_IMAGE { return Err(err_str("linked image is empty or exceeds 64 MiB")); }
    let mut image = Vec::with_capacity(pc);
    let mut imports = Vec::new();
    for node in &nodes {
        if let Node::Instruction(op) = node {
            let here = image.len();
            encode_instruction(op, here, &labels, &mut image, &mut imports)?;
        }
    }
    if image.len() != pc { return Err(err_str("internal linker size mismatch")); }
    let callback = |name: &str| -> Result<u32, String> {
        labels.get(name).copied().and_then(|v| u32::try_from(v).ok()).ok_or_else(|| err_str("required hpx_step callback is missing"))
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
pub fn link_modules(modules: &[&str]) -> Result<LinkedImage, String> {
    if modules.is_empty() { return Err(err_str("link requires at least one input module")); }
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
                let (mnemonic, target) = trimmed.split_once(' ').ok_or_else(|| err_str("invalid branch instruction"))?;
                combined.push_str("    "); combined.push_str(mnemonic); combined.push(' ');
                combined.push_str(local_labels.get(target).map(String::as_str).unwrap_or(target));
            } else { combined.push_str(line); }
            combined.push('\n');
        }
    }
    link(&combined)
}
