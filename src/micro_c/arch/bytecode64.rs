//! 64-bit Bytecode Virtual Machine backend.
//!
//! This backend translates Micro-C Intermediate Representation (IR) into
//! 64-bit bytecode instructions for a virtual machine architecture.

use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::arch::Architecture;
use crate::ir::IRInst;
use crate::regalloc::RegisterAllocator;
use crate::stackframe::StackFrame;

/// Backend for generating 64-bit virtual machine bytecode.
pub struct Bytecode64Backend {
    regs: RegisterAllocator,
    function_params: HashMap<String, Vec<String>>,
}

impl Bytecode64Backend {
    /// Creates a new Bytecode64 backend instance with the given function parameter map.
    pub fn new(function_params: HashMap<String, Vec<String>>) -> Self {
        Self {
            regs: RegisterAllocator::new(vec![
                "r0".into(),
                "r1".into(),
                "r2".into(),
                "r3".into(),
                "r4".into(),
                "r5".into(),
                "r6".into(),
                "r7".into(),
            ]),
            function_params,
        }
    }

    fn split_functions(&self, ir: &[IRInst]) -> Vec<(String, Vec<IRInst>)> {
        let mut funcs = Vec::new();
        let mut current_name: Option<String> = None;
        let mut current_body = Vec::new();

        for inst in ir {
            match inst {
                IRInst::Extern(_) => {}

                IRInst::Label(name) if self.function_params.contains_key(name) => {
                    if let Some(prev) = current_name.take() {
                        funcs.push((prev, current_body));
                        current_body = Vec::new();
                    }
                    current_name = Some(name.clone());
                }

                IRInst::Label(_) if current_name.is_none() => {}

                _ => current_body.push(inst.clone()),
            }
        }

        if let Some(last) = current_name {
            funcs.push((last, current_body));
        }

        funcs
    }

    fn build_frame(&self, name: &str, body: &[IRInst]) -> StackFrame {
        let mut frame = StackFrame::new();

        if let Some(params) = self.function_params.get(name) {
            for p in params {
                frame.alloc(p);
            }
        }

        for inst in body {
            match inst {
                IRInst::StoreVar(name, _) => {
                    frame.alloc(name);
                }
                IRInst::LoadVar(_, name) => {
                    frame.alloc(name);
                }
                _ => {}
            }
        }

        frame
    }

    fn emit_function(&mut self, out: &mut String, name: &str, body: &[IRInst]) {
        let mut frame = self.build_frame(name, body);
        let frame_size = frame.frame_size();

        out.push_str(&format!("{}:\n", name));
        out.push_str(&format!("    enter {}\n", frame_size));

        if let Some(params) = self.function_params.get(name) {
            for (i, param) in params.iter().enumerate() {
                let off = frame.get(param);
                out.push_str(&format!("    store64 [rbp-{}], a{}\n", off, i));
            }
        }

        for inst in body {
            self.emit_inst(out, inst, &mut frame);
        }

        out.push_str("    leave\n");
        out.push_str("    ret\n\n");
    }

    fn emit_inst(&mut self, out: &mut String, inst: &IRInst, frame: &mut StackFrame) {
        match inst {
            IRInst::LoadConst(dst, val) => {
                let rd = self.regs.alloc(dst);
                out.push_str(&format!("    const64 {}, {}\n", rd, val));
            }

            IRInst::LoadVar(dst, src) => {
                let rd = self.regs.alloc(dst);
                let off = frame.get(src);
                out.push_str(&format!("    load64 {}, [rbp-{}]\n", rd, off));
            }

            IRInst::StoreVar(dst, src) => {
                let rs = self.regs.alloc(src);
                let off = frame.get(dst);
                out.push_str(&format!("    store64 [rbp-{}], {}\n", off, rs));
            }

            IRInst::StackAlloc(dst, size) => {
                let rd = self.regs.alloc(dst);
                let size = if *size <= 0 { 8 } else { *size };
                out.push_str(&format!("    alloca64 {}, {}\n", rd, size));
            }

            IRInst::LoadMem(dst, addr) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(addr);
                out.push_str(&format!("    load64 {}, [{}]\n", rd, ra));
            }

            IRInst::StoreMem(addr, src) => {
                let ra = self.regs.alloc(addr);
                let rs = self.regs.alloc(src);
                out.push_str(&format!("    store64 [{}], {}\n", ra, rs));
            }

            IRInst::Add(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    add64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Sub(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    sub64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Mul(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    mul64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Div(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    div64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Eq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    eq64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Neq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    neq64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Lt(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    lt64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Gt(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    gt64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::LtEq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    lteq64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::GtEq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);
                out.push_str(&format!("    gteq64 {}, {}, {}\n", rd, ra, rb));
            }

            IRInst::Call(dst, func, args) => {
                let mut arg_str = String::new();
                for (i, arg) in args.iter().enumerate() {
                    let r = self.regs.alloc(arg);
                    if i > 0 {
                        arg_str.push_str(", ");
                    }
                    arg_str.push_str(&r);
                }
                let rd = self.regs.alloc(dst);
                out.push_str(&format!("    call64 {}, {}({})\n", rd, func, arg_str));
            }

            IRInst::Return(src) => {
                let rs = self.regs.alloc(src);
                out.push_str(&format!("    ret64 {}\n", rs));
            }

            IRInst::JumpIfZero(cond, label) => {
                let rc = self.regs.alloc(cond);
                out.push_str(&format!("    jz {}, {}\n", rc, label));
            }

            IRInst::Jump(label) => {
                out.push_str(&format!("    jmp {}\n", label));
            }

            IRInst::Label(name) => {
                out.push_str(&format!("{}:\n", name));
            }

            IRInst::Extern(_) => {}
        }
    }
}

impl Architecture for Bytecode64Backend {
    fn emit_program(&mut self, ir: &[IRInst]) -> String {
        let mut out = String::new();

        out.push_str("; ARCH bytecode64\n");
        out.push_str("; Generated Bytecode file. Modifications will not be preserved\n");
        out.push_str(".target bytecode64\n\n");

        for inst in ir {
            if let IRInst::Extern(name) = inst {
                out.push_str(&format!(".extern {}\n", name));
            }
        }

        let funcs = self.split_functions(ir);

        for (name, body) in funcs {
            self.emit_function(&mut out, &name, &body);
        }

        out
    }
}
