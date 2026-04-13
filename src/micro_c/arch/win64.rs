use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use hashbrown::HashMap;
use crate::arch::Architecture;
use crate::error::error;
use crate::ir::IRInst;
use crate::regalloc::RegisterAllocator;
use crate::stackframe::StackFrame;

pub struct WIN64Backend {
    regs: RegisterAllocator,
    function_params: HashMap<String, Vec<String>>,
}

impl WIN64Backend {
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

    fn is_temp(name: &str) -> bool {
        name.starts_with("t")
    }
}

impl Architecture for WIN64Backend {
    fn emit_program(&mut self, ir: &[IRInst]) -> String {
        let mut out = String::new();
        let arg_regs = ["rcx", "rdx", "r8", "r9"];

        out.push_str("global main\n");
        out.push_str("extern printf\n");

        out.push_str("section .data\n");
        out.push_str("fmt: db \"%lld\", 10, 0\n");

        out.push_str("section .text\n\n");

        let mut frame = StackFrame::new();

        // preallocate stack slots
        for inst in ir {
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

        let frame_size = frame.frame_size();


        out.push_str("main:\n");
        out.push_str("    push rbx\n");
        out.push_str("    sub rsp, 40\n");
        out.push_str("    call micro_main\n");
        out.push_str("    mov rdx, rax\n");
        out.push_str("    lea rcx, [rel fmt]\n");
        out.push_str("    sub rsp, 32\n");
        out.push_str("    call printf\n");
        out.push_str("    add rsp, 32\n");
        out.push_str("    xor eax, eax\n");
        out.push_str("    add rsp, 40\n");
        out.push_str("    pop rbx\n");
        out.push_str("    ret\n\n");


        // emit instructions
        for inst in ir {
            match inst {


                // func label
                IRInst::Label(name) => {
                    let actual = if name == "main" {
                        "micro_main"
                    } else {
                        name
                    };

                    out.push_str(&format!("{}:\n", actual));
                    out.push_str("    push rbp\n");
                    out.push_str("    mov rbp, rsp\n");
                    out.push_str(&format!("    sub rsp, {}\n", (frame_size + 8)));

                    // Save incoming params
                    if let Some(params) = self.function_params.get(name) {
                        for param in params {
                            frame.alloc(param);
                        }

                        for (i, param) in params.iter().enumerate() {
                            if i >= arg_regs.len() {
                                error("Too many parameters for Windows ABI");
                                return "".to_string()
                            }

                            let off = frame.get(param);
                            // println!("PARAM {:?}, {:?}", param, arg_regs[i]);
                            out.push_str(&format!(
                                "    mov [rbp-{}], {}\n",
                                off,
                                arg_regs[i]
                            ));
                        }
                    }
                }

                IRInst::LoadConst(dst, val) => {
                    let rd = self.regs.alloc(dst);
                    out.push_str(&format!("    mov {}, {}\n", rd, val));
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

                    out.push_str(&format!("    mov {}, {}\n", rd, ra));
                    out.push_str(&format!("    add {}, {}\n", rd, rb));
                }

                IRInst::Sub(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    mov {}, {}\n", rd, ra));
                    out.push_str(&format!("    sub {}, {}\n", rd, rb));
                }

                IRInst::Call(dst, func, args) => {
                    let arg_regs = ["rcx", "rdx", "r8", "r9"];

                    for (i, arg) in args.iter().enumerate() {
                        let r = self.regs.alloc(arg);
                        out.push_str(&format!(
                            "    mov {}, {}\n",
                            arg_regs[i],
                            r
                        ));
                    }

                    out.push_str("    sub rsp, 40\n");
                    out.push_str(&format!("    call {}\n", func));
                    out.push_str("    add rsp, 40\n");

                    let rd = self.regs.alloc(dst);
                    out.push_str(&format!("    mov {}, rax\n", rd));
                }

                IRInst::Return(src) => {
                    let rs = self.regs.alloc(src);

                    out.push_str(&format!("    mov rax, {}\n", rs));
                    out.push_str("    mov rsp, rbp\n");
                    out.push_str("    pop rbp\n");
                    out.push_str("    ret\n");
                }

                IRInst::Eq(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    cmp {}, {}\n", ra, rb));
                    out.push_str("    sete al\n");
                    out.push_str("    movzx rax, al\n");
                    out.push_str(&format!("    mov {}, rax\n", rd));
                }

                IRInst::Neq(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    cmp {}, {}\n", ra, rb));
                    out.push_str("    setne al\n");
                    out.push_str("    movzx rax, al\n");
                    out.push_str(&format!("    mov {}, rax\n", rd));
                }

                IRInst::Lt(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    cmp {}, {}\n", ra, rb));
                    out.push_str("    setl al\n");
                    out.push_str("    movzx rax, al\n");
                    out.push_str(&format!("    mov {}, rax\n", rd));
                }

                IRInst::Gt(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    cmp {}, {}\n", ra, rb));
                    out.push_str("    setg al\n");
                    out.push_str("    movzx rax, al\n");
                    out.push_str(&format!("    mov {}, rax\n", rd));
                }

                IRInst::JumpIfZero(cond, label) => {
                    let rc = self.regs.alloc(cond);

                    out.push_str(&format!("    cmp {}, 0\n", rc));
                    out.push_str(&format!("    je {}\n", label));
                }

                IRInst::Jump(label) => {
                    out.push_str(&format!("    jmp {}\n", label));
                }

                _ => {}
            }
        }

        out
    }
}