use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{x4_getprocessheap, x4_heapalloc, x4_heapfree, x4__vsnwprintf};
use crate::x4::spp::string::{SppDuplicateString, SppStringCchLengthW, SppStringCchVPrintfW};
use crate::x4::types::{HRESULT, SIZE_T};

pub unsafe fn SppFormatStringAlloc(
    ppszDestination: *mut *mut u16,
    pszFormat: *const u16,
    argList: *mut c_void,
) -> HRESULT {
    let mut v5: *mut u16 = core::ptr::null_mut();
    let mut v6: u32 = 0;
    let mut v7: *mut u16 = core::ptr::null_mut();
    let mut v10: HRESULT;
    let mut v11: u32;

    let mut cchLength: [u32; 2] = [0, 0];
    let mut ppszDestinationa: *mut u16 = core::ptr::null_mut();
    let mut buffer: [u16; 260] = [0; 260];

    let v8 = x4__vsnwprintf(buffer.as_mut_ptr(), 0x103, pszFormat, argList);

    if v8 >= 0x104 {
        buffer[259] = 0;
        v11 = 260;

        loop {
            v10 = 0;
            if v11 != 0 {
                if (2 * v11) >> 1 == v11 {
                    v11 *= 2;
                } else {
                    v10 = -2147024362; // INTSAFE_E_ARITHMETIC_OVERFLOW (0x80070216)
                    HandleSubsystemError(-2147024362);
                }
            } else {
                v11 = 0;
            }

            LogTraceEvent(v10);
            if v10 < 0 {
                break;
            }

            v10 = 0;
            if v11 != 0 {
                if (2 * v11) >> 1 == v11 {
                    v6 = 2 * v11;
                } else {
                    v10 = -2147024362;
                    HandleSubsystemError(-2147024362);
                }
            } else {
                v6 = 0;
            }

            LogTraceEvent(v10);
            if v10 < 0 {
                break;
            }

            if !v5.is_null() {
                let process_heap = x4_getprocessheap();
                x4_heapfree(process_heap, 0, v5 as *mut c_void);
            }

            let process_heap = x4_getprocessheap();
            let v14 = x4_heapalloc(process_heap, 0, v6 as usize as SIZE_T) as *mut u16;
            v5 = v14;

            if v14.is_null() {
                v10 = -2147024882; // E_OUTOFMEMORY (0x8007000E)
                v5 = core::ptr::null_mut();
                break;
            }

            let v15 = SppStringCchVPrintfW(v14, v11 as usize, pszFormat, argList);
            v10 = v15;

            if v15 != -2147024774 { // STRSAFE_E_INSUFFICIENT_BUFFER (0x8007007A)
                if v15 < 0 {
                    HandleSubsystemError(v15);
                    break;
                }

                cchLength[0] = 0;
                let v17 = SppStringCchLengthW(
                    v5,
                    *(cchLength.as_mut_ptr() as *mut usize),
                    0x7FFFFFFF as *mut usize,
                );
                v10 = v17;

                if v17 >= 0 {
                    let v18 = SppDuplicateString(v5, cchLength[0], &mut ppszDestinationa);
                    v10 = v18;
                    if v18 < 0 {
                        HandleSubsystemError(v18);
                    }
                    v7 = ppszDestinationa;
                } else {
                    HandleSubsystemError(v17);
                }

                LogTraceEvent(v10);

                if v10 >= 0 {
                    v10 = 0;
                    if !ppszDestination.is_null() {
                        *ppszDestination = v7;
                    }
                    v7 = core::ptr::null_mut();
                    break;
                }
                break;
            }
        }
    } else {
        if v8 == 259 {
            buffer[259] = 0;
        }

        cchLength[0] = 0;
        let mut v9 = SppStringCchLengthW(
            buffer.as_mut_ptr(),
            *(cchLength.as_mut_ptr() as *mut usize),
            0x7FFFFFFF as *mut usize,
        );
        v10 = v9;

        if v9 >= 0 {
            v9 = SppDuplicateString(buffer.as_mut_ptr(), cchLength[0], ppszDestination);
            v10 = v9;
        }

        if v9 < 0 {
            HandleSubsystemError(v9);
        }

        LogTraceEvent(v10);
    }

    LogTraceEvent(v10);

    if !v7.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v7.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    if !v5.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v5 as *mut c_void);
    }

    v10
}

pub unsafe fn SppFormatString(
    ppszDestination: *mut *mut u16,
    pszFormat: *const u16,
    mut args: ...
) -> HRESULT {
    let va = args.as_va_list();
    let v2 = SppFormatStringAlloc(ppszDestination, pszFormat, va.as_arg_ptr() as *mut c_void);
    let v3 = v2;

    if v2 < 0 {
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v3);
    v3
}

