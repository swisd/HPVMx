use core::ffi::c_void;

pub const fn MK_FP(seg: u16, off: u64) -> *mut c_void {
    (((seg as usize) << 16) | (off as usize)) as *mut c_void
}

pub unsafe extern "C" fn nullsub_104() -> i64 {
    let mut retaddr: [u8; 16] = [0; 16];

    let seg = *(retaddr.as_ptr() as *const u16);
    let off = *(retaddr.as_ptr() as *const u64);

    let fn_ptr = MK_FP(seg, off);
    let code_fn: extern "C" fn() -> i64 = core::mem::transmute(fn_ptr);
    code_fn()
}