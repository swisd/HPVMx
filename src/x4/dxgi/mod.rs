use alloc::ffi::CString;
use alloc::vec::Vec;
use pipeline::PipelineCoordinatorDestroy;
use crate::x4::dxgi::lane::{CleanupFrameTracking, FreeLanePayloadBuffers, InitContextInternalPools, InitSharedMapping, LookupExistingSharedContext, SwapLaneSnapshots};
use crate::x4::dxgi::pipeline::BindPipelineShaderResources;
use crate::x4::error::TraceProviderEvent;
use crate::x4::externals::{AcquireSRWLockExclusive, CloseHandle, CloseThreadpoolTimer, CreateMutexExW, DeleteCriticalSection, EnterCriticalSection, GetCurrentProcessId, GetLastError, GetModuleHandleW, GetProcAddress, GetProcessHeap, HeapFree, InitializeCriticalSectionEx, LeaveCriticalSection, ReleaseMutex, ReleaseSRWLockExclusive, SetLastError, SetThreadpoolTimer, WaitForSingleObjectEx, WaitForThreadpoolTimerCallbacks};
use crate::x4::globals::{pQueueMgr, GlobalDXGISwapChainPresentWrapper, GlobalExecutionCoordinator, g_pfnConditionCheck, g_DisableBypassCheck, _guard_check_icall_fptr, GlobalPfnPresentPrimary, GlobalPfnPresentFallback, lpSource};
use crate::x4::helpers::{AllocateFromHeap, NormalizingExitRegisterWrapper, StringCchPrintfW};
use crate::x4::rawasm::FARPROC;
use crate::x4::threading::{HandleHandleCloseError, ReportLockFailureException, SafeResetThreadpoolTimer};
use crate::x4::types::{ExecutionCoordinator, RenderContextBlock, ShaderManager, LPCRITICAL_SECTION, ContextSyncHandles, PTP_TIMER, PipelineBindingDescriptor, PSRWLOCK, SharedContextBlock};

pub mod pipeline;
pub mod buffer;
mod lane;

pub unsafe fn OnShaderManagerModuleExit() -> i64 {
    // Capture your cleanup hook and cast/transmute it to match the expected exit signature
    let exit_callback: unsafe fn() -> i32 = CheckAndCleanupShaderSystem;

    NormalizingExitRegisterWrapper(core::mem::transmute(exit_callback))
}

pub unsafe fn InitializeExecutionCoordinator() -> i64 {
    // Initialize critical section structures
    InitializeCriticalSectionEx(core::ptr::addr_of_mut!(GlobalExecutionCoordinator.queue_lock), 0, 0);
    GlobalExecutionCoordinator.unk_state_trigger = 0;

    // *(_OWORD *)GlobalExecutionCoordinator.pad_112_135 = 0;
    // *(_OWORD *)&GlobalExecutionCoordinator.pad_112_135[16] = 0;
    let pad_112_ptr = GlobalExecutionCoordinator.pad_112_135.as_mut_ptr();
    *(pad_112_ptr as *mut u128) = 0;
    *(pad_112_ptr.add(16) as *mut u128) = 0;

    InitializeCriticalSectionEx(core::ptr::addr_of_mut!(GlobalExecutionCoordinator.state_lock), 0, 0);
    GlobalExecutionCoordinator.heap_buffer_2 = core::ptr::null_mut();
    GlobalExecutionCoordinator.is_active_flag = 1;

    // *(_OWORD *)GlobalExecutionCoordinator.pad_192_215 = 0;
    // *(_OWORD *)&GlobalExecutionCoordinator.pad_192_215[16] = 0;
    let pad_192_ptr = GlobalExecutionCoordinator.pad_192_215.as_mut_ptr();
    *(pad_192_ptr as *mut u128) = 0;
    *(pad_192_ptr.add(16) as *mut u128) = 0;

    // *(_OWORD *)&GlobalExecutionCoordinator.telemetry_flag = 0;
    let telemetry_ptr = GlobalExecutionCoordinator.telemetry_flag.as_mut_ptr();
    *(telemetry_ptr as *mut u128) = 0;

    // *(_OWORD *)&GlobalExecutionCoordinator.pad_232_255[8] = 0;
    let pad_232_ptr = GlobalExecutionCoordinator.pad_232_255.as_mut_ptr().add(8);
    *(pad_232_ptr as *mut u128) = 0;

    // Return using your explicit cleanup registration callback
    // (Casting function pointer to the expected _onexit_t signature format)
    let exit_callback: unsafe fn() -> i32 = CheckAndCleanupExecutionSystem;
    NormalizingExitRegisterWrapper(core::mem::transmute(exit_callback))
}

pub unsafe extern "fastcall" fn DXGISwapChainPresentWrapper(
    SyncInterval: u32,
    presentFlags: i32,
    Reserved: u32,
) -> i64 {
    let mut result: i64 = 0; // Uninitialized storage fallback to match decompilation path

    // PresentFn = GlobalPfnPresentPrimary;
    let mut PresentFn: Option<unsafe extern "fastcall" fn(u32, u32, u32, u32) -> i64> =
        core::mem::transmute(GlobalPfnPresentPrimary);

    // modifiedFlags = presentFlags | 0x40000000; (DXGI_PRESENT_ALLOW_TEARING)
    let modifiedFlags = (presentFlags as u32 | 0x40000000) as u32;

    if GlobalPfnPresentPrimary != 0 || {
        PresentFn = core::mem::transmute(GlobalPfnPresentFallback);
        PresentFn.is_some()
    } {
        // Control Flow Guard check on the target function pointer
        _guard_check_icall_fptr();

        if let Some(func) = PresentFn {
            return func(SyncInterval, modifiedFlags, Reserved, 0);
        }
    }

    result
}

pub unsafe fn RenderContextBlockFinalizeTeardown(a1: *mut RenderContextBlock) -> i32 {
    let mut result: i32 = 0;

    // Capture the return address (retaddr) off the execution stack frames
    let retaddr: usize;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
    "mov {}, [rbp + 8]", // Standard x64 layout offset for the parent frame pointer 
    out(reg) retaddr
    );
    #[cfg(not(target_arch = "x86_64"))]
    { retaddr = 0; } // Fallback tracking for non-x64 targeting compilation steps

    // 1. Destroy the nested pipeline coordinator structure
    PipelineCoordinatorDestroy(core::ptr::addr_of_mut!((*a1).pipeline_manager));

    // 2. Handle completion_event_1
    let completion_event_1 = (*a1).sync_events.completion_event_1;
    if !completion_event_1.is_null() {
        result = CloseHandle(completion_event_1);
        if result == 0 {
            // Uninitialized registers v4/v5 are passed down. Rust defaults them safely to zero.
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
    }

    // 3. Handle completion_event_0
    let completion_event_0 = (*a1).sync_events.completion_event_0;
    if !completion_event_0.is_null() {
        result = CloseHandle(completion_event_0);
        if result == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
    }

    // 4. Handle mutex_handle
    let mutex_handle = (*a1).mutex_handle;
    if !mutex_handle.is_null() {
        result = CloseHandle(mutex_handle);
        if result == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
    }

    result
}

pub unsafe fn DestroyShaderManagerAndFree(manager: *mut ShaderManager) {
    // 1. Mark status as inactive and halt immediate timers
    (*manager).status_flag = 0;
    SafeResetThreadpoolTimer(core::ptr::addr_of_mut!((*manager).timer), core::ptr::null_mut());
    CleanupShaderResourcesTimerReset(manager);

    let sub_object_2 = (*manager).sub_object_2;

    // Core function pointer targets resolved cleanly under __fastcall
    let mut v3: Option<unsafe extern "fastcall" fn(*mut core::ffi::c_void)> =
        core::mem::transmute(qword_14046B490);
    let mut v4: Option<unsafe extern "fastcall" fn(*mut core::ffi::c_void)> =
        core::mem::transmute(qword_14046B4B8);

    if !sub_object_2.is_null() {
        if qword_14046B490 != 0 {
            _guard_check_icall_fptr();
            if let Some(func) = v3 {
                func(sub_object_2);
            }
        } else {
            if qword_14046B4B8 != 0 {
                _guard_check_icall_fptr();
                if let Some(func) = v4 {
                    func(sub_object_2);
                }
            } else {
                // Skips processing layout blocks matching LABEL_8
                v3 = core::mem::transmute(qword_14046B490);
                v4 = core::mem::transmute(qword_14046B4B8);
                return finalize_allocations_and_timer(manager, v3, v4);
            }
        }
        v3 = core::mem::transmute(qword_14046B490);
    }
    v4 = core::mem::transmute(qword_14046B4B8);

    finalize_allocations_and_timer(manager, v3, v4);
}

/// Extracted block handling label sequences following the cleanup of sub-objects.
unsafe fn finalize_allocations_and_timer(
    manager: *mut ShaderManager,
    v3: Option<unsafe extern "fastcall" fn(*mut core::ffi::c_void)>,
    v4: Option<unsafe extern "fastcall" fn(*mut core::ffi::c_void)>,
) {
    let sub_object_1 = (*manager).sub_object_1;
    if !sub_object_1.is_null() {
        if let Some(func) = v3 {
            _guard_check_icall_fptr();
            func(sub_object_1);
        } else if let Some(func) = v4 {
            _guard_check_icall_fptr();
            func(sub_object_1);
        }
    }

    // 2. Free internal heap allocations 
    let heap_buffer_2 = (*manager).heap_buffer_2;
    (*manager).heap_buffer_2 = core::ptr::null_mut();
    if !heap_buffer_2.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, heap_buffer_2);
    }

    let heap_buffer_1 = (*manager).heap_buffer_1;
    (*manager).heap_buffer_1 = core::ptr::null_mut();
    if !heap_buffer_1.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, heap_buffer_1);
    }

    // 3. Dismantle Threadpool Windows Kernel Objs
    let timer = (*manager).timer;
    if !timer.is_null() {
        SetThreadpoolTimer(timer, core::ptr::null_mut(), 0, 0);
        WaitForThreadpoolTimerCallbacks(timer, 1);
        CloseThreadpoolTimer(timer);
    }
}

pub unsafe fn ShutdownAsyncQueueSystem(a1: *mut ExecutionCoordinator) {
    // 1. Mark system as inactive and pull timers out of processing states
    (*a1).is_active_flag = 0;
    SafeResetThreadpoolTimer(core::ptr::addr_of_mut!((*a1).update_timer_0), core::ptr::null_mut());
    SafeResetThreadpoolTimer(core::ptr::addr_of_mut!((*a1).update_timer_1), core::ptr::null_mut());

    // 2. Clear out heap_buffer_2
    let heap_buffer_2 = (*a1).heap_buffer_2;
    (*a1).heap_buffer_2 = core::ptr::null_mut();
    if !heap_buffer_2.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, heap_buffer_2);
    }

    // 3. Conditional critical context removal
    let telemetry_flag = (*a1).telemetry_flag;
    let base_ptr_from_pad = *(GlobalExecutionCoordinator.pad_24_47.as_ptr() as *const *mut core::ffi::c_void);

    if telemetry_flag != 0 && !base_ptr_from_pad.is_null() {
        // Offset mapping match: *base_ptr + 200LL for critical section
        let critical_section_target = (base_ptr_from_pad as usize).wrapping_add(200) as *mut core::ffi::c_void;
        let srwlock_target = base_ptr_from_pad;

        SafeRemoveContextEntryLPCritical(
            critical_section_target,
            srwlock_target,
            telemetry_flag as i64,
        );
    }

    // 4. Clear out heap_buffer_1
    let heap_buffer_1 = (*a1).heap_buffer_1;
    (*a1).heap_buffer_1 = core::ptr::null_mut();
    if !heap_buffer_1.is_null() {
        let proc_heap_handle_0 = GetProcessHeap();
        HeapFree(proc_heap_handle_0, 0, heap_buffer_1);
    }

    // 5. Release state lock and clean up notifications
    DeleteCriticalSection(core::ptr::addr_of_mut!((*a1).state_lock));
    let unk_state_trigger = (*a1).unk_state_trigger;
    if unk_state_trigger != 0 {
        UnregisterFeatureConfigurationChangeNotificationWrapper(unk_state_trigger as i64);
    }

    // 6. Clear out heap_buffer_0
    let heap_buffer_0 = (*a1).heap_buffer_0;
    (*a1).heap_buffer_0 = core::ptr::null_mut();
    if !heap_buffer_0.is_null() {
        let v9 = GetProcessHeap();
        HeapFree(v9, 0, heap_buffer_0);
    }

    // 7. Delete queue lock and dismantle kernel thread timers
    DeleteCriticalSection(core::ptr::addr_of_mut!((*a1).queue_lock));

    let update_timer_1 = (*a1).update_timer_1;
    if !update_timer_1.is_null() {
        SetThreadpoolTimer(update_timer_1, core::ptr::null_mut(), 0, 0);
        WaitForThreadpoolTimerCallbacks(update_timer_1, 1);
        CloseThreadpoolTimer(update_timer_1);
    }

    let update_timer_0 = (*a1).update_timer_0;
    if !update_timer_0.is_null() {
        SetThreadpoolTimer((*a1).update_timer_0, core::ptr::null_mut(), 0, 0);
        WaitForThreadpoolTimerCallbacks(update_timer_0, 1);
        CloseThreadpoolTimer(update_timer_0);
    }

    // 8. Unlink structural render contexts
    let render_context = (*a1).render_context;
    if !render_context.is_null() {
        RenderContextBlockRelease(render_context);
    }
}

pub unsafe fn GetOrCreateRenderContextBlock(
    pSubsystemName: *const core::ffi::c_char,
    ppOutContext: *mut *mut RenderContextBlock,
) -> i32 {
    // 1. Initial configuration safeguards
    *ppOutContext = core::ptr::null_mut();

    let mut retaddr: usize;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("mov {}, [rbp + 8]", out(reg) retaddr);
    #[cfg(not(target_arch = "x86_64"))]
    { retaddr = 0; }

    let current_process_id = GetCurrentProcessId();
    let mut name_buffer = [0u16; 264];

    // Format wide-string: "Local\\SM0:%lu:%lu:%hs"
    // Format parameters require explicit matching strings
    let format_str = encode_wide_str("Local\\SM0:%lu:%lu:%hs\0");
    StringCchPrintfW(
        name_buffer.as_mut_ptr(),
        0x104,
        format_str.as_ptr(),
        current_process_id,
        120,
        pSubsystemName,
    );

    let mut v6 = CreateMutexExW(core::ptr::null_mut(), name_buffer.as_ptr(), 0, 0x1F0001);
    if v6.is_null() {
        return GetLastError() as i32;
    }

    let mut v10: *mut core::ffi::c_void = core::ptr::null_mut();
    let v8 = WaitForSingleObjectEx(v6, 0xFFFFFFFF, 0);
    if v8 != 258 {
        if (v8 & 0xFFFFFF7F) != 0 {
            ReportLockFailureException(retaddr as u64);
        }
        v10 = v6;
    }

    // 2. Lookup existing mapping logic
    let mut v43: u64 = 0;
    let v11 = LookupExistingSharedContext(name_buffer.as_ptr() as i64, 0, &mut v43);
    let mut v13 = v11 as u32;

    if v11 < 0 {
        TraceProviderEvent(retaddr as u64, 100, 0, v11 as u32 as u64);
        TraceProviderEvent(retaddr as u64, 109, 0, v13 as u64);
        TraceProviderEvent(retaddr as u64, 299, 0, v13 as u64);
        cleanup_locks_and_exit(retaddr, v10, v6);
        return v13 as i32;
    }

    let v20 = (4 * v43) as *mut RenderContextBlock;
    if !v20.is_null() {
        *ppOutContext = v20;
        (*v20).reference_count += 1;
        return finalize_success_path(retaddr, v10, v6);
    }

    *ppOutContext = core::ptr::null_mut();

    // Allocate 0x78 bytes (120 bytes) from custom heap framework
    let v21 = AllocateFromHeap(8, 0x78) as *mut RenderContextBlock;
    let v23 = v21;

    let mut h_object = [core::ptr::null_mut(); 3];
    let mut v24: i32;

    if !v21.is_null() {
        let inited = InitSharedMapping(h_object.as_mut_ptr() as i64, name_buffer.as_ptr() as i64, v21 as u64);
        v24 = inited as i32;

        if inited >= 0 {
            let v37 = h_object[0];
            (*v23).mutex_handle = v6;
            v6 = core::ptr::null_mut();
            (*v23).sync_events.completion_event_0 = v37;

            let v38 = h_object[1];
            h_object[0] = core::ptr::null_mut();
            h_object[1] = core::ptr::null_mut();
            (*v23).reference_count = 1;
            (*v23).sync_events.completion_event_1 = v38;

            // Equivalent to: memset((char *)&v23->pipeline_manager.unk_flags_or_id + 2, 0, 0x56u);
            let unk_flags_byte_ptr = (core::ptr::addr_of_mut!((*v23).pipeline_manager.unk_flags_or_id) as *mut u8).add(2);
            core::ptr::write_bytes(unk_flags_byte_ptr, 0, 0x56);

            // Equivalent to: LOWORD(...) = 88; HIDWORD(...) = 1;
            let unk_flags_ptr = core::ptr::addr_of_mut!((*v23).pipeline_manager.unk_flags_or_id);
            *(unk_flags_ptr as *mut u16) = 88;
            *(unk_flags_ptr as *mut u32).add(1) = 1;

            // Equivalent to: memset(v23->pipeline_manager.8, 0, 0x50u);
            let padding_array_ptr = (*v23).pipeline_manager.pad_240_263.as_mut_ptr();
            core::ptr::write_bytes(padding_array_ptr, 0, 0x50);

            *ppOutContext = v23;
            return finalize_success_path(retaddr, v10, v6);
        }

        TraceProviderEvent(retaddr as u64, 331, 0, inited as u32 as u64);
        if !h_object[1].is_null() && CloseHandle(h_object[1]) == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        if !h_object[0].is_null() && CloseHandle(h_object[0]) == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }

        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v23 as *mut _);
    } else {
        v24 = -2147024882; // 0x8007000E (E_OUTOFMEMORY)
        TraceProviderEvent(retaddr as u64, 328, 0, 2147942414);
    }

    TraceProviderEvent(retaddr as u64, 308, 0, v24 as u32 as u64);
    cleanup_locks_and_exit(retaddr, v10, v6);
    v24
}

/// Consolidated logic block mapping structural mutex unlock sequences
unsafe fn finalize_success_path(retaddr: usize, v10: *mut core::ffi::c_void, v6: *mut core::ffi::c_void) -> i32 {
    if !v10.is_null() && ReleaseMutex(v10) == 0 {
        HandleHandleCloseError(retaddr as i32, 2535, 0, 0);
    }
    if !v6.is_null() && CloseHandle(v6) == 0 {
        HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
    }
    0
}

/// Fallback error path clearing active synchronization state descriptors
unsafe fn cleanup_locks_and_exit(retaddr: usize, v10: *mut core::ffi::c_void, v6: *mut core::ffi::c_void) {
    if !v10.is_null() && ReleaseMutex(v10) == 0 {
        HandleHandleCloseError(retaddr as i32, 2535, 0, 0);
    }
    if !v6.is_null() && CloseHandle(v6) == 0 {
        HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
    }
}

/// Helper function to create null-terminated UTF-16 arrays
fn encode_wide_str(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

pub unsafe fn CloseContextSyncHandles(pSyncStruct: *mut ContextSyncHandles) {
    let mut retaddr: usize;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!(
    "mov {}, [rbp + 8]",
    out(reg) retaddr
    );
    #[cfg(not(target_arch = "x86_64"))]
    { retaddr = 0; }

    // 1. Process completion_event_0
    let completion_event_0 = (*pSyncStruct).completion_event_0;
    if !completion_event_0.is_null() {
        let LastError = GetLastError();
        if CloseHandle(completion_event_0) == 0 {
            // Uninitialized registers v4/v5 passed down default safely to zero
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        SetLastError(LastError);
    }
    (*pSyncStruct).completion_event_0 = core::ptr::null_mut();

    // 2. Process completion_event_1
    let completion_event_1 = (*pSyncStruct).completion_event_1;
    if !completion_event_1.is_null() {
        let v7 = GetLastError();
        if CloseHandle(completion_event_1) == 0 {
            // Uninitialized registers v8/v9 passed down default safely to zero
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        SetLastError(v7);
    }
    (*pSyncStruct).completion_event_1 = core::ptr::null_mut();
}

pub unsafe fn CleanupShaderResourcesTimerReset(manager: *mut ShaderManager) {
    // 1. Reset state indicators
    (*manager).status_flag = 0;

    // Capture the pointer address of the SRW lock
    let p_lock = core::ptr::addr_of_mut!((*manager).lock);

    let timer = (*manager).timer;
    (*manager).timer = core::ptr::null_mut();

    // Local stack tracking variable: pti = timer;
    let mut pti: PTP_TIMER = timer;

    // 2. Lock context and flush resources
    AcquireSRWLockExclusive(p_lock);
    FlushDirtyShaderResources(manager);

    // Original decompiler sanity check: if ( p_lock )
    if !p_lock.is_null() {
        ReleaseSRWLockExclusive(p_lock);
    }

    // 3. Threadpool clean up lifecycle sequence
    SafeResetThreadpoolTimer(core::ptr::addr_of_mut!(pti), core::ptr::null_mut());
    let v4 = pti;

    if !pti.is_null() {
        SetThreadpoolTimer(pti, core::ptr::null_mut(), 0, 0);
        WaitForThreadpoolTimerCallbacks(v4, 1);
        CloseThreadpoolTimer(v4);
    }
}

pub unsafe fn FlushDirtyShaderResources(context: *mut ShaderManager) -> i64 {
    let mut vector_end = (*context).vector_end;
    let mut vector_begin = (*context).vector_begin;

    // Pointer subtraction tracking bytes distance: (char*)vector_end - (char*)vector_begin
    let distance_bytes = (vector_end as usize).saturating_sub(vector_begin as usize);
    let mut result = distance_bytes as i64;

    if distance_bytes >= 0x10 {
        // Stack block structures configured continuously to mirror local stack frame allocation
        // C++ maps: PipelineBindingDescriptor pArray; char v13;
        // This is a 16-byte structure. We configure a contiguous array buffer to represent this safely.
        let mut pArray_buffer = [core::mem::MaybeUninit::<PipelineBindingDescriptor>::uninit().assume_init(); 8];
        let pArray_base_ptr = pArray_buffer.as_mut_ptr();

        while vector_begin != vector_end {
            let StatePtr = (*vector_begin).StatePtr;
            let BindSlot = (*vector_begin).BindSlot;

            // _m_prefetchw((const void *)StatePtr);
            // Rust architecture intrinsic for write-intent hardware data caching prefetch
            #[cfg(target_arch = "x86_64")]
            core::arch::x86_64::_m_prefetchw(StatePtr as *const core::ffi::c_void);

            // Atomic bitwise AND operation tracking state bit flag groups
            // v7 = _InterlockedAnd((volatile signed __int32 *)StatePtr, 0xFFC0401E);
            let v7 = core::intrinsics::atomic_and_seqcst(StatePtr as *mut u32, 0xFFC0401E);

            let mut v8 = (v7 >> 1) & 0xF;
            if v8 != 0 {
                // _m_prefetchw((char *)StatePtr + 4);
                #[cfg(target_arch = "x86_64")]
                core::arch::x86_64::_m_prefetchw((StatePtr as *const u8).add(4) as *const core::ffi::c_void);

                // v8 &= ~_InterlockedOr((volatile signed __int32 *)StatePtr + 1, v8);
                let high_dword_ptr = (StatePtr as *mut u32).add(1);
                let old_high_dword = core::intrinsics::atomic_or_seqcst(high_dword_ptr, v8);
                v8 &= !old_high_dword;
            }

            let mut p_pArray = pArray_base_ptr;

            if (v8 & 1) != 0 {
                (*p_pArray).BindSlot = BindSlot;
                // *(_DWORD *)&pArray.ResourceCount = 65538;
                // Writing 65538 (0x00010002) starting at ResourceCount overrides both ResourceCount and VisibilityFlags
                let casting_ptr = core::ptr::addr_of_mut!((*p_pArray).ResourceCount) as *mut u32;
                *casting_ptr = 65538;

                // p_pArray = (PipelineBindingDescriptor *)&v13;
                // C++ steps forward by 1 (the size of PipelineBindingDescriptor, because of how structural stack alias tracks)
                p_pArray = p_pArray.add(1);
            }
            if (v8 & 2) != 0 {
                (*p_pArray).BindSlot = BindSlot;
                let casting_ptr = core::ptr::addr_of_mut!((*p_pArray).ResourceCount) as *mut u32;
                *casting_ptr = 65542; // 0x00010006
                p_pArray = p_pArray.add(1);
            }
            if (v8 & 4) != 0 {
                (*p_pArray).BindSlot = BindSlot;
                let casting_ptr = core::ptr::addr_of_mut!((*p_pArray).ResourceCount) as *mut u32;
                *casting_ptr = 65539; // 0x00010003
                p_pArray = p_pArray.add(1);
            }
            if v8 >= 8 {
                (*p_pArray).BindSlot = BindSlot;
                let casting_ptr = core::ptr::addr_of_mut!((*p_pArray).ResourceCount) as *mut u32;
                *casting_ptr = 65543; // 0x00010007
                p_pArray = p_pArray.add(1);
            }
            if ((v7 >> 5) & 0x1FF) != 0 {
                (*p_pArray).BindSlot = BindSlot;
                (*p_pArray).VisibilityFlags = ((v7 >> 5) & 0x1FF) as u16;
                (*p_pArray).ResourceCount = (4 * ((v7 >> 14) & 1)) as u16;
                p_pArray = p_pArray.add(1);
            }
            if ((v7 >> 15) & 0x7F) != 0 {
                (*p_pArray).BindSlot = BindSlot;
                (*p_pArray).VisibilityFlags = ((v7 >> 15) & 0x7F) as u16;
                (*p_pArray).ResourceCount = (4 * ((v7 >> 22) & 1) + 1) as u16;
                p_pArray = p_pArray.add(1);
            }

            // Calculation of populated item depth slice: p_pArray - &pArray;
            let v10 = (p_pArray as usize - pArray_base_ptr as usize) / core::mem::size_of::<PipelineBindingDescriptor>();
            if v10 > 0 {
                BindPipelineShaderResources(pArray_base_ptr, v10 as i64);
            }

            vector_begin = vector_begin.add(1);
        }

        // 2. Transmute and invoke the completion frame callbacks
        let mut v11: Option<unsafe extern "fastcall" fn(u32, u32, u32, u32) -> i64> =
            core::mem::transmute(GlobalPfnPresentPrimary);

        result = (*context).vector_begin as i64;
        (*context).vector_end = (*context).vector_begin;

        if GlobalPfnPresentPrimary != 0 || {
            v11 = core::mem::transmute(GlobalPfnPresentFallback);
            v11.is_some()
        } {
            _guard_check_icall_fptr();
            if let Some(func) = v11 {
                return func(0, 254, 0, 0);
            }
        }
    }

    result
}

pub unsafe fn DXGISwapChainGetOrInitializePresentWrapper() -> unsafe extern "fastcall" fn(u32, i32, u32) -> i64 {

    // GlobalDXGISwapChainPresentWrapper = (i64)DXGISwapChainPresentWrapper;
    GlobalDXGISwapChainPresentWrapper = DXGISwapChainPresentWrapper as usize as i64;

    // return DXGISwapChainPresentWrapper;
    DXGISwapChainPresentWrapper
}

pub unsafe fn RenderContextBlockRelease(lpMem: *mut RenderContextBlock) {
    let mut retaddr: usize;
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("mov {}, [rbp + 8]", out(reg) retaddr);
    #[cfg(not(target_arch = "x86_64"))]
    { retaddr = 0; }

    // Resolve condition checking function pointer
    let v2: Option<unsafe extern "fastcall" fn() -> u8> = core::mem::transmute(g_pfnConditionCheck);

    // if ( g_DisableBypassCheck || (v2 != nullptr && (_guard_check_icall_fptr(), v2())) )
    if g_DisableBypassCheck || (g_pfnConditionCheck != 0 && {
        _guard_check_icall_fptr();
        v2.unwrap_unchecked()() != 0
    }) {
        // Decrement reference count
        (*lpMem).reference_count -= 1;
        if (*lpMem).reference_count == 0 {
            let mut pDestSlot = core::mem::MaybeUninit::<SharedContextBlock>::uninit().assume_init();
            InitContextInternalPools(&mut pDestSlot);

            // Swap lanes conditionally
            if (*lpMem).pipeline_manager.lane_array[0].is_dirty_or_active != 0 {
                SwapLaneSnapshots(
                    pDestSlot.lanes.as_mut_ptr(),
                    (*lpMem).pipeline_manager.lane_array.as_mut_ptr(),
                );
            }
            if (*lpMem).pipeline_manager.lane_array[1].is_dirty_or_active != 0 {
                SwapLaneSnapshots(
                    pDestSlot.lanes.as_mut_ptr().add(1),
                    (*lpMem).pipeline_manager.lane_array.as_mut_ptr().add(1),
                );
            }
            if (*lpMem).pipeline_manager.lane_array[2].is_dirty_or_active != 0 {
                SwapLaneSnapshots(
                    pDestSlot.lanes.as_mut_ptr().add(2),
                    (*lpMem).pipeline_manager.lane_array.as_mut_ptr().add(2),
                );
            }

            CleanupFrameTracking(&mut pDestSlot);
            FreeLanePayloadBuffers(&mut pDestSlot as *mut _ as *mut u8);
        }
    } else {
        // Mutex synchronization fallback path
        let mut mutex_handle = (*lpMem).mutex_handle;
        let v4 = WaitForSingleObjectEx(mutex_handle, 0xFFFFFFFF, 0);

        if v4 == 258 {
            mutex_handle = core::ptr::null_mut();
        } else if (v4 & 0xFFFFFF7F) != 0 {
            ReportLockFailureException(retaddr as u64);
        }

        (*lpMem).reference_count -= 1;
        if (*lpMem).reference_count != 0 {
            if !mutex_handle.is_null() && ReleaseMutex(mutex_handle) == 0 {
                HandleHandleCloseError(retaddr as i32, 2535, 0, 0);
            }
        } else {
            // Full system lifecycle reclamation path
            CloseContextSyncHandles(core::ptr::addr_of_mut!((*lpMem).sync_events) as *mut _);

            if !mutex_handle.is_null() {
                let last_error = GetLastError();
                if ReleaseMutex(mutex_handle) == 0 {
                    HandleHandleCloseError(retaddr as i32, 2535, 0, 0);
                }
                SetLastError(last_error);
            }

            RenderContextBlockFinalizeTeardown(lpMem);
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, lpMem as *mut _);
        }
    }
}

pub unsafe fn UnregisterFeatureConfigurationChangeNotificationWrapper(a1: i64) -> FARPROC {
    let mut v1 = core::mem::transmute::<usize, FARPROC>(qword_14046B270);

    if qword_14046B270 != 0 {
        // Equivalent to: goto LABEL_5;
        _guard_check_icall_fptr();
        let target_fn = core::mem::transmute::<FARPROC, unsafe extern "fastcall" fn(i64) -> FARPROC>(v1);
        return target_fn(a1);
    }

    let mut module_handle_w = lpSource;
    if lpSource.is_null() {
        let ntdll_name = encode_wide_str("ntdll.dll\0");
        module_handle_w = GetModuleHandleW(ntdll_name.as_ptr()) as *mut core::ffi::c_void;
        lpSource = module_handle_w;
    }

    // Lookup process entry point address
    let proc_name = core::ffi::CString::new("RtlUnregisterFeatureConfigurationChangeNotification").unwrap();
    let result = GetProcAddress(module_handle_w as *mut _, proc_name.as_ptr());

    qword_14046B270 = result as usize;
    v1 = core::mem::transmute::<*mut core::ffi::c_void, FARPROC>(result);

    if !result.is_null() {
        // LABEL_5 execution sequence
        _guard_check_icall_fptr();
        let target_fn = core::mem::transmute::<FARPROC, unsafe extern "fastcall" fn(i64) -> FARPROC>(v1);
        return target_fn(a1);
    }

    core::mem::transmute::<*mut core::ffi::c_void, FARPROC>(result)
}

pub unsafe fn SafeRemoveContextEntryLPCritical(
    lpCriticalSection: LPCRITICAL_SECTION,
    srw_lock: PSRWLOCK,
    a3: i64,
) {
    if a3 != 0 {
        EnterCriticalSection(lpCriticalSection);
        AcquireSRWLockExclusive(srw_lock);

        // DebugInfo = lpCriticalSection[1].DebugInfo;
        // In C++, indexing a pointer like array[1] steps forward by the size of the type.
        // We compute the address of index 1 and read its fields.
        let lp_critical_section_1 = lpCriticalSection.add(1);
        let debug_info = (*lp_critical_section_1).DebugInfo as usize;

        // *(_QWORD *)&lpCriticalSection[1].LockCount
        // Reads 8 bytes starting at the LockCount offset of index 1.
        let lock_count_ptr = core::ptr::addr_of!((*lp_critical_section_1).LockCount) as *const u64;
        let lock_count_val = *lock_count_ptr;

        // Condition matching: (a3 - 1) < (lock_count_val - debug_info) >> 4
        let index_to_zero = (a3 - 1) as u64;
        let bounds_check_limit = lock_count_val.saturating_sub(debug_info as u64) >> 4;

        if index_to_zero < bounds_check_limit {
            // *((_OWORD *)DebugInfo + a3 - 1) = 0;
            // _OWORD matches a 128-bit block (16 bytes). We step the casted pointer and zero it.
            let oword_base_ptr = debug_info as *mut u128;
            let target_oword_ptr = oword_base_ptr.offset((a3 - 1) as isize);
            *target_oword_ptr = 0;
        }

        if !srw_lock.is_null() {
            ReleaseSRWLockExclusive(srw_lock);
        }
        if !lpCriticalSection.is_null() {
            LeaveCriticalSection(lpCriticalSection);
        }
    }
}

pub unsafe fn CheckAndCleanupShaderSystem() -> i32 {
    // Resolve condition checking function pointer signature
    let condition_fn: Option<unsafe extern "fastcall" fn() -> u8> = core::mem::transmute(g_pfnConditionCheck);

    // if ( g_DisableBypassCheck || g_pfnConditionCheck && (_guard_check_icall_fptr(), (unsigned __int8)g_pfnConditionCheck()) )
    if g_DisableBypassCheck || (g_pfnConditionCheck != 0 && {
        _guard_check_icall_fptr();
        condition_fn.unwrap_unchecked()() != 0
    }) {
        CleanupShaderResourcesTimerReset(core::ptr::addr_of_mut!(pQueueMgr));
        0
    } else {
        DestroyShaderManagerAndFree(core::ptr::addr_of_mut!(pQueueMgr));
        0
    }
    
}

pub unsafe fn CheckAndCleanupExecutionSystem() -> i32 {
    // Resolve condition checking function pointer signature matching your calling convention
    let condition_fn: Option<unsafe extern "fastcall" fn() -> u8> = core::mem::transmute(g_pfnConditionCheck);

    // if ( g_DisableBypassCheck || g_pfnConditionCheck && (_guard_check_icall_fptr(), (unsigned __int8)g_pfnConditionCheck()) )
    if g_DisableBypassCheck || (g_pfnConditionCheck != 0 && {
        _guard_check_icall_fptr();
        condition_fn.unwrap_unchecked()() != 0
    }) {
        GlobalExecutionCoordinator.is_active_flag = 0;

        let render_context = GlobalExecutionCoordinator.render_context;
        if !render_context.is_null() {
            RenderContextBlockRelease(render_context);
        }
        0
    } else {
        ShutdownAsyncQueueSystem(core::ptr::addr_of_mut!(GlobalExecutionCoordinator));
        0
    }
}