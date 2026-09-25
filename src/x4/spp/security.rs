use core::ffi::c_void;
use crate::u16_str;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{CheckTokenMembership, ConvertStringSidToSidW, FreeSid, GetLastError, GetProcessHeap, HeapFree, I_RpcMapWin32Status, RpcImpersonateClient, RpcRevertToSelfEx};
use crate::x4::globals::{HKEY_LOCAL_MACHINE, SPP_E_ACCESS_DENIED};
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva30_Len0, ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0, ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31, ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36};
use crate::x4::spp::{SppCheckBuiltinAdminMembership, SppCheckWellKnownGroupMembership};
use crate::x4::spp::query::{SppQueryClientContainerToken, SppQueryRegistryDword, SppQueryRegistryMultiString};
use crate::x4::spp::registry::SppParseRegistryContainersList;
use crate::x4::spp::string::SppMatchWildcardString;
use crate::x4::spp::vector::SppVectorReallocateHeap;
use crate::x4::types::{VectorLayoutBuffered, BOOL, HRESULT, PSID};

pub unsafe fn SppVerifyAccessSecurity() -> HRESULT {
    let mut origin_flag: i32 = 0;
    let mut is_member: BOOL = 0;
    let mut sid: PSID = core::ptr::null_mut();

    let service_sids: [*const u16; 2] = [
        u16_str!("S-1-5-80-123231216-2592883651-3715271367-3753151631-4175906628"),
        u16_str!("S-1-5-80-4215458991-2034252225-2287069555-1155419622-2701885083"),
    ];

    let rpc_status = RpcImpersonateClient(core::ptr::null_mut());

    if rpc_status == RPC_S_NO_CALL_ACTIVE {
        let hr = 0;
        LogTraceEvent(hr);
        return hr;
    }

    let mut hr: HRESULT;

    if rpc_status == 0 {
        is_member = 0;
        hr = SppCheckBuiltinAdminMembership(0, &mut is_member);
        ProcessReloc_Rva28_Len0(&stru_14044BE60, &dword_140467674);

        if hr < 0 {
            ProcessReloc_Rva30_Len0(&stru_140447578, &dword_140462640);
            HandleSubsystemError(hr);
            RpcRevertToSelfEx(core::ptr::null_mut());
            LogTraceEvent(hr);
            return hr;
        }

        if is_member != 0 {
            hr = 0;
            RpcRevertToSelfEx(core::ptr::null_mut());
            LogTraceEvent(hr);
            return hr;
        }

        ProcessReloc_Rva36_Len0(&stru_140440D48, &dword_1404662D4);

        if dword_14046B8B4 != 0 {
            for &sid_str in &service_sids {
                let mut local_is_member: BOOL = 0;
                let mut service_sid: PSID = core::ptr::null_mut();
                let mut sid_check_pass = false;

                if ConvertStringSidToSidW(sid_str, &mut service_sid) != 0
                    && CheckTokenMembership(core::ptr::null_mut(), service_sid, &mut local_is_member) != 0
                {
                    sid_check_pass = true;
                    is_member = local_is_member;
                } else {
                    let last_error = GetLastError();
                    if last_error != 0 {
                        hr = (last_error & 0xFFFF) as i32 | (-0x7FFF0000); // 0x80070000 | (last_error & 0xFFFF)
                    } else {
                        hr = E_FAIL;
                    }
                    HandleSubsystemError(hr);
                }

                LogTraceEvent(hr);
                sid = service_sid;

                if !service_sid.is_null() {
                    FreeSid(service_sid);
                }

                if hr < 0 {
                    HandleSubsystemError(hr);
                    RpcRevertToSelfEx(core::ptr::null_mut());
                    LogTraceEvent(hr);
                    return hr;
                }

                if sid_check_pass && is_member != 0 {
                    hr = 0;
                    RpcRevertToSelfEx(core::ptr::null_mut());
                    LogTraceEvent(hr);
                    return hr;
                }
            }

            let mut check_status = SppCheckWellKnownGroupMembership(sid as i64, 0x12, &mut is_member);
            hr = check_status;

            if check_status < 0 {
                HandleSubsystemError(check_status);
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            if is_member != 0 {
                hr = 0;
                ProcessReloc_Rva33_Len3(&stru_14043ED08, &dword_140463C20);
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            check_status = SppCheckWellKnownGroupMembership(0, 0x13, &mut is_member);
            hr = check_status;

            if check_status < 0 {
                HandleSubsystemError(check_status);
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            ProcessReloc_Rva0_Len36(&stru_14044B1EC, &dword_140461AF8);

            if is_member != 0 {
                hr = 0;
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            hr = SppCheckWellKnownGroupMembership(0, 0x14, &mut is_member);
            ProcessReloc_Rva3_Len36(&dword_14043F7DC, &dword_1404649C8);

            if hr < 0 {
                HandleSubsystemError(hr);
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            if is_member != 0 {
                hr = 0;
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }

            if SppVerifyContainerOrigin(&mut origin_flag) >= 0 && origin_flag != 0 {
                ProcessReloc_Rva36_Len3(&stru_14044CEF0, &dword_140468AC4);
                hr = 0;
                RpcRevertToSelfEx(core::ptr::null_mut());
                LogTraceEvent(hr);
                return hr;
            }
        }

        hr = SPP_E_ACCESS_DENIED;
        HandleSubsystemError(SPP_E_ACCESS_DENIED);
        ProcessReloc_Rva2_Len30(&stru_14043F3D8, &dword_140464388);
        RpcRevertToSelfEx(core::ptr::null_mut());
    } else {
        let mapped_status = I_RpcMapWin32Status(rpc_status);
        hr = mapped_status;

        if mapped_status > 0 {
            hr = (mapped_status & 0xFFFF) as i32 | (-0x7FFF0000);
            ProcessReloc_Rva3_Len31(&stru_140451E08, &dword_140463AF8);
        } else {
            ProcessReloc_Rva33_Len3(&stru_140451468, &dword_140463140);
        }

        if hr >= 0 {
            hr = E_FAIL;
        }

        ProcessReloc_Rva3_Len31(
            (&stru_140443DC8 as *const _ as *const u8).wrapping_add(4) as *const c_void,
            &dword_140469948,
        );
        HandleSubsystemError(hr);
        ProcessReloc_Rva8_Len36(&stru_140451EE8, &dword_140463BBC);
    }

    LogTraceEvent(hr);
    hr
}

// Helper macro for compile-time null-terminated wide string conversion
#[macro_export]
macro_rules! u16_str {
    ($s:expr) => {{
        const UTF16: &[u16] = &{
            let mut buf = [0u16; $s.len() + 1];
            let bytes = $s.as_bytes();
            let mut i = 0;
            while i < bytes.len() {
                buf[i] = bytes[i] as u16;
                i += 1;
            }
            buf[bytes.len()] = 0;
            buf
        };
        UTF16.as_ptr()
    }};
}

pub unsafe fn SppVerifyContainerOrigin(pbIsAuthorized: *mut i32) -> HRESULT {
    let mut v1: usize = 0;
    let mut v2: i64 = 0;
    let mut v15: i64 = 0;
    let mut v20: i64 = 0;
    let mut v19: i32 = 0;
    let mut v16 = VectorLayoutBuffered {
        capacity: 0,
        count: 0,
        ppBuffer: core::ptr::null_mut(),
    };
    let mut lp_mem: *mut *mut u16 = core::ptr::null_mut();

    ProcessReloc_Rva2_Len30(&stru_1404478F8, &dword_140462854);

    let mut hr = SppQueryClientContainerToken(&mut v15, &mut v19);
    let h_client_container_token = v15;

    if hr < 0 {
        HandleSubsystemError(hr);
    } else {
        let spp_reg_key = u16_str!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\SoftwareProtectionPlatform");

        let _ = SppQueryRegistryDword(
            HKEY_LOCAL_MACHINE,
            spp_reg_key,
            u16_str!("IgnoreContainerOrigin"),
            0 as *mut u32,
        );
        ProcessReloc_Rva3_Len36(&dword_14044D388, &dword_140462060);

        if v19 == 0 {
            ProcessReloc_Rva0_Len36(&stru_14043E3B8, &dword_14046304C);
            hr = SPP_E_ACCESS_DENIED;
            HandleSubsystemError(hr);
        } else {
            hr = SppQueryRegistryMultiString(
                HKEY_LOCAL_MACHINE,
                spp_reg_key,
                u16_str!("AuthorizedContainers"),
                0 as *mut i64,
            );
            ProcessReloc_Rva8_Len36(&stru_140435228, &dword_140464A00);
            ProcessReloc_Rva36_Len0(
                &stru_140447A48 as *const _ as *const c_void,
                &dword_140462A94,
            );

            if hr >= 0 {
                v2 = v20;
                hr = SppParseRegistryContainersList(v20, 0, &mut v16);
                ProcessReloc_Rva28_Len0(
                    (&stru_140449788 as *const _ as *const u8).wrapping_add(4) as *const c_void,
                    &dword_140464A28,
                );
                ProcessReloc_Rva3_Len36(
                    (&stru_14043E80C as *const _ as *const u8).wrapping_add(4) as *const c_void,
                    &dword_140463568,
                );

                if hr < 0 {
                    HandleSubsystemError(hr);
                } else {
                    let mut is_authorized = 0;
                    let num_containers = v16.count;

                    if num_containers > 0 {
                        ProcessReloc_Rva3_Len31(&stru_1404514CC, &dword_1404631AC);
                        let mut idx = 0;

                        while idx < num_containers {
                            let pattern_ptr = if !lp_mem.is_null() {
                                *lp_mem.add(v1)
                            } else {
                                core::ptr::null()
                            };

                            if SppMatchWildcardString(h_client_container_token, pattern_ptr) != 0 {
                                is_authorized = 1;
                                break;
                            }

                            idx += 1;
                            v1 += 1; // standard array indexing stride for pointer array
                        }
                    }

                    if !pbIsAuthorized.is_null() {
                        *pbIsAuthorized = is_authorized;
                    }
                }
            } else {
                HandleSubsystemError(hr);
                ProcessReloc_Rva36_Len3(&stru_140439078, &dword_1404686B4);
                v2 = v20;
            }
        }
    }

    LogTraceEvent(hr);
    SppVectorReallocateHeap(&mut v16, 0);

    let v10 = lp_mem as *mut c_void;
    if !v10.is_null() {
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, v10);
        ProcessReloc_Rva33_Len3(&stru_140439BD8, &dword_140469310);
    }

    if v2 != 0 {
        let v12 = GetProcessHeap();
        ProcessReloc_Rva30_Len0(&stru_14044FC88, &dword_1404623F0);
        HeapFree(v12, 0, (v2 - 4) as *mut c_void);
        LogTraceEvent(0);
    }

    ProcessReloc_Rva30_Len0(&stru_14043BC70, &dword_14046A718);

    if h_client_container_token != 0 {
        let v13 = GetProcessHeap();
        HeapFree(v13, 0, (h_client_container_token - 4) as *mut c_void);
        LogTraceEvent(0);
    }

    hr
}