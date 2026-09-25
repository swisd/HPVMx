use core::ffi::c_void;
use core::sync::atomic::{AtomicI32, AtomicI64, Ordering};
use crate::x4::dxgi::buffer::ReallocVectorBufferWithCacheAlignment64;
use crate::x4::dxgi::FlushDirtyShaderResources;
use crate::x4::externals::{memcpy_s, AcquireSRWLockExclusive, ReleaseSRWLockExclusive, GetTickCount, GetProcAddress, GetLastError, SetThreadpoolTimer, SetLastError, CreateThreadpoolTimer, GetModuleHandleExW};
use crate::x4::globals::{GlobalDXGISwapChainPresentWrapper, GlobalPfnPresentFallback, GlobalPfnPresentPrimary, _guard_check_icall_fptr, pQueueMgr, GlobalSppPacketControlFlags};
use crate::x4::spp::calls::SppGetOrUpdateControlFlags;
use crate::x4::spp::queue::SppQueueRegistrationPacket;
use crate::x4::spp::SppIsSubsystemInitialized;
use crate::x4::spp::timer::SppTelemetryTimerCallback;
use crate::x4::threading::SafeResetThreadpoolTimer;
use crate::x4::types::{RtlNtStatusToDosErrorFn, ShaderManager, SppTelemetryQueueManager, HRESULT, NTSTATUS, BOOL, SppStateTransitionResult, SppTelemetryContext, DWORD, UCHAR, HMODULE, _FILETIME};

pub unsafe fn SppCheckTelemetryState(
    pTelemetryContext: *mut SppTelemetryContext,
) -> UCHAR {
    let mut v8: i64 = 0;

    SppGetOrUpdateControlFlags(pTelemetryContext, &mut v8);
    let v3 = (v8 & 1) as UCHAR;

    let mut v4 = GlobalSppPacketControlFlags[0];

    if (GlobalSppPacketControlFlags[0] & 4) == 0 {
        let mut v10: i64 = 0;
        let v9_ptr = SppGetOrUpdateControlFlags(pTelemetryContext, &mut v10);
        if !v9_ptr.is_null() {
            v4 = *v9_ptr as u32;
        }
    }

    let event_data = EtwEventData { v6: 0, v7: 2 };
    let param1_offset = (pTelemetryContext as usize).wrapping_add(8);

    SppEmitEtwEvent(
        param1_offset,
        59974269,
        (v4 >> 10) & 1,
        (v4 >> 11) & 1,
        &event_data.v6 as *const i32,
        v3,
        3,
    );

    unk_14046B398 = GetTickCount();

    v3
}

pub unsafe fn SppEmitEtwEvent(
    hEtwRegistration: ETW_REG_HANDLE,
    eventId: EventId,
    enableVerbose: i32,
    isProviderLive: i32,
    pEventUserData: *const c_void,
    subsystemActiveState: u32,
    eventCategory: i32,
) -> u64 {
    let mut retaddr: usize = 0;
    let mut result = &mut retaddr as *mut usize as u64;

    if eventCategory != 0 {
        let active = subsystemActiveState != 0;
        let translated_opcode = match eventCategory {
            1 => {
                if !active { 4 } else { 0 }
            }
            2 => {
                if active { 1 } else { 5 }
            }
            3 => {
                if active { 2 } else { 6 }
            }
            4 => {
                if active { 3 } else { 7 }
            }
            5 => {
                if active { 8 } else { 10 }
            }
            6 => {
                if active { 9 } else { 11 }
            }
            _ => {
                let diff = eventCategory - 100;
                if diff < 0 || diff > 0x31 {
                    255
                } else {
                    (diff as u32) + if active { 100 } else { 150 }
                }
            }
        };

        let channel_byte = if !pEventUserData.is_null() {
            *(pEventUserData as *const u8).add(4)
        } else {
            0
        };

        let is_enabled = SppIsEtwChannelEnabled(
            hEtwRegistration,
            eventId,
            enableVerbose,
            isProviderLive,
            translated_opcode,
            0,
            0,
            channel_byte,
        );

        if is_enabled != 0 {
            if let Some(icall_ptr) = qword_14046B4C8 {
                _guard_check_icall_fptr();
                let mut mutable_category = eventCategory;
                return icall_ptr(
                    eventId,
                    pEventUserData,
                    0,
                    subsystemActiveState,
                    &mut mutable_category,
                    0,
                    0,
                    1,
                );
            }
        }
    }

    result
}

pub unsafe fn SppFlushTelemetrySynchronous(pQueueMgr: *mut ShaderManager) {
    if pQueueMgr.is_null() {
        return;
    }

    let mgr = &mut *pQueueMgr;

    if mgr.status_flag != 0 {
        AcquireSRWLockExclusive(&mut mgr.lock);

        if mgr.status_flag != 0 {
            FlushDirtyShaderResources(pQueueMgr);
            // Clear the low byte / flag of unk_config_flags
            mgr.unk_config_flags &= !1;
        }

        ReleaseSRWLockExclusive(&mut mgr.lock);
    }
}

pub unsafe fn SppGetRuntimeLoggingOverride(
    _unused_param: u64,
    p_out_control_flags: *mut DWORD,
) -> *mut DWORD {
    if p_out_control_flags.is_null() {
        return p_out_control_flags;
    }

    *p_out_control_flags = 0;
    let mut v3 = dword_14046B420[0] as u8;
    *p_out_control_flags = dword_14046B420[0];

    if (v3 & 6) != 6 {
        let subsystem_status = SppIsSubsystemInitialized();
        let mut policy_mask_payload: i32 = 0;

        let pfn_query_policy = if let Some(func) = qword_14046B478 {
            Some(func)
        } else {
            qword_14046B480
        };

        let raw_policy_bits = if let Some(func) = pfn_query_policy {
            _guard_check_icall_fptr();
            func(58988972, 3, &mut policy_mask_payload)
        } else {
            0
        };

        let v8 = 8 * ((raw_policy_bits & 0x80) | (4 * ((raw_policy_bits & 0x40) | (4 * (raw_policy_bits & 3)))));
        let v9 = if (raw_policy_bits & 0xFFFFFF3F) != 0 {
            let v10 = if (raw_policy_bits & 0xFFFFFF3F) == 2 { 64 } else { 0 };
            v10 | v8
        } else {
            v8 | 0x40
        };

        let mut i = *p_out_control_flags;
        let control_flags_atomic = dword_14046B420.as_mut_ptr() as *const AtomicI32;

        loop {
            let mut v13 = i | 0x40000;
            *p_out_control_flags = v13;

            if policy_mask_payload != 0 && (i & 2) == 0 {
                let v14 = (i | 0x40000)
                    ^ (((v9 as u16) ^ (i as u16)) & 0x180)
                    ^ ((v9
                    ^ (i | 0x40000)
                    ^ (((v9 as u16) ^ (i as u16)) & 0x180) as i32)
                    & 0x40)
                    | 1;
                v13 = (v14 as u16 ^ ((v9 as u16) ^ (v14 as u16)) & 0x800) as i32 | 2;
                *p_out_control_flags = v13 as DWORD;
            }

            if (i & 4) == 0 {
                v13 = (((v9 as u16) ^ (v13 as u16)) & 0x400) as i32 ^ v13 | 4;
                *p_out_control_flags = v13 as DWORD;
            }

            let atomic_ref = &*control_flags_atomic;
            let v15 = atomic_ref.compare_exchange(
                i as i32,
                v13,
                Ordering::SeqCst,
                Ordering::SeqCst,
            );

            match v15 {
                Ok(_) => {
                    i = v13 as DWORD;
                    break;
                }
                Err(actual) => {
                    i = actual as DWORD;
                }
            }
        }

        if (i & 4) == 0 {
            SppQueueRegistrationPacket(dword_14046B420.as_mut_ptr(), 3, subsystem_status);
        }

        if (*p_out_control_flags & 2) == 0 {
            let current_flags = *p_out_control_flags;
            let computed = (current_flags as i32
                ^ (((v9 as u16) ^ (current_flags as u16)) & 0x180)
                ^ ((v9
                ^ current_flags as i32
                ^ (((v9 as u16) ^ (current_flags as u16)) & 0x180) as i32)
                & 0x40)
                | 1)
                ^ (((v9 as u16)
                ^ (current_flags as u16
                ^ (((v9 as u16) ^ (current_flags as u16)) & 0x180)
                ^ (((v9 as u16)
                ^ (current_flags as u16)
                ^ (((v9 as u16) ^ (current_flags as u16)) & 0x180))
                & 0x40)
                | 1))
                & 0x800) as i32;

            *p_out_control_flags = computed as DWORD;
        }
    }

    p_out_control_flags
}

pub unsafe fn SppIsEtwChannelEnabled(
    hEtwRegistration: usize,
    eventId: u32,
    enableVerbose: i32,
    isProviderLive: i32,
    translatedOpcode: u32,
    _a6: u64,
    _a7: u64,
    a8: u8,
) -> bool {
    let mut v25 = SppStateTransitionResult {
        isFirstInitialization: 0,
        previousCounter: 0,
        previousState: 0,
        isTransitionValid: 0,
    };

    let p_calculated_state = SppTransitionEtwControlState(
        &mut v25,
        hEtwRegistration as *const core::sync::atomic::AtomicI32,
        0x0,
        translatedOpcode as i32,
    );

    let should_log = true;
    let v23_0 = (*p_calculated_state).isFirstInitialization;
    let v23_1 = (*p_calculated_state).previousCounter;
    let v23_2 = (*p_calculated_state).previousState;
    let v24 = (*p_calculated_state).isTransitionValid;

    if GlobalDXGISwapChainPresentWrapper.is_some()
        && (translatedOpcode == 0 || translatedOpcode.wrapping_sub(100) <= 0x31)
    {
        _guard_check_icall_fptr();
        if let Some(f) = GlobalDXGISwapChainPresentWrapper {
            f(eventId, translatedOpcode, 1);
        }
    }

    let mut v15 = v23_1;
    if v23_0 != 0 {
        SppScheduleDeferredTelemetry(pQueueMgr, eventId, hEtwRegistration);
        v15 = (*p_calculated_state).previousCounter;
    }

    if v15 != 0 {
        let pfn_etw_write_hook = if let Some(f) = GlobalPfnPresentPrimary {
            Some(f)
        } else {
            GlobalPfnPresentFallback
        };

        if let Some(f) = pfn_etw_write_hook {
            _guard_check_icall_fptr();
            f(eventId, v23_2, v15, 0);
        }
    }

    if v24 == 0 && pQueueMgr.status_flag != 0 {
        AcquireSRWLockExclusive(&mut pQueueMgr.lock);
        if pQueueMgr.sub_object_2.is_null() {
            let pfn_create_sub_object = if let Some(f) = qword_14046B380 {
                Some(f)
            } else {
                qword_14046B410
            };

            if let Some(f) = pfn_create_sub_object {
                _guard_check_icall_fptr();
                f(&mut pQueueMgr.sub_object_2, SppFlushTelemetrySynchronous, -1);
            }
        }
        ReleaseSRWLockExclusive(&mut pQueueMgr.lock);
    }

    if enableVerbose != 0 {
        let v19 = if isProviderLive == 0 {
            translatedOpcode
        } else {
            translatedOpcode | 0x80000000
        };

        let pfn_etw_write_verbose = if let Some(f) = GlobalPfnPresentPrimary {
            Some(f)
        } else {
            GlobalPfnPresentFallback
        };

        if let Some(f) = pfn_etw_write_verbose {
            _guard_check_icall_fptr();
            f(eventId, v19, 0, 0);
        }
    }

    if v24 != 0 {
        return false;
    }

    if let Some(f) = qword_14046B390 {
        _guard_check_icall_fptr();
        f(eventId, translatedOpcode, a8);
    }

    should_log
}

pub unsafe fn SppNtStatusToHresult(
    ntStatus: NTSTATUS,
    pOutHresult: *mut HRESULT,
) -> BOOL {
    let mut v4: HRESULT;

    if pOutHresult.is_null() {
        return 0;
    }

    if ntStatus == 0 {
        *pOutHresult = 0;
        v4 = 0;
        return if v4 >= 0 { 1 } else { 0 };
    }

    let atomic_qword = &qword_14046B538 as *const i64 as *const AtomicI64;
    let mut v5_val = (*atomic_qword).load(Ordering::SeqCst);

    if v5_val == 0 {
        let mut phModule: HMODULE = core::ptr::null_mut();
        // L"ntdll.dll"
        let ntdll_name: &[u16] = &[
            b'n' as u16, b't' as u16, b'd' as u16, b'l' as u16, b'l' as u16,
            b'.' as u16, b'd' as u16, b'l' as u16, b'l' as u16, 0,
        ];

        if GetModuleHandleExW(1, ntdll_name.as_ptr(), &mut phModule) != 0 && !phModule.is_null() {
            let proc_name = b"RtlNtStatusToDosError\0";
            let proc_addr = GetProcAddress(phModule, proc_name.as_ptr());
            if !proc_addr.is_null() {
                let _ = (*atomic_qword).compare_exchange(
                    0,
                    proc_addr as i64,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                v5_val = (*atomic_qword).load(Ordering::SeqCst);
            }
        }
    }

    if v5_val != 0 {
        let f: RtlNtStatusToDosErrorFn = core::mem::transmute(v5_val as usize);
        _guard_check_icall_fptr();
        let v10 = f(ntStatus) as i32;

        if v10 == 317 {
            v4 = ntStatus | 0x10000000;
        } else if v10 > 0 {
            v4 = ((v10 as u16) as i32) | 0x80070000;
        } else {
            v4 = v10;
        }

        *pOutHresult = v4;
        if v4 >= 0 {
            v4 = -2147467259; // E_FAIL (0x80004005)
            *pOutHresult = v4;
        }
        return if v4 >= 0 { 1 } else { 0 };
    }

    let last_error = GetLastError() as i32;
    let mut v7 = last_error;
    if last_error != 0 {
        if last_error > 0 {
            v7 = ((last_error as u16) as i32) | 0x80070000;
        }
    } else {
        v7 = -2147467259; // E_FAIL
    }
    *pOutHresult = v7;
    0
}

pub unsafe fn SppScheduleDeferredTelemetry(
    pQueueMgr: *mut SppTelemetryQueueManager,
    eventControlCode: i32,
    eventPayload: i64,
) {
    if pQueueMgr.is_null() {
        return;
    }

    let mgr = &mut *pQueueMgr;

    if mgr.status_flag == 0 {
        return;
    }

    if g_DisableBypassCheck == 0 {
        if let Some(cond_check) = g_pfnConditionCheck {
            _guard_check_icall_fptr();
            if cond_check() != 0 {
                return;
            }
        }
    }

    AcquireSRWLockExclusive(&mut mgr.lock);

    if mgr.status_flag != 0 {
        if g_DisableBypassCheck == 0 {
            let mut bypassed = false;
            if let Some(cond_check) = g_pfnConditionCheck {
                _guard_check_icall_fptr();
                if cond_check() != 0 {
                    bypassed = true;
                }
            }

            if !bypassed {
                let mut source: [u64; 2] = [0; 2];
                source[0] = eventControlCode as u64;
                source[1] = eventPayload as u64;

                let queue = &mut mgr.telemetryQueue;
                if ReallocVectorBufferWithCacheAlignment64(queue, 0x10) != 0 {
                    let cap = queue.capacity as *mut u8;
                    let cur = queue.currentCursor as *mut u8;

                    let diff = if (cap as usize) < (cur as usize) {
                        (cur as usize) - (cap as usize)
                    } else {
                        0
                    };

                    memcpy_s(
                        queue.capacity,
                        diff,
                        source.as_ptr() as *const c_void,
                        0x10,
                    );

                    queue.capacity = (cap.add(16)) as *mut c_void;
                }

                let timer_ptr = &mut mgr.hThreadpoolTimer;
                if mgr.isTimerActive == 0 {
                    if (*timer_ptr).is_null() {
                        let last_error = GetLastError();
                        let new_timer = CreateThreadpoolTimer(
                            SppTelemetryTimerCallback,
                            pQueueMgr as *mut c_void,
                            core::ptr::null_mut(),
                        );
                        SafeResetThreadpoolTimer(timer_ptr, new_timer);
                        SetLastError(last_error);
                    }

                    let current_timer = *timer_ptr;
                    if !current_timer.is_null() {
                        // -3000000000 hundred-nanosecond intervals = -300 seconds
                        let due_time_val: i64 = -3000000000;
                        let pft_due_time = &due_time_val as *const i64 as *const _FILETIME;

                        SetThreadpoolTimer(current_timer, pft_due_time, 0, 0x124F8);
                        mgr.isTimerActive = 1;
                    }
                }
            }
        }
    }

    ReleaseSRWLockExclusive(&mut mgr.lock);
}

pub unsafe fn SppTransitionEtwControlState(
    pOutTransitionResult: *mut SppStateTransitionResult,
    pStateBitmask: *mut i32,
    transitionOpcode: i32,
) -> *mut SppStateTransitionResult {
    if pOutTransitionResult.is_null() || pStateBitmask.is_null() {
        return pOutTransitionResult;
    }

    let result = &mut *pOutTransitionResult;
    result.isFirstInitialization = 0;
    result.previousState = 0;
    result.previousCounter = 0;
    result.isTransitionValid = 0;
    result.validationError = 0;

    let atomic_bitmask = pStateBitmask as *const AtomicI32;

    match transitionOpcode {
        0 | 4 => {
            let v26 = transitionOpcode == 4;
            let mut current_mask_value3 = (*atomic_bitmask).load(Ordering::SeqCst);

            loop {
                result.previousCounter = 0;
                let mut target_mask_payload2 = current_mask_value3 | 1;
                let v28 = current_mask_value3;

                if (((current_mask_value3 | 1) >> 14) & 1) != (v26 as i32) {
                    if ((target_mask_payload2 >> 5) & 0x1FF) != 0 {
                        result.previousCounter = ((target_mask_payload2 >> 5) & 0x1FF) as u32;
                        result.previousState = if transitionOpcode == 0 { 4 } else { 0 };
                        target_mask_payload2 = (current_mask_value3 & !0x3FFE) | 1;
                    }
                    let v29 = if transitionOpcode == 4 { 0x4000 } else { 0 };
                    target_mask_payload2 = (target_mask_payload2 & !0x4000) | v29;
                }

                let extracted_counter1 = ((target_mask_payload2 >> 5) & 0x1FF) as u32;
                let mut incremented_counter1 = extracted_counter1 + 1;

                if extracted_counter1 + 1 > 0x1FF || incremented_counter1 < extracted_counter1 {
                    incremented_counter1 = 1;
                    result.previousState = transitionOpcode as u32 as i32;
                    result.previousCounter = extracted_counter1;
                }

                let original_mask_snapshot3 = current_mask_value3;
                let exchange_val = ((target_mask_payload2 as u16
                    ^ (32 * incremented_counter1) as u16)
                    & 0x3FE0) as i32
                    ^ target_mask_payload2;

                let actual = (*atomic_bitmask).compare_exchange(
                    original_mask_snapshot3,
                    exchange_val,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );

                match actual {
                    Ok(val) => {
                        current_mask_value3 = val;
                        break;
                    }
                    Err(val) => {
                        current_mask_value3 = val;
                    }
                }
            }

            result.isFirstInitialization = if (v28 & 1) == 0 { 1 } else { 0 };
            result.isTransitionValid = 0;
        }
        1 | 5 => {
            let v17 = transitionOpcode == 5;
            let mut current_mask_value2 = (*atomic_bitmask).load(Ordering::SeqCst);

            loop {
                result.previousCounter = 0;
                let mut target_mask_payload1 = current_mask_value2 | 1;
                let v19 = current_mask_value2;

                if (((current_mask_value2 | 1) >> 22) & 1) != (v17 as i32) {
                    if ((target_mask_payload1 >> 15) & 0x7F) != 0 {
                        result.previousCounter = ((target_mask_payload1 >> 15) & 0x7F) as u32;
                        let v20 = if transitionOpcode != 1 { 5 } else { 1 };
                        target_mask_payload1 = (current_mask_value2 & !0x3F8001) | 1;
                        result.previousState = v20;
                    }
                    let v21 = if transitionOpcode == 5 { 0x400000 } else { 0 };
                    target_mask_payload1 = (target_mask_payload1 & !0x400000) | v21;
                }

                let extracted_counter = ((target_mask_payload1 >> 15) & 0x7F) as u32;
                let mut incremented_counter = extracted_counter + 1;

                if extracted_counter + 1 > 0x7F || incremented_counter < extracted_counter {
                    incremented_counter = 1;
                    result.previousState = transitionOpcode as u32;
                    result.previousCounter = extracted_counter;
                }

                let original_mask_snapshot2 = current_mask_value2;
                let exchange_val = ((target_mask_payload1
                    ^ (incremented_counter << 15) as i32)
                    & 0x3F8000)
                    ^ target_mask_payload1;

                let actual = (*atomic_bitmask).compare_exchange(
                    original_mask_snapshot2,
                    exchange_val,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );

                match actual {
                    Ok(val) => {
                        current_mask_value2 = val;
                        break;
                    }
                    Err(val) => {
                        current_mask_value2 = val;
                    }
                }
            }

            result.isFirstInitialization = if (v19 & 1) == 0 { 1 } else { 0 };
            result.isTransitionValid = 0;
        }
        2 | 3 | 6 | 7 => {
            let v10 = match transitionOpcode {
                2 => 2,
                3 => 8,
                6 => 4,
                _ => 16,
            };

            let mut current_mask_value1 = (*atomic_bitmask).load(Ordering::SeqCst);
            let mut target_mask_payload = 0;
            let mut v13 = 0;

            loop {
                v13 = current_mask_value1;
                result.isTransitionValid = if (current_mask_value1 | v10) == current_mask_value1 { 1 } else { 0 };
                target_mask_payload = current_mask_value1 | v10 | 1;
                if (current_mask_value1 | v10) == current_mask_value1 {
                    target_mask_payload = current_mask_value1 | v10;
                }

                let original_mask_snapshot1 = current_mask_value1;
                let actual = (*atomic_bitmask).compare_exchange(
                    original_mask_snapshot1,
                    target_mask_payload,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );

                match actual {
                    Ok(val) => {
                        current_mask_value1 = val;
                        break;
                    }
                    Err(val) => {
                        current_mask_value1 = val;
                    }
                }
            }

            let mut v12 = 1;
            if (target_mask_payload & 1) == 0 || (v13 & 1) != 0 {
                v12 = 0;
            }
            result.isFirstInitialization = v12 as u32;
        }
        _ => {
            let v6 = transitionOpcode - 320;
            if v6 >= 0 && v6 < 64 {
                let second_dword_ptr = pStateBitmask.add(1) as *const AtomicI32;
                let mut current_mask_value = (*second_dword_ptr).load(Ordering::SeqCst);

                loop {
                    let is_valid = (current_mask_value & 0x10) != 0 && (((current_mask_value >> 5) & 0x3F) == v6 as u32);
                    result.isTransitionValid = if is_valid { 1 } else { 0 };

                    let original_mask_snapshot = current_mask_value;
                    let exchange_val = current_mask_value
                        ^ (((current_mask_value as u16 ^ (32 * v6) as u16) & 0x7E0) as i32)
                        | 0x10;

                    let actual = (*second_dword_ptr).compare_exchange(
                        original_mask_snapshot,
                        exchange_val,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );

                    match actual {
                        Ok(val) => {
                            current_mask_value = val;
                            break;
                        }
                        Err(val) => {
                            current_mask_value = val;
                        }
                    }
                }

                if result.isTransitionValid == 0 {
                    result.validationError = 0;
                    result.previousState = transitionOpcode as u32;
                    result.previousCounter = 1;
                }
            } else {
                result.validationError = 0;
                result.previousState = transitionOpcode as u32;
                result.previousCounter = 1;
            }
        }
    }

    pOutTransitionResult
}