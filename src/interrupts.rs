// use x86_64::structures::idt::{Entry, HandlerFunc, HandlerFuncWithErrCode, InterruptStackFrame, PageFaultHandlerFunc, InterruptDescriptorTable};
// use crate::imx;
//
// macro_rules! message {
//     ($start:expr, $($arg:tt)*) => {
//         uefi::system::with_stdout(|stdout| {
//             use core::fmt::Write;
//             let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
//             let _ = write!(stdout, $start);
//             let _ = write!(stdout, $($arg)*);
//             let _ = write!(stdout, "\n");
//         })
//     }
// }
//
// struct IDT {
//     __private_field: (),
// }
// static IDT: IDT = IDT { __private_field: () };
//
// pub fn init_idt() {
//     IDT.load();
// }
//
// extern "x86-interrupt" fn breakpoint_handler(
//     stack_frame: InterruptStackFrame)
// {
//     message!("\n", "EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
// }