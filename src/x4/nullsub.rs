use core::ffi::c_void;

pub const fn MK_FP(seg: u16, off: u64) -> *mut c_void {
    (((seg as usize) << 16) | (off as usize)) as *mut c_void
}

//nullsub_104
pub unsafe fn nullsub_fp_retaddr() -> i64 {
    let mut retaddr: [u8; 16] = [0; 16];

    let seg = *(retaddr.as_ptr() as *const u16);
    let off = *(retaddr.as_ptr() as *const u64);

    let fn_ptr = MK_FP(seg, off);
    let code_fn: fn() -> i64 = core::mem::transmute(fn_ptr);
    code_fn()
}


// nullsub_15
pub unsafe fn nullsub_iret() {
    core::arch::asm!("iret");
}

//nullsub_1
pub unsafe fn nullsub() {
    ;
}