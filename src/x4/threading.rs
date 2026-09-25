
use core::sync::atomic::{AtomicI32, Ordering};
use crate::x4::error::{ReportAlignmentAssertionFailure, TraceProviderEvent};
use crate::x4::externals::{AcquireSRWLockExclusive, AcquireSRWLockShared, CloseHandle, CloseThreadpoolTimer, CreateMutexExW, CreateSemaphoreExW, EnterCriticalSection, EtwWriteTransfer, GetCurrentProcessId, GetLastError, GetProcessHeap, HeapFree, InitializeCriticalSectionEx, LeaveCriticalSection, NtCurrentTeb, OpenSemaphoreW, ReleaseMutex, ReleaseSRWLockExclusive, ReleaseSRWLockShared, ReleaseSemaphore, SetLastError, SetThreadpoolTimer, WaitForSingleObject, WaitForSingleObjectEx, WaitForThreadpoolTimerCallbacks};
use crate::x4::globals::_guard_check_icall_fptr;
use crate::x4::helpers::{AllocateFromHeap, StringCchCatW, StringCchPrintfW};
use crate::x4::types::{SharedContextBlock, SRWLOCK, ItemManager, ItemTracker, LaneFrameTracker, HANDLE, PVOID, __m128i, ContextEntry, LPCRITICAL_SECTION, PSRWLOCK, CRITICAL_SECTION, WCHAR, LONG, FileTime, TP_TIMER, __int64, FILETIME};

pub unsafe fn InitThreadNotify(p_sequence_id: *mut u32) {
    let srw_lock_ptr = &mut SRWLOCK as *mut SRWLOCK;
    AcquireSRWLockExclusive(srw_lock_ptr);

    let v2 = TlsIndex as usize;
    InitSequence = InitSequence.wrapping_add(1);

    if !p_sequence_id.is_null() {
        *p_sequence_id = InitSequence;
    }

    let teb = NtCurrentTeb();
    if !teb.is_null() {
        let tls_pointer_array = (*teb).ThreadLocalStoragePointer;
        if !tls_pointer_array.is_null() {
            let tls_slot = *tls_pointer_array.add(v2);
            if !tls_slot.is_null() {
                // Replicates *(_DWORD *)(*((_QWORD *)NtCurrentTeb()->ThreadLocalStoragePointer + v2) + 4LL)
                let target_dword_ptr = (tls_slot as *mut u8).add(4) as *mut u32;
                *target_dword_ptr = InitSequence;
            }
        }
    }

    ReleaseSRWLockExclusive(srw_lock_ptr);

    let cv_ptr = &mut ConditionVariable as *mut CONDITION_VARIABLE;
    WakeAllConditionVariable(cv_ptr);
}

pub unsafe extern "system" fn pfnti(
    _instance: *mut core::ffi::c_void,
    context: *mut SharedContextBlock,
    _timer: *mut core::ffi::c_void,
) {
    if context.is_null() {
        return;
    }

    // Checks LOBYTE(Context->lanes)
    if (*context).lanes != 0 {
        let lock = (*context).lock;

        if !lock.is_null() {
            AcquireSRWLockExclusive(lock);
        }

        // Replicates BYTE1(Context->?[0].reservedPool[1]) = 0;
        // Adjust based on your exact struct layout offset
        let byte_target = (context as *mut u8).add(17);
        *byte_target = 0;

        if !lock.is_null() {
            ReleaseSRWLockExclusive(lock);
        }

        if g_DisableBypassCheck == 0 {
            let check_callback = g_pfnConditionCheck;
            let mut execute_body = true;

            if let Some(func) = check_callback {
                _guard_check_icall_fptr();
                if func() != 0 {
                    execute_body = false;
                }
            }

            if execute_body {
                if LazyInitTarget(context) != 0 {
                    // Context offset adjustments mapping to Context->? + 5
                    let dispatch_arg = (context as *mut u8).add(40) as *mut core::ffi::c_void;
                    DispatchCallbacks(dispatch_arg, lock);
                    SyncBuffers(lock);
                }
            }
        }
    }
}

pub unsafe fn InvalidateTrackedObjects(manager: *mut ItemManager) {
    if manager.is_null() {
        return;
    }

    if (*manager).is_active != 0 {
        let lock_ptr = &mut (*manager).srw_lock as *mut SRWLOCK;
        AcquireSRWLockExclusive(lock_ptr);

        // manager[1] corresponds to an offset of one ItemManager struct size.
        let next_manager = manager.add(1);
        let mut current_item = (*next_manager).items_start;
        let end_item = (*next_manager).items_end;

        while current_item != end_item {
            if !(*current_item).target_status_ptr.is_null() {
                // Replicates _InterlockedAnd
                let atomic_target = &*((*current_item).target_status_ptr as *const AtomicI32);
                let mask = if (*current_item).state != 0 { -5 } else { -2111 };
                atomic_target.fetch_and(mask, Ordering::SeqCst);
            }
            current_item = current_item.add(1);
        }

        (*next_manager).items_end = (*next_manager).items_start;

        // Handling the version stored in the high DWORD of items_start pointer field
        let items_start_raw = &mut (*manager).items_start as *mut *mut ItemTracker as *mut u64;
        let packed_val = *items_start_raw;
        let low_ptr = packed_val as u32;
        let high_version = (packed_val >> 32) as i32;

        let next_version = if high_version != -1 {
            high_version + 1
        } else {
            1
        };

        // Reconstruct the 64-bit value with the updated high DWORD version
        *items_start_raw = ((next_version as u64) << 32) | (low_ptr as u64);

        ReleaseSRWLockExclusive(lock_ptr);
    }
}

pub unsafe fn InitContextInternalPools(
    p_context: *mut SharedContextBlock,
) -> *mut SharedContextBlock {
    if p_context.is_null() {
        return core::ptr::null_mut();
    }

    // pContext->lanes = 0x40000;
    (*p_context).lanes = [LaneFrameTracker{
        dynamic_payload_bytes: [0u8;56],
        is_dirty_or_active: 0,
        alignment_padding: [0u8;7],
        reservedPool: (),
    };3];

    let base = p_context as *mut u8;

    // Note: Replace these byte offsets (.add(N)) with your actual struct field 
    // references once the struct definition is fully resolved.

    // LOBYTE(pContext->?) = 1;
    *base.add(4) = 1;

    // LOBYTE(pContext->?) = 0;
    *base.add(5) = 0;

    // LOWORD(pContext->?[0].reservedPool[0]) = 0;
    *(base.add(6) as *mut u16) = 0;

    // BYTE2(pContext->?[0].reservedPool[0]) = 0;
    *base.add(8) = 0;

    // HIWORD(pContext->?) = 4;
    *(base.add(10) as *mut u16) = 4;

    // pContext->? = (void *)4;
    *(base.add(16) as *mut *mut core::ffi::c_void) = 4 as *mut core::ffi::c_void;

    // pContext->? = nullptr;
    *(base.add(24) as *mut *mut core::ffi::c_void) = core::ptr::null_mut();

    // *(_QWORD *)&pContext->?[0].version = 0;
    *(base.add(32) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[0].subStatus = 0;
    *(base.add(40) as *mut u64) = 0;

    // pContext->?[0].capacity = 0;
    *(base.add(48) as *mut i32) = 0;

    // HIWORD(pContext->?[0].reservedPool[1]) = 4;
    *(base.add(54) as *mut u16) = 4;

    // LODWORD(pContext->?[0].reservedPool[1]) = 0x40000;
    *(base.add(52) as *mut u32) = 0x40000;

    // BYTE4(pContext->?[0].reservedPool[1]) = 1;
    *base.add(56) = 1;

    // LOBYTE(pContext->?[0].reservedPool[2]) = 2;
    *base.add(60) = 2;

    // *(_QWORD *)&pContext->?[0].trailerId = 0;
    *(base.add(64) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[1].version = 0;
    *(base.add(72) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[1].subStatus = 0;
    *(base.add(80) as *mut u64) = 0;

    // pContext->?[1].capacity = 0;
    *(base.add(88) as *mut i32) = 0;

    // pContext->?[0].reservedPool[3] = 8;
    *base.add(92) = 8;

    // LOWORD(pContext->?[1].reservedPool[0]) = 0;
    *(base.add(96) as *mut u16) = 0;

    // BYTE2(pContext->?[1].reservedPool[0]) = 0;
    *base.add(98) = 0;

    // LODWORD(pContext->?[1].reservedPool[1]) = 0x40000;
    *(base.add(100) as *mut u32) = 0x40000;

    // BYTE4(pContext->?[1].reservedPool[1]) = 1;
    *base.add(104) = 1;

    // HIWORD(pContext->?[1].reservedPool[1]) = 0;
    *(base.add(106) as *mut u16) = 0;

    // LOBYTE(pContext->?[1].reservedPool[2]) = 1;
    *base.add(108) = 1;

    // *(_QWORD *)&pContext->?[1].trailerId = 0;
    *(base.add(112) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[2].version = 0;
    *(base.add(120) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[2].subStatus = 0;
    *(base.add(128) as *mut u64) = 0;

    // pContext->?[2].capacity = 0;
    *(base.add(136) as *mut i32) = 0;

    // pContext->?[1].reservedPool[3] = 0;
    *base.add(140) = 0;

    // LOWORD(pContext->?[2].reservedPool[0]) = 0;
    *(base.add(144) as *mut u16) = 0;

    // BYTE2(pContext->?[2].reservedPool[0]) = 0;
    *base.add(146) = 0;

    p_context
}

pub unsafe fn FreeLanePayloadBuffers(p_frame_array: *mut u64) {
    if p_frame_array.is_null() {
        return;
    }

    // Lane 2 buffer (index 22)
    let p_lane2_buffer = *p_frame_array.add(22) as *mut core::ffi::c_void;
    *p_frame_array.add(22) = 0;
    if !p_lane2_buffer.is_null() {
        let h_heap2 = GetProcessHeap();
        HeapFree(h_heap2, 0, p_lane2_buffer);
    }

    // Lane 1 buffer (index 14)
    let p_lane1_buffer = *p_frame_array.add(14) as *mut core::ffi::c_void;
    *p_frame_array.add(14) = 0;
    if !p_lane1_buffer.is_null() {
        let h_heap1 = GetProcessHeap();
        HeapFree(h_heap1, 0, p_lane1_buffer);
    }

    // Lane 0 buffer (index 6)
    let p_lane0_buffer = *p_frame_array.add(6) as *mut core::ffi::c_void;
    *p_frame_array.add(6) = 0;
    if !p_lane0_buffer.is_null() {
        let h_heap0 = GetProcessHeap();
        HeapFree(h_heap0, 0, p_lane0_buffer);
    }
}

pub unsafe fn InitSharedObject(
    context_string: *const core::ffi::c_char,
    ptp_out_shared_object: *mut *mut core::ffi::c_void,
) -> i32 {
    if !ptp_out_shared_object.is_null() {
        *ptp_out_shared_object = core::ptr::null_mut();
    }

    let current_process_id = GetCurrentProcessId();
    let mut mutex_name_buffer = [0u16; 264];

    // Format string: L"Local\\SM0:%lu:%lu:%hs"
    let format_str: &[u16] = &[
        b'L' as u16, b'o' as u16, b'c' as u16, b'a' as u16, b'l' as u16, b'\\' as u16,
        b'S' as u16, b'M' as u16, b'0' as u16, b':' as u16, b'%' as u16, b'l' as u16,
        b'u' as u16, b':' as u16, b'%' as u16, b'l' as u16, b'u' as u16, b':' as u16,
        b'h' as u16, b's' as u16, 0
    ];

    StringCchPrintfW(
        mutex_name_buffer.as_mut_ptr(),
        260,
        format_str.as_ptr(),
        current_process_id,
        304,
        context_string,
    );

    let mutex = CreateMutexExW(core::ptr::null_mut(), mutex_name_buffer.as_ptr(), 0, 0x1F0001);
    if mutex.is_null() {
        return GetLastError() as i32;
    }

    let mut h_mutex = mutex;
    let wait_result = WaitForSingleObjectEx(mutex, 0xFFFFFFFF, 0);
    let mut h_held_mutex: HANDLE = core::ptr::null_mut();

    if wait_result == 258 {
        h_held_mutex = core::ptr::null_mut();
    } else {
        if (wait_result & 0xFFFFFF7F) != 0 {
            ReportLockFailureException(0);
        }
        h_held_mutex = h_mutex;
    }

    let mut p_cached_address: u64 = 0;
    let status = LookupExistingSharedContext(mutex_name_buffer.as_ptr(), 0, &mut p_cached_address);
    if status < 0 {
        TraceProviderEvent(0, 100, 0, status as u64);
        TraceProviderEvent(0, 109, 0, status as u64);
        TraceProviderEvent(0, 299, 0, status as u64);
        if !h_held_mutex.is_null() && ReleaseMutex(h_held_mutex) == 0 {
            HandleHandleCloseError(/* i32 */, 2535, /* i32 */, /* i32 */);
        }
        if CloseHandle(h_mutex) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2525);
        }
        return status as i32;
    }

    if p_cached_address != 0 {
        let p_existing_object = (4 * p_cached_address) as *mut i32;
        if !p_existing_object.is_null() {
            *ptp_out_shared_object = p_existing_object as *mut core::ffi::c_void;
            *p_existing_object = *p_existing_object + 1;

            if !h_held_mutex.is_null() && ReleaseMutex(h_held_mutex) == 0 {
                HandleHandleCloseError(core::ptr::null_mut(), 2535);
            }
            if !h_mutex.is_null() && CloseHandle(h_mutex) == 0 {
                HandleHandleCloseError(core::ptr::null_mut(), 2525);
            }
            return 0;
        }
    }

    *ptp_out_shared_object = core::ptr::null_mut();
    let v19 = AllocateFromHeap(8, 0x130);
    let p_new_object = v19 as *mut u8;

    if !p_new_object.is_null() {
        let mut h_shared_sections: [HANDLE; 3] = [core::ptr::null_mut(); 3];
        let map_status = InitSharedMapping(h_shared_sections.as_mut_ptr(), mutex_name_buffer.as_ptr(), v19);
        if map_status >= 0 {
            *(p_new_object.add(16) as *mut HANDLE) = h_shared_sections[0];
            let first_shared_section = h_shared_sections[1];
            *(p_new_object.add(8) as *mut HANDLE) = h_mutex;
            h_mutex = core::ptr::null_mut();
            *(p_new_object.add(24) as *mut HANDLE) = first_shared_section;
            *(p_new_object as *mut i32) = 1;

            core::ptr::write_bytes(p_new_object.add(40), 0, 0x108);

            *(p_new_object.add(32) as *mut u64) = 0;
            InitContextInternalPools(p_new_object.add(40) as *mut core::ffi::c_void);

            InitializeCriticalSectionEx(p_new_object.add(232) as *mut core::ffi::c_void, 0, 0);

            *(p_new_object.add(272) as *mut u64) = 0;
            *(p_new_object.add(280) as *mut u64) = 0;
            *(p_new_object.add(288) as *mut u64) = 0;
            *(p_new_object.add(296) as *mut u64) = 0;

            *ptp_out_shared_object = p_new_object as *mut core::ffi::c_void;

            if !h_held_mutex.is_null() && ReleaseMutex(h_held_mutex) == 0 {
                HandleHandleCloseError(core::ptr::null_mut(), 2535);
            }
            if !h_mutex.is_null() && CloseHandle(h_mutex) == 0 {
                HandleHandleCloseError(core::ptr::null_mut(), 2525);
            }
            return 0;
        }

        TraceProviderEvent(0, 331, 0, map_status);
        if !h_shared_sections[1].is_null() && CloseHandle(h_shared_sections[1]) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2525);
        }
        if !h_shared_sections[0].is_null() && CloseHandle(h_shared_sections[0]) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2525);
        }
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, p_new_object as *mut core::ffi::c_void);

        let out_status = map_status;
        TraceProviderEvent(0, 308, 0, out_status);
        if !h_held_mutex.is_null() && ReleaseMutex(h_held_mutex) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2535);
        }
        if CloseHandle(h_mutex) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2525);
        }
        return out_status as i32;
    } else {
        let out_status = -2147024882i32;
        TraceProviderEvent(0, 328, 0, out_status as u64);
        TraceProviderEvent(0, 308, 0, out_status as u64);
        if !h_held_mutex.is_null() && ReleaseMutex(h_held_mutex) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2535);
        }
        if CloseHandle(h_mutex) == 0 {
            HandleHandleCloseError(core::ptr::null_mut(), 2525);
        }
        return out_status as i32;
    }
}

pub unsafe fn InitSharedMapping(
    ph_out_handles: *mut HANDLE,
    psz_base_name: *const u16,
    allocation_size: usize,
) -> u32 {
    if (allocation_size & 3) != 0 {
        ReportAlignmentAssertionFailure(
            ph_out_handles as *mut core::ffi::c_void,
            psz_base_name,
            psz_base_name,
        );
    }

    let primary_capacity = allocation_size >> 2;
    let mut psz_dest = [0u16; 264];

    // Replicates the wide string copy loop
    let mut src_ptr = psz_base_name;
    let mut dest_ptr = psz_dest.as_mut_ptr();
    let mut remaining = 260isize;

    while remaining > 0 {
        let val = *src_ptr;
        *dest_ptr = val;
        if val == 0 {
            break;
        }
        src_ptr = src_ptr.offset(1);
        dest_ptr = dest_ptr.offset(1);
        remaining -= 1;
    }
    if remaining == 0 {
        psz_dest[259] = 0;
    }

    // Append "_p0"
    let suffix_p0: &[u16] = &[b'_' as u16, b'p' as u16, b'0' as u16, 0];
    StringCchCatW(psz_dest.as_mut_ptr(), 260, suffix_p0.as_ptr());

    let secondary_capacity = primary_capacity >> 31;
    let primary_capacity_clone = (primary_capacity & 0x7FFFFFFF) as i32;
    let mut primary_initial_count = 1;
    if primary_capacity_clone != 0 {
        primary_initial_count = primary_capacity_clone;
    }

    let mut status = CreateOrOpenSemaphoreW(
        ph_out_handles,
        primary_initial_count,
        primary_capacity_clone,
        psz_dest.as_ptr(),
    );

    let v17: i32;
    if status >= 0 {
        // Append "h"
        let suffix_h: &[u16] = &[b'h' as u16, 0];
        StringCchCatW(psz_dest.as_mut_ptr(), 260, suffix_h.as_ptr());

        let mut secondary_initial_count = 1i32;
        if (secondary_capacity as i32) != 0 {
            secondary_initial_count = secondary_capacity as i32;
        }

        // phOutHandles + 8 (next HANDLE slot in the array)
        let next_handle_slot = (ph_out_handles as *mut u8).add(8) as *mut HANDLE;
        status = CreateOrOpenSemaphoreW(
            next_handle_slot,
            secondary_initial_count,
            secondary_capacity as i32,
            psz_dest.as_ptr(),
        );

        if status >= 0 {
            return 0;
        }
        v17 = 141;
    } else {
        v17 = 136;
    }

    TraceProviderEvent(*core::ptr::null_mut(), v17 as u64, 0, status as u64);
    status as u32
}

pub unsafe fn LazyInitTarget(ctx: *mut ContextEntry) -> bool {
    if ctx.is_null() {
        return false;
    }

    // Check ctx[3].Ptr
    if (*ctx.add(3)).ptr.is_null() {
        let last_error = GetLastError();

        if (*ctx.add(3)).ptr.is_null() {
            let target_address: *mut core::ffi::c_void;

            // if (!ctx[2].Ptr)
            if (*ctx.add(2)).ptr.is_null() {
                let creation_context = (*ctx.add(1)).ptr;
                let mut p_created_object: PVOID = core::ptr::null_mut();

                if InitSharedObject(creation_context, &mut p_created_object) >= 0 {
                    if (*ctx.add(2)).ptr.is_null() {
                        (*ctx.add(2)).ptr = p_created_object;
                    }
                }
            }

            let ptr2 = (*ctx.add(2)).ptr as usize;
            // Replicates: ((__int64)ctx[2].Ptr + 32) & -(__int64)(ctx[2].Ptr != nullptr)
            let calculated = if ptr2 != 0 {
                (ptr2 + 32) & !0 // Mask is all 1s when non-null
            } else {
                0
            };
            target_address = calculated as *mut core::ffi::c_void;

            // AcquireSRWLockExclusive(ctx + 4);
            let lock_ptr = ctx.add(4) as *mut SRWLOCK;
            AcquireSRWLockExclusive(lock_ptr);

            if (*ctx.add(3)).ptr.is_null() {
                (*ctx.add(3)).ptr = target_address;
            }

            // Replicates: if ( ctx != (RTL_SRWLOCK *)-32LL ) ReleaseSRWLockExclusive(ctx + 4);
            if ctx as usize != (!31_usize) {
                ReleaseSRWLockExclusive(lock_ptr);
            }

            SetLastError(last_error);
        }
    }

    // Return ctx[3].Ptr != nullptr
    !(*ctx.add(3)).ptr.is_null()
}

pub unsafe fn ReportLockFailureException(address: u64) -> /*!*/ {
    let file_path = b"onecore\\internal\\sdk\\inc\\wil\\opensource\\wil\\resource.h\0";
    TerminateWithFatalError(
        address,
        3127,
        file_path.as_ptr() as *const core::os::raw::c_char,
    );
}

pub unsafe fn QueryAndValidateSemaphoreCount(hHandle: HANDLE, a2: *mut i32) -> i64 {
    let retaddr = 0usize;
    let v5 = 0;
    let v6 = 0;

    let v4 = WaitForSingleObject(hHandle, 0);
    if v4 == u32::MAX {
        return LogDiagnosticEventWithStatus(retaddr, 153, v5, v6);
    }

    let v9: i32;
    if v4 == 0 || v4 == 258 {
        let mut previous_count = 0i32;
        if v4 != 0 {
            // v4 == 258 (WAIT_TIMEOUT)
            let mut v13 = 0i32;
            if ReleaseSemaphore(hHandle, 1, &mut v13) == 0 {
                return LogDiagnosticEventWithStatus(retaddr, 177, v5, v6);
            }
            if v13 != 0 {
                v9 = 178;
                let status = -2147418113; // E_UNEXPECTED
                TraceProviderEvent(retaddr as u64, v9 as u64, v5, status);
                return 2147549183i64;
            }
            if ReleaseSemaphore(hHandle, 1, core::ptr::null_mut()) != 0 || GetLastError() != 298 {
                v9 = 181;
                let status = -2147418113;
                TraceProviderEvent(retaddr as u64, v9 as u64, v5, status);
                return 2147549183i64;
            }
            let v10 = WaitForSingleObject(hHandle, 0);
            if v10 == u32::MAX {
                return LogDiagnosticEventWithStatus(retaddr, 184, v5, v6);
            }
            if v10 != 0 {
                v9 = 185;
                let status = -2147418113;
                TraceProviderEvent(retaddr as u64, v9 as u64, v5, status);
                return 2147549183i64;
            }
        } else {
            // v4 == 0 (WAIT_OBJECT_0)
            if ReleaseSemaphore(previous_count, 1, hHandle) == 0 {
                return LogDiagnosticEventWithStatus(retaddr, 162, v5, v6);
            }
            previous_count += 1;
            if ReleaseSemaphore(hHandle, 1, core::ptr::null_mut()) != 0 || GetLastError() != 298 {
                v9 = 167;
                let status = -2147418113;
                TraceProviderEvent(retaddr as u64, v9 as u64, v5, status);
                return 2147549183i64;
            }
        }

        if !a2.is_null() {
            *a2 = previous_count;
        }
        return 0;
    } else {
        v9 = 154;
    }

    let status = -2147418113i32;
    TraceProviderEvent(retaddr as u64, v9 as u64, v5, status);
    2147549183i64
}

pub unsafe fn DispatchCallbacks(
    lpCriticalSection: LPCRITICAL_SECTION,
    SRWLock: PSRWLOCK,
) {
    if lpCriticalSection.is_null() {
        return;
    }

    if !SRWLock.is_null() {
        AcquireSRWLockShared(SRWLock);
    }

    let base_ptr = lpCriticalSection as *mut u8;
    let cs_size = core::mem::size_of::<CRITICAL_SECTION>() as isize;

    let field_lock_count_ptr = base_ptr.offset(cs_size);
    let field_debug_info_ptr = base_ptr.offset(cs_size + 8);

    let lock_count_val = *(field_lock_count_ptr as *const u64);
    let debug_info_val = *(field_debug_info_ptr as *const u64);

    let total_callbacks = (lock_count_val.wrapping_sub(debug_info_val)) >> 4;

    if !SRWLock.is_null() {
        ReleaseSRWLockShared(SRWLock);
    }

    let mut current_index: usize = 0;
    while (current_index as u64) < total_callbacks {
        let mut callback_fn: Option<unsafe extern "fastcall" fn(u64)> = None;
        let mut callback_arg: u64 = 0;

        EnterCriticalSection(lpCriticalSection);
        if !SRWLock.is_null() {
            AcquireSRWLockExclusive(SRWLock);
        }

        if (current_index as u64) < total_callbacks {
            let debug_info_struct = *(base_ptr.offset(cs_size + 8) as *const *const u8);

            if !debug_info_struct.is_null() {
                let array_base = debug_info_struct as *const u64;
                let mut scan_index = current_index;

                // Scan for a non-zero function pointer slot
                loop {
                    let next_index = scan_index + 1;
                    let target_slot = array_base.add(scan_index * 2);

                    if *target_slot != 0 {
                        current_index = scan_index;
                        break;
                    }

                    scan_index = next_index;
                    current_index = scan_index;

                    if (current_index as u64) >= total_callbacks {
                        break;
                    }
                }

                if (current_index as u64) < total_callbacks {
                    // Load 16 bytes (two 64-bit values: function pointer and argument)
                    let entry_ptr = debug_info_struct.add(current_index * 16) as *const __m128i;
                    let v11 = *entry_ptr;

                    current_index += 1;

                    // Extract the first 64-bit value as the function pointer
                    let fn_ptr_bits = _mm_cvtsi128_si64(v11) as usize;
                    callback_fn = core::mem::transmute(fn_ptr_bits as *const ());

                    // Use _mm_srli_si128 to shift right by 8 bytes, isolating the second 64-bit value (argument)
                    let shifted = _mm_srli_si128::<8>(v11);
                    callback_arg = _mm_cvtsi128_si64(shifted) as u64;
                }
            }
        }

        if !SRWLock.is_null() {
            ReleaseSRWLockExclusive(SRWLock);
        }

        if let Some(func) = callback_fn {
            _guard_check_icall_fptr();
            func(callback_arg);
        }

        LeaveCriticalSection(lpCriticalSection);
    }
}

pub unsafe fn CleanupFrameTracking(p_frame_array: *mut u8) -> i32 {
    let mut result: i32 = 0;

    if p_frame_array.is_null() {
        return 0;
    }

    // Check pFrameArray[56]
    if *p_frame_array.add(56) != 0 {
        let telemetry_hash0: u64 = 0x418A073AA3BC1C75;
        let telemetry_hash1: u64 = 0x418A073AA3BC2475;
        let telemetry_hash2: u64 = 0x418A073AA3BC2C75;

        let hashes = [telemetry_hash0, telemetry_hash1, telemetry_hash2];
        result = EtwWriteTransfer(hashes.as_ptr() as u64 as __int64, 3, p_frame_array as u64 as __int64);
    }

    // Check pFrameArray[120]
    if *p_frame_array.add(120) != 0 {
        let telemetry_hash0: u64 = 0x418A073AA3BC3475;
        let telemetry_hash1: u64 = 0x418A073AA3BC3C75;
        let telemetry_hash2: u64 = 0x418A073AA3BC4475;

        let hashes = [telemetry_hash0, telemetry_hash1, telemetry_hash2];
        result = EtwWriteTransfer(hashes.as_ptr() as u64 as __int64, 3, p_frame_array.add(64) as u64 as __int64);
    }

    // Check pFrameArray[184]
    if *p_frame_array.add(184) != 0 {
        let telemetry_hash0: u64 = 0x418A073AA3BC4C75;
        let telemetry_hash1: u64 = 0x418A073AA3BC5475;
        let telemetry_hash2: u64 = 0x418A073AA3BC5C75;
        let telemetry_hash3: u64 = 0x418A073AA3BC6475;
        let telemetry_hash4: u64 = 0x418A073AA3BC6C75;
        let telemetry_hash5: u64 = 0x418A073AA3BC7475;

        let hashes = [
            telemetry_hash0,
            telemetry_hash1,
            telemetry_hash2,
            telemetry_hash3,
            telemetry_hash4,
            telemetry_hash5,
        ];
        return EtwWriteTransfer(hashes.as_ptr() as __int64, 6, p_frame_array.add(128) as __int64);
    }

    result
}

pub unsafe fn SyncBuffers(p_context_block: *mut u8) -> i64 {
    if p_context_block.is_null() {
        return 0;
    }

    // Local stack buffers corresponding to laneFrame0, v4, and v5
    let mut lane_frame0 = [0u8; 64];
    let mut v4 = [0u8; 64];
    let mut v5 = [0u8; 72];

    InitContextInternalPools(*p_context_block as *mut SharedContextBlock);

    let lock_ptr = p_context_block as SRWLOCK;
    AcquireSRWLockExclusive(lock_ptr);

    // Note: Replace these byte offsets (.add(N)) with your exact resolved struct layout offsets
    // corresponding to the `?` placeholder fields in the decompiler.

    // if ( LOBYTE(pContextBlock->?[0].reservedPool[1]) )
    if *p_context_block.add(56) != 0 {
        let dest_ptr = p_context_block.add(60);
        SwapLaneSnapshots(lane_frame0.as_mut_ptr(), dest_ptr);
    }

    // if ( LOBYTE(pContextBlock->?[1].reservedPool[1]) )
    if *p_context_block.add(104) != 0 {
        let dest_ptr = p_context_block.add(108);
        SwapLaneSnapshots(v4.as_mut_ptr(), dest_ptr);
    }

    // if ( LOBYTE(pContextBlock->?[2].reservedPool[1]) )
    if *p_context_block.add(152) != 0 {
        let dest_ptr = p_context_block.add(156);
        SwapLaneSnapshots(v5.as_mut_ptr(), dest_ptr);
    }

    ReleaseSRWLockExclusive(lock_ptr as PSRWLOCK);

    CleanupFrameTracking(lane_frame0.as_mut_ptr());
    FreeLanePayloadBuffers(lane_frame0.as_mut_ptr() as *mut u64);
    0
}

pub unsafe fn SwapLaneSnapshots(
    p_dest_slot: *mut u8,
    p_src_slot: *mut u8,
) -> u8 {
    if p_dest_slot.is_null() || p_src_slot.is_null() {
        return 0;
    }

    // Save temporary snapshots from destination slot
    let temp_pool_snapshot0 = *(p_dest_slot.add(24) as *const u128);
    let temp_pool_snapshot1 = *(p_dest_slot.add(40) as *const u64);

    let old_dest_buffer = *(p_dest_slot.add(48) as *const *mut core::ffi::c_void);
    *(p_dest_slot.add(48) as *mut *mut core::ffi::c_void) = core::ptr::null_mut();

    // Copy source snapshot data to destination slot
    let src_snapshot0 = *(p_src_slot.add(24) as *const u128);
    let src_snapshot1 = *(p_src_slot.add(40) as *const u64);
    *(p_dest_slot.add(24) as *mut u128) = src_snapshot0;
    *(p_dest_slot.add(40) as *mut u64) = src_snapshot1;

    let new_src_buffer = *(p_src_slot.add(48) as *const *mut core::ffi::c_void);
    *(p_src_slot.add(48) as *mut *mut core::ffi::c_void) = core::ptr::null_mut();

    // Check and free orphaned memory 0
    let p_orphaned_memory0 = *(p_dest_slot.add(48) as *const *mut core::ffi::c_void);
    *(p_dest_slot.add(48) as *mut *mut core::ffi::c_void) = new_src_buffer;
    if !p_orphaned_memory0.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, p_orphaned_memory0);
    }

    // Restore saved destination snapshot data into source slot
    *(p_src_slot.add(24) as *mut u128) = temp_pool_snapshot0;
    *(p_src_slot.add(40) as *mut u64) = temp_pool_snapshot1;

    // Check and free orphaned memory 1
    let p_orphaned_memory1 = *(p_src_slot.add(48) as *const *mut core::ffi::c_void);
    *(p_src_slot.add(48) as *mut *mut core::ffi::c_void) = old_dest_buffer;
    if !p_orphaned_memory1.is_null() {
        let proc_heap_handle = GetProcessHeap();
        HeapFree(proc_heap_handle, 0, p_orphaned_memory1);
    }

    // Replicate trailer and alignment padding byte swaps at the end of the function.
    // Note: Adjust these field offsets to match your resolved LaneSnapshotData struct definition.
    // Assuming offset 56 for is_dirty_or_active and offset 57 for alignment_padding[0] as an example.
    let is_dirty_offset = 56;
    let padding_offset = 57;

    let old_trailer_id = *p_dest_slot.add(is_dirty_offset);
    *p_dest_slot.add(is_dirty_offset) = *p_src_slot.add(is_dirty_offset);

    let result = *p_src_slot.add(padding_offset);
    *p_src_slot.add(is_dirty_offset) = old_trailer_id;

    let old_cleanup_byte = *p_dest_slot.add(padding_offset);
    *p_dest_slot.add(padding_offset) = result;
    *p_src_slot.add(padding_offset) = old_cleanup_byte;

    result
}

pub unsafe fn LookupExistingSharedContext(
    pszBaseName: *const WCHAR,
    _unused_param: u64,
    ptpOutCachedAddress: *mut u64,
) -> i64 {
    let retaddr = 0usize;
    let mut sem_name_buffer = [0u16; 264];

    if !ptpOutCachedAddress.is_null() {
        *ptpOutCachedAddress = 0;
    }

    // Safely copy pszBaseName into sem_name_buffer (up to 260 characters)
    if !pszBaseName.is_null() {
        let mut i = 0;
        while i < 260 {
            let c = *pszBaseName.add(i);
            sem_name_buffer[i] = c;
            if c == 0 {
                break;
            }
            i += 1;
        }
        sem_name_buffer[263] = 0;
    }

    // Append "_p0" to semaphore name buffer
    let suffix_p0: [WCHAR; 4] = [b'_' as WCHAR, b'p' as WCHAR, b'0' as WCHAR, 0];
    StringCchCatW(sem_name_buffer.as_mut_ptr(), 260, suffix_p0.as_ptr());

    // Open primary semaphore
    let h_primary_semaphore = OpenSemaphoreW(0x1F0003, 0, sem_name_buffer.as_ptr());
    if h_primary_semaphore.is_null() || h_primary_semaphore == (-1isize as HANDLE) {
        if GetLastError() != 2 {
            let v11 = 0;
            let v12 = 0;
            return LogDiagnosticEventWithStatus(retaddr, 205, v11, v12);
        }
        return 0;
    }

    let mut v25 = [0i32; 3];
    let mut v24 = 0i32;

    let primary_status = QueryAndValidateSemaphoreCount(h_primary_semaphore, v25.as_mut_ptr());
    if primary_status < 0 {
        let v15 = 0;
        TraceProviderEvent(retaddr as u64, 211, v15, primary_status as u64);
        if CloseHandle(h_primary_semaphore) == 0 {
            HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
        }
        return primary_status;
    }

    // Append "h" to semaphore name buffer
    let suffix_h: [WCHAR; 2] = [b'h' as WCHAR, 0];
    StringCchCatW(sem_name_buffer.as_mut_ptr(), 260, suffix_h.as_ptr());

    // Open secondary semaphore
    let h_secondary_semaphore = OpenSemaphoreW(0x1F0003, 0, sem_name_buffer.as_ptr());
    if h_secondary_semaphore.is_null() || h_secondary_semaphore == (-1isize as HANDLE) {
        let v18 = 0;
        let v19 = 0;
        let log_res = LogDiagnosticEventWithStatus(retaddr, 217, v18, v19);
        if CloseHandle(h_primary_semaphore) == 0 {
            HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
        }
        return log_res;
    }

    let secondary_status = QueryAndValidateSemaphoreCount(h_secondary_semaphore, &mut v24);
    if secondary_status >= 0 {
        if CloseHandle(h_secondary_semaphore) == 0 {
            HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
        }
        if !ptpOutCachedAddress.is_null() {
            *ptpOutCachedAddress = (v25[0] as u64) | ((v24 as u64) << 31);
        }
        if CloseHandle(h_primary_semaphore) == 0 {
            HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
        }
        return 0;
    }

    let v22 = 0;
    TraceProviderEvent(retaddr as u64, 219, v22, secondary_status as u64);
    if CloseHandle(h_secondary_semaphore) == 0 {
        HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
    }
    if CloseHandle(h_primary_semaphore) == 0 {
        HandleHandleCloseError(*core::ptr::null_mut(), 2525, 0, 0);
    }
    secondary_status
}

pub unsafe fn HandleHandleCloseError(a1: i32, a2: i32, a3: i32, a4: i32) /*-> !*/ {
    TerminateWithResourceFailure(a1, a2, a3, a4);
}

pub unsafe fn CreateOrOpenSemaphoreW(
    ptpOutHandle: *mut HANDLE,
    lInitialCount: LONG,
    lMaximumCount: LONG,
    pszName: *const WCHAR,
) -> u32 {
    let status: u32 = 0;

    // CreateSemaphoreExW with sem_all_access (0x1F0003)
    let hSemaphore = CreateSemaphoreExW(
        core::ptr::null_mut(),
        lInitialCount,
        lMaximumCount,
        pszName,
        0,
        0x1F0003,
    );

    if !hSemaphore.is_null() {
        GetLastError();

        if !ptpOutHandle.is_null() {
            let hOldHandle = *ptpOutHandle;
            if !hOldHandle.is_null() {
                let last_error = GetLastError();
                if CloseHandle(hOldHandle) == 0 {
                    HandleHandleCloseError(0, 2525, 0, 0);
                }
                SetLastError(last_error);
            }
            *ptpOutHandle = hSemaphore;
        }
    } else {
        return GetLastError();
    }

    status
}

pub unsafe fn SafeResetThreadpoolTimer(
    ppOldTimerSlot: *mut *mut TP_TIMER,
    pNewTimer: *mut TP_TIMER,
) {
    if ppOldTimerSlot.is_null() {
        return;
    }

    let pTimer = *ppOldTimerSlot;
    if !pTimer.is_null() {
        let last_error = GetLastError();
        SetThreadpoolTimer(pTimer, core::ptr::null(), 0, 0);
        WaitForThreadpoolTimerCallbacks(pTimer, 1);
        CloseThreadpoolTimer(pTimer);
        SetLastError(last_error);
    }

    *ppOldTimerSlot = pNewTimer;
}

pub unsafe fn AtomicIncrementRefCount72(a1: *mut u8) -> u32 {
    if a1.is_null() {
        return 0;
    }

    // Offset by 72 bytes to target the reference count field
    let target_ptr = a1.add(72) as *const AtomicI32;
    let atomic_ref = &*target_ptr;

    // _InterlockedIncrement performs an atomic add of 1 and returns the resulting incremented value.
    // fetch_add returns the previous value, so adding 1 yields the new value.
    (atomic_ref.fetch_add(1, Ordering::SeqCst) + 1) as u32
}



pub unsafe fn SetRelativeTimer(a1: *mut TP_TIMER, a2: u32) {
    if a1.is_null() {
        return;
    }

    // Windows FILETime relative time is expressed in 100-nanosecond intervals (negative value)
    let relative_time = -10000i64 * (a2 as i64);
    let pft_due_time = FILETIME {
        dwLowDateTime: (relative_time & 0xFFFFFFFF) as u32,
        dwHighDateTime: ((relative_time >> 32) & 0xFFFFFFFF) as u32,
    };

    if dword_14046B7E8 == 0 {
        // 0x1388u = 5000 milliseconds window length
        SetThreadpoolTimer(a1, &pft_due_time, 0, 0x1388);
    }
}