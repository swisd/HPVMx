use x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable, PageFaultErrorCode};
use crate::{hpvm_error, hpvm_log, gdt};
use uefi::proto::console::text::Color;
use core::ptr::addr_of_mut;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        let idt_ptr = addr_of_mut!(IDT);
        (*idt_ptr).breakpoint.set_handler_fn(breakpoint_handler);
        (*idt_ptr).double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        (*idt_ptr).general_protection_fault.set_handler_fn(general_protection_fault_handler);
        (*idt_ptr).page_fault.set_handler_fn(page_fault_handler);
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

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    hpvm_error!("EXCEPTION", "GENERAL PROTECTION FAULT: {}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    hpvm_error!("EXCEPTION", "PAGE FAULT: {:?}\nAccessed Address: {:?}\n{:#?}", 
        error_code, Cr2::read(), stack_frame);
}