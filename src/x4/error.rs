use core::ffi::c_void;
use core::result;
use crate::vdebug_autoprefix;
use crate::x4::externals::x4_ntcurrentteb;
use crate::x4::globals::{BreakCode, GlobalEtwEventRegister, _guard_check_icall_fptr};
use crate::x4::helpers::HResultToNtStatus;
use crate::x4::ops::__fastfail;
use crate::x4::types::{CONTEXT, _CONTEXT, _EXCEPTION_RECORD};

pub unsafe fn HandleSubsystemError(a1: i32) -> u64 {
    let result = BreakCode;

    if result != 0 && a1 < 0 && (result == a1 || result == -1) {
        vdebug_autoprefix!("progbreak int3 {result:X}");
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            core::arch::asm!("int3");
        }
        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        {
            core::intrinsics::breakpoint();
        }
    } else {
        vdebug_autoprefix!("progbreak noint {result:X}");
    } 

    result as u32 as u64
}

pub unsafe fn LogTraceEvent(a1: i32) -> i64 {
    let result_val = a1 as u64;
    let mut v3: u128 = 0;

    // Direct byte manipulation corresponding to BYTE2, BYTE4, and LOWORD assignments
    let v3_bytes = &mut v3 as *mut u128 as *mut u8;
    *v3_bytes.add(2) = 1;
    *v3_bytes.add(4) = 1;

    let v2: u64;
    if a1 < 0 {
        // LOWORD(v3) = 1
        *v3_bytes.add(0) = 1;
        *v3_bytes.add(1) = 0;
        v2 = 0x4000000000000000;
    } else {
        // LOWORD(v3) = 0
        *v3_bytes.add(0) = 0;
        *v3_bytes.add(1) = 0;
        v2 = 0x8000000000000000;
    }

    // Set upper 64-bit QWORD part of the 128-bit descriptor
    let upper_half = &mut v3 as *mut u128 as *mut u64;
    *upper_half.add(1) = result_val | v2;

    let reg_register = core::ptr::addr_of!(GlobalEtwEventRegister).read_volatile();
    let reg_handle = core::ptr::addr_of!(GlobalEtwRegHandle).read_volatile();

    if !reg_register.is_null() && reg_handle != 0 {
        _guard_check_icall_fptr();
        let enabled = GlobalEtwEventEnabled(reg_handle, &v3);
        if enabled != 0 {
            _guard_check_icall_fptr();
            return GlobalEtwEventWrite(reg_handle, &v3, 0, core::ptr::null());
        }
    }

    vdebug_autoprefix!("EventTrace {reg_handle:X}@{reg_register:X} -> {v3:X}");

    result_val as i64
}

pub unsafe fn TraceProviderEvent(
    callerReturnAddress: u64,
    telemetryEventId: u64,
    errorStatus: u64,
    passThru3: u64,
) -> u64 {
    let v5: u64 = 0;
    let retaddr: u64 = 0;

    TranslateAndNotifyError(
        callerReturnAddress,
        telemetryEventId,
        errorStatus,
        passThru3,
        v5,
        retaddr,
        passThru3,
    )
}

pub unsafe fn TranslateAndNotifyError(
    passThru0: u64,
    passThru1: u64,
    passThru2: u64,
    passThru3: u64,
    passThru4: u64,
    callerReturnAddress: u64,
    hresult: i32,
) -> u64 {
    // Convert HRESULT to NTSTATUS code
    let result = HResultToNtStatus(hresult);

    // Replicating local uninitialized variables v8 and v7 from decompiler output
    let v8: u64 = 0;
    let v7: u32 = 0;

    NotifyErrorSubsystem(result, callerReturnAddress, passThru0, passThru1, passThru2, passThru3, passThru4)
}

pub fn NotifyErrorSubsystem(p0: i64, p1: u64, p2: u64, p3: u64, p4: u64, p5: u64, p6: u64) -> u64 {
    vdebug_autoprefix!("error {p0:x} at {p1:x} with {p2:x} {p3:x} {p4:x} {p5:x} {p6:x}");
    p0 as u64
}

pub fn ReportAlignmentAssertionFailure() {
    __fastfail(7u32);
}

pub fn RaiseFastFailException(pExceptionRecord: _EXCEPTION_RECORD , pContext: CONTEXT ) {
    vdebug_autoprefix!("fastfail exception {pExceptionRecord} with {pContext}");
}

pub  fn TerminateExecutionStub() {
    RaiseFastFailException(_EXCEPTION_RECORD {
        ExceptionCode: 0,
        ExceptionFlags: 0,
        ExceptionRecord: (),
        ExceptionAddress: (),
        NumberParameters: 0,
        ExceptionInformation: [],
    });
}
pub unsafe fn HandleFatalRelocError() {
    let teb = x4_ntcurrentteb();
    let mut stack_limit = (*teb).nt_tib.stack_limit as *mut *const c_void;

    // Define the local stack variable anchor (v1)
    let v1 = TerminateExecutionStub as *const c_void;
    let v1_addr = &v1 as *const *const c_void;

    // Overwrite the stack memory up to the local anchor with null pointers
    while stack_limit < v1_addr {
        *stack_limit = core::ptr::null();
        stack_limit = stack_limit.add(1);
    }

    // Transmute and invoke the terminal execution stub function pointer
    let terminate_fn: extern "fastcall" fn(*const c_void, u64, u64, u64) -> ! =
        core::mem::transmute(TerminateExecutionStub as *const c_void);

    terminate_fn(TerminateExecutionStub as *const c_void, 0, 0, 0);
}