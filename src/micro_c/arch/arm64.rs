use alloc::{format, vec};
use alloc::string::String;
use crate::ir::IRInst;
use crate::regalloc::RegisterAllocator;
use crate::arch::Architecture;

pub struct ARM64Backend {
    regs: RegisterAllocator,
}

impl ARM64Backend {
    pub fn new() -> Self {
        Self {
            regs: RegisterAllocator::new(vec![
                "x0".into(),
                "x1".into(),
                "x2".into(),
                "x3".into(),
            ])
        }
    }
}

impl Architecture for ARM64Backend {
    fn emit_program(&mut self, ir: &[IRInst]) -> String {
        let mut out = String::new();

        for inst in ir {
            match inst {
                IRInst::Label(name) => {
                    out.push_str(&format!("{}:\n", name));
                }

                IRInst::LoadConst(dst, val) => {
                    let rd = self.regs.alloc(dst);
                    out.push_str(&format!("    mov {}, #{}\n", rd, val));
                }

                IRInst::Add(dst, a, b) => {
                    let rd = self.regs.alloc(dst);
                    let ra = self.regs.alloc(a);
                    let rb = self.regs.alloc(b);

                    out.push_str(&format!("    add {}, {}, {}\n", rd, ra, rb));
                }

                IRInst::Return(src) => {
                    let rs = self.regs.alloc(src);
                    out.push_str(&format!("    mov x0, {}\n", rs));
                    out.push_str("    ret\n");
                }

                _ => {}
            }
        }

        out
    }
}