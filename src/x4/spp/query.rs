use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{memcpy, RegOpenKeyExW, RegCloseKey, RegQueryValueExW, GetProcessHeap, HeapAlloc, LocalAlloc, LocalFree, HeapFree, GetLastError, RtlQueryPackageClaims, _wcsicmp};
use crate::x4::globals::{_guard_check_icall_fptr, E_OUTOFMEMORY, E_INVALIDARG, GlobalPtrSppNamespace};
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva30_Len0, ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0,
                       ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31, ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36, ProcessPeloc_Rva31_Len31};
use crate::x4::spp::calls::SppGetComponentInterface;
use crate::x4::spp::{SppConvertContextToIdString, SppCreateComponentInstance, SppHashAndSerializeLicensingId, SppNormalizeLicensingId, SppValidateLicenseComponentState};
use crate::x4::spp::string::{SppDuplicateStringBounded, SppDuplicateStringLocal, SppGetStringByteLengthSafe, SppGuidFromString, SppMarshalRegistryStringToContext};
use crate::x4::spp::telemetry::{SppEmitEtwEvent, SppGetRuntimeLoggingOverride, SppNtStatusToHresult};
use crate::x4::types::{SppPolicyVariant, BOOL, HANDLE, HRESULT, PWSTR, HKEY, LPCWSTR, DWORD, BYTE, HLOCAL, PCWSTR, LPWSTR, SIZE_T, LSTATUS, GUID, ISppLicenseComponent,
                       ISppNamespace, PropertyStruct, WCHAR};

pub unsafe fn SppQueryClientContainerToken(
    ppwszOutContainerId: *mut PWSTR,
    pbIsAppContainer: *mut BOOL,
) -> HRESULT {
    let mut v4: PWSTR = core::ptr::null_mut();
    let mut v11: HRESULT = 0;
    let mut v12: usize = 0;
    let mut ppwsz_destination: PWSTR = core::ptr::null_mut();
    let mut token_handle: HANDLE = core::ptr::null_mut();
    let mut v15: u64 = 0;
    let mut src: [WCHAR; 128] = [0; 128];

    // Dummy addresses corresponding to relocation metadata in the decompiled binary
    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    ProcessReloc_Rva3_Len36(*dummy_table, *dummy_data);

    let current_thread = GetCurrentThread();
    if OpenThreadToken(current_thread, 8, 0, &mut token_handle) == 0 {
        let last_error = GetLastError();
        let mut v7: HRESULT = last_error as HRESULT;

        if last_error != 0 {
            if last_error > 0 {
                v7 = ((last_error & 0xFFFF) as HRESULT) | i32::MIN; // 0x80070000 | (last_error & 0xFFFF)
                ProcessReloc_Rva31_Len31(dummy_table, dummy_data);
            }
            ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);
        } else {
            ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
            v7 = -2147467259; // E_FAIL (0x80004005)
        }

        HandleSubsystemError(v7);
        return cleanup(v7, v4, token_handle, *dummy_table, *dummy_data);
    }

    v12 = 256;
    let nt_status = RtlQueryPackageClaims(
        token_handle,
        src.as_ptr(),
        &mut v12,
        core::ptr::null(),
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut v15,
        core::ptr::null_mut(),
    );

    if SppNtStatusToHresult(nt_status, &mut v11) == 0 {
        ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
        let v7 = v11;
        HandleSubsystemError(v7);
        return cleanup(v7, v4, token_handle, *dummy_table, *dummy_data);
    }

    if v12 == 0 {
        ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);
        let v7 = -1073418222; // 0xC004F012
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        HandleSubsystemError(v7);
        return cleanup(v7, v4, token_handle, *dummy_table, *dummy_data);
    }

    let mut v7 = SppDuplicateStringBounded(src.as_ptr(), (v12 >> 1) as i64, &mut ppwsz_destination);
    ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);

    if v7 >= 0 {
        if !ppwszOutContainerId.is_null() {
            *ppwszOutContainerId = ppwsz_destination;
        }
        ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);
        if !pbIsAppContainer.is_null() {
            let claim_type = ((v15 >> 32) & 0xFF) as u8;
            *pbIsAppContainer = if claim_type == 3 { 1 } else { 0 };
        }
    } else {
        HandleSubsystemError(v7);
        v4 = ppwsz_destination;
    }

    cleanup(v7, v4, token_handle, *dummy_table, *dummy_data)
}

unsafe fn cleanup(
    v7: HRESULT,
    v4: PWSTR,
    token_handle: HANDLE,
    dummy_table: *const c_void,
    dummy_data: *const c_void,
) -> HRESULT {
    extern "system" {
        fn HeapFree(hHeap: HANDLE, dwFlags: u32, lpMem: *mut c_void) -> BOOL;
        fn GetProcessHeap() -> HANDLE;
        fn LogTraceEvent(status: HRESULT);
        fn ProcessReloc_Rva36_Len3(table: *const c_void, data: *const c_void);
        fn ProcessReloc_Rva30_Len0(table: *const c_void, data: *const c_void);
        fn ProcessReloc_Rva0_Len36(table: *const c_void, data: *const c_void);
        fn CloseHandle(hObject: HANDLE) -> BOOL;
    }

    LogTraceEvent(v7);

    if !v4.is_null() {
        let process_heap = GetProcessHeap();
        // v4 - 2 skips backward 2 WCHARs (4 bytes) before freeing, as in original code
        HeapFree(process_heap, 0, v4.offset(-2) as *mut c_void);
        LogTraceEvent(0);
        ProcessReloc_Rva36_Len3(dummy_table, dummy_data);
    }

    ProcessReloc_Rva30_Len0(dummy_table, dummy_data);

    if !token_handle.is_null() {
        CloseHandle(token_handle);
    }

    ProcessReloc_Rva0_Len36(dummy_table, dummy_data);

    v7
}



pub unsafe fn SppQueryInternalPolicyNode(
    pInputIdContext: PCWSTR,
    pwszValueName: PCWSTR,
    ppOutVariant: *mut *mut SppPolicyVariant,
) -> HRESULT {
    let mut v5: *mut ISppLicenseComponent = core::ptr::null_mut();
    let mut v26: *mut c_void = core::ptr::null_mut();
    let mut v30: HLOCAL = core::ptr::null_mut();
    let mut h_mem: HLOCAL = core::ptr::null_mut();

    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);
    ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);

    let v7 = GlobalPtrSppNamespace;

    // Fastcall vtable lookup for SppNamespace method (index 7)
    type NamespaceFunc = unsafe fn(
        *mut ISppNamespace,
        PCWSTR,
        *mut HLOCAL,
    ) -> HRESULT;

    let v8_ptr = (*(*v7).lpVtbl)._Reserved1[7];
    let v8: NamespaceFunc = core::mem::transmute(v8_ptr);

    _guard_check_icall_fptr();
    let mut v10 = v8(v7, pInputIdContext, &mut h_mem);

    if v10 >= 0 {
        let mut pp_out_component: *mut ISppLicenseComponent = core::ptr::null_mut();
        v10 = SppCreateComponentInstance(h_mem as usize as i64, core::ptr::null_mut(), &mut pp_out_component);
        v5 = pp_out_component;

        if v10 < 0 {
            HandleSubsystemError(v10);
        } else {
            let query_interface = (*(*v5).lpVtbl).QueryInterface;
            _guard_check_icall_fptr();
            v10 = query_interface(v5, &unk_1403E9920, &mut v26);

            if v10 < 0 {
                HandleSubsystemError(v10);
            } else {
                let binding_key: [u16; 17] = [
                    'S' as u16, 'p' as u16, 'p' as u16, 'B' as u16, 'i' as u16, 'n' as u16,
                    'd' as u16, 'i' as u16, 'n' as u16, 'g' as u16, 'P' as u16, 'k' as u16,
                    'e' as u16, 'y' as u16, 'I' as u16, 'd' as u16, 0,
                ];

                let prop_struct = PropertyStruct {
                    key_name: binding_key.as_ptr(),
                    property_type: 2,
                    padding: 0,
                    value: pInputIdContext,
                    unused: 0,
                };

                ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);

                type Unknown2Func = unsafe fn(
                    *mut ISppLicenseComponent,
                    *const PropertyStruct,
                    i64,
                ) -> HRESULT;

                let v14_ptr = (*(*v5).lpVtbl).Unknown2[1];
                let v14: Unknown2Func = core::mem::transmute(v14_ptr);

                _guard_check_icall_fptr();
                v10 = v14(v5, &prop_struct, 1);

                ProcessReloc_Rva30_Len0(*dummy_table, *dummy_data);

                if v10 < 0 {
                    HandleSubsystemError(v10);
                } else {
                    let v15 = v26;
                    let vtbl_slot_24 = *(*(v15 as *mut *mut usize)).add(3);
                    type InterfaceFunc = unsafe fn(*mut c_void) -> HRESULT;
                    let v16: InterfaceFunc = core::mem::transmute(vtbl_slot_24);

                    _guard_check_icall_fptr();
                    v10 = v16(v15);

                    if v10 < 0 {
                        ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
                        HandleSubsystemError(v10);
                    } else {
                        let get_property_value = (*(*v5).lpVtbl).GetPropertyValue;
                        _guard_check_icall_fptr();
                        v10 = get_property_value(v5, pwszValueName, &mut v30);

                        ProcessReloc_Rva3_Len31(*dummy_table, *dummy_data);

                        if v10 == -2147024894 { // ERROR_FILE_NOT_FOUND (0x80070002)
                            v10 = -1073418218; // 0xC004F016
                            HandleSubsystemError(v10);
                        } else if v10 < 0 {
                            HandleSubsystemError(v10);
                        } else {
                            let v18 = v30 as *mut SppPolicyVariant;
                            ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
                            v30 = core::ptr::null_mut();
                            ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);

                            if !ppOutVariant.is_null() {
                                *ppOutVariant = v18;
                            }
                        }
                    }
                }
            }
        }
    } else {
        HandleSubsystemError(v10);
    }

    // Cleanup block
    LogTraceEvent(v10);

    if !h_mem.is_null() {
        ProcessReloc_Rva3_Len36(*dummy_table, *dummy_data);
        LocalFree(h_mem);
        h_mem = core::ptr::null_mut();
    }

    ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);

    if !v30.is_null() {
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        LocalFree(v30);
    }

    if !v26.is_null() {
        let v21 = v26;
        let vtbl_slot_16 = *(*(v21 as *mut *mut usize)).add(2);
        type ReleaseInterfaceFunc = unsafe fn(*mut c_void);
        let v22: ReleaseInterfaceFunc = core::mem::transmute(vtbl_slot_16);

        _guard_check_icall_fptr();
        v22(v21);
    }

    if !v5.is_null() {
        ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
        let release = (*(*v5).lpVtbl).Release.unwrap();
        _guard_check_icall_fptr();
        release(v5);
    }

    v10
}

macro_rules! wide_str {
    ($str:expr) => {{
        let mut vec: Vec<u16> = $str.encode_utf16().iter().collect();
        vec.push(0);
        vec.as_ptr()
    }};
}


pub unsafe fn SppQueryLicenseAttribute(
    _hContext: i64,
    hLicenseHandle: i64,
    pwszFriendlyName: PCWSTR,
    ppOutValue: *mut HLOCAL,
) -> HRESULT {
    let mut h_mem: HLOCAL = core::ptr::null_mut();
    let mut v7: HRESULT;

    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);

    if hLicenseHandle == 0 || pwszFriendlyName.is_null() || ppOutValue.is_null() {
        if !pwszFriendlyName.is_null() {
            ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        } else {
            ProcessReloc_Rva3_Len31(*dummy_table, *dummy_data);
        }
        v7 = -2147024809; // E_INVALIDARG (0x80070057)
        HandleSubsystemError(v7);
    } else {
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);

        // Map friendly attribute names to their internal canonical keys
        let internal_name: PCWSTR = if _wcsicmp(wide_str!("Name"), pwszFriendlyName) == 0 {
            wide_str!("productName")
        } else if _wcsicmp(wide_str!("Description"), pwszFriendlyName) == 0 {
            ProcessReloc_Rva3_Len36(*dummy_table, *dummy_data);
            wide_str!("productDescription")
        } else if _wcsicmp(wide_str!("Author"), pwszFriendlyName) == 0 {
            wide_str!("productAuthor")
        } else if _wcsicmp(wide_str!("LicensorUrl"), pwszFriendlyName) == 0 {
            ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
            wide_str!("licensorUrl")
        } else {
            let res_spc = _wcsicmp(wide_str!("SPCURL"), pwszFriendlyName);
            ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);

            if res_spc == 0 {
                wide_str!("SPCUrl")
            } else if _wcsicmp(wide_str!("RACURL"), pwszFriendlyName) == 0 {
                wide_str!("RACUrl")
            } else if _wcsicmp(wide_str!("PKCURL"), pwszFriendlyName) == 0 {
                ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
                wide_str!("PKCUrl")
            } else if _wcsicmp(wide_str!("EULURL"), pwszFriendlyName) == 0 {
                wide_str!("EULUrl")
            } else if _wcsicmp(wide_str!("PAURL"), pwszFriendlyName) == 0 {
                wide_str!("PAUrl")
            } else {
                let mapped = if _wcsicmp(wide_str!("ActivationSequence"), pwszFriendlyName) == 0 {
                    wide_str!("ActivationSequence")
                } else {
                    pwszFriendlyName
                };
                ProcessReloc_Rva30_Len0(*dummy_table, *dummy_data);
                mapped
            }
        };

        let store_ptr = pMetadataStore;
        let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(3);

        type QueryFunc = unsafe fn(
            *mut c_void,
            i64,
            PCWSTR,
            *mut HLOCAL,
        ) -> HRESULT;

        let query_fn: QueryFunc = core::mem::transmute(vtbl_slot);

        ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);
        _guard_check_icall_fptr();
        ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);

        v7 = query_fn(store_ptr, hLicenseHandle, internal_name, &mut h_mem);

        if v7 >= 0 {
            *ppOutValue = h_mem;
            h_mem = core::ptr::null_mut();
        } else {
            HandleSubsystemError(v7);
            ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
        }
    }

    LogTraceEvent(v7);
    ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);

    if !h_mem.is_null() {
        LocalFree(h_mem);
    }

    v7
}

pub unsafe fn SppQueryLicensingDatabase(
    _hContextFlags: i64,
    repositoryType: u32,
    pInputIdContext: i64,
    propertyClass: u32,
    pOutElementsCount: *mut u32,
    ppOutGuidArray: *mut *mut GUID,
) -> HRESULT {
    let mut v9: *mut GUID = core::ptr::null_mut();
    let mut v10: PWSTR = core::ptr::null_mut();
    let mut v11: PWSTR = core::ptr::null_mut();
    let mut v12: HRESULT;
    let mut v69: *mut c_void = core::ptr::null_mut();
    let mut lp_mem: *mut c_void = core::ptr::null_mut();
    let mut p_out_data_type: i32 = 0;
    let mut pb_is_valid: [i32; 3] = [0; 3];
    let mut p_out_guid: GUID = core::mem::zeroed();
    let mut v76: [u128; 2] = [0; 2];
    let mut p_out_data_size: u32 = 0;
    let mut v78: u32 = 0;

    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    if repositoryType >= 8 || propertyClass >= 8 || pOutElementsCount.is_null() {
        v12 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v12);
        return v12;
    }

    if ppOutGuidArray.is_null() {
        v12 = -2147024809; // E_INVALIDARG
        HandleSubsystemError(v12);
        ProcessReloc_Rva30_Len0(*dummy_table, *dummy_data);
        LogTraceEvent(v12);
        return v12;
    }

    let mut ppwsz_out_string: PWSTR = core::ptr::null_mut();
    v12 = SppNormalizeLicensingId(pInputIdContext, &mut ppwsz_out_string);
    ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);

    if v12 < 0 {
        HandleSubsystemError(v12);
        v10 = ppwsz_out_string;
    } else {
        v10 = ppwsz_out_string;
        let selector = (propertyClass + 8 * repositoryType) as i32;

        let res = match selector {
            0 => {
                let v27 = GlobalPtrSppNamespace;
                type Func28 = unsafe fn(*mut ISppNamespace, *mut *mut c_void) -> HRESULT;
                let v28: Func28 = core::mem::transmute((*(*v27).lpVtbl)._Reserved3[2]);
                _guard_check_icall_fptr();
                v28(v27, &mut v69)
            }
            1 => {
                let v19 = GlobalPtrSppNamespace;
                type Func20 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v20: Func20 = core::mem::transmute((*(*v19).lpVtbl)._Reserved3[3]);
                _guard_check_icall_fptr();
                v20(v19, v10, &mut v69)
            }
            3 => {
                let v19 = GlobalPtrSppNamespace;
                type Func20 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v20: Func20 = core::mem::transmute((*(*v19).lpVtbl)._Reserved1[2]);
                _guard_check_icall_fptr();
                v20(v19, v10, &mut v69)
            }
            5 => {
                let v19 = GlobalPtrSppNamespace;
                type Func20 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v20: Func20 = core::mem::transmute((*(*v19).lpVtbl)._Reserved1[5]);
                ProcessReloc_Rva3_Len31(dummy_table, dummy_data);
                _guard_check_icall_fptr();
                v20(v19, v10, &mut v69)
            }
            6 => {
                let v19 = GlobalPtrSppNamespace;
                type Func20 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v20: Func20 = core::mem::transmute((*(*v19).lpVtbl)._Reserved1[6]);
                _guard_check_icall_fptr();
                v20(v19, v10, &mut v69)
            }
            8 => {
                let v25 = GlobalPtrSppNamespace;
                type Func26 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v26: Func26 = core::mem::transmute((*(*v25).lpVtbl)._Reserved3[4]);
                _guard_check_icall_fptr();
                let status = v26(v25, v10, &mut v69);
                ProcessReloc_Rva8_Len36(dummy_table, dummy_data);
                status
            }
            9 => {
                let v19 = GlobalPtrSppNamespace;
                type Func20 = unsafe fn(*mut ISppNamespace, *mut *mut c_void, *mut c_void) -> HRESULT;
                let v20: Func20 = core::mem::transmute((*(*v19).lpVtbl)._Reserved3[1]);
                _guard_check_icall_fptr();
                v20(v19, &mut v69, core::ptr::null_mut())
            }
            11 => {
                let v29 = GlobalPtrSppNamespace;
                type Func30 = unsafe fn(*mut ISppNamespace, PWSTR, *mut *mut c_void) -> HRESULT;
                let v30: Func30 = core::mem::transmute((*(*v29).lpVtbl)._Reserved1[4]);
                _guard_check_icall_fptr();
                v30(v29, v10, &mut v69)
            }
            12 => {
                let v48 = GlobalPtrSppNamespace;
                let open_namespace = (*(*v48).lpVtbl).OpenNamespace;
                _guard_check_icall_fptr();
                let mut status = open_namespace(v48, 1, v10, wide_str!("pkeyId"), &mut v69 as *mut _ as *mut HLOCAL);
                if status >= 0 {
                    status = SppValidateLicenseComponentState(v10 as i64, pb_is_valid.as_mut_ptr());
                    if status >= 0 && pb_is_valid[0] == 0 {
                        status = -1073418222; // 0xC004F012
                        ProcessReloc_Rva36_Len3(dummy_table, dummy_data);
                    }
                }
                status
            }
            19 => {
                let store_ptr = pMetadataStore;
                type Func47 = unsafe fn(*mut c_void, PWSTR, *mut *mut c_void) -> HRESULT;
                let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(24);
                let v47: Func47 = core::mem::transmute(vtbl_slot);
                _guard_check_icall_fptr();
                let status = v47(store_ptr, v10, &mut v69);
                ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);
                status
            }
            26 => {
                let store_ptr = pMetadataStore;
                type Func45 = unsafe fn(*mut c_void, PWSTR, *mut *mut c_void, u64) -> HRESULT;
                let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(25);
                let v45: Func45 = core::mem::transmute(vtbl_slot);
                _guard_check_icall_fptr();
                v45(store_ptr, v10, &mut v69, 0)
            }
            27 => {
                let store_ptr = pMetadataStore;
                type Func43 = unsafe fn(*mut c_void, *mut *mut c_void) -> HRESULT;
                let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(23);
                let v43: Func43 = core::mem::transmute(vtbl_slot);
                _guard_check_icall_fptr();
                v43(store_ptr, &mut v69)
            }
            39 => {
                let mut status = SppQueryRegistryPolicyValue(
                    0,
                    pInputIdContext,
                    wide_str!("DigitalPID2"),
                    &mut p_out_data_type,
                    &mut p_out_data_size,
                    &mut lp_mem,
                );
                if status >= 0 {
                    if p_out_data_size != 48 || p_out_data_type != 1 {
                        status = -2147418113; // E_UNEXPECTED
                    } else {
                        let mut v73: PWSTR = core::ptr::null_mut();
                        status = SppHashAndSerializeLicensingId(lp_mem as i64, &mut v73);
                        if status >= 0 {
                            v11 = v73;
                            let store_ptr = pMetadataStore;
                            type Func41 = unsafe fn(
                                *mut c_void,
                                u64,
                                *mut *mut c_void,
                                PCWSTR,
                                PCWSTR,
                                PCWSTR,
                                PWSTR,
                                u64,
                                u64,
                            ) -> HRESULT;
                            let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(55);
                            let v41: Func41 = core::mem::transmute(vtbl_slot);

                            ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
                            _guard_check_icall_fptr();
                            ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);

                            status = v41(
                                store_ptr,
                                1,
                                &mut v69,
                                wide_str!("metaInfoType"),
                                wide_str!("metaInfoTypeStoreToken"),
                                wide_str!("licenseId"),
                                v11,
                                0,
                                0,
                            );
                        } else {
                            v11 = v73;
                        }
                    }
                }
                status
            }
            51 => {
                let store_ptr = pMetadataStore;
                type Func37 = unsafe fn(
                    *mut c_void,
                    u64,
                    *mut *mut c_void,
                    PCWSTR,
                    PCWSTR,
                    u64,
                    u64,
                    u64,
                    u64,
                ) -> HRESULT;
                let vtbl_slot = *(*(store_ptr as *mut *mut usize)).add(55);
                let v37: Func37 = core::mem::transmute(vtbl_slot);
                _guard_check_icall_fptr();
                v37(
                    store_ptr,
                    1,
                    &mut v69,
                    wide_str!("metaInfoType"),
                    wide_str!("metaInfoTypeStoreToken"),
                    0,
                    0,
                    0,
                    0,
                )
            }
            _ => -1073418218, // 0xC004F016
        };

        v12 = res;

        if v12 >= 0 {
            ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
            if v69.is_null() {
                v12 = -1073418222;
                HandleSubsystemError(v12);
            } else {
                type GetCountFunc = unsafe fn(*mut c_void, *mut u32) -> HRESULT;
                let vtbl_slot = *(*(v69 as *mut *mut usize)).add(3);
                let get_count: GetCountFunc = core::mem::transmute(vtbl_slot);

                _guard_check_icall_fptr();
                v12 = get_count(v69, &mut v78);

                if v12 >= 0 {
                    if v78 == 0 {
                        v12 = -1073418222;
                        HandleSubsystemError(v12);
                    } else {
                        let alloc_size = v78.checked_mul(16);
                        let v52 = match alloc_size {
                            Some(sz) if (v78 & 0x0FFF_FFFF) == v78 => sz,
                            _ => {
                                v12 = -2147024362; // 0x80070216
                                HandleSubsystemError(v12);
                                0
                            }
                        };

                        LogTraceEvent(v12);

                        if v12 >= 0 {
                            let alloc_ptr = LocalAlloc(0x40, v52 as usize as SIZE_T) as *mut GUID;
                            if alloc_ptr.is_null() {
                                v12 = -2147024882; // E_OUTOFMEMORY
                                HandleSubsystemError(v12);
                            } else {
                                v9 = alloc_ptr;
                                type GetItemFunc = unsafe fn(
                                    *mut c_void,
                                    u32,
                                    *mut u128,
                                ) -> HRESULT;
                                let vtbl_slot_item = *(*(v69 as *mut *mut usize)).add(4);
                                let get_item: GetItemFunc = core::mem::transmute(vtbl_slot_item);

                                for v54 in 0..v78 {
                                    _guard_check_icall_fptr();
                                    v12 = get_item(v69, v54, v76.as_mut_ptr());

                                    if v12 < 0 {
                                        break;
                                    }

                                    let str_ptr = v76[0] as *const u16;
                                    if str_ptr.is_null() {
                                        v12 = -2147024809;
                                        break;
                                    }

                                    let mut len = 0usize;
                                    while *str_ptr.add(len) != 0 {
                                        len += 1;
                                    }

                                    let target_str = if len == 38 {
                                        if *str_ptr != 123 || *str_ptr.add(37) != 125 {
                                            v12 = -2147024809;
                                            break;
                                        }
                                        str_ptr.add(1)
                                    } else if len == 36 {
                                        str_ptr
                                    } else {
                                        v12 = -2147024809;
                                        break;
                                    };

                                    let mut parse_res = SppGuidFromString(target_str, &mut p_out_guid);
                                    if parse_res < 0 {
                                        HandleSubsystemError(parse_res);
                                    }
                                    LogTraceEvent(parse_res);

                                    if parse_res == -2147024809 {
                                        parse_res = -1073422330; // 0xC004E006
                                    }

                                    if parse_res < 0 {
                                        HandleSubsystemError(parse_res);
                                    }
                                    LogTraceEvent(parse_res);

                                    v12 = parse_res;
                                    if v12 < 0 {
                                        break;
                                    }

                                    *v9.add(v54 as usize) = p_out_guid;
                                }

                                if v12 >= 0 {
                                    *pOutElementsCount = v78;
                                    *ppOutGuidArray = v9;
                                    v9 = core::ptr::null_mut();
                                } else {
                                    HandleSubsystemError(v12);
                                }
                            }
                        }
                    }
                } else {
                    HandleSubsystemError(v12);
                }
            }
        } else {
            HandleSubsystemError(v12);
        }
    }

    LogTraceEvent(v12);

    if !v11.is_null() {
        ProcessReloc_Rva3_Len36(dummy_table, dummy_data);
        ProcessReloc_Rva36_Len0(dummy_table, dummy_data);
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, (v11 as *mut u16).offset(-1) as *mut c_void);
        LogTraceEvent(0);
    }

    if !lp_mem.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, lp_mem);
    }

    if !v69.is_null() {
        type ReleaseFunc = unsafe fn(*mut c_void);
        let vtbl_slot = *(*(v69 as *mut *mut usize)).add(2);
        let release: ReleaseFunc = core::mem::transmute(vtbl_slot);
        _guard_check_icall_fptr();
        release(v69);
        ProcessReloc_Rva2_Len30(dummy_table, dummy_data);
    }

    if !v10.is_null() {
        LocalFree(v10 as HLOCAL);
    }

    if !v9.is_null() {
        LocalFree(v9 as HLOCAL);
    }

    v12
}

pub unsafe fn SppQueryPropertyInternal(
    skuId: i64,
    pwszPropertyName: PCWSTR,
    ppVariantOutput: *mut HLOCAL,
) -> HRESULT {
    let mut pp_out_component: [*mut ISppLicenseComponent; 5] = [core::ptr::null_mut(); 5];
    let mut h_mem: HLOCAL = core::ptr::null_mut();

    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);
    let mut component_interface = SppGetComponentInterface(skuId, pp_out_component.as_mut_ptr());
    ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);

    let v7 = pp_out_component[0];

    if component_interface >= 0 && !v7.is_null() {
        let validate_state = (*(*v7).lpVtbl).ValidateState;
        ProcessReloc_Rva3_Len31(*dummy_table, *dummy_data);
        _guard_check_icall_fptr();
        component_interface = validate_state(v7);
        ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);
        ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);

        if component_interface >= 0 {
            let lp_vtbl = (*v7).lpVtbl;
            ProcessReloc_Rva30_Len0(*dummy_table, *dummy_data);
            _guard_check_icall_fptr();
            component_interface = (lp_vtbl.GetPropertyValue.unwrap())(v7, pwszPropertyName, &mut h_mem) as i32;
            ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
        }
    }

    if component_interface < 0 {
        HandleSubsystemError(component_interface);
    } else if !ppVariantOutput.is_null() {
        let v10 = h_mem;
        h_mem = core::ptr::null_mut();
        *ppVariantOutput = v10;
    }

    LogTraceEvent(component_interface);
    ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);

    if !h_mem.is_null() {
        LocalFree(h_mem);
        h_mem = core::ptr::null_mut();
        ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
    }

    ProcessReloc_Rva3_Len36(*dummy_table, *dummy_data);

    if !v7.is_null() {
        ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
        let release = (*(*v7).lpVtbl).Release.unwrap();
        _guard_check_icall_fptr();
        release(v7);
    }

    ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);

    component_interface
}

#[inline]
fn hresult_from_win32(win32_err: LSTATUS) -> HRESULT {
    if win32_err > 0 {
        ((win32_err as u32) & 0x0000_FFFF | 0x8007_0000) as HRESULT
    } else {
        win32_err as HRESULT
    }
}


pub unsafe fn SppQueryRegistryDword(
    hKey: HKEY,
    pwszSubKey: LPCWSTR,
    pwszValueName: LPCWSTR,
    pdwOutValue: *mut DWORD,
) -> HRESULT {
    let mut v4: *mut BYTE = core::ptr::null_mut();
    let mut v5: HKEY = core::ptr::null_mut();
    let mut h_key_a: HKEY = core::ptr::null_mut();
    let mut val_type: DWORD = 0;
    let mut cb_data: DWORD = 0;

    if hKey == HKEY_CURRENT_USER {
        LogTraceEvent(0);
    }

    let status = RegOpenKeyExW(hKey, pwszSubKey, 0, 1, &mut h_key_a);
    let mut v11 = status;

    if status != 0 {
        if status > 0 {
            v11 = (status as u16 as u32 | 0x80070000) as HRESULT;
        }
        HandleSubsystemError(v11);
    } else {
        v5 = h_key_a;
        h_key_a = core::ptr::null_mut();
    }

    LogTraceEvent(v11);

    if !h_key_a.is_null() {
        RegCloseKey(h_key_a);
        h_key_a = core::ptr::null_mut();
    }

    if v11 >= 0 {
        cb_data = 0;

        loop {
            let query_status = RegQueryValueExW(
                v5,
                pwszValueName,
                core::ptr::null_mut(),
                &mut val_type,
                v4,
                &mut cb_data,
            );

            v11 = query_status;

            if query_status != 0 {
                if query_status > 0 {
                    v11 = (query_status as u16 as u32 | 0x80070000) as HRESULT;
                }
                HandleSubsystemError(v11);
                break;
            }

            if !v4.is_null() {
                break;
            }

            if cb_data == 0 {
                v11 = -2147418113; // E_UNEXPECTED
                HandleSubsystemError(v11);
                break;
            }

            let process_heap = GetProcessHeap();
            v4 = HeapAlloc(process_heap, 0, cb_data as usize as SIZE_T) as *mut BYTE;

            if v4.is_null() {
                v11 = -2147024882; // E_OUTOFMEMORY
                HandleSubsystemError(v11);
                break;
            }
        }

        if v11 >= 0 {
            if val_type != REG_DWORD {
                v11 = -2147024883; // ERROR_INVALID_DATA
                HandleSubsystemError(v11);
            } else {
                let mut v15 = 0;

                if cb_data == 4 {
                    if !pdwOutValue.is_null() {
                        *pdwOutValue = *(v4 as *const DWORD);
                    }
                } else {
                    v15 = -2147024883; // ERROR_INVALID_DATA
                    HandleSubsystemError(-2147024883);
                }

                LogTraceEvent(v15);
                v11 = v15;

                if v15 < 0 {
                    HandleSubsystemError(v15);
                }
            }
        }
    }

    LogTraceEvent(v11);

    if !v4.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v4 as *mut c_void);
    }

    if !v5.is_null() {
        RegCloseKey(v5);
    }

    v11
}

pub unsafe fn SppQueryRegistryMultiString(
    hKey: HKEY,
    pwszSubKey: LPCWSTR,
    pwszValueName: LPCWSTR,
    pOutParsedContainerContext: *mut i64,
) -> HRESULT {
    let mut v4: *mut BYTE = core::ptr::null_mut();
    let mut v5: HKEY = core::ptr::null_mut();
    let mut h_key_a: HKEY = core::ptr::null_mut();
    let mut val_type: DWORD = 0;
    let mut cb_data: DWORD = 0;
    let mut pb_take_buffer_ownership: BOOL = 0;

    if hKey == HKEY_CURRENT_USER {
        LogTraceEvent(0);
    }

    let status = RegOpenKeyExW(hKey, pwszSubKey, 0, 1, &mut h_key_a);
    let mut v11 = status;

    if status != 0 {
        if status > 0 {
            v11 = (status as u16 as u32 | 0x80070000) as HRESULT;
        }
        HandleSubsystemError(v11);
    } else {
        v5 = h_key_a;
        h_key_a = core::ptr::null_mut();
    }

    LogTraceEvent(v11);

    if !h_key_a.is_null() {
        RegCloseKey(h_key_a);
        h_key_a = core::ptr::null_mut();
    }

    if v11 >= 0 {
        cb_data = 0;

        loop {
            let query_status = RegQueryValueExW(
                v5,
                pwszValueName,
                core::ptr::null_mut(),
                &mut val_type,
                v4,
                &mut cb_data,
            );

            v11 = query_status;

            if query_status != 0 {
                if query_status > 0 {
                    v11 = (query_status as u16 as u32 | 0x80070000) as HRESULT;
                }
                HandleSubsystemError(v11);
                break;
            }

            if !v4.is_null() {
                break;
            }

            if cb_data == 0 {
                v11 = -2147418113; // E_UNEXPECTED
                HandleSubsystemError(v11);
                break;
            }

            let process_heap = GetProcessHeap();
            v4 = HeapAlloc(process_heap, 0, cb_data as usize as SIZE_T) as *mut BYTE;

            if v4.is_null() {
                v11 = -2147024882; // E_OUTOFMEMORY
                HandleSubsystemError(v11);
                break;
            }
        }

        if v11 >= 0 {
            if val_type != REG_SZ {
                v11 = -2147024883; // ERROR_INVALID_DATA
                HandleSubsystemError(v11);
            } else {
                let v15 = SppMarshalRegistryStringToContext(
                    v4 as *const u16,
                    cb_data,
                    pOutParsedContainerContext as *mut LPWSTR,
                    &mut pb_take_buffer_ownership,
                );

                v11 = v15;

                if v15 < 0 {
                    HandleSubsystemError(v15);
                } else if pb_take_buffer_ownership != 0 {
                    v4 = core::ptr::null_mut();
                }
            }
        }
    }

    LogTraceEvent(v11);

    if !v4.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v4 as *mut c_void);
    }

    if !v5.is_null() {
        RegCloseKey(v5);
    }

    v11
}

pub unsafe fn SppQueryRegistryPolicyValue(
    hUnusedContext: i64,
    pInputIdContext: i64,
    pwszValueName: PCWSTR,
    pOutDataType: *mut i32,
    pOutDataSize: *mut u32,
    ppOutDataBuffer: *mut *mut c_void,
) -> HRESULT {
    let mut v7: *mut SppPolicyVariant = core::ptr::null_mut();
    let mut v10: PWSTR = core::ptr::null_mut();
    let mut v11: PWSTR = core::ptr::null_mut();
    let mut pp_out_variant: *mut SppPolicyVariant = core::ptr::null_mut();
    let mut ppwsz_destination: PWSTR = core::ptr::null_mut();
    let mut pcb_out_bytes: u32 = 0;
    let mut ppwsz_out_string: PWSTR = core::ptr::null_mut();

    let dummy_table = core::ptr::null();
    let dummy_data = core::ptr::null();

    ProcessReloc_Rva3_Len36(*dummy_table, *dummy_data);

    if pwszValueName.is_null() {
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    ProcessReloc_Rva33_Len3(*dummy_table, *dummy_data);

    if pOutDataType.is_null() {
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        HandleSubsystemError(E_INVALIDARG);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    ProcessReloc_Rva3_Len31(*dummy_table, *dummy_data);

    if pOutDataSize.is_null() {
        HandleSubsystemError(E_INVALIDARG);
        ProcessReloc_Rva36_Len0(*dummy_table, *dummy_data);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    if ppOutDataBuffer.is_null() {
        ProcessReloc_Rva2_Len30(*dummy_table, *dummy_data);
        HandleSubsystemError(E_INVALIDARG);
        ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);
        LogTraceEvent(E_INVALIDARG);
        return E_INVALIDARG;
    }

    let mut string_byte_length_safe = SppConvertContextToIdString(pInputIdContext, &mut ppwsz_out_string);
    if string_byte_length_safe < 0 {
        HandleSubsystemError(string_byte_length_safe);
        v10 = ppwsz_out_string;
    } else {
        v10 = ppwsz_out_string;
        string_byte_length_safe = SppQueryInternalPolicyNode(v10, pwszValueName, &mut pp_out_variant);
        if string_byte_length_safe < 0 {
            HandleSubsystemError(string_byte_length_safe);
            v7 = pp_out_variant;
        } else {
            v7 = pp_out_variant;
            let mut v18: i32 = 0;
            let mut binary_buffer_size: u32 = 0;
            let mut v21: *mut c_void = core::ptr::null_mut();

            match (*v7).policyType {
                1 => {
                    v18 = 4;
                    let alloc_ptr = LocalAlloc(0x40, 4) as *mut u32;
                    v21 = alloc_ptr as *mut c_void;
                    if alloc_ptr.is_null() {
                        string_byte_length_safe = E_OUTOFMEMORY;
                        HandleSubsystemError(E_OUTOFMEMORY);
                        ProcessReloc_Rva28_Len0(*dummy_table, *dummy_data);
                    } else {
                        binary_buffer_size = 4;
                        *alloc_ptr = (*v7).data.dwordValue;
                    }
                }
                2 => {
                    v18 = 1;
                    let v22 = SppDuplicateStringLocal((*v7).data.pwszValue, &mut ppwsz_destination);
                    string_byte_length_safe = v22;
                    if v22 < 0 {
                        HandleSubsystemError(v22);
                        v11 = ppwsz_destination;
                    } else {
                        v11 = ppwsz_destination;
                        string_byte_length_safe = SppGetStringByteLengthSafe(
                            v11,
                            &mut pcb_out_bytes,
                            0x7FFF_FFFF,
                        );
                        ProcessReloc_Rva8_Len36(*dummy_table, *dummy_data);
                        if string_byte_length_safe < 0 {
                            HandleSubsystemError(string_byte_length_safe);
                        } else {
                            binary_buffer_size = pcb_out_bytes;
                            v21 = v11 as *mut c_void;
                            v11 = core::ptr::null_mut();
                        }
                    }
                }
                4 => {
                    v18 = 3;
                    ProcessReloc_Rva0_Len36(*dummy_table, *dummy_data);
                    binary_buffer_size = (*v7).binaryBufferSize as u32;
                    let v20 = LocalAlloc(0x40, binary_buffer_size as usize as SIZE_T);
                    v21 = v20;
                    if v20.is_null() {
                        string_byte_length_safe = E_OUTOFMEMORY;
                        HandleSubsystemError(E_OUTOFMEMORY);
                        ProcessReloc_Rva36_Len3(*dummy_table, *dummy_data);
                    } else {
                        memcpy(v20, (*v7).data.pwszValue as *const c_void, binary_buffer_size as usize);
                    }
                }
                _ => {
                    string_byte_length_safe = E_UNEXPECTED;
                    HandleSubsystemError(E_UNEXPECTED);
                }
            }

            if string_byte_length_safe >= 0 {
                *pOutDataType = v18;
                *pOutDataSize = binary_buffer_size;
                *ppOutDataBuffer = v21;
            }
        }
    }

    LogTraceEvent(string_byte_length_safe);

    if !v11.is_null() {
        ProcessReloc_Rva30_Len0(*dummy_table, *dummy_data);
        LocalFree(v11 as HLOCAL);
    }
    if !v10.is_null() {
        LocalFree(v10 as HLOCAL);
    }
    if !v7.is_null() {
        LocalFree(v7 as HLOCAL);
    }

    string_byte_length_safe
}

pub unsafe fn SppQuerySystemPolicyBits(
    _unusedParam: i64,
    pOutControlFlags: *mut u32,
    hPolicyContext: i64,
) -> *mut u32 {
    let pfn_query_policy_field: Option<unsafe extern "fastcall" fn(i64, i64, i64) -> i32> =
        if qword_14046B478.is_some() {
            qword_14046B478
        } else {
            qword_14046B480
        };

    let raw_policy_bits = if let Some(pfn) = pfn_query_policy_field {
        _guard_check_icall_fptr();
        pfn(59974269, 3, hPolicyContext)
    } else {
        0
    };

    // *(_QWORD *)pOutControlFlags = 0;
    *(pOutControlFlags as *mut u64) = 0;

    let initial_mask = 8 * ((raw_policy_bits & 0x80) | (4 * ((raw_policy_bits & 0x40) | (4 * (raw_policy_bits & 3)))));
    *pOutControlFlags = initial_mask as u32;

    let unmasked_bits = raw_policy_bits as u32 & 0xFFFFFF3F;
    let state_override_bit = if unmasked_bits != 0 {
        if unmasked_bits == 2 { 64 } else { 0 }
    } else {
        64
    };

    let final_translated_mask = (state_override_bit | initial_mask) as u64;
    *pOutControlFlags = final_translated_mask as u32;

    let mut is_logging_triggered = false;
    let mut final_boolean_bit = 1;

    if (final_translated_mask & 0xC00) == 0xC00 || (final_translated_mask & 0x40) != 0 {
        let mut etw_control_flags = dword_14046B420[0];

        if (etw_control_flags & 4) == 0 {
            let mut v16: i8 = 0;
            let override_ptr = SppGetRuntimeLoggingOverride(final_translated_mask as i64 as u64, &mut v16);
            etw_control_flags = *override_ptr as u32;
        }

        let mut etw_payload_block: u64 = 0;
        let payload_ptr = &mut etw_payload_block as *mut u64 as *mut EtwPayload;
        (*payload_ptr).low = 0;
        (*payload_ptr).high_word = 3;

        SppEmitEtwEvent(
            &hEtwRegistration as *const _ as usize,
            58988972,
            (etw_control_flags >> 10) & 1,
            (etw_control_flags >> 11) & 1,
            &etw_payload_block as *const _ as i64,
            1,
            0,
        );

        is_logging_triggered = true;
    }

    if (*pOutControlFlags & 0x40) == 0 || !is_logging_triggered {
        final_boolean_bit = 0;
    }

    *pOutControlFlags = final_boolean_bit | (*pOutControlFlags & 0xFFFFFFFE);

    pOutControlFlags
}