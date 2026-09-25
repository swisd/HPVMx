use alloc::vec::Vec;
use core::ffi::{c_void, VaList};
use core::ptr::addr_of_mut;
use crate::x4::dxgi::FlushDirtyShaderResources;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{AcquireSRWLockExclusive, GetLastError, GetProcessHeap, HeapAlloc, HeapFree, LocalFree, QueryPerformanceCounter, ReleaseSRWLockExclusive};
use crate::x4::globals::{_guard_check_icall_fptr, pManager, GlobalPtrSppNamespace, GlobalSppTimerCallbackBaseVtbl};
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva30_Len0, ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0, ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31, ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36};
use crate::x4::spp::lock::{SppCustomLockAcquireExclusive, SppCustomLockRelease};
use crate::x4::spp::security::SppVerifyAccessSecurity;
use crate::x4::spp::{SppConvertContextToIdString, SppNormalizeLicensingId, SppPrepareComponentParameters};
use crate::x4::spp::calls::SppGetProtectedEditionString;
use crate::x4::spp::format::SppFormatString;
use crate::x4::spp::pointer::PointerVectorResize;
use crate::x4::spp::query::{SppQueryLicenseAttribute, SppQueryLicensingDatabase};
use crate::x4::spp::string::{SppGetAndDuplicateString, SppGuidFromString, SppSelectAndDuplicateString, SppStringBuilderAppendFormat};
use crate::x4::types::{ShaderManager, GUID, HRESULT, SppTimerCallbackBase, SppStringBuilder, HLOCAL, LARGE_INTEGER, SppCustomLockV2, PointerVector, ISppPropertyBag, PCWSTR, SIZE_T, PTP_TIMER, PSRWLOCK, RTL_SRWLOCK, SppTimerCallback, ISppTimerRegistry, RelocTableHeader, SppCustomLock};

pub unsafe fn SppCancelTimerQueueItem(pTargetGuid: *const GUID) -> HRESULT {
    let mut v2: *mut u16 = core::ptr::null_mut();
    let mut performance_count = LARGE_INTEGER { QuadPart: 0 };
    let mut ppwsz_out_timer_handle: *mut u16 = core::ptr::null_mut();
    let mut v3: usize = 0;
    let mut v4 = 0;

    let v5 = QueryPerformanceCounter(&mut performance_count);
    ProcessReloc_Rva36_Len3(&mut stru_14043A6F0, &mut dword_140469D80);

    let mut hr: HRESULT;

    if v5 == 0 {
        let last_error = GetLastError() as i32;
        hr = last_error;
        if last_error != 0 {
            if last_error > 0 {
                hr = ((last_error as u16) as i32) | 0x80070000;
            }
        } else {
            hr = -2147467259; // E_FAIL
        }
        HandleSubsystemError(hr);
    } else {
        hr = SppVerifyAccessSecurity();
        if hr >= 0 {
            let manager_ptr = *(pManager.padding.as_ptr() as *const usize);
            if manager_ptr != 0 {
                v3 = manager_ptr + 8;
                if manager_ptr == (-8_i64) as usize {
                    // skip acquire lock block
                    v3 = 0; // or flag accordingly
                }
            } else {
                v3 = 8;
            }

            if v3 != 0 && manager_ptr != (-8_i64) as usize {
                SppCustomLockAcquireExclusive(v3 as *mut SppCustomLockV2);
                v4 = 1;
            }

            ProcessReloc_Rva3_Len36(&mut dword_14044A9D8, &mut dword_140465F90);

            let v10 = SppLookupTimerHandleByGuid(pMetadataStore, pTargetGuid as usize, &mut ppwsz_out_timer_handle);
            hr = v10;
            if v10 < 0 {
                HandleSubsystemError(v10);
                v2 = ppwsz_out_timer_handle;
            } else {
                let v11: usize = 0; // placeholder for unbind parameter
                hr = SppUnbindTimerFromNamespace(v11, pTargetGuid as usize);
                ProcessReloc_Rva28_Len0(&mut stru_14044FBE0, &mut dword_1404605C8);
                ProcessReloc_Rva0_Len36(&mut stru_14044B408, &mut dword_140466AD0);

                if hr < 0 {
                    HandleSubsystemError(hr);
                    v2 = ppwsz_out_timer_handle;
                } else {
                    v2 = ppwsz_out_timer_handle;
                    if !ppwsz_out_timer_handle.is_null() {
                        ProcessReloc_Rva0_Len36(&mut stru_140439FE8, &mut dword_140469794);
                        ProcessReloc_Rva2_Len30(&mut stru_140437AB8, &mut dword_140467138);

                        let active_queue_ctx = *(pManager.padding.as_ptr().offset(32) as *const usize);
                        let v9 = SppRemoveTimerFromActiveQueue(active_queue_ctx, v2);
                        hr = v9;
                        if v9 < 0 {
                            HandleSubsystemError(v9);
                        }
                    }
                }
            }
        }
    }

    if GlobalSubsystemIdentifier > 5 {
        let v13 = qword_14045F8A0 & 0x400000000000;
        let reloc_entry = (stru_14044DEC8.entries.as_mut_ptr() as *mut u8).offset(4) as *mut RelocTableHeader;
        ProcessReloc_Rva36_Len3(reloc_entry, &mut dword_1404620A0);

        if v13 != 0 {
            let v14 = qword_14045F8A8;
            ProcessReloc_Rva36_Len0(&mut stru_140443088, &mut dword_140468968);

            if (v14 & 0x400000000000) == qword_14045F8A8 {
                let v24 = sub_14006A8B8(performance_count.QuadPart);
                let timer_handle_param = ppwsz_out_timer_handle;
                ProcessReloc_Rva8_Len36(&mut stru_14044B138, &mut dword_140466730);

                let v20 = pTargetGuid;
                let v21: i64 = 0x1000000;
                ProcessReloc_Rva30_Len0(&mut stru_1404497E8, &mut dword_1404616E0);

                BuildAndWriteEtwTransferPayload_6Fields(
                    0,
                    byte_140453DE9,
                    0,
                    0,
                    &v21,
                    &v20,
                    &(timer_handle_param as *const u16),
                    &v24,
                );
            }
        }
    }

    LogTraceEvent(hr);

    if !v2.is_null() {
        let process_heap = GetProcessHeap();
        // v2 - 2 in C pointer arithmetic for WCHAR means subtracting 2 bytes (or 1 wide char element depending on layout)
        let free_ptr = (v2 as *mut u8).offset(-2) as *mut c_void;
        HeapFree(process_heap, 0, free_ptr);
        LogTraceEvent(0);
    }

    if v3 != 0 {
        ProcessReloc_Rva33_Len3(&mut stru_140435878, &mut dword_140465018);
        if v4 != 0 {
            SppCustomLockRelease(v3 as *mut SppCustomLock);
        }
    }

    hr
}

pub unsafe fn SppInitializeTimerParameters(
    a1: u64,
    a2: *mut c_void,
    a3: u64,
    a4: u64,
) -> HRESULT {
    let mut plugin_vector = PointerVector {
        capacity: 0,
        size: 0,
        elements: core::ptr::null_mut(),
    };

    let mut v17: u128 = 0;

    // a1 is stored in the low 64 bits of v17 (or directly cast)
    v17 = a1 as u128;

    let mut v5 = SppPrepareComponentParameters(
        a1 as i64,
        a2,
        a3 as i64,
        a4 as *mut i64,
        &mut plugin_vector,
    );
    let mut v6 = v5;

    if v5 < 0 {
        HandleSubsystemError(v5 as i32);
    } else {
        let size = plugin_vector.size;
        if size > 0 {
            let elements = plugin_vector.elements;
            let mut v9 = 0;
            while v9 < size {
                let host_ptr = qword_14046B818;
                let element = *elements.offset(v9 as isize);

                if !host_ptr.is_null() && !(*host_ptr).vtbl.is_null() {
                    let initialize_fn = (*(*host_ptr).vtbl).initialize_timer;
                    _guard_check_icall_fptr();

                    v5 = initialize_fn(
                        host_ptr as *mut c_void,
                        &v17,
                        element,
                    );
                    v6 = v5;

                    if v5 == -1073425653 {
                        v6 = 0;
                    } else if v5 < 0 {
                        HandleSubsystemError(v5 as i32);
                        break;
                    }
                }
                v9 += 1;
            }
        }
    }

    LogTraceEvent(v6 as i32);

    PointerVectorResize(&mut plugin_vector, 0);

    let v13 = plugin_vector.elements;
    if !v13.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v13 as *mut c_void);
    }

    v6 as HRESULT
}

pub unsafe fn SppLoadAndRegisterTimers(a1: u64, a2: u64) -> HRESULT {
    let padding_ptr = pManager.padding.as_ptr();
    let v2 = *(padding_ptr.offset(64) as *const usize);

    let mut p_out_property_bag: *mut ISppPropertyBag = core::ptr::null_mut();
    let mut v24: u128 = 0;
    let a2a: [*mut c_void; 2] = [core::ptr::null_mut(); 2];
    let mut v22: u128 = 0;
    let a3: [u64; 2] = [0; 2];
    let mut v26: [u128; 2] = [0; 2];
    let mut v27: [u128; 2] = [0; 2];

    let vptr = *(v2 as *const usize);
    let v5_fn: unsafe extern "system" fn(
        usize,
        *const u16,
        *const u64,
        u64,
        *mut *mut ISppPropertyBag,
    ) -> HRESULT = core::mem::transmute(*(vptr.offset(24) as *const usize));

    _guard_check_icall_fptr();

    let param_name: &[u16] = &[
        b'm' as u16, b's' as u16, b'f' as u16, b't' as u16, b':' as u16,
        b's' as u16, b'p' as u16, b'p' as u16, b'/' as u16,
        b's' as u16, b't' as u16, b'o' as u16, b'c' as u16, b'k' as u16,
        b'o' as u16, b'b' as u16, b'j' as u16, b'e' as u16, b'c' as u16,
        b't' as u16, b's' as u16, b'/' as u16,
        b'p' as u16, b'l' as u16, b'u' as u16, b'g' as u16, b'i' as u16,
        b'n' as u16, b's' as u16, b'/' as u16,
        b'p' as u16, b'a' as u16, b'r' as u16, b'a' as u16, b'm' as u16,
        b's' as u16, b'/' as u16,
        b'n' as u16, b'a' as u16, b'm' as u16, b'e' as u16, b'd' as u16, 0,
    ];

    let mut v6 = v5_fn(
        v2,
        param_name.as_ptr(),
        &qword_1403E9940 as *const u64,
        0,
        &mut p_out_property_bag,
    );
    let mut v7 = v6;

    if v6 < 0 {
        HandleSubsystemError(v6);
    } else {
        v6 = SppResolveAndInjectTimerProperties(a1, a2, p_out_property_bag);
        v7 = v6;
        if v6 < 0 {
            HandleSubsystemError(v6);
        } else {
            let bag_ref = &*p_out_property_bag;
            let bag_vtbl = &*bag_ref.lpVtbl;

            let name1: Vec<u16> = "SppHostParameterUniqueGraceTimerSeed"
                .encode_utf16()
                .chain(core::iter::once(0))
                .collect();
            _guard_check_icall_fptr();
            v6 = (bag_vtbl.GetProperty.unwrap())(
                p_out_property_bag,
                name1.as_ptr(),
                &mut v24 as *mut u128 as *mut c_void,
            ) as HRESULT;
            v7 = v6;
            if v6 < 0 {
                HandleSubsystemError(v6);
            } else {
                let name2: Vec<u16> = "SppHostParameterGraceTimerSuffix"
                    .encode_utf16()
                    .chain(core::iter::once(0))
                    .collect();
                _guard_check_icall_fptr();
                v6 = (bag_vtbl.GetProperty.unwrap())(
                    p_out_property_bag,
                    name2.as_ptr(),
                    &mut v22 as *mut u128 as *mut c_void,
                ) as HRESULT;
                v7 = v6;
                if v6 < 0 {
                    HandleSubsystemError(v6);
                } else {
                    let name3: Vec<u16> = "SppHostParameterUniqueOOTTimerSeed"
                        .encode_utf16()
                        .chain(core::iter::once(0))
                        .collect();
                    _guard_check_icall_fptr();
                    v6 = (bag_vtbl.GetProperty.unwrap())(
                        p_out_property_bag,
                        name3.as_ptr(),
                        v26.as_mut_ptr() as *mut c_void,
                    ) as HRESULT;
                    v7 = v6;
                    if v6 < 0 {
                        HandleSubsystemError(v6);
                    } else {
                        let name4: Vec<u16> = "SppHostParameterOOTTimerSuffix"
                            .encode_utf16()
                            .chain(core::iter::once(0))
                            .collect();
                        _guard_check_icall_fptr();
                        v6 = (bag_vtbl.GetProperty.unwrap())(
                            p_out_property_bag,
                            name4.as_ptr(),
                            v27.as_mut_ptr() as *mut c_void,
                        ) as HRESULT;
                        v7 = v6;
                        if v6 < 0 {
                            HandleSubsystemError(v6);
                        } else {
                            let mut a4: [u64; 2] = [0; 2];
                            a4[0] = &GlobalSppTimerCallbackVtbl as *const _ as u64;
                            a4[1] = 0x600000001;

                            v6 = SppInitializeTimerParameters(a1, a2a[0], a3[0], *a4.as_ptr());
                            v7 = v6;
                            if v6 < 0 {
                                HandleSubsystemError(v6);
                            }
                        }
                    }
                }
            }
        }
    }

    LogTraceEvent(v7);

    if !p_out_property_bag.is_null() {
        let bag_ref = &*p_out_property_bag;
        let bag_vtbl = &*bag_ref.lpVtbl;
        _guard_check_icall_fptr();
        (bag_vtbl.Release.unwrap())(p_out_property_bag);
    }

    v7
}

pub unsafe fn SppLookupTimerHandleByGuid(
    pMetadataStore: usize,
    pTargetGuidContext: usize,
    ppwszOutTimerHandle: *mut *mut u16,
) -> HRESULT {
    let mut v3: *mut u16 = core::ptr::null_mut();
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut v21: usize = 0;
    let mut v26: i32 = 0;
    let mut ppsz_destination_string: *mut u16 = core::ptr::null_mut();
    let mut v24: u128 = 0;
    let mut src: [*mut c_void; 2] = [core::ptr::null_mut(); 2];

    let mut v6 = SppConvertContextToIdString(pTargetGuidContext as i64, &mut h_mem as *mut *mut u16);
    let v7 = h_mem;
    let mut v8 = v6;

    if v6 < 0 {
        HandleSubsystemError(v6);
    } else {
        // Retrieve function pointer from vtable at offset 440 (index 55 if pointers are 8 bytes)
        let vtable_ptr = *(pMetadataStore as *const usize);
        let lookup_fn: unsafe extern "system" fn(
            usize,
            i32,
            *mut usize,
            *const u16,
            *const u16,
            *const u16,
            HLOCAL,
            u64,
            u64,
        ) -> HRESULT = core::mem::transmute(*(vtable_ptr.offset(440 / 8) as *const usize));

        _guard_check_icall_fptr();

        let meta_info_type: Vec<u16> = "metaInfoType".encode_utf16().chain(core::iter::once(0)).collect();
        let meta_info_store_token: Vec<u16> = "metaInfoTypeStoreToken".encode_utf16().chain(core::iter::once(0)).collect();
        let file_id: Vec<u16> = "fileId".encode_utf16().chain(core::iter::once(0)).collect();

        let hr = lookup_fn(
            pMetadataStore,
            1,
            &mut v21,
            meta_info_type.as_ptr(),
            meta_info_store_token.as_ptr(),
            file_id.as_ptr(),
            v7,
            0,
            0,
        );

        if hr >= 0 {
            let obj_ptr = v21 as *const *const usize;
            let obj_vtable = *obj_ptr;
            let get_count_fn: unsafe extern "system" fn(usize, *mut i32) -> HRESULT =
                core::mem::transmute(*(obj_vtable.offset(24 / 8) as *const usize));

            _guard_check_icall_fptr();
            v6 = get_count_fn(v21, &mut v26);
            v8 = v6;

            if v6 < 0 {
                HandleSubsystemError(v6);
            } else if v26 != 1 {
                v8 = -2147418113; // E_UNEXPECTED
                HandleSubsystemError(v8);
            } else {
                let get_data_fn: unsafe extern "system" fn(usize, u64, *mut u128) -> HRESULT =
                    core::mem::transmute(*(obj_vtable.offset(32 / 8) as *const usize));

                _guard_check_icall_fptr();
                v6 = get_data_fn(v21, 0, &mut v24);
                v8 = v6;

                if v6 < 0 {
                    HandleSubsystemError(v6);
                } else {
                    let v15 = SppGetAndDuplicateString(src[0] as *const u16, &mut ppsz_destination_string);
                    v8 = v15;
                    if v15 < 0 {
                        HandleSubsystemError(v15);
                        v3 = ppsz_destination_string;
                    } else {
                        v3 = ppsz_destination_string;
                    }
                }
            }
        } else {
            v8 = hr;
            HandleSubsystemError(v8);
        }
    }

    if v8 >= 0 {
        let v16 = v3;
        v3 = core::ptr::null_mut();
        *ppwszOutTimerHandle = v16;
    }

    LogTraceEvent(v8);

    if v21 != 0 {
        let obj_ptr = v21 as *const *const usize;
        let obj_vtable = *obj_ptr;
        let release_fn: unsafe extern "system" fn(usize) -> u32 =
            core::mem::transmute(*(obj_vtable.offset(16 / 8) as *const usize));

        _guard_check_icall_fptr();
        release_fn(v21);
        v21 = 0;
    }

    if !v7.is_null() {
        LocalFree(v7);
    }

    if !v3.is_null() {
        let process_heap = GetProcessHeap();
        let free_ptr = (v3 as *mut u8).offset(-2) as *mut c_void;
        HeapFree(process_heap, 0, free_ptr);
        LogTraceEvent(0);
    }

    v8
}

pub unsafe fn SppRemoveTimerFromActiveQueue(
    pRegistryContainer: usize,
    pwszTimerContextName: *const u16,
) -> HRESULT {
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut v15: u128 = 0;
    let mut v4: *mut u8 = core::ptr::null_mut();

    let raw_context_ptr = core::ptr::addr_of!(pRawContext) as usize;
    let v5 = SppConvertContextToIdString(raw_context_ptr, &mut h_mem);
    let v6 = h_mem;
    let mut v7 = v5;

    if v5 < 0 {
        HandleSubsystemError(v5);
        return v7;
    }

    h_mem = core::ptr::null_mut();

    let fmt_str: Vec<u16> = "%s:%s\0".encode_utf16().collect();
    let arg1_str: Vec<u16> = "StoreTokenPID2\0".encode_utf16().collect();

    let v8 = SppFormatString(
        &mut h_mem,
        fmt_str.as_ptr(),
        arg1_str.as_ptr(),
        pwszTimerContextName,
        v6,
        0,
    );
    v7 = v8;

    let mut v9: *mut u8 = core::ptr::null_mut();
    if v8 >= 0 {
        v4 = h_mem as *mut u8;
    } else {
        HandleSubsystemError(v8);
        v9 = h_mem as *mut u8;
    }

    LogTraceEvent(v7);

    if !v9.is_null() {
        let process_heap = GetProcessHeap();
        let free_ptr = v9.offset(-4) as *mut c_void;
        HeapFree(process_heap, 0, free_ptr);
        LogTraceEvent(0);
    }

    if v7 >= 0 {
        let vtable_ptr = *(pRegistryContainer as *const usize);
        let remove_fn: unsafe extern "system" fn(
            usize,
            *mut u128,
            *mut u8,
        ) -> HRESULT = core::mem::transmute(*(vtable_ptr.offset(152 / 8) as *const usize));

        _guard_check_icall_fptr();
        let v5_result = remove_fn(pRegistryContainer, &mut v15, v4);
        v7 = v5_result;
        if v5_result < 0 {
            HandleSubsystemError(v5_result);
        }
    } else {
        HandleSubsystemError(v7);
    }

    LogTraceEvent(v7);

    if !v4.is_null() {
        let process_heap = GetProcessHeap();
        let free_ptr = v4.offset(-4) as *mut c_void;
        HeapFree(process_heap, 0, free_ptr);
        LogTraceEvent(0);
    }

    if !v6.is_null() {
        LocalFree(v6);
    }

    v7
}

pub unsafe fn SppResolveAndInjectTimerProperties(
    app_id: u64,
    sku_id: u64,
    p_out_property_bag: *mut ISppPropertyBag,
) -> HRESULT {
    let mut v3 = sku_id as *const u16;
    let mut v4: *mut u16 = core::ptr::null_mut();
    let mut v5: HLOCAL = core::ptr::null_mut();
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut pp_out_value: HLOCAL = core::ptr::null_mut();
    let mut v39: usize = 0;
    let mut v38: usize = 0;
    let mut ppwsz_out_string: *mut u16 = core::ptr::null_mut();
    let mut v42: u128 = 0;
    let mut v43: u128 = 0;
    let mut v49: u128 = 0;
    let v50: u128 = 0;

    let v52: [*const u16; 5] = [
        "SppHostParameterUniqueGraceTimerSeed\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
        "OOTTimerUniqueness\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
        "SppHostParameterUniqueOOTTimerSeed\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
        "ValidityTimerUniqueness\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
        "SppHostParameterUniqueValidityTimerSeed\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
    ];

    let v48: [*const u16; 3] = [
        "SppHostParameterGraceTimerSuffix\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
        "msft:spp/notifications/common/getoottimersuffix\0".encode_utf16().collect::<Vec<u16>>().as_ptr(), // Note: array index alignment mapping
        "SppHostParameterOOTTimerSuffix\0".encode_utf16().collect::<Vec<u16>>().as_ptr(),
    ];

    let pkey_binding_edition_id: Vec<u16> = "SppPkeyBindingEditionId\0".encode_utf16().collect();
    let mut hresult = SppGetProtectedEditionString(
        sku_id as i64,
        pkey_binding_edition_id.as_ptr(),
        &mut h_mem,
    );
    let mut hr = hresult;

    if hr >= 0 {
        let mut v10: u32 = 0;
        let mut v11_idx = 1; // corresponds to v11 - 1 offset logic in loop

        loop {
            if !v5.is_null() {
                LocalFree(v5);
                pp_out_value = core::ptr::null_mut();
                v5 = core::ptr::null_mut();
            }

            let attr_name = v52[v11_idx - 1];
            hr = SppQueryLicenseAttribute(0, v3 as u64 as i64, attr_name, &mut pp_out_value);

            if (hr as u32).wrapping_add(0x80000000) < 0x80000000 && hr != -1073418222 {
                HandleSubsystemError(hr);
                v5 = pp_out_value;
                break;
            }

            if !v4.is_null() {
                LocalFree(v4 as HLOCAL);
                ppwsz_out_string = core::ptr::null_mut();
                v4 = core::ptr::null_mut();
            }

            v5 = pp_out_value;
            let result = SppSelectAndDuplicateString(
                v3 as PCWSTR,
                h_mem as *const u16,
                v5 as *const u16,
                &mut ppwsz_out_string,
            );
            hr = result;

            if result < 0 {
                HandleSubsystemError(result);
                v4 = ppwsz_out_string;
                break;
            }

            v4 = ppwsz_out_string;
            let bag_ref = &*p_out_property_bag;
            let bag_vtbl = &*bag_ref.lpVtbl;

            v42 = v52[v11_idx - 1] as u128 | ((2u128) << 64);
            v43 = v4 as u128;

            _guard_check_icall_fptr();
            hresult = (bag_vtbl.SetProperty.unwrap())(p_out_property_bag, &v42, 1);
            hr = hresult;

            if hresult < 0 {
                break;
            }

            v10 += 1;
            v11_idx += 2;
            if v10 >= 3 {
                let padding_ptr = pManager.padding.as_ptr();
                let v16 = *(padding_ptr.offset(64) as *const usize);
                let vptr = *(v16 as *const usize);
                let get_named_params_fn: unsafe extern "system" fn(
                    usize,
                    *const u16,
                    *const u64,
                    u64,
                    *mut usize,
                ) -> HRESULT = core::mem::transmute(*(vptr.offset(24) as *const usize));

                _guard_check_icall_fptr();
                let named_param_str: Vec<u16> = "lsrv:spp/stockobjects/plugins/params/named\0".encode_utf16().collect();
                hresult = get_named_params_fn(
                    v16,
                    named_param_str.as_ptr(),
                    &qword_1403E9940 as *const u64,
                    0,
                    &mut v39,
                );
                hr = hresult;

                if hresult >= 0 {
                    let v18 = v39;
                    let app_id_str: Vec<u16> = "SppNotificationAppId\0".encode_utf16().collect();
                    v42 = app_id_str.as_ptr() as u128 | ((2u128) << 64);
                    v43 = app_id as u128;

                    let obj_vtable = *(v39 as *const *const usize);
                    let set_prop_fn: unsafe extern "system" fn(usize, *const u128, u32) -> HRESULT =
                        core::mem::transmute(*(obj_vtable.offset(72 / 8) as *const usize));

                    _guard_check_icall_fptr();
                    hresult = set_prop_fn(v18, &v42, 0);
                    hr = hresult;

                    if hresult >= 0 {
                        let v20 = v39;
                        let sku_id_str: Vec<u16> = "SppNotificationSkuId\0".encode_utf16().collect();
                        v42 = sku_id_str.as_ptr() as u128 | ((2u128) << 64);
                        v43 = sku_id as u128;

                        _guard_check_icall_fptr();
                        hresult = set_prop_fn(v20, &v42, 0);
                        hr = hresult;

                        if hresult >= 0 {
                            let mut loop_count = 0;
                            let mut v22_idx = 1;

                            loop {
                                let v23 = v38;
                                if v38 != 0 {
                                    let sub_vtable = *(v38 as *const *const usize);
                                    let release_fn: unsafe extern "system" fn(usize) -> u32 =
                                        core::mem::transmute(*(sub_vtable.offset(16 / 8) as *const usize));
                                    _guard_check_icall_fptr();
                                    release_fn(v23);
                                    v38 = 0;
                                }

                                let registry_obj = qword_14046B840;
                                let reg_vtable = *(registry_obj as *const *const usize);
                                let query_notif_fn: unsafe extern "system" fn(
                                    usize,
                                    *const u16,
                                    usize,
                                    u32,
                                    *mut usize,
                                ) -> HRESULT = core::mem::transmute(*(reg_vtable.offset(40 / 8) as *const usize));

                                _guard_check_icall_fptr();
                                hresult = query_notif_fn(
                                    registry_obj,
                                    v48[v22_idx - 1],
                                    v39,
                                    1,
                                    &mut v38,
                                );
                                hr = hresult;

                                if hresult < 0 {
                                    break;
                                }

                                let v27 = v38;
                                if v27 != 0 {
                                    let notif_vtable = *(v27 as *const *const usize);
                                    let get_prop_fn: unsafe extern "system" fn(
                                        usize,
                                        *const u16,
                                        *mut u128,
                                    ) -> HRESULT = core::mem::transmute(*(notif_vtable.offset(48 / 8) as *const usize));

                                    let timer_suffix_str: Vec<u16> = "SppNotificationTimerSuffix\0".encode_utf16().collect();
                                    _guard_check_icall_fptr();
                                    hresult = get_prop_fn(v27, timer_suffix_str.as_ptr(), &mut v49);
                                    hr = hresult;

                                    if hresult != -2147024894 {
                                        if hresult < 0 {
                                            break;
                                        }
                                        if ((v49 >> 64) & 0xFFFFFFFF) != 2 {
                                            hr = -2147418113;
                                            HandleSubsystemError(hr);
                                            break;
                                        }
                                    }
                                }

                                let final_val_ptr = if v38 != 0 {
                                    v50 as *const u16
                                } else {
                                    core::ptr::addr_of!(pRawBuffer)
                                };

                                let bag_ref2 = &*p_out_property_bag;
                                let bag_vtbl2 = &*bag_ref2.lpVtbl;
                                v42 = v48[v22_idx] as u128 | ((2u128) << 64); // Note: index mapping matches original register pattern offset
                                v43 = final_val_ptr as u128;

                                _guard_check_icall_fptr();
                                hresult = (bag_vtbl2.SetProperty.unwrap())(p_out_property_bag, &v42, 1);
                                hr = hresult;

                                if hresult < 0 {
                                    break;
                                }

                                v22_idx += 2;
                                loop_count += 1;
                                if loop_count >= 2 {
                                    break;
                                }
                            }
                        }
                    }
                }
                break;
            }

            v3 = sku_id as *const u16;
        }
    }

    if hr < 0 {
        HandleSubsystemError(hr);
    }

    LogTraceEvent(hr);

    if v38 != 0 {
        let sub_vtable = *(v38 as *const *const usize);
        let release_fn: unsafe extern "system" fn(usize) -> u32 =
            core::mem::transmute(*(sub_vtable.offset(16 / 8) as *const usize));
        _guard_check_icall_fptr();
        release_fn(v38);
        v38 = 0;
    }

    if v39 != 0 {
        let sub_vtable = *(v39 as *const *const usize);
        let release_fn: unsafe extern "system" fn(usize) -> u32 =
            core::mem::transmute(*(sub_vtable.offset(16 / 8) as *const usize));
        _guard_check_icall_fptr();
        release_fn(v39);
        v39 = 0;
    }

    if !v4.is_null() {
        LocalFree(v4 as HLOCAL);
    }
    if !v5.is_null() {
        LocalFree(v5);
    }
    if !h_mem.is_null() {
        LocalFree(h_mem);
    }

    hr
}

pub unsafe fn SppTeardownActiveTimer(pwsz_guid_string: *const u16) -> HRESULT {
    let mut v1: HLOCAL = core::ptr::null_mut();
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut v12: u32 = 0;
    let mut p_out_guid = GUID { Data1: 0, Data2: 0, Data3: 0, Data4: [0;8] };

    let mut v4: HRESULT;
    let v6: usize = 0; // corresponds to uninitialized/default register container parameter context in decompilation

    if pwsz_guid_string.is_null() {
        v4 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v4);
    } else {
        // Calculate string length
        let mut len = 0;
        while *pwsz_guid_string.offset(len) != 0 {
            len += 1;
        }

        let mut guid_ptr = pwsz_guid_string;
        if len == 38 {
            if *guid_ptr != 123 || *guid_ptr.offset(37) != 125 {
                v4 = -2147024809;
                HandleSubsystemError(v4);
                LogTraceEvent(v4);
                if v4 == -2147024809 {
                    v4 = -1073422330;
                }
                HandleSubsystemError(v4);
                LogTraceEvent(v4);
                return v4;
            }
            guid_ptr = guid_ptr.offset(1);
        } else if len != 36 {
            v4 = -2147024809;
            HandleSubsystemError(v4);
            LogTraceEvent(v4);
            if v4 == -2147024809 {
                v4 = -1073422330;
            }
            HandleSubsystemError(v4);
            LogTraceEvent(v4);
            return v4;
        }

        let v3 = SppGuidFromString(guid_ptr, &mut p_out_guid);
        v4 = v3;
        if v3 < 0 {
            HandleSubsystemError(v3);
        }
    }

    LogTraceEvent(v4);

    if v4 == -2147024809 {
        v4 = -1073422330;
    } else if v4 >= 0 {
        let v8 = SppQueryLicensingDatabase(
            v6 as i64,
            1,
            &p_out_guid,
            3,
            &mut v12,
            &mut h_mem,
        );
        v4 = v8;
        if v8 < 0 {
            HandleSubsystemError(v8);
            v1 = h_mem;
        } else {
            v1 = h_mem;
            let v9 = SppTeardownSysprepTimers(h_mem, v12);
            v4 = v9;
            if v9 < 0 {
                HandleSubsystemError(v9);
            }
        }
        LogTraceEvent(v4);
        if !v1.is_null() {
            LocalFree(v1);
        }
        return v4;
    }

    HandleSubsystemError(v4);
    LogTraceEvent(v4);

    if v4 < 0 {
        HandleSubsystemError(v4);
    } else {
        let v8 = SppQueryLicensingDatabase(
            v6 as i64,
            1,
            &p_out_guid,
            3,
            &mut v12,
            &mut h_mem,
        );
        v4 = v8;
        if v8 < 0 {
            HandleSubsystemError(v8);
            v1 = h_mem;
        } else {
            v1 = h_mem;
            let v9 = SppTeardownSysprepTimers(h_mem, v12);
            v4 = v9;
            if v9 < 0 {
                HandleSubsystemError(v9);
            }
        }
    }

    LogTraceEvent(v4);
    if !v1.is_null() {
        LocalFree(v1);
    }

    v4
}

pub unsafe fn SppTeardownSysprepTimers(
    p_raw_context_array: *mut u64,
    cch_element_count: u32,
) -> HRESULT {
    let mut v2: *mut u8 = core::ptr::null_mut();
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut v4: HLOCAL = core::ptr::null_mut();
    let mut v5: u32 = 0;
    let mut v6: HRESULT = 0;
    let mut pp_out_guid_array: HLOCAL = core::ptr::null_mut();

    if p_raw_context_array.is_null() || cch_element_count == 0 {
        LogTraceEvent(v6);
        return v6;
    }

    let v8 = match (cch_element_count as usize).checked_mul(16) {
        Some(val) => val,
        None => usize::MAX,
    };

    let process_heap = GetProcessHeap();
    v2 = HeapAlloc(process_heap, 0, v8 as SIZE_T) as *mut u8;
    if v2.is_null() {
        v6 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v6);
        LogTraceEvent(v6);
        return v6;
    }

    let mut v14: u32 = 0;
    let mut v24 = p_raw_context_array;

    if cch_element_count > 0 {
        loop {
            if !v4.is_null() {
                LocalFree(v4);
                v4 = core::ptr::null_mut();
            }

            let context_ptr = v24 as usize;
            let v15 = SppConvertContextToIdString(context_ptr as i64, &mut pp_out_guid_array);
            v6 = v15;
            if v15 < 0 {
                HandleSubsystemError(v15);
                v4 = pp_out_guid_array;
                break;
            }

            if !h_mem.is_null() {
                LocalFree(h_mem);
                h_mem = core::ptr::null_mut();
            }

            let namespace_ptr = GlobalPtrSppNamespace;
            v4 = pp_out_guid_array;
            let ns_ref = &*namespace_ptr;
            let ns_vtbl = &*ns_ref.vtbl;

            let v25 = if !pp_out_guid_array.is_null() {
                pp_out_guid_array
            } else {
                core::ptr::null_mut()
            };

            let sysprep_action: Vec<u16> = "sysprepAction\0".encode_utf16().collect();
            _guard_check_icall_fptr();
            let v17 = (ns_vtbl.get_namespace_provider)(
                namespace_ptr,
                1,
                v25,
                sysprep_action.as_ptr(),
                &mut h_mem,
            );

            if v17 != -1073418222 {
                v6 = v17;
                if v17 < 0 {
                    HandleSubsystemError(v17);
                    break;
                }
                let dest_offset = (v5 as usize) * 16;
                let src_ptr = v24 as *const u8;
                core::ptr::copy_nonoverlapping(src_ptr, v2.offset(dest_offset as isize), 16);
                v5 += 1;
            }

            v24 = v24.offset(2);
            v14 += 1;
            if v14 >= cch_element_count {
                break;
            }
        }
    }

    if v6 >= 0 && v5 > 0 {
        let mut v19 = 0;
        loop {
            pp_out_guid_array = core::ptr::null_mut();
            let mut out_elements_count: u32 = 0;
            let guid_slice_ptr = v2.offset((v19 * 16) as isize) as *const c_void;

            let licensing_database = SppQueryLicensingDatabase(
                0,
                3,
                guid_slice_ptr,
                2,
                &mut out_elements_count,
                &mut pp_out_guid_array,
            );
            let v21 = pp_out_guid_array;
            v6 = licensing_database;

            if licensing_database == -1073418222 {
                v6 = 0;
            } else if licensing_database >= 0 {
                let mut v22 = 0;
                if out_elements_count > 0 {
                    let mut success = true;
                    loop {
                        let v23 = if !v21.is_null() {
                            v21 as *mut u8
                        } else {
                            core::ptr::null_mut()
                        };
                        let item_ptr = v23.offset((v22 * 16) as isize) as *const GUID;
                        let cancel_hr = SppCancelTimerQueueItem(item_ptr);
                        v6 = cancel_hr;
                        if cancel_hr < 0 {
                            success = false;
                            break;
                        }
                        v22 += 1;
                        if v22 >= out_elements_count {
                            break;
                        }
                    }
                    if !success {
                        HandleSubsystemError(v6);
                    }
                }
            } else {
                HandleSubsystemError(licensing_database);
            }

            LogTraceEvent(v6);
            if !v21.is_null() {
                LocalFree(v21);
            }

            if v6 < 0 {
                break;
            }

            v19 += 1;
            if v19 >= v5 {
                break;
            }
        }
    }

    LogTraceEvent(v6);

    if !h_mem.is_null() {
        LocalFree(h_mem);
    }
    if !v4.is_null() {
        LocalFree(v4);
    }
    if !v2.is_null() {
        let heap = GetProcessHeap();
        HeapFree(heap, 0, v2 as *mut c_void);
    }

    v6
}

pub unsafe extern "system" fn SppTelemetryTimerCallback(
    _instance: PTP_CALLBACK_INSTANCE,
    context: *mut ShaderManager,
    _timer: PTP_TIMER,
) {
    if context.is_null() {
        return;
    }

    let ctx = &mut *context;
    if ctx.status_flag != 0 {
        let p_lock = ctx.lock as PSRWLOCK;
        AcquireSRWLockExclusive(p_lock);

        if ctx.status_flag != 0 {
            FlushDirtyShaderResources(context);
            // Equivalent to LOBYTE(Context->unk_config_flags) = 0
            ctx.unk_config_flags &= !0xFF;
        }

        if !p_lock.is_null() {
            ReleaseSRWLockExclusive(p_lock);
        }
    }
}

pub unsafe fn SppTimerCallbackBaseDefaultPlaceholder() -> HRESULT {
    // E_NOTIMPL (0x80004001)
    0x80004001_u32 as i32
}

pub unsafe fn SppTimerCallbackBaseSerialize(
    this: *mut SppTimerCallbackBase,
    p_str_builder: *mut SppStringBuilder,
) -> HRESULT {
    // *((unsigned int *)this + 3) and *((unsigned int *)this + 4) correspond to u32 indices 3 and 4
    let field3 = *(this as *const u32).offset(3);
    let field4 = *(this as *const u32).offset(4);

    let format_str: Vec<u16> = " %d 0x%08X\0".encode_utf16().collect();

    let appended = SppStringBuilderAppendFormat(
        p_str_builder,
        format_str.as_ptr(),
        field3,
        field4
    );
    let hr = appended;

    if appended < 0 {
        HandleSubsystemError(appended);
    }

    LogTraceEvent(hr);
    hr
}

pub unsafe fn SppTimerCallbackDestructor(
    ptr: *mut c_void,
    deleting_flags: u8,
) -> *mut c_void {
    if !ptr.is_null() {
        // Set vtable pointer: *(_QWORD *)ptr = &GlobalSppTimerCallbackBaseVtbl;
        *(ptr as *mut *const c_void) = GlobalSppTimerCallbackBaseVtbl;

        // If deleting flag is set (deletingFlags & 1 != 0), free the heap memory
        if (deleting_flags & 1) != 0 {
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, ptr);
        }
    }

    ptr
}

pub unsafe fn SppTimerCallbackEvaluateState(
    timer: *mut SppTimerCallback,
    h_timer_provider: *mut c_void,
) -> HRESULT {
    let mut v4: HRESULT;
    let mut v9: [u32; 4] = [0; 4];
    let mut v10: u128 = 0;

    if h_timer_provider.is_null() || timer.is_null() {
        v4 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v4);
    } else {
        let t = &mut *timer;
        if t.initializationState != 0 {
            v4 = -2147418113; // E_UNEXPECTED
            HandleSubsystemError(v4);
        } else {
            // Vtable pointer is at offset 0 of hTimerProvider
            let vtable_ptr = *(h_timer_provider as *const *const usize);
            // Offset 48 corresponds to index 6 (48 / 8)
            let eval_fn: unsafe extern "system" fn(*mut c_void, *mut [u32; 8]) -> HRESULT =
                core::mem::transmute(*vtable_ptr.offset(6));

            _guard_check_icall_fptr();
            // Pass a combined 32-byte buffer representing v9 and v10 consecutively on the stack
            let mut stack_buf: [u32; 8] = [0; 8];
            let v7 = eval_fn(h_timer_provider, &mut stack_buf as *mut [u32; 8]);
            v4 = v7;

            if v7 < 0 {
                HandleSubsystemError(v7);
            } else {
                // Copy back values from stack buffer into v9 and v10 representations
                core::ptr::copy_nonoverlapping(stack_buf.as_ptr() as *const u8, v9.as_mut_ptr() as *mut u8, 16);
                core::ptr::copy_nonoverlapping(stack_buf.as_ptr().offset(4) as *const u8, &mut v10 as *mut u128 as *mut u8, 16);

                // DWORD2(v9) corresponds to index 2 of the u32 array for v9
                if v9[2] == 1 {
                    t.timeoutDuration = v10 as u32;
                    t.initializationState = 1;
                    t.isActive = 1;
                } else {
                    v4 = -2147024883; // E_NOT_SUFFICIENT_BUFFER / custom error code
                    HandleSubsystemError(v4);
                }
            }
        }
    }

    LogTraceEvent(v4);
    v4
}

pub unsafe extern "system" fn SppTimerCallbackInitialize(
    timer: *mut SppTimerCallback,
    p_timer_registry: *mut ISppTimerRegistry,
) -> HRESULT {
    let mut hr: HRESULT = 0;
    let mut v9: [u32; 4] = [0; 4];
    let mut v10: [u32; 4] = [0; 4];

    if p_timer_registry.is_null() || timer.is_null() {
        hr = -2147024809; // E_INVALIDARG
        HandleSubsystemError(hr);
    } else {
        let t = &mut *timer;
        if t.initializationState != 1 {
            hr = -2147418113; // E_UNEXPECTED
            HandleSubsystemError(hr);
        } else {
            // LODWORD(v10) = timer->timeoutDuration;
            v10[0] = t.timeoutDuration as u32;

            // DWORD2(v9) = 1; (index 2 of v9 u32 array)
            v9[2] = 1;

            let registry_ref = &*p_timer_registry;
            let registry_vtbl = &*registry_ref.lpVtbl;

            _guard_check_icall_fptr();
            // Pass v9 as the 128-bit structure pointer matching the layout
            let v7 = (registry_vtbl.RegisterTimerHook.unwrap())(
                p_timer_registry,
                v9.as_ptr() as *mut c_void,
            );
            hr = v7;

            if v7 < 0 {
                HandleSubsystemError(v7);
            }
        }
    }

    LogTraceEvent(hr);
    hr
}

pub unsafe fn SppTimerCallbackSerialize(
    a1: *const u32,
    a2: *mut SppStringBuilder,
) -> u64 {
    if a1.is_null() || a2.is_null() {
        return -2147024809_i32 as u32 as u64;
    }

    let val3 = *a1.offset(3);
    let val4 = *a1.offset(4);
    let fmt1: Vec<u16> = " %d 0x%08X\0".encode_utf16().collect();

    let appended = SppStringBuilderAppendFormat(a2, fmt1.as_ptr(), val3, val4);
    let mut v5 = appended;

    if appended < 0 {
        HandleSubsystemError(appended);
    }

    LogTraceEvent(v5);

    if v5 >= 0 {
        let val8 = *a1.offset(8);
        let val9 = *a1.offset(9);
        let fmt2: Vec<u16> = " %d %d\0".encode_utf16().collect();

        let v7 = SppStringBuilderAppendFormat(a2, fmt2.as_ptr(), val8, val9);
        v5 = v7;

        if v7 < 0 {
            HandleSubsystemError(v7);
        }
    } else {
        HandleSubsystemError(v5);
    }

    LogTraceEvent(v5);
    v5 as u32 as u64
}

pub unsafe fn SppTimerCallbackStop(a1: usize, a2: usize) -> u64 {
    let mut v3: HRESULT;

    if a2 != 0 {
        v3 = sub_140111948(a1 as *mut i64);
        ProcessReloc_Rva28_Len0(&mut stru_14043AA58, &dword_140460070);
        if v3 >= 0 {
            // *(_DWORD *)(a1 + 20) = 0; (Offset 20 = index 5 for u32)
            *(a1 as *mut u32).offset(5) = 0;
        } else {
            ProcessReloc_Rva33_Len3(&mut stru_140451F18, &dword_140463C38);
            HandleSubsystemError(v3);
            ProcessReloc_Rva36_Len3(&mut stru_14043E620, &dword_140463324);
        }
    } else {
        ProcessReloc_Rva8_Len36(&mut stru_140452128, &dword_140463F1C);
        v3 = -2147024809; // E_INVALIDARG
        ProcessReloc_Rva2_Len30(&mut stru_14045274C, &dword_140464510);
        HandleSubsystemError(-2147024809);
        ProcessReloc_Rva0_Len36(&mut stru_14044F4C8, &dword_14046A76C);
    }

    ProcessReloc_Rva3_Len36(
        core::ptr::addr_of_mut!(dword_14044B388) as *mut RelocTableHeader,
        &dword_140466990,
    );

    // *(_DWORD *)(a1 + 16) corresponds to index 4 for u32
    let compare_val = *(a1 as *const u32).offset(4);
    if v3 as u32 == compare_val {
        v3 = 0;
        ProcessReloc_Rva30_Len0(&mut stru_140439994, &dword_140469034);
    } else {
        ProcessReloc_Rva30_Len0(&mut stru_140448178, &dword_1404631A4);
        if v3 >= 0 {
            v3 = -1073422333;
        }
    }

    LogTraceEvent(v3);

    if v3 < 0 {
        ProcessReloc_Rva3_Len36(
            core::ptr::addr_of_mut!(dword_14043F978) as *mut RelocTableHeader,
            &dword_140464B00,
        );
        HandleSubsystemError(v3);
        ProcessReloc_Rva36_Len0(&mut stru_140450DF8, &dword_14046356C);
    }

    LogTraceEvent(v3);
    ProcessReloc_Rva3_Len31(&mut stru_140444B28, &dword_140463724);

    v3 as u32 as u64
}

pub unsafe fn SppUnbindTimerFromNamespace(
    _unused_param: usize,
    p_target_guid_context: usize,
) -> HRESULT {
    let mut h_mem: HLOCAL = core::ptr::null_mut();

    ProcessReloc_Rva3_Len36(
        core::ptr::addr_of_mut!(stru_1404350B8) as *mut RelocTableHeader,
        &dword_140464898,
    );

    let mut v3 = SppNormalizeLicensingId(p_target_guid_context as i64, &mut h_mem);

    ProcessReloc_Rva8_Len36(&mut stru_14044C8A0, &dword_140468340);
    ProcessReloc_Rva30_Len0(&mut stru_140446978, &dword_140461578);

    let v4 = h_mem;

    if v3 >= 0 {
        let v5 = pMetadataStore as usize;
        let v6 = *(pMetadataStore as *const usize);

        ProcessReloc_Rva0_Len36(&mut stru_14044AAF8, &dword_1404660B8);
        ProcessReloc_Rva30_Len0(&mut stru_140444B50, &dword_140461950);
        ProcessReloc_Rva3_Len36(
            core::ptr::addr_of_mut!(dword_140443800) as *mut RelocTableHeader,
            &dword_140469260,
        );

        _guard_check_icall_fptr();
        // Index 5 corresponds to offset 40 (40 / 8 = 5) in the vtable
        let func_ptr: unsafe extern "system" fn(usize, HLOCAL) -> HRESULT =
            core::mem::transmute(*(v6 as *const *const c_void).offset(5));

        v3 = func_ptr(v5, v4);

        ProcessReloc_Rva28_Len0(
            core::ptr::addr_of_mut!(stru_140441C08) as *mut RelocTableHeader,
            &dword_1404605F8,
        );
        ProcessReloc_Rva36_Len0(
            core::ptr::addr_of_mut!(stru_140443268) as *mut RelocTableHeader,
            &dword_140468BA0,
        );

        if v3 < 0 {
            HandleSubsystemError(v3);
        }
    } else {
        ProcessReloc_Rva36_Len3(&mut stru_140450088, &dword_140462484);
        HandleSubsystemError(v3);
        ProcessReloc_Rva2_Len30(&mut stru_1404472B0, &dword_140462280);
    }

    LogTraceEvent(v3);
    ProcessReloc_Rva33_Len3(&mut stru_14045252C, &dword_140464300);

    if !v4.is_null() {
        LocalFree(v4);
        ProcessReloc_Rva3_Len31(&mut stru_140451F18, &dword_140463D14);
    }

    v3
}