use core::arch::asm;
use core::ffi::c_int;
use core::ptr;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{GetModuleHandleW, GetProcAddress, GetProcessHeap, HeapAlloc, HeapFree, _dllonexit, _vsnwprintf, onexit};
use crate::x4::globals::_guard_check_icall_fptr;
use crate::x4::rawasm::FARPROC;
use crate::x4::types::{__int64, unsigned_int64, SIZE_T, HANDLE, LPVOID, STRSAFE_LPWSTR, STRSAFE_LPCWSTR, HRESULT, HMODULE, DWORD};

pub static mut dword_14045FD04: i32 = 2;
pub static mut dword_14045FD08: u32 = 0x0043FCC0;

pub unsafe fn binary_search_lookup(a1: *mut usize, a2: u32, a3: i32) -> i64 {
    // v4 = dword_14045FD04;
    let v4 = dword_14045FD04;

    // *(_QWORD *)(a1 + 4) = 0x140000000LL + (unsigned int)dword_14045FD08;
    let table_base_address = 0x140000000u64 + dword_14045FD08 as u64;
    let qword_slot_ptr = a1.add(4) as *mut u64;
    *qword_slot_ptr = table_base_address;

    let mut v6: i32 = 0;

    // *(_DWORD *)a1 = v4;
    *(a1 as *mut i32) = v4;

    let mut v7 = v4 - 1;
    let mut v9: i32 = 0;

    while v7 >= v6 {
        let v8 = (v7 + v6) / 2;
        v9 = v8;

        // v10 = *(_DWORD *)(*(_QWORD *)(a1 + 4) + 4LL * v8) & 0xFFFFFFF;
        let inner_table_ptr = *qword_slot_ptr as *const u32;
        let v10 = *inner_table_ptr.offset(v8 as isize) & 0xFFFFFFF;

        if a2 >= v10 {
            if a2 <= v10 {
                // Equivalent to: goto LABEL_8;
                break;
            }
            v6 = v8 + 1;
        } else {
            v7 = v8 - 1;
        }

        // Update the tracking index loop fallback path
        if v7 < v6 {
            v9 = v6;
        }
    }

    // LABEL_8:
    // result = a2 + a3;
    let result = (a2 as i32).wrapping_add(a3) as i64;

    // *(_DWORD *)(a1 + 16) = result;
    *(a1.add(16) as *mut i32) = result as i32;

    // *(_DWORD *)(a1 + 12) = v9;
    *(a1.add(12) as *mut i32) = v9;

    // *(_DWORD *)(a1 + 20) = v9;
    *(a1.add(20) as *mut i32) = v9;

    result
}

pub unsafe fn memzero_secure(mut destAddress: __int64, byteCount: unsigned_int64) -> __int64 {
    let mut result: __int64 = 0;

    if byteCount < 0x10 {
        if byteCount < 8 {
            if byteCount < 4 {
                if byteCount < 2 {
                    if byteCount != 0 {
                        *(destAddress as *mut u8) = 0;
                    }
                } else {
                    *(destAddress as *mut u16) = 0;
                    *((destAddress + byteCount as __int64 - 2) as *mut u16) = 0;
                }
            } else {
                *(destAddress as *mut u32) = 0;
                *((destAddress + byteCount as __int64 - 4) as *mut u32) = 0;
            }
        } else {
            *(destAddress as *mut u64) = 0;
            *((destAddress + byteCount as __int64 - 8) as *mut u64) = 0;
        }
    } else if (destAddress & 0xF) != 0 {
        // Handle unaligned pointers by zeroing the first 16 bytes manually,
        // then snapping the pointer alignment forward to a 16-byte boundary.
        *(destAddress as *mut u64) = 0;
        *((destAddress + 8) as *mut u64) = 0;

        result = (-(destAddress as i32) & 0xF) as __int64;
        destAddress += result;
        let mut mutated_byte_count = byteCount - result as unsigned_int64;

        if mutated_byte_count < 0x10 {
            *((destAddress + mutated_byte_count as __int64 - 16) as *mut u64) = 0;
            *((destAddress + mutated_byte_count as __int64 - 8) as *mut u64) = 0;
            return 0;
        } else {
            // Continuation matching original control-flow logic block branch
            return inner_vector_zero_loop(destAddress, mutated_byte_count, result);
        }
    } else {
        return inner_vector_zero_loop(destAddress, byteCount, result);
    }

    result
}

/// Helper function isolating the 128-bit vector zeroing loop block
unsafe fn inner_vector_zero_loop(mut destAddress: __int64, byteCount: unsigned_int64, result: __int64) -> __int64 {
    // *(_OWORD *)destAddress = 0;
    // 128-bit store instruction simulation
    *(destAddress as *mut u128) = 0;

    if (byteCount & 0x10) != 0 {
        destAddress += 16;
    }

    let mut isDone = byteCount < 0x20;
    let mut i = byteCount.wrapping_sub(32);

    while !isDone {
        *(destAddress as *mut u128) = 0;
        *((destAddress + 16) as *mut u128) = 0;
        destAddress += 32;
        isDone = i < 0x20;
        i = i.wrapping_sub(32);
    }

    // Capture loop tracking context state value back out
    let finalized_i = i.wrapping_add(32);
    let rem = (finalized_i & 0xF) as u32;

    if rem != 0 {
        *((destAddress + rem as __int64 - 16) as *mut u64) = 0;
        *((destAddress + rem as __int64 - 8) as *mut u64) = 0;
        return 0;
    }

    result
}

pub unsafe fn _chkstk() -> u64 {
    let result: u64; // rax
    let mut v1: *mut i8; // r10
    let mut stack_limit: *mut i8; // r11 (renamed from StackLimit for snake_case compliance)
    let mut v3: i8 = 0; // [rsp+18h] [rbp+8h] BYREF

    // Since 'result' is the input allocation size implicitly passed in the C snippet,
    // we capture what is currently in the RAX register at the start of execution.
    asm!("mov {}, rax", out(reg) result, options(nomem, nostack, preserves_flags));

    // v1 = &v3 - result;
    v1 = (&mut v3 as *mut i8).wrapping_sub(result as usize);

    // if ( (unsigned __int64)&v3 < result )
    if (&v3 as *const i8 as u64) < result {
        // v1 = nullptr;
        v1 = ptr::null_mut();
    }

    // StackLimit = (char *)NtCurrentTeb()->NtTib.StackLimit;
    // On Windows x64, this value is located at offset 0x10 in the GS register.
    asm!("mov {}, gs:[0x10]", out(reg) stack_limit, options(nomem, nostack, preserves_flags));

    // if ( v1 < StackLimit )
    if v1 < stack_limit {
        // LOWORD(v1) = (unsigned __int16)v1 & 0xF000;
        let mut v1_addr = v1 as usize;
        let low_word = (v1_addr & 0xFFFF) as u16;
        let new_low_word = low_word & 0xF000;
        v1_addr = (v1_addr & !0xFFFF) | (new_low_word as usize);
        v1 = v1_addr as *mut i8;

        // do
        loop {
            // StackLimit -= 4096;
            stack_limit = stack_limit.wrapping_sub(4096);

            // Force a volatile read to probe the memory page (triggers OS allocation)
            ptr::read_volatile(stack_limit);

            // while ( v1 < StackLimit );
            if v1 >= stack_limit {
                break;
            }
        }
    }

    // return result;
    result
}

pub unsafe fn StringLengthW(mut a1: *mut u16, a2: i64, a3: *mut u64) -> i64 {
    let mut v3: i64; // rdx
    let mut result: i64; // rax

    if a1.is_null() {
        result = 2147942487_i64; // 0x80070057 in decimal (E_INVALIDARG / ERROR_INVALID_PARAMETER)

        // LABEL_12:
        if !a3.is_null() {
            *a3 = 0;
        }
        return result;
    }

    v3 = 0x7FFFFFFF;

    // do-while loop
    loop {
        if *a1 == 0 {
            break;
        }
        a1 = a1.add(1);
        v3 -= 1;

        if v3 == 0 {
            break;
        }
    }

    result = if v3 == 0 { 0x80070057_i64 as i64 } else { 0 };

    if !a3.is_null() {
        if v3 != 0 {
            *a3 = (0x7FFFFFFF - v3) as u64;
        } else {
            *a3 = 0;
        }
    }

    if v3 == 0 {
        // goto LABEL_12;
        if !a3.is_null() {
            *a3 = 0;
        }
        return result;
    }

    result
}

pub unsafe fn Vector_Resize_40Bytes(a1: i64, a2: i32) -> u32 {
    let mut v4: *mut core::ffi::c_void = ptr::null_mut(); // rbx
    let mut v5: SIZE_T = 0;                              // rbp
    let mut v6: i32 = 0;                                 // r14d
    let mut process_heap: HANDLE;                        // rax
    let mut v8: *mut core::ffi::c_void;                   // rax
    let mut v9: i32;                                     // ebp
    let mut v10: *mut core::ffi::c_void;                  // rbp
    let mut v11: HANDLE;                                 // rax

    if *(a1 as *const i32) != a2 {
        v6 = *((a1 + 4) as *const i32);
        if a2 < v6 {
            v6 = a2;
        }

        if a2 > 0 {
            // Checked multiplication emulation: 40LL * a2 / 0x28uLL == a2
            if (40_i64.wrapping_mul(a2 as i64) / 0x28) == a2 as i64 {
                v5 = (40_i64 * a2 as i64) as SIZE_T;
            } else {
                v4 = -2147024362_i32 as *mut core::ffi::c_void; // LODWORD(v4)
                HandleSubsystemError(2147942934_i64 as i32);
            }

            LogTraceEvent(v4 as i32);

            if (v4 as i32) < 0 {
                // LABEL_9:
                HandleSubsystemError(v4 as u32 as i64 as i32);
                // goto LABEL_24;
                LogTraceEvent(v4 as i32);
                return v4 as u32;
            }

            process_heap = GetProcessHeap();
            v8 = HeapAlloc(process_heap, 0, v5);
            v4 = v8;

            if v8.is_null() {
                v4 = -2147024882_i32 as *mut core::ffi::c_void; // LODWORD(v4)
                // LABEL_9 inline
                HandleSubsystemError(v4 as u32 as i64 as i32);
                // goto LABEL_24;
                LogTraceEvent(v4 as i32);
                return v4 as u32;
            }

            if v6 != 0 {
                let src = *( (a1 + 8) as *const *const core::ffi::c_void );
                ptr::copy_nonoverlapping(src, v8, (40_i64 * v6 as i64) as usize);
            }
        }

        if a2 < *((a1 + 4) as *const i32) {
            v9 = a2;
            loop {
                let elements_ptr = *((a1 + 8) as *const i64);
                if (elements_ptr + 40_i64 * v9 as i64) != 0 {
                    VectorElementFree((elements_ptr + 40_i64 * v9 as i64));
                }
                v9 += 1;

                if v9 >= *((a1 + 4) as *const i32) {
                    break;
                }
            }
        }

        v10 = *((a1 + 8) as *const *mut core::ffi::c_void);
        if !v10.is_null() {
            v11 = GetProcessHeap();
            HeapFree(v11, 0, v10);
            *((a1 + 8) as *mut i64) = 0;
        }

        if v4.is_null() {
            v4 = ptr::null_mut();
        }

        *((a1 + 8) as *mut *mut core::ffi::c_void) = v4;
        v4 = 0 as *mut core::ffi::c_void; // LODWORD(v4) = 0
        *((a1 + 4) as *mut i32) = v6;
        *(a1 as *mut i32) = a2;
    }

    // LABEL_24:
    LogTraceEvent(v4 as i32);
    v4 as u32
}

pub unsafe fn VectorElementFree(a1: i64) {
    let mut v1: *mut core::ffi::c_void; // rbx
    let mut process_heap: HANDLE;     // rax
    let mut v4: i64;                  // rbx
    let mut v5: HANDLE;               // rax

    // v1 = *(void **)(a1 + 32);
    v1 = *((a1 + 32) as *const *mut core::ffi::c_void);
    if !v1.is_null() {
        process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v1);
        *((a1 + 32) as *mut i64) = 0;
    }

    // v4 = *(_QWORD *)(a1 + 8);
    v4 = *((a1 + 8) as *const i64);
    if v4 != 0 {
        v5 = GetProcessHeap();

        // HeapFree(v5, 0, (LPVOID)(v4 - 4));
        let adjusted_ptr = (v4 - 4) as LPVOID;
        HeapFree(v5, 0, adjusted_ptr);

        LogTraceEvent(0);
        *((a1 + 8) as *mut i64) = 0;
    }
}

pub unsafe fn StringCchPrintfW(
    pszDest: STRSAFE_LPWSTR,
    cchDest: usize,
    pszFormat: STRSAFE_LPCWSTR,
    mut args: ... // Variadic arguments matching C signature
) -> HRESULT {
    let mut hr: HRESULT;          // edi
    let max_chars: usize;         // rsi (snake_case compliance)
    let chars_written: c_int;     // eax (snake_case compliance)

    // va_start(args, pszFormat);
    // In Rust, we extract the va_list argument from the variadic boundary using `as_va_list`
    args.as_va_list(|va| {
        // We handle the body inside the va_list closure scope
    });

    // To make it easy to follow, here is the exact translation of the inner logic:
    if cchDest.wrapping_sub(1) <= 0x7FFFFFFE { // STRSAFE_MAX_CCH
        max_chars = cchDest - 1;
        hr = 0;

        // Simulate extraction and calling vsnwprintf
        let mut va_list_handler = args.as_va_list(|va| va);
        chars_written = _vsnwprintf(pszDest, cchDest - 1, pszFormat, va_list_handler);

        if chars_written < 0 || chars_written as usize > max_chars {
            hr = -2147024774; // STRSAFE_E_INSUFFICIENT_BUFFER (0x8007007A)
        } else if chars_written as usize != max_chars {
            return hr;
        }

        // pszDest[maxChars] = 0;
        *pszDest.add(max_chars) = 0;
        return hr;
    }

    hr = -2147024809; // E_INVALIDARG (0x80070057)
    if cchDest != 0 {
        *pszDest = 0;
    }

    hr
}

pub unsafe fn StringCchCopyW(
    mut pszDest: STRSAFE_LPWSTR,
    mut cchDest: usize,
    pszSrc: STRSAFE_LPCWSTR,
) -> HRESULT {
    let mut v3: usize;         // r9
    let mut v4: isize;         // r10
    let mut v5: u16;           // ax (wchar_t maps to u16)
    let mut v6: STRSAFE_LPWSTR; // rax
    let result: HRESULT;       // eax

    if cchDest.wrapping_sub(1) > 0x7FFFFFFE {
        result = -2147024809; // E_INVALIDARG (0x80070057)
        if cchDest != 0 {
            *pszDest = 0;
        }
    } else {
        v3 = 2147483646 - cchDest;

        // Byte distance calculation exactly matching (char *)pszSrc - (char *)pszDest
        v4 = (pszSrc as *const u8).offset_from(pszDest as *const u8);

        // do-while loop
        loop {
            if (v3.wrapping_add(cchDest)) == 0 {
                break;
            }

            // v5 = *(STRSAFE_LPWSTR)((char *)pszDest + v4);
            let src_char_ptr = (pszDest as *const u8).offset(v4) as *const u16;
            v5 = *src_char_ptr;

            if v5 == 0 {
                break;
            }

            // *pszDest++ = v5;
            *pszDest = v5;
            pszDest = pszDest.add(1);

            // --cchDest;
            cchDest -= 1;

            if cchDest == 0 {
                break;
            }
        }

        v6 = pszDest.sub(1);
        if cchDest != 0 {
            v6 = pszDest;
        }
        *v6 = 0;

        return if cchDest == 0 { 0x8007007A_u32 as i32 } else { 0 }; // STRSAFE_E_INSUFFICIENT_BUFFER
    }

    result
}

pub unsafe fn StringCchCatW(
    pszDest: STRSAFE_LPWSTR,
    cchDest: usize,
    pszSrc: STRSAFE_LPCWSTR,
) -> HRESULT {
    let mut v5: usize;          // r10
    let mut v6: STRSAFE_LPWSTR; // rax
    let mut v7: HRESULT;        // edx
    let v8: usize;              // r8
    let mut v9: *mut u16;       // rdx
    let mut v10: usize;         // rcx
    let mut v11: i64;           // rax
    let v12: isize;             // r11
    let mut v13: u16;           // r8
    let mut v14: *mut u16;      // rax

    if cchDest.wrapping_sub(1) > 0x7FFFFFFE {
        return -2147024809; // E_INVALIDARG (0x80070057)
    }

    v5 = cchDest;
    v6 = pszDest;

    // do-while loop to find the end of the destination string
    loop {
        if *v6 == 0 {
            break;
        }
        v6 = v6.add(1);
        v5 -= 1;

        if v5 == 0 {
            break;
        }
    }

    v7 = if v5 == 0 { 0x80070057_u32 as i32 } else { 0 };

    // Replicate bitwise math: (cchDest - v5) & -(__int64)(v5 != 0)
    let condition_mask = if v5 != 0 { -1_i64 } else { 0_i64 };
    v8 = ((cchDest - v5) as i64 & condition_mask) as usize;

    if v5 != 0 {
        // v9 = &pszDest[v8];
        v9 = pszDest.add(v8);
        v10 = cchDest - v8;

        if cchDest != v8 {
            v11 = 2147483646;

            // Byte distance calculation: (char *)pszSrc - (char *)v9
            v12 = (pszSrc as *const u8).offset_from(v9 as *const u8);

            // do-while loop to copy source string characters
            loop {
                if v11 == 0 {
                    break;
                }

                // v13 = *(wchar_t *)((char *)v9 + (_QWORD)v12);
                let src_char_ptr = (v9 as *const u8).offset(v12) as *const u16;
                v13 = *src_char_ptr;

                if v13 == 0 {
                    break;
                }

                *v9 = v13;
                v11 -= 1;
                v9 = v9.add(1);
                v10 -= 1;

                if v10 == 0 {
                    break;
                }
            }
        }

        v14 = v9.sub(1);
        if v10 != 0 {
            v14 = v9;
        }

        v7 = if v10 == 0 { 0x8007007A_u32 as i32 } else { 0 }; // STRSAFE_E_INSUFFICIENT_BUFFER
        *v14 = 0;
    }

    v7
}

pub unsafe fn AllocateFromHeap(dwFlags: DWORD, dwBytes: SIZE_T) -> LPVOID {
    let mut process_heap: HANDLE;                         // rsi
    let p_allocation: LPVOID;                             // rax
    let mut rtl_disown_module_heap_allocation: FARPROC;   // rbx
    let mut module_handle_w: HMODULE;                     // rax

    process_heap = GetProcessHeap();
    p_allocation = HeapAlloc(process_heap, dwFlags, dwBytes);

    rtl_disown_module_heap_allocation = core::mem::transmute::<i64, FARPROC>(g_pfnRtlDisownModuleHeapAllocation);

    // Conditional initialization block evaluating g_pfnRtlDisownModuleHeapAllocation
    if g_pfnRtlDisownModuleHeapAllocation != 0
        || (!g_bIsAllocationDisownInitialized
        && {
        // Wide string array for L"ntdll.dll"
        let ntdll_name: [u16; 10] = [
            'n' as u16, 't' as u16, 'd' as u16, 'l' as u16, 'l' as u16,
            '.' as u16, 'd' as u16, 'l' as u16, 'l' as u16, 0
        ];
        module_handle_w = GetModuleHandleW(ntdll_name.as_ptr());

        if module_handle_w.is_null() {
            rtl_disown_module_heap_allocation = core::mem::transmute::<i64, FARPROC>(g_pfnRtlDisownModuleHeapAllocation);
        } else {
            // C-string literal for "RtlDisownModuleHeapAllocation" (null-terminated)
            let proc_name = b"RtlDisownModuleHeapAllocation\0";
            rtl_disown_module_heap_allocation = GetProcAddress(module_handle_w, proc_name.as_ptr());
            g_pfnRtlDisownModuleHeapAllocation = core::mem::transmute::<FARPROC, i64>(rtl_disown_module_heap_allocation);
        }

        g_bIsAllocationDisownInitialized = true;
        rtl_disown_module_heap_allocation.is_some()
    })
    {
        _guard_check_icall_fptr();

        // Execute fastcall invocation if a valid pointer was recovered
        if let Some(func_ptr) = rtl_disown_module_heap_allocation {
            let typed_func = core::mem::transmute::<unsafe extern "system" fn(), PFN_RtlDisownModuleHeapAllocation>(func_ptr);
            typed_func(process_heap, p_allocation);
        }
    }

    p_allocation
}

pub fn HResultToNtStatus(value: i32) -> i64 {
    let mut nt_status: u32 = value as u32; // edx

    if value > -2147024662 {
        if value > -2147023746 {
            match value {
                -2147023604 => return -1073740757_i32 as u32 as i64,
                -2147023537 => return -1073741595_i32 as u32 as i64,
                -2147024311 => return -1073700733_i32 as u32 as i64, // (Adjusted typo matching case 3 from snippet)
                0 => return 0,                                      // STATUS_SUCCESS
                _ => {}
            }
        } else {
            match value {
                -2147023746 => return -1073741735_i32 as u32 as i64,
                -2147024362 => return -1073741675_i32 as u32 as i64,
                -2147024322 => return -1073741787_i32 as u32 as i64,
                -2147024314 => return -1073741471_i32 as u32 as i64,
                -2147024313 => return -1073741469_i32 as u32 as i64,
                -2147024270 => return -1073741197_i32 as u32 as i64,
                _ => {}
            }
        }
    } else {
        if value == -2147024662 {
            return -2147483643_i32 as u32 as i64;
        }
        if value > -2147024809 {
            match value {
                -2147024784 => return -1073741697_i32 as u32 as i64,
                -2147024774 => return -1073741789_i32 as u32 as i64,
                -2147024773 => return -1073741773_i32 as u32 as i64,
                -2147024770 => return -1073741515_i32 as u32 as i64,
                _ => {}
            }
        } else {
            match value {
                -2147024809 => return -1073741811_i32 as u32 as i64,
                -2147467259 => return -1073741823_i32 as u32 as i64,
                -2147024895 => return -1073741822_i32 as u32 as i64,
                -2147024894 => return -1073741772_i32 as u32 as i64,
                -2147024893 => return -1073741766_i32 as u32 as i64,
                -2147024882 => return -1073741801_i32 as u32 as i64,
                _ => {}
            }
        }
    }

    if (value & 0x10000000) != 0 {
        return (value & 0x1FFFFFFF) as u32 as i64;
    }

    if (value & 0x1FFF0000) == 0x70000 {
        nt_status = (value & 0xFFFF) as u32;
        if (value & 0xFFFF) != 0 {
            return ((value & 0xFFFF) as u32 | 0xC0070000) as i64;
        }
        return nt_status as i64;
    }

    if (value & 0x1FFF0000) != 0x90000 {
        return -1073741595_i32 as u32 as i64;
    }

    if value > 0 {
        return ((value & 0xFFFF) as u32 | 0xC0090000) as i64;
    }

    nt_status as i64
}


pub unsafe fn NormalizingExitRegisterWrapper(pfn_callback: OnExitT) -> i64 {
    let result = RegisterDllExitCallback(pfn_callback);
    let condition_val = if result != 0 { 1u32 } else { 0u32 };
    let wrapped = condition_val.wrapping_sub(1);
    wrapped as i64
}

pub unsafe fn RegisterDllExitCallback(pfnCallback: OnExitT) -> i64 {
    let start = core::ptr::addr_of!(GlobalPOnExitTableStart).read_volatile();
    if start == -1 {
        return onexit();
    }

    lock();
    let v2 = GlobalPOnExitTableStart;
    let v3 = GlobalPOnExitTableEnd;

    let v1 = _dllonexit();

    GlobalPOnExitTableStart = v2;
    GlobalPOnExitTableEnd = v3;
    unlock();

    v1
}