use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};
use crate::{hpvm_error, hpvm_log};
use uefi::proto::console::text::Color;
use core::ptr::addr_of_mut;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        let idt_ptr = addr_of_mut!(IDT);
        (*idt_ptr).breakpoint.set_handler_fn(breakpoint_handler);
        (*idt_ptr).double_fault.set_handler_fn(double_fault_handler);
        (*idt_ptr).load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    hpvm_error!("EXCEPTION", "BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    hpvm_error!("EXCEPTION", "DOUBLE FAULT\n{:#?}", stack_frame);
    panic!("DOUBLE FAULT");
}