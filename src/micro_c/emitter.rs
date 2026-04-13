// use alloc::string::{String, ToString};
// use crate::backend::TargetSpec;
// use crate::ir::*;
// use crate::regalloc::RegisterAllocator;
// 
// pub fn emit_asm(ir: &[IRInst], target: &TargetSpec) -> String {
//     let mut out = String::new();
//     let mut regs = RegisterAllocator::new(target.registers.clone());
// 
//     for inst in ir {
//         match inst {
//             IRInst::LoadConst(dst, val) => {
//                 let r = regs.alloc(dst);
//                 let tmpl = &target.instructions["LoadConst"];
// 
//                 out.push_str(
//                     &tmpl.replace("{dst}", &r)
//                         .replace("{value}", &val.to_string())
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Add(dst, a, b) => {
//                 let rd = regs.alloc(dst);
//                 let ra = regs.alloc(a);
//                 let rb = regs.alloc(b);
// 
//                 let tmpl = &target.instructions["Add"];
// 
//                 out.push_str(
//                     &tmpl.replace("{dst}", &rd)
//                         .replace("{a}", &ra)
//                         .replace("{b}", &rb)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Sub(dst, a, b) => {
//                 let rd = regs.alloc(dst);
//                 let ra = regs.alloc(a);
//                 let rb = regs.alloc(b);
// 
//                 let tmpl = &target.instructions["Sub"];
// 
//                 out.push_str(
//                     &tmpl.replace("{dst}", &rd)
//                         .replace("{a}", &ra)
//                         .replace("{b}", &rb)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Mul(dst, a, b) => {
//                 let rd = regs.alloc(dst);
//                 let ra = regs.alloc(a);
//                 let rb = regs.alloc(b);
// 
//                 let tmpl = &target.instructions["Mul"];
// 
//                 out.push_str(
//                     &tmpl.replace("{dst}", &rd)
//                         .replace("{a}", &ra)
//                         .replace("{b}", &rb)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Div(dst, a, b) => {
//                 let rd = regs.alloc(dst);
//                 let ra = regs.alloc(a);
//                 let rb = regs.alloc(b);
// 
//                 let tmpl = &target.instructions["Div"];
// 
//                 out.push_str(
//                     &tmpl.replace("{dst}", &rd)
//                         .replace("{a}", &ra)
//                         .replace("{b}", &rb)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Return(src) => {
//                 let rs = regs.alloc(src);
//                 let tmpl = &target.instructions["Return"];
// 
//                 out.push_str(
//                     &tmpl.replace("{src}", &rs)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Label(name) => {
//                 let tmpl = &target.instructions["Label"];
//                 out.push_str(
//                     &tmpl.replace("{name}", name)
//                 );
//                 out.push('\n');
//             }
// 
//             IRInst::Jump(label) => {
//                 let tmpl = &target.instructions["Jump"];
//                 out.push_str(
//                     &tmpl.replace("{label}", label)
//                 );
//                 out.push('\n');
//             }
// 
//             _ => {}
//         }
//     }
// 
//     out
// }