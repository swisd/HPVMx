use crate::vdebug_autoprefix;
use crate::x4::types::{BYTE, DWORD, WORD};

#[inline]
pub unsafe fn __outbyte(port: u16, value: u8) {
    vdebug_autoprefix!("OUTBYTE p{port:x} v{value:x}");
    core::arch::asm!(
    "out dx, al",
    in("dx") port,
    in("al") value,
    options(nomem, nostack, preserves_flags)
    );
}

#[inline]
pub unsafe fn __inbyte(port: u16) -> BYTE {
    vdebug_autoprefix!("INBYTE p{port:x}");
    let result: u8;
    core::arch::asm!(
    "in al, dx",
    out("al") result,
    in("dx") port,
    options(nomem, nostack, preserves_flags)
    );
    result
}


#[inline]
pub unsafe fn __outword(port: u16, value: u16) {
    vdebug_autoprefix!("OUTWORD p{port:x} v{value:x}");
    core::arch::asm!(
    "out dx, ax",
    in("dx") port,
    in("ax") value,
    options(nomem, nostack, preserves_flags)
    );
}


#[inline]
pub unsafe fn __outdword(port: u16, value: u32) {
    vdebug_autoprefix!("OUTDWORD p{port:x} v{value:x}");
    core::arch::asm!(
    "out dx, eax",
    in("dx") port,
    in("eax") value,
    options(nomem, nostack, preserves_flags)
    );
}


#[inline]
pub unsafe fn __inword(port: u16) -> WORD {
    vdebug_autoprefix!("INWORD p{port:x}");
    let result: u16;
    core::arch::asm!(
    "in ax, dx",
    out("ax") result,
    in("dx") port,
    options(nomem, nostack, preserves_flags)
    );
    result
}


#[inline]
pub unsafe fn __indword(port: u16) -> DWORD {
    vdebug_autoprefix!("INDWORD p{port:x}");
    let result: u32;
    core::arch::asm!(
    "in eax, dx",
    out("eax") result,
    in("dx") port,
    options(nomem, nostack, preserves_flags)
    );
    result
}