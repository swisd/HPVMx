use core::arch::asm;
use crate::vdebug_autoprefix;
use crate::x4::io::{__inbyte, __outbyte, __outdword};
use crate::x4::nullsub::MK_FP;
use crate::x4::ops::{HIDWORD, _QWORD, __PAIR64__, __ROR1__, __ROR4__};

static mut GLOBAL_HALT_ENABLED: bool = false;

pub fn JUMPOUT(addr: u64){
    vdebug_autoprefix!("JUMPOUT({addr})")
}

pub fn __halt() {
    unsafe {
        if GLOBAL_HALT_ENABLED {
            vdebug_autoprefix!("HALT");
            core::intrinsics::abort();
        } else {
            vdebug_autoprefix!("HALT NE")
        }
    }
}

pub unsafe fn assign_a1_off_longpos_ror4_a1_off_longpos_a1(a1: i64) {
    let ptr = a1.wrapping_add(1_737_016_461) as *mut u32;
    *ptr = __ROR4__(*ptr, a1 as i32);
}

pub unsafe fn call_a2(a1: i64, a2: unsafe fn(i64) -> i64) -> i64 {
    a2(a1)
}

pub unsafe fn call_with_sign_flag(a1: i64) -> i64 {
    let mut v1: i32 = core::mem::zeroed();
    let mut retaddr: [u8; 16] = core::mem::zeroed();

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(i64, u32) -> i64 = core::mem::transmute(raw_ptr);
    func(a1, (v1 >> 31) as u32)
}

pub unsafe fn farptr_stub_segment_at_addr() -> i64 {
    let mut retaddr: [u8; 16] = core::mem::zeroed();
    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn() -> i64 = core::mem::transmute(raw_ptr);
    func()
}

pub unsafe fn farptr_stub_segment_at_addr_onearg() -> i64 {
    let mut v0: u32 = core::mem::zeroed();
    let mut retaddr: [u8; 16] = core::mem::zeroed();
    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(u32) -> i64 = core::mem::transmute(raw_ptr);
    func(v0)
}

pub unsafe fn insd_farptr_at_retaddr_twoarg_v3_rdx(a1: i64, rdx: i64) -> i64 {
    let mut v3: u64 = core::mem::zeroed();
    let mut retaddr: [u8; 16] = core::mem::zeroed();

    asm!("insd", options(nomem, nostack, preserves_flags));

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(u64, i64) -> i64 = core::mem::transmute(raw_ptr);
    func(v3, rdx)
}

pub unsafe fn iret_if_v0_and_dd_is_zero() {
    let mut v0: u8 = core::mem::zeroed();

    if (v0 & 0xDD) == 0 {
        asm!("iret", options(noreturn));
    }
    JUMPOUT(0x14032750B_u64);
}

pub unsafe fn rcl_dword_ptr_a1_rcx(a1: i64) {
    let mut v1: bool = core::mem::zeroed();
    let rcx = a1.wrapping_sub(1);

    if !v1 || rcx == 0 {
        __halt()
    }

    asm!("rcl dword ptr [{rcx}], 1", rcx = in(reg) rcx);
    JUMPOUT(0x14027EB84_u64);
}

pub unsafe fn return_a1_and_a2_as_ptr(a1: *mut u8, a2: i8) {
    *a1 &= a2 as u8;
}

pub unsafe fn return_a1_as_int_ptr_off_24(a1: i64) -> i64 {
    *(a1.wrapping_add(24) as *const u32) as i64
}

pub unsafe fn return_a1_as_qword_ptr(a1: i64) -> _QWORD {
    *(a1 as *const _QWORD)
}

pub unsafe fn return_a1_off8_as_qword_ptr(a1: i64) -> _QWORD {
    *(a1.wrapping_add(8) as *const _QWORD)
}

pub unsafe fn return_a1_off_n30_rshift_a1_as_int(a1: i64) {
    *(a1.wrapping_sub(30) as *mut i32) >>= a1;
}

pub unsafe fn return_farptr_retaddr_noarg_al_inbyte_outbyte() -> i64 {
    let mut retaddr: [u8; 16] = core::mem::zeroed();
    let mut al = __inbyte(0x14);

    asm!("xlat", inout("al") al);
    __outbyte(0xEF, al);

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn() -> i64 = core::mem::transmute(raw_ptr);
    func()
}

pub unsafe fn return_farptr_retaddr_onearg_a1_with_byte1_sub_v1_off100_as_byte(mut a1: i64) -> i64 {
    let v1: i64 = core::mem::zeroed();
    let mut retaddr: [u8; 16] = core::mem::zeroed();

    // Perform the BYTE1 math inline (extracting byte 1, subtracting, and clearing/updating that byte in a1)
    let mut b1 = ((a1 >> 8) & 0xFF) as u8;
    b1 = b1.wrapping_sub(*(v1.wrapping_add(100) as *const u8));
    a1 = (a1 & !(0xFF << 8)) | ((b1 as i64) << 8);

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(i64) -> i64 = core::mem::transmute(raw_ptr);
    func(a1)
}

pub unsafe fn return_farptr_retaddr_onearg_a1_with_lobyte_as_20dec(mut a1: i64) -> i64 {
    let mut retaddr: [u8; 16] = core::mem::zeroed();

    a1 = (a1 & !0xFF) | 20;

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(i64) -> i64 = core::mem::transmute(raw_ptr);
    func(a1)
}

pub unsafe fn return_farptr_retaddr_onearg_a1_with_outdword_0x90_v2(mut a1: i64, a2: i8) -> i64 {
    let v2: u32 = core::mem::zeroed();
    let mut retaddr: [u8; 16] = core::mem::zeroed();

    __outdword(0x90, v2);

    let b1 = (((a1 >> 8) & 0xFF) as i8 & a2) as i64;
    a1 = (a1 & !(0xFF << 8)) | (b1 << 8);

    let raw_ptr = MK_FP(*(retaddr.as_ptr() as *const u16), *(retaddr.as_ptr() as *const u64));
    let func: unsafe fn(i64) -> i64 = core::mem::transmute(raw_ptr);
    func(a1)
}

pub unsafe fn return_farptr_seg_a2_off125_off_a2_off121_noarg(a1: i64, a2: i64) -> i64 {
    let raw_ptr = MK_FP(
        *(a2.wrapping_add(125) as *const u16),
        *(a2.wrapping_add(121) as *const u32) as u64,
    );
    let func: unsafe fn() -> i64 = core::mem::transmute(raw_ptr);
    func()
}

pub unsafe fn return_none_a3_off61_xor_v3_halting(a1: i64, a2: i64, a3: i64) /*-> !*/ {
    let v3: i32 = core::mem::zeroed();
    *(a3.wrapping_add(61) as *mut i32) ^= v3;
    __halt();
}


pub unsafe fn return_none_outbyte_multiassign_halting(a1: i8, a2: u16) /*-> !*/ {
    let v2: i8 = core::mem::zeroed();
    let v3: *mut u8 = core::mem::zeroed();
    let v4: *mut u8 = core::mem::zeroed();
    let v5: *mut u8 = core::mem::zeroed();

    *v4 = *v5;
    *v3 = (*v3).wrapping_add((a1.wrapping_add(v2)) as u8);
    __outbyte(a2, v3 as usize as u8);
    __halt();
}


pub unsafe fn return_ptr_a2_ror1_a2_ptr_a1(a1: i8, a2: *mut u8) /*-> !*/ {
    *a2 = __ROR1__(*a2, a1 as i32);
    JUMPOUT(0x1295124E7_u64);
}


pub unsafe fn return_v1_off4_by_a1_and_a1byte1(a1: i64) {
    let v1: i64 = core::mem::zeroed();
    let target_ptr = v1.wrapping_add(a1.wrapping_mul(4)) as *mut u8;
    *target_ptr &= ((a1 >> 8) & 0xFF) as u8;
}


pub unsafe fn return_v1_off5_xor_v0_as_byte_ptr() {
    let v0: i8 = core::mem::zeroed();
    let v1: i64 = core::mem::zeroed();

    *(v1.wrapping_add(5) as *mut u8) ^= v0 as u8;
}

pub unsafe fn return_v1_off79_or_v0_as_dword() /*-> !*/ {
    let v0: i32 = core::mem::zeroed();
    let v1: i64 = core::mem::zeroed();

    *(v1.wrapping_add(79) as *mut u32) |= v0 as u32
    //__halt()
}

pub unsafe fn return_zero_assign_a1_off104_to_a2(a1: i64, a2: i32) -> i64 {
    *(a1.wrapping_add(104) as *mut i32) = a2;
    0
}

pub unsafe fn return_zero_assign_a1_off40_to_a2(a1: i64, a2: i32) -> i64 {
    *(a1.wrapping_add(40) as *mut i32) = a2;
    0
}

pub unsafe fn return_zero_dec_a1_off104_as_dword(a1: i64) -> i64 {
    let ptr = a1.wrapping_add(104) as *mut i32;
    *ptr = (*ptr).wrapping_sub(1);
    0
}

pub unsafe fn return_zero_dec_a1_off48_as_dword(a1: i64) -> i64 {
    let ptr = a1.wrapping_add(48) as *mut i32;
    *ptr = (*ptr).wrapping_sub(1);
    0
}

pub unsafe fn send_port_result_data(result: i64, a2: u16) -> i64 {
    __outbyte(a2, result as u8);
    result
}

pub unsafe fn set_result_firstbyte_v0() -> i64 {
    let v0: i8 = core::mem::zeroed();
    let mut result: i64 = core::mem::zeroed();

    result = (result & !(0xFF << 8)) | (((v0 as u8) as i64) << 8);
    result
}

pub unsafe fn shift_int_by_address_byte(a1: *mut i32) {
    *a1 >>= a1 as usize as u8;
}

pub unsafe fn v0_off18_assign_v1() {
    let v0: i64 = core::mem::zeroed();
    let v1: i32 = core::mem::zeroed();

    *(v0.wrapping_add(18) as *mut i32) = v1;
    asm!("ret", options(noreturn));
}

pub unsafe fn v0_or_816b72d7() -> i64 {
    let v0: i32 = core::mem::zeroed();
    (v0 | 0x816B72D7_u32 as i32) as i64
}

pub unsafe fn v2_sub_107_min_v0_v1_as_byte_halt() /*-> !*/ {
    let v0: i8 = core::mem::zeroed();
    let v1: i8 = core::mem::zeroed();
    let v2: i64 = core::mem::zeroed();

    let ptr = v2.wrapping_sub(107) as *mut u8;
    *ptr = (*ptr).wrapping_sub((v0.wrapping_add(v1)) as u8);
    __halt();
}

pub unsafe fn v4_pair64_offset_struct_assign(a1: i64) {
    let v1: i64 = core::mem::zeroed();
    let v2: u32 = core::mem::zeroed();
    let v3: u32 = core::mem::zeroed();

    let target_ptr = v1.wrapping_add(a1.wrapping_mul(4)) as *mut u32;
    let base_val = (v3 as u64) | 0xFECE2B4A00000000_u64;
    let v4 = base_val.wrapping_add(__PAIR64__(*target_ptr, v2));

    *target_ptr = HIDWORD!(v4) as u32;
    JUMPOUT(0x14009894C_u64);
}
