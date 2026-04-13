// src/arch/x86_64_raw.rs

use alloc::string::String;
use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::arch::Architecture;
use crate::error::error;
use crate::ir::IRInst;
use crate::regalloc::RegisterAllocator;
use crate::stackframe::StackFrame;

pub struct X86_64RawBackend {
    regs: RegisterAllocator,
    function_params: HashMap<String, Vec<String>>,
}

impl X86_64RawBackend {
    pub fn new(function_params: HashMap<String, Vec<String>>) -> Self {
        Self {
            regs: RegisterAllocator::new(vec![
                "rax".into(),
                "rbx".into(),
                "rcx".into(),
                "rdx".into(),
            ]),
            function_params,
        }
    }

    fn split_functions(ir: &[IRInst]) -> Vec<(String, Vec<IRInst>)> {
        let mut funcs = Vec::new();
        let mut current_name: Option<String> = None;
        let mut current_body = Vec::new();

        for inst in ir {
            match inst {
                IRInst::Label(name) => {
                    if let Some(prev) = current_name.take() {
                        funcs.push((prev, current_body));
                        current_body = Vec::new();
                    }
                    current_name = Some(name.clone());
                }

                _ => current_body.push(inst.clone()),
            }
        }

        if let Some(last) = current_name {
            funcs.push((last, current_body));
        }

        funcs
    }

    fn build_frame(
        &self,
        name: &str,
        body: &[IRInst],
    ) -> StackFrame {
        let mut frame = StackFrame::new();


        // Allocate params first

        if let Some(params) = self.function_params.get(name) {
            for p in params {
                frame.alloc(p);
            }
        }


        // Allocate locals used in function body

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

    fn emit_function(
        &mut self,
        out: &mut String,
        name: &str,
        body: &[IRInst],
    ) {
        let arg_regs = ["rcx", "rdx", "r8", "r9"];

        let mut frame = self.build_frame(name, body);
        let frame_size = frame.frame_size() + 8;


        // Function label

        out.push_str(&format!("{}:\n", name));
        out.push_str("    push rbp\n");
        out.push_str("    mov rbp, rsp\n");
        out.push_str(&format!("    sub rsp, {}\n", frame_size));


        // Save incoming parameters

        if let Some(params) = self.function_params.get(name) {
            for (i, param) in params.iter().enumerate() {
                if i >= arg_regs.len() {
                    error("Too many parameters for x86_64 ABI");
                    return;
                }

                let off = frame.get(param);

                out.push_str(&format!(
                    "    mov [rbp-{}], {}\n",
                    off,
                    arg_regs[i]
                ));
            }
        }


        // Emit body

        for inst in body {
            self.emit_inst(out, inst, &mut frame);
        }


        // Default epilogue if no return emitted

        out.push_str("    mov rsp, rbp\n");
        out.push_str("    pop rbp\n");
        out.push_str("    ret\n\n");
    }

    fn emit_inst(
        &mut self,
        out: &mut String,
        inst: &IRInst,
        frame: &mut StackFrame,
    ) {
        match inst {

            IRInst::LoadConst(dst, val) => {
                let rd = self.regs.alloc(dst);
                out.push_str(&format!(
                    "    mov {}, {}\n",
                    rd,
                    val
                ));
            }

            IRInst::LoadVar(dst, src) => {
                let rd = self.regs.alloc(dst);
                let off = frame.get(src);

                out.push_str(&format!(
                    "    mov {}, [rbp-{}]\n",
                    rd,
                    off
                ));
            }

            IRInst::StoreVar(dst, src) => {
                let rs = self.regs.alloc(src);
                let off = frame.get(dst);

                out.push_str(&format!(
                    "    mov [rbp-{}], {}\n",
                    off,
                    rs
                ));
            }

            IRInst::Add(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    mov {}, {}\n",
                    rd,
                    ra
                ));
                out.push_str(&format!(
                    "    add {}, {}\n",
                    rd,
                    rb
                ));
            }

            IRInst::Sub(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    mov {}, {}\n",
                    rd,
                    ra
                ));
                out.push_str(&format!(
                    "    sub {}, {}\n",
                    rd,
                    rb
                ));
            }


            IRInst::Call(dst, func, args) => {
                let arg_regs = ["rcx", "rdx", "r8", "r9"];

                for (i, arg) in args.iter().enumerate() {
                    if i >= arg_regs.len() {
                        panic!("Too many call args");
                    }

                    let r = self.regs.alloc(arg);

                    out.push_str(&format!(
                        "    mov {}, {}\n",
                        arg_regs[i],
                        r
                    ));
                }


                // RAW backend: no Windows shadow space

                out.push_str(&format!(
                    "    call {}\n",
                    func
                ));

                let rd = self.regs.alloc(dst);
                out.push_str(&format!(
                    "    mov {}, rax\n",
                    rd
                ));
            }

            IRInst::Return(src) => {
                let rs = self.regs.alloc(src);

                out.push_str(&format!(
                    "    mov rax, {}\n",
                    rs
                ));
                out.push_str("    mov rsp, rbp\n");
                out.push_str("    pop rbp\n");
                out.push_str("    ret\n");
            }

            IRInst::Eq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    cmp {}, {}\n",
                    ra,
                    rb
                ));
                out.push_str("    sete al\n");
                out.push_str("    movzx rax, al\n");
                out.push_str(&format!(
                    "    mov {}, rax\n",
                    rd
                ));
            }

            IRInst::Neq(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    cmp {}, {}\n",
                    ra,
                    rb
                ));
                out.push_str("    setne al\n");
                out.push_str("    movzx rax, al\n");
                out.push_str(&format!(
                    "    mov {}, rax\n",
                    rd
                ));
            }

            IRInst::Lt(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    cmp {}, {}\n",
                    ra,
                    rb
                ));
                out.push_str("    setl al\n");
                out.push_str("    movzx rax, al\n");
                out.push_str(&format!(
                    "    mov {}, rax\n",
                    rd
                ));
            }

            

            IRInst::Gt(dst, a, b) => {
                let rd = self.regs.alloc(dst);
                let ra = self.regs.alloc(a);
                let rb = self.regs.alloc(b);

                out.push_str(&format!(
                    "    cmp {}, {}\n",
                    ra,
                    rb
                ));
                out.push_str("    setg al\n");
                out.push_str("    movzx rax, al\n");
                out.push_str(&format!(
                    "    mov {}, rax\n",
                    rd
                ));
            }

            

            IRInst::JumpIfZero(cond, label) => {
                let rc = self.regs.alloc(cond);

                out.push_str(&format!(
                    "    cmp {}, 0\n",
                    rc
                ));
                out.push_str(&format!(
                    "    je {}\n",
                    label
                ));
            }

            

            IRInst::Jump(label) => {
                out.push_str(&format!(
                    "    jmp {}\n",
                    label
                ));
            }


            // Ignore labels here

            IRInst::Label(_) => {}

            _ => {}
        }
    }
}

impl Architecture for X86_64RawBackend {
    fn emit_program(&mut self, ir: &[IRInst]) -> String {
        let mut out = String::new();

        // raw binary asm header
        out.push_str("; ARCH x86_64\n");
        out.push_str("; Generated ASM file. Modifications will not be preserved\n");


        out.push_str("BITS 64\n");
        out.push_str("ORG 0x100000\n\n");

        // x86_64 bare metal entrypoint
        out.push_str("_start:\n");
        out.push_str("    call main\n");
        out.push_str(".halt:\n");
        out.push_str("    hlt\n");
        out.push_str("    jmp .halt\n\n");

        let funcs = Self::split_functions(ir);

        for (name, body) in funcs {
            self.emit_function(&mut out, &name, &body);
        }

        out
    }
}