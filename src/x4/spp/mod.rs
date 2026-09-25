use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{wcsncmp, x4_acquiresrwlockexclusive, x4_getlasterror, x4_getprocessheap, x4_heapfree, x4_localfree, x4_releasesrwlockexclusive, x4_checktokenmembership, x4_allocateandinitializesid, x4_freesid, x4_bcryptfinishhash, x4_lcmapstringw, x4_bcryptdestroyhash, x4_bcryptclosealgorithmprovider, x4_bcrypthashdata, x4_bcryptopenalgorithmprovider, x4_bcryptcreatehash};
use crate::x4::globals::{_guard_check_icall_fptr, pManager, pQueueMgr, GlobalPtrTelemetryContext};
use crate::x4::misc::SwapVectorBuffersAndFreeOrphans;
use crate::x4::ops::HIDWORD;
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva30_Len0,
                       ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0, ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31,
                       ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36};
use crate::x4::spp::calls::SppGetComponentInterface;
use crate::x4::spp::pointer::{PointerVectorPushBack, PointerVectorResize};
use crate::x4::spp::server::SppValidateServerPropertiesToken;
use crate::x4::spp::string::{SppDuplicateString, SppDuplicateStringLocal, SppGetAndDuplicateString, SppGetStringByteLength, SppGuidFromString, SppSplitString, SppStringBuilderAppend, SppStringBuilderAppendFormat, SppStringCchLengthW, SppStringFromGuid};
use crate::x4::spp::telemetry::SppCheckTelemetryState;
use crate::x4::spp::thread::SppGetCurrentThreadContext;
use crate::x4::spp::timer::{SppLoadAndRegisterTimers, SppTeardownActiveTimer};
use crate::x4::threading::InvalidateTrackedObjects;
use crate::x4::types::{ISppLicenseComponent, ISppLicenseComponentVtbl, ItemManager, PointerVector, ShaderManager, SppBindingContext, SppCryptoContext, SppRearmContext, SID_IDENTIFIER_AUTHORITY, HLOCAL, SppStringBuilder, GUID, SppTelemetryContext};

pub mod lock;
pub mod vector;
pub mod query;
pub mod queue;
pub mod registry;
pub mod safe;
pub mod security;
pub mod sequence;
pub mod server;
pub mod calls;
pub mod string;
pub mod telemetry;
pub mod thread;
pub mod timer;
pub mod format;
pub mod tokenizer;
pub mod subsystem;
pub mod pointer;

// OUTPUT MAY BE INCORRECT, should use spp-related context, not dxgi, although dxgi context is structurally correct. [dxgi -> (global)context]
pub unsafe fn SppIsSubsystemInitialized() -> i64 {
    let mut unk_config_flags_high: u32;
    let mut v1: Option<
        unsafe fn(
            *mut *mut core::ffi::c_void,
            Option<unsafe fn(*mut ItemManager)>,
            *mut ShaderManager,
        ),
    >;

    unk_config_flags_high = HIDWORD!(pQueueMgr.unk_config_flags) as u32;
    if HIDWORD!(pQueueMgr.unk_config_flags) == 0 {
        if pQueueMgr.status_flag != 0 {
            x4_acquiresrwlockexclusive(&mut pQueueMgr.lock);
            if !pQueueMgr.sub_object_1.is_null() {
                unk_config_flags_high = HIDWORD!(pQueueMgr.unk_config_flags) as u32;
                // LABEL_5:
                x4_releasesrwlockexclusive(&mut pQueueMgr.lock);
                return unk_config_flags_high as i64;
            }

            pQueueMgr.sub_object_1 = core::ptr::null_mut();

            v1 = core::mem::transmute(qword_14046B380);
            if qword_14046B380 == 0 {
                v1 = core::mem::transmute(qword_14046B410);
            }

            if let Some(func) = v1 {
                _guard_check_icall_fptr();
                func(
                    &mut pQueueMgr.sub_object_1 as *mut _ as *mut *mut core::ffi::c_void,
                    Some(InvalidateTrackedObjects),
                    &mut pQueueMgr as *mut _ as *mut ShaderManager,
                );
            }

            if !pQueueMgr.sub_object_1.is_null() {
                unk_config_flags_high = 1;
                SET_HIDWORD(&mut pQueueMgr.unk_config_flags, 1);
                // goto LABEL_5;
                x4_releasesrwlockexclusive(&mut pQueueMgr.lock);
                return unk_config_flags_high as i64;
            }

            x4_releasesrwlockexclusive(&mut pQueueMgr.lock);
        }
        return 0;
    }

    unk_config_flags_high as i64
}

pub unsafe fn SppDecodeHexByte(
    pwszTwoHexChars: *const u16,
    pOutByte: *mut u8,
) -> i32 {
    let mut v2: i32; // ebx
    let mut v3: u8; // r8
    let mut v4: u32; // r10d
    let mut v5: u16; // r9
    let mut v6: i8; // r8

    v2 = 0;
    v3 = 0;
    v4 = 0;
    let mut pwszTwoHexChars = pwszTwoHexChars;

    loop {
        v5 = *pwszTwoHexChars;
        if v5.wrapping_sub(48) > 9 {
            break;
        }
        v6 = 16i8.wrapping_mul((v3 as i8).wrapping_add(13));

        // LABEL_8:
        v3 = (*(pwszTwoHexChars as *const u8)).wrapping_add(v6 as u8);
        v4 += 1;
        pwszTwoHexChars = pwszTwoHexChars.offset(1);
        if v4 >= 2 {
            *pOutByte = v3;
            // goto LABEL_11;
            LogTraceEvent(v2);
            return v2;
        }
    }

    if v5.wrapping_sub(65) <= 5 {
        v6 = (16 * (v3 as i8)).wrapping_sub(55);
        // goto LABEL_8;
        v3 = (*(pwszTwoHexChars as *const u8)).wrapping_add(v6 as u8);
        v4 += 1;
        pwszTwoHexChars = pwszTwoHexChars.offset(1);
        if v4 >= 2 {
            *pOutByte = v3;
            // goto LABEL_11;
            LogTraceEvent(v2);
            return v2;
        }
    } else if v5.wrapping_sub(97) <= 5 {
        v6 = (16 * (v3 as i8)).wrapping_sub(87);
        // goto LABEL_8;
        v3 = (*(pwszTwoHexChars as *const u8)).wrapping_add(v6 as u8);
        v4 += 1;
        pwszTwoHexChars = pwszTwoHexChars.offset(1);
        if v4 >= 2 {
            *pOutByte = v3;
            // goto LABEL_11;
            LogTraceEvent(v2);
            return v2;
        }
    } else {
        v2 = -2147024809; // 0x80070057 (E_INVALIDARG)
        HandleSubsystemError(-2147024809);
    }

    // LABEL_11:
    LogTraceEvent(v2);
    v2
}

/// Input | RID Enum                 | Binary SID | Identity
///
/// 18     SECURITY_LOCAL_SYSTEM_RID    S-1-5-18     LocalSystem
///
/// 19     SECURITY_LOCAL_SERVICE_RID   S-1-5-19     LocalService
///
/// 20     SECURITY_NETWORK_SERVICE_RID S-1-5-20     NetworkService
pub unsafe fn SppCheckWellKnownGroupMembership(
    unusedParam: i64,
    dwTargetRid: u32,
    pbIsMember: *mut i32,
) -> i32 {
    let mut v4: i32; // ebx
    let mut LastError: i32; // eax
    let mut IsMember: i32 = 0; // [rsp+60h] [rbp-20h] BYREF
    let mut SidToCheck: *mut core::ffi::c_void = core::ptr::null_mut(); // [rsp+68h] [rbp-18h] BYREF
    let mut pIdentifierAuthority: SID_IDENTIFIER_AUTHORITY = core::mem::zeroed(); // [rsp+70h] [rbp-10h] BYREF

    // *(_WORD *)&pIdentifierAuthority.Value[4] = 1280; (0x0500 - SECURITY_NT_AUTHORITY)
    let val_ptr = pIdentifierAuthority.Value.as_mut_ptr();
    *(val_ptr.offset(4) as *mut u16) = 1280u16.to_be();
    // *(_DWORD *)pIdentifierAuthority.Value = 0;
    *(val_ptr as *mut u32) = 0;

    IsMember = 0;
    v4 = 0;
    SidToCheck = core::ptr::null_mut();

    if pbIsMember.is_null() {
        v4 = -2147024809; // 0x80070057 (E_INVALIDARG)
        // LABEL_3:
        HandleSubsystemError(v4);
        // goto LABEL_11;
    } else {
        if x4_allocateandinitializesid(
            &mut pIdentifierAuthority,
            1,
            dwTargetRid,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut SidToCheck,
        ) == 0
            || x4_checktokenmembership(core::ptr::null_mut(), SidToCheck, &mut IsMember) == 0
        {
            LastError = x4_getlasterror() as i32;
            v4 = LastError as u32 as i32;
            if LastError != 0 {
                if LastError > 0 {
                    v4 = (((LastError as u32) & 0xFFFF) | 0x80070000) as i32;
                }
            } else {
                v4 = -2147467259; // 0x80004005 (E_FAIL)
            }
            // LABEL_3:
            HandleSubsystemError(v4);
        } else {
            *pbIsMember = IsMember;
        }
    }

    // LABEL_11:
    LogTraceEvent(v4);
    if !SidToCheck.is_null() {
        x4_freesid(SidToCheck);
    }
    v4
}

pub unsafe fn SppNormalizeLicensingId(
    pRawInputContext: i64,
    ppwszOutString: *mut *mut u16,
) -> i32 {
    let mut v3: i32; // eax
    let mut v4: i32; // edi
    let mut v5: *mut u16; // rbx
    let mut hMem: *mut u16 = core::ptr::null_mut(); // [rsp+40h] [rbp+18h] BYREF

    hMem = core::ptr::null_mut();
    v3 = SppConvertContextToIdString(pRawInputContext, &mut hMem);
    v4 = v3;

    if v3 >= 0 {
        v5 = core::ptr::null_mut();
        if !ppwszOutString.is_null() {
            *ppwszOutString = hMem;
        }
    } else {
        HandleSubsystemError(v3);
        v5 = hMem;
        v4 = -1073422330;
    }

    LogTraceEvent(v4);
    if !v5.is_null() {
        x4_localfree(v5 as HLOCAL);
    }

    v4
}

pub unsafe fn SppNormalizeStringProperty(
    pwszSource: *const u16,
    dwMapFlags: u32,
    ppwszOutString: *mut *mut u16,
) -> i32 {
    let mut lpDestStr: *mut u16 = core::ptr::null_mut(); // rbx
    let mut cchDest: i32 = 0; // ebp
    let mut v8: i32 = 0; // edi
    let mut v9: i32; // eax
    let mut v10: i32; // eax
    let mut v11: i32; // esi
    let mut LastError: i32; // eax
    let v13: *mut u16; // rax
    let ProcessHeap: *mut core::ffi::c_void; // rax
    let mut cchLength: u32 = 0; // [rsp+60h] [rbp+8h] BYREF
    let mut ppszDestination: *mut u16 = core::ptr::null_mut(); // [rsp+78h] [rbp+20h] BYREF

    lpDestStr = core::ptr::null_mut();
    ppszDestination = core::ptr::null_mut();
    cchDest = 0;
    v8 = 0;

    if !pwszSource.is_null() && *pwszSource != 0 {
        cchLength = 0;
        v9 = SppStringCchLengthW(pwszSource, cchLength as usize, 0x7FFFFFFF as *mut usize);
        v8 = v9;
        if v9 >= 0 {
            v10 = SppDuplicateString(pwszSource, cchLength, &mut ppszDestination);
            v8 = v10;
            if v10 < 0 {
                HandleSubsystemError(v10);
            }
            lpDestStr = ppszDestination;
        } else {
            HandleSubsystemError(v9);
        }
        LogTraceEvent(v8);
        if v8 < 0 {
            // goto LABEL_9;
            HandleSubsystemError(v8);
        } else {
            if !lpDestStr.is_null() {
                v11 = *(lpDestStr.cast::<i32>().offset(-1));
            } else {
                v11 = 0;
            }
            LogTraceEvent(0);
            v8 = 0;
            if v11 < 0 {
                v8 = -2147024362; // 0x80070216
                HandleSubsystemError(-2147024362);
            }
            LogTraceEvent(v8);
            if v8 >= 0 {
                cchDest = v11;
            } else {
                HandleSubsystemError(v8);
            }
            LogTraceEvent(v8);
            if v8 < 0 {
                // goto LABEL_9;
                HandleSubsystemError(v8);
            } else {
                if x4_lcmapstringw(0x400, dwMapFlags, lpDestStr, cchDest, lpDestStr, cchDest) == 0 {
                    LastError = x4_getlasterror() as i32;
                    v8 = LastError;
                    if LastError != 0 {
                        if LastError > 0 {
                            v8 = ((LastError as u32 & 0xFFFF) | 0x80070000) as i32;
                        }
                    } else {
                        v8 = -2147467259; // 0x80004005 (E_FAIL)
                    }
                    // LABEL_9:
                    HandleSubsystemError(v8);
                } else {
                    v13 = lpDestStr;
                    lpDestStr = core::ptr::null_mut();
                    if !ppwszOutString.is_null() {
                        *ppwszOutString = v13;
                    }
                }
            }
        }
    } else {
        v13 = lpDestStr;
        lpDestStr = core::ptr::null_mut();
        if !ppwszOutString.is_null() {
            *ppwszOutString = v13;
        }
    }

    // LABEL_25:
    LogTraceEvent(v8);
    if !lpDestStr.is_null() {
        ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, lpDestStr.offset(-2) as *mut core::ffi::c_void);
        LogTraceEvent(0);
    }

    v8
}

pub unsafe fn SppPrepareComponentParameters(
    a1: i64,
    pConfigTarget: *mut core::ffi::c_void,
    a3: i64,
    pComponentMeta: *mut i64,
    pOutVector: *mut PointerVector,
) -> i64 {
    let v5: bool;
    let mut v6: *mut core::ffi::c_void;
    let mut v7: u32;
    let mut v8: i32;
    let mut v9: *mut u32 = core::ptr::null_mut();
    let mut v10: *mut u32 = core::ptr::null_mut();
    let mut v11: i32;
    let mut v12: i32;
    let mut elements: *mut *mut core::ffi::c_void;
    let mut ProcessHeap: *mut core::ffi::c_void;
    let mut v15: i32;
    let v16: *mut *mut core::ffi::c_void;
    let v17: *mut core::ffi::c_void;
    let v18: *mut core::ffi::c_void;
    let v19: *mut u16;
    let v20: *mut core::ffi::c_void;
    let v21: *mut *mut core::ffi::c_void;
    let v22: *mut core::ffi::c_void;
    let v23: *mut *mut core::ffi::c_void;
    let v24: *mut core::ffi::c_void;

    let mut Src: SppStringBuilder = core::mem::zeroed();
    let mut v27: PointerVector = core::mem::zeroed();
    let mut ppszDestinationString: *mut u16 = core::ptr::null_mut();
    let mut a1a: PointerVector = core::mem::zeroed();

    let v30: i64 = a1;
    let v31: i64 = a3;
    let v32: *mut PointerVector = pOutVector;

    let mut v33: [u32; 2] = [5177423, 66];
    let mut v34: [u32; 2] = [5177423, 84];
    let mut v35: [u32; 2] = [5177422, 4522062];
    let _v36: i16 = 0;
    let mut v37: [u32; 4] = [4259926, 4784204, 4784196, 5832788];
    let _v38: i16 = 0;
    let v39: [u32; 14] = [
        7536749, 7602278, 7536698, 3080300, 6881396, 6619245, 3080306,
        7536677, 2424879, 3080307, 7536677, 2424879, 115, 0,
    ];

    v5 = *(pComponentMeta.cast::<i32>().offset(3)) == 2;
    v6 = core::ptr::null_mut();
    Src.totalCapacity = 0;
    v27.capacity = 0;
    v27.elements = core::ptr::null_mut();
    a1a.capacity = 0;
    a1a.elements = core::ptr::null_mut();
    ppszDestinationString = core::ptr::null_mut();
    Src.elements = core::ptr::null_mut();

    if v5 {
        v9 = *pComponentMeta.offset(8) as *mut u32;
        v10 = v34.as_mut_ptr();
    } else if *(pComponentMeta.cast::<i32>().offset(3)) == 6 {
        v10 = v33.as_mut_ptr();
        v9 = v35.as_mut_ptr();
    } else {
        if *(pComponentMeta.cast::<i32>().offset(3)) != 9 {
            v7 = -2147024809i32 as u32; // 0x80070057 (E_INVALIDARG)
            v8 = -2147024809;
            // LABEL_5:
            HandleSubsystemError(v8);
            // goto LABEL_24;
        } else {
            v9 = *pComponentMeta.offset(9) as *mut u32;
            v10 = v37.as_mut_ptr();

            v11 = SppSplitString(pConfigTarget as *const u16, 0x3B, &mut v27);
            v7 = v11 as u32;
            if v11 < 0 {
                // LABEL_10:
                v8 = v11;
                HandleSubsystemError(v8);
            } else {
                if !v9.is_null() && v27.size > 0 {
                    v12 = 0;
                    elements = v27.elements;
                    loop {
                        if !v6.is_null() {
                            ProcessHeap = x4_getprocessheap();
                            x4_heapfree(ProcessHeap, 0, v6);
                            Src.totalCapacity = 0;
                        }
                        Src.elements = core::ptr::null_mut();
                        v15 = SppStringBuilderAppendFormat(
                            &mut Src as &mut SppStringBuilder,
                            v39.as_ptr(),
                            v10,
                            v30,
                            *elements.offset(v12 as isize),
                            v9,
                        );
                        v7 = v15 as u32;
                        if v15 < 0 {
                            HandleSubsystemError(v15);
                            v6 = Src.capacity as *mut core::ffi::c_void;
                            break;
                        }
                        if v31 != 0 {
                            v15 = SppStringBuilderAppend(&mut Src, v31);
                            v7 = v15 as u32;
                            if v15 < 0 {
                                HandleSubsystemError(v15);
                                v6 = Src.capacity as *mut core::ffi::c_void;
                                break;
                            }
                        }
                        v6 = Src.capacity as *mut core::ffi::c_void;
                        v11 = SppGetAndDuplicateString(
                            Src.capacity as *const u16,
                            &mut ppszDestinationString,
                        );
                        v7 = v11 as u32;
                        if v11 < 0 {
                            v8 = v11;
                            HandleSubsystemError(v8);
                            break;
                        }
                        v11 = PointerVectorPushBack(
                            &mut a1a,
                            &mut ppszDestinationString as *mut _ as *mut *mut core::ffi::c_void,
                        );
                        v7 = v11 as u32;
                        if v11 < 0 {
                            v8 = v11;
                            HandleSubsystemError(v8);
                            break;
                        }
                        v12 += 1;
                        if v12 >= v27.size {
                            // LABEL_22:
                            Src.capacity = 0;
                            Src.elements = core::ptr::null_mut();
                            SwapVectorBuffersAndFreeOrphans(&mut Src.capacity, &mut a1a.capacity);
                            SwapVectorBuffersAndFreeOrphans(&mut Src.capacity, &mut (*v32).capacity);
                            PointerVectorResize(&mut Src, 0);
                            v16 = Src.elements;
                            if !Src.elements.is_null() {
                                v17 = x4_getprocessheap();
                                x4_heapfree(v17, 0, v16 as *mut core::ffi::c_void);
                            }
                            break;
                        }
                    }
                } else {
                    // LABEL_22:
                    Src.capacity = 0;
                    Src.elements = core::ptr::null_mut();
                    SwapVectorBuffersAndFreeOrphans(&mut Src.capacity, &mut a1a.capacity);
                    SwapVectorBuffersAndFreeOrphans(&mut Src.capacity, &mut (*v32).capacity);
                    PointerVectorResize(&mut Src, 0);
                    v16 = Src.elements;
                    if !Src.elements.is_null() {
                        v17 = x4_getprocessheap();
                        x4_heapfree(v17, 0, v16 as *mut core::ffi::c_void);
                    }
                }
            }
        }
    }

    // LABEL_24:
    LogTraceEvent(v7 as i32);
    if !v6.is_null() {
        v18 = x4_getprocessheap();
        x4_heapfree(v18, 0, v6);
    }
    if !ppszDestinationString.is_null() {
        v19 = ppszDestinationString.offset(-2);
        v20 = x4_getprocessheap();
        x4_heapfree(v20, 0, v19 as *mut core::ffi::c_void);
        LogTraceEvent(0);
    }
    PointerVectorResize(&mut a1a, 0);
    v21 = a1a.elements;
    if !a1a.elements.is_null() {
        v22 = x4_getprocessheap();
        x4_heapfree(v22, 0, v21 as *mut core::ffi::c_void);
    }
    PointerVectorResize(&mut v27, 0);
    v23 = v27.elements;
    if !v27.elements.is_null() {
        v24 = x4_getprocessheap();
        x4_heapfree(v24, 0, v23 as *mut core::ffi::c_void);
    }

    v7 as i64
}

pub unsafe fn SppHashAndSerializeLicensingId(
    pRawTokenSource: i64,
    ppwszOutGuidString: *mut *mut u16,
) -> i32 {
    let mut v2: *mut u16 = core::ptr::null_mut(); // rbx
    let mut StringByteLength: i32; // eax
    let mut v5: *mut u8 = core::ptr::null_mut(); // rdi
    let mut v6: i32; // esi
    let mut v7: i32; // ecx
    let mut v8: i32; // eax
    let mut v9: i32; // eax
    let mut ProcessHeap: *mut core::ffi::c_void; // rax
    let mut v11: *mut core::ffi::c_void; // rax

    let mut pcbOutBytes: u32 = 0; // [rsp+20h] [rbp-60h] BYREF
    let mut ppwszOutString: *mut u16 = core::ptr::null_mut(); // [rsp+28h] [rbp-58h] BYREF
    let mut pCryptoCtx: SppCryptoContext = core::mem::zeroed(); // [rsp+30h] [rbp-50h] BYREF
    let mut pGuid: GUID = core::mem::zeroed(); // [rsp+48h] [rbp-38h] BYREF
    let mut pbOutput: [u8; 16] = [0; 16]; // [rsp+58h] [rbp-28h] BYREF
    let v18: GUID = core::mem::zeroed(); // [rsp+68h] [rbp-18h]

    v2 = core::ptr::null_mut();
    pCryptoCtx.isHashActive = 0;
    *(pGuid.Data1_mut()) = 0;
    pcbOutBytes = 0;
    ppwszOutString = core::ptr::null_mut();
    pCryptoCtx.hAlgProvider = core::ptr::null_mut();

    StringByteLength = SppNormalizeProductKey(pRawTokenSource as *mut c_void, &mut pGuid);
    v5 = *(pGuid.Data1_ptr());
    v6 = StringByteLength;

    if StringByteLength < 0 {
        v7 = StringByteLength;
        // LABEL_3:
        HandleSubsystemError(v7);
    } else {
        StringByteLength = SppGetStringByteLength(
            *(pGuid.Data1_ptr() as *const *const u16),
            &mut pcbOutBytes,
        );
        v6 = StringByteLength;
        if StringByteLength < 0 {
            v7 = StringByteLength;
            // LABEL_3:
            HandleSubsystemError(v7);
        } else {
            StringByteLength = SppUpdateCryptoHash(&mut pCryptoCtx, v5, pcbOutBytes);
            v6 = StringByteLength;
            if StringByteLength < 0 {
                v7 = StringByteLength;
                // LABEL_3:
                HandleSubsystemError(v7);
            } else {
                v6 = 0;
                if pCryptoCtx.isHashActive != 0 {
                    v8 = x4_bcryptfinishhash(
                        pCryptoCtx.hHashInstance,
                        pbOutput.as_mut_ptr(),
                        0x20,
                        0,
                    );
                    v6 = v8;
                    if v8 < 0 {
                        HandleSubsystemError(v8);
                    }
                }
                LogTraceEvent(v6);
                if v6 < 0 {
                    v7 = v6;
                    // LABEL_3:
                    HandleSubsystemError(v7);
                } else {
                    pGuid = v18;
                    if !ppwszOutGuidString.is_null() {
                        v9 = SppStringFromGuid(&mut pGuid, &mut ppwszOutString);
                        v6 = v9;
                        if v9 >= 0 {
                            *ppwszOutGuidString = ppwszOutString;
                        } else {
                            HandleSubsystemError(v9);
                            v2 = ppwszOutString;
                        }
                    }
                }
            }
        }
    }

    // LABEL_15:
    LogTraceEvent(v6);
    if !v5.is_null() {
        ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, v5.offset(-4) as *mut c_void);
        LogTraceEvent(0);
    }
    if SppCheckTelemetryState(&mut GlobalPtrTelemetryContext as *mut SppTelemetryContext) != 0 {
        pCryptoCtx.isHashActive = 0;
        if !pCryptoCtx.hHashInstance.is_null() {
            x4_bcryptdestroyhash(pCryptoCtx.hHashInstance);
            pCryptoCtx.hHashInstance = core::ptr::null_mut();
        }
        if !pCryptoCtx.hAlgProvider.is_null() {
            x4_bcryptclosealgorithmprovider(pCryptoCtx.hAlgProvider, 0);
            pCryptoCtx.hAlgProvider = core::ptr::null_mut();
        }
    }
    if !v2.is_null() {
        v11 = x4_getprocessheap();
        x4_heapfree(v11, 0, v2.offset(-2) as *mut c_void);
        LogTraceEvent(0);
    }

    v6
}

pub unsafe fn SppNormalizeProductKey(
    pRawTokenSource: *mut c_void,
    ppwszOutPidString: *mut *mut u16,
) -> i32 {
    let mut v3: i32; // eax
    let mut v4: i32; // esi
    let mut v5: *mut u16; // rbx
    let mut v6: i32; // eax
    let mut v7: i32; // ecx
    let mut v8: *mut u16; // rax
    let ProcessHeap: *mut core::ffi::c_void; // rax
    let mut cchMax: u64 = 0; // [rsp+60h] [rbp+18h] BYREF
    let mut pszString: *mut u16 = core::ptr::null_mut(); // [rsp+68h] [rbp+20h]

    pszString = core::ptr::null_mut();
    cchMax = 0;

    v3 = SppNormalizeStringProperty(pRawTokenSource as *const u16, 0x200, &mut pszString);
    v4 = v3;

    if v3 < 0 {
        HandleSubsystemError(v3);
        v5 = pszString;
        // goto LABEL_11;
    } else {
        v5 = pszString;
        v6 = SppStringCchLengthW(pszString, &mut cchMax as *mut _ as usize, 0x7FFFFFFF as *mut usize);
        v4 = v6;

        if v6 < 0 {
            v7 = v6;
            // LABEL_5:
            HandleSubsystemError(v7);
            // goto LABEL_11;
        } else if (cchMax as u32) != 23 {
            v4 = -1073418160; // 0xC004F050
            v7 = -1073418160;
            // LABEL_5:
            HandleSubsystemError(v7);
            // goto LABEL_11;
        } else {
            let oem_str: [u16; 4] = [0x004F, 0x0045, 0x004D, 0x0000]; // L"OEM"
            if wcsncmp(v5.offset(20), oem_str.as_ptr(), 3) != 0 {
                *(v5.cast::<u32>().offset(10)) = 3145776;
                *(v5.offset(22)) = 48; // '0'
            }
            v8 = v5;
            v5 = core::ptr::null_mut();
            if !ppwszOutPidString.is_null() {
                *ppwszOutPidString = v8;
            }
        }
    }

    // LABEL_11:
    LogTraceEvent(v4);
    if !v5.is_null() {
        ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, v5.offset(-2) as *mut core::ffi::c_void);
        LogTraceEvent(0);
    }

    v4
}

pub unsafe fn SppConvertContextToIdString(
    pRawContext: i64,
    ppwszOutString: *mut *mut u16,
) -> i32 {
    let mut v3: *mut u16 = core::ptr::null_mut(); // rbx
    let mut v4: i32; // eax
    let mut v5: *mut u16; // rdi
    let mut v6: i32; // esi
    let mut v7: i32; // eax
    let ProcessHeap: *mut core::ffi::c_void; // rax
    let mut ppwszDestination: *mut u16 = core::ptr::null_mut(); // [rsp+50h] [rbp+18h] BYREF
    let mut pwszSource: *mut u16 = core::ptr::null_mut(); // [rsp+58h] [rbp+20h] BYREF

    pwszSource = core::ptr::null_mut();
    v3 = core::ptr::null_mut();
    ppwszDestination = core::ptr::null_mut();

    v4 = SppStringFromGuid(pRawContext as *const GUID, &mut pwszSource);
    v5 = pwszSource;
    v6 = v4;

    if v4 >= 0 {
        v7 = SppDuplicateStringLocal(pwszSource, &mut ppwszDestination);
        v6 = v7;
        if v7 >= 0 {
            if !ppwszOutString.is_null() {
                *ppwszOutString = ppwszDestination;
            }
        } else {
            HandleSubsystemError(v7);
            v3 = ppwszDestination;
        }
    } else {
        HandleSubsystemError(v4);
    }

    LogTraceEvent(v6);
    if !v3.is_null() {
        x4_localfree(v3 as *mut core::ffi::c_void);
    }
    if !v5.is_null() {
        ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, v5.offset(-2) as *mut core::ffi::c_void);
        LogTraceEvent(0);
    }

    v6
}

pub unsafe fn SppCheckBuiltinAdminMembership(
    unusedParam: i64,
    pbIsAdmin: *mut i32,
) -> i32 {
    let mut v3: i32; // ebx
    let mut LastError: i32; // eax
    let mut IsMember: i32 = 0; // [rsp+60h] [rbp-20h] BYREF
    let mut SidToCheck: *mut core::ffi::c_void = core::ptr::null_mut(); // [rsp+68h] [rbp-18h] BYREF
    let mut pIdentifierAuthority: SID_IDENTIFIER_AUTHORITY = core::mem::zeroed(); // [rsp+70h] [rbp-10h] BYREF

    // *(_WORD *)&pIdentifierAuthority.Value[4] = 1280; (0x0500 - SECURITY_NT_AUTHORITY)
    let val_ptr = pIdentifierAuthority.Value.as_mut_ptr();
    *(val_ptr.offset(4) as *mut u16) = 1280u16.to_be();
    // *(_DWORD *)pIdentifierAuthority.Value = 0;
    *(val_ptr as *mut u32) = 0;

    IsMember = 0;
    v3 = 0;
    SidToCheck = core::ptr::null_mut();

    if pbIsAdmin.is_null() {
        v3 = -2147024809; // 0x80070057 (E_INVALIDARG)
        // LABEL_3:
        HandleSubsystemError(v3);
        // goto LABEL_11;
    } else {
        // 0x20u = SECURITY_BUILTIN_DOMAIN_RID, 0x220u = DOMAIN_ALIAS_RID_ADMINS
        if x4_allocateandinitializesid(
            &mut pIdentifierAuthority,
            2,
            0x20,
            0x220,
            0,
            0,
            0,
            0,
            0,
            0,
            &mut SidToCheck,
        ) == 0
            || x4_checktokenmembership(core::ptr::null_mut(), SidToCheck, &mut IsMember) == 0
        {
            LastError = x4_getlasterror() as i32;
            v3 = LastError;
            if LastError != 0 {
                if LastError > 0 {
                    v3 = (((LastError as u32) & 0xFFFF) | 0x80070000) as i32;
                }
            } else {
                v3 = -2147467259; // 0x80004005 (E_FAIL)
            }
            // LABEL_3:
            HandleSubsystemError(v3);
        } else {
            *pbIsAdmin = IsMember;
        }
    }

    // LABEL_11:
    LogTraceEvent(v3);
    if !SidToCheck.is_null() {
        x4_freesid(SidToCheck);
    }
    v3 as i32
}

pub unsafe fn SppExecuteRearmSequence(
    pRearmCtx: *mut SppRearmContext,
    hNotificationContext: i64,
    notificationFlags: u32,
) -> i32 {
    let mut v6: i32;
    let mut v7: i32 = 0;
    let mut v9: i32;
    let mut v11: i32;
    let mut v12: i32;
    let mut v15: i32;
    let mut v17: i32 = -1;
    let mut v19: i32;
    let mut v20: i32;
    let mut v23: i32;
    let mut v38: i32;
    let mut v39: i32;
    let mut v42: i32;
    let mut v43: i32;

    let hLicenseContext = (*pRearmCtx).hLicenseContext;
    let componentsCount = (*pRearmCtx).componentsCount;

    let mut a1: [i64; 2] = [hLicenseContext, 0];
    let mut v50: GUID = core::mem::zeroed();
    let mut v52: GUID = core::mem::zeroed();

    if !(*pRearmCtx).pwszSkuId.is_null() || (*pRearmCtx).isNotificationRequired == 0 {
        v6 = 0;
    } else {
        v6 = 1;
    }
    let v46 = v6;

    let v10 = qword_14046B880;
    let v8_vtbl = *(v10 as *const *const usize);
    let v8: unsafe fn(i64, i64) -> i32 =
        core::mem::transmute(*v8_vtbl.offset(7)); // offset 56LL -> 7 pointers

    _guard_check_icall_fptr();
    v9 = v8(v10, hLicenseContext);
    v11 = v9;

    if v9 < 0 {
        v12 = v9;
        // LABEL_90
        HandleSubsystemError(v12);
        LogTraceEvent(v11);
        return v11;
    }

    let v51 = componentsCount;
    if componentsCount > 0 {
        let mut v13: usize = 0;
        let mut remaining = componentsCount;
        loop {
            let pComponentsArray = (*pRearmCtx).pComponentsArray;
            let component = &*pComponentsArray.add(v13);
            if component.isTimerEnabled != 0 && component.isActive != 0 {
                v15 = SppTeardownActiveTimer(component.pTimerContext as *const u16);
                if v15 < 0
                    && v15 != -1073425643
                    && v15 != -1073425660
                    && v7 >= 0
                {
                    v7 = v15;
                }
                LogTraceEvent(0);
            }
            v13 += 1;
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }

    let mut pwszAppId = (*pRearmCtx).pwszAppId;
    v50 = xmmword_1403E5A50;

    if pwszAppId.is_null() {
        v11 = -2147024809; // 0x80070057
        v20 = -2147024809;
        HandleSubsystemError(v20);
    } else {
        let mut v18: usize = 0;
        while *pwszAppId.add(v18) != 0 {
            v18 += 1;
        }

        if v18 == 38 {
            if *pwszAppId == 123 && *pwszAppId.add(37) == 125 { // '{' and '}'
                pwszAppId = pwszAppId.add(1);
                v19 = SppGuidFromString(pwszAppId, &mut v50);
                v11 = v19;
                if v19 < 0 {
                    HandleSubsystemError(v19);
                }
            } else {
                v11 = -2147024809;
                HandleSubsystemError(-2147024809);
            }
        } else if v18 == 36 {
            v19 = SppGuidFromString(pwszAppId, &mut v50);
            v11 = v19;
            if v19 < 0 {
                HandleSubsystemError(v19);
            }
        } else {
            v11 = -2147024809;
            HandleSubsystemError(-2147024809);
        }
    }

    LogTraceEvent(v11);
    if v11 == -2147024809 {
        v11 = -1073422330;
    } else if v11 < 0 {
        HandleSubsystemError(v11);
    }
    LogTraceEvent(v11);

    if v11 < 0 {
        v12 = v11;
        HandleSubsystemError(v12);
        LogTraceEvent(v11);
        return v11;
    }

    let mut pwszSkuId = (*pRearmCtx).pwszSkuId;
    v52 = xmmword_1403E5A50;

    if !pwszSkuId.is_null() {
        let mut v22: usize = 0;
        while *pwszSkuId.add(v22) != 0 {
            v22 += 1;
        }

        let mut do_guid_conv = false;
        if v22 == 38 {
            if *pwszSkuId == 123 && *pwszSkuId.add(37) == 125 {
                pwszSkuId = pwszSkuId.add(1);
                do_guid_conv = true;
            }
        } else if v22 == 36 {
            do_guid_conv = true;
        }

        if do_guid_conv {
            v23 = SppGuidFromString(pwszSkuId, &mut v52);
            v11 = v23;
            if v23 < 0 {
                HandleSubsystemError(v23);
            }
        } else {
            v23 = -2147024809;
            v11 = -2147024809;
            HandleSubsystemError(v23);
        }

        LogTraceEvent(v11);
        if v11 == -2147024809 {
            v11 = -1073422330;
        } else if v11 < 0 {
            HandleSubsystemError(v11);
        }
        LogTraceEvent(v11);

        if v11 < 0 {
            v12 = v11;
            HandleSubsystemError(v12);
            LogTraceEvent(v11);
            return v11;
        }
    }

    SppValidateServerPropertiesToken(&mut v50, &mut v52);

    let v24 = qword_14046B840;
    let v25_vtbl = *(v24 as *const *const usize);
    let v25: unsafe fn(
        i64,
        *const u16,
        i64,
        i64,
        u64,
    ) -> i32 = core::mem::transmute(*v25_vtbl.offset(5)); // offset 40LL -> 5 pointers

    let notification_str: [u16; 39] = [
        0x006d, 0x0073, 0x0066, 0x0074, 0x003a, 0x0073, 0x0070, 0x0070, 0x002f, 0x006e,
        0x006f, 0x0074, 0x0069, 0x0066, 0x0069, 0x0063, 0x0061, 0x0074, 0x0069, 0x006f,
        0x006e, 0x0073, 0x002f, 0x0063, 0x006f, 0x006d, 0x006d, 0x006f, 0x006e, 0x002f,
        0x0073, 0x006c, 0x0072, 0x0065, 0x0061, 0x0072, 0x006d, 0x0000, 0x0000,
    ]; // L"msft:spp/notifications/common/slrearm"

    _guard_check_icall_fptr();
    v9 = v25(
        v24,
        notification_str.as_ptr(),
        hNotificationContext,
        1,
        0,
    );
    v11 = v9;

    let mut failed = v9 < 0;
    if !failed && !(*pRearmCtx).pwszSkuId.is_null() {
        v9 = SppLoadAndRegisterTimers(
            a1[0],
            (*(*pRearmCtx).pComponentsArray).pTimerContext as i64,
        );
        v11 = v9;
        if v9 != -1073418220 && v9 < 0 {
            failed = true;
        }
    }

    if failed {
        v12 = v9;
        HandleSubsystemError(v12);
        LogTraceEvent(v11);
        return v11;
    }

    let mut v26 = notificationFlags;
    if (notificationFlags & 1) != 0 && v46 == 0 {
        let mut v27 = 0;
        if componentsCount > 0 {
            let mut v28 = 0;
            let mut v29 = componentsCount;
            loop {
                if (*(*pRearmCtx).pComponentsArray.add(v28)).isActive != 0 {
                    v17 = v27;
                }
                v28 += 1;
                v27 += 1;
                v29 -= 1;
                if v29 == 0 {
                    break;
                }
            }
            v26 = notificationFlags;
        }
    }

    let v48 = v17 as i64;
    let mut v31 = v17 as i64;
    let mut v32: i64 = 0;
    let mut v33: usize = 0;

    if componentsCount > 0 {
        loop {
            let v34 = (*pRearmCtx).pComponentsArray;
            let record = &*v34.add(v33);
            if record.isTimerEnabled != 0 && record.isActive != 0 {
                let mut v35 = v26;
                a1[1] = record.pTimerContext as i64;
                if v32 != v31 {
                    v35 = v26 & 0xFFFFFFFE;
                }

                let pMgrObj = *(pManager.padding.as_ptr().offset(32) as *const usize);
                let v36 = pMgrObj as i64;
                let v37_vtbl = *(pMgrObj as *const *const usize);
                let v37: unsafe fn(i64, *mut [i64; 2], i32, u32) -> i32 =
                    core::mem::transmute(*v37_vtbl.offset(27)); // offset 216LL -> 27 pointers

                _guard_check_icall_fptr();
                v38 = v37(v36, &mut a1, 0x7FFFFFFF, v35);
                v39 = v38;
                if v38 < 0 {
                    HandleSubsystemError(v38);
                }
                LogTraceEvent(v39);
                if v39 < 0
                    && v39 != -1073425643
                    && v39 != -1073425660
                    && v7 >= 0
                {
                    v7 = v39;
                }
                LogTraceEvent(0);
                v31 = v48;
            }
            v26 = notificationFlags;
            v32 += 1;
            v33 += 1;
            if v32 >= v51 as i64 {
                break;
            }
        }
    }

    if v46 != 0 {
        a1[1] = 0;
        let pMgrObj = *(pManager.padding.as_ptr().offset(32) as *const usize);
        let v40 = pMgrObj as i64;
        let v41_vtbl = *(pMgrObj as *const *const usize);
        let v41: unsafe fn(i64, *mut [i64; 2], i32, u32) -> i32 =
            core::mem::transmute(*v41_vtbl.offset(27)); // offset 216LL -> 27 pointers

        _guard_check_icall_fptr();
        v42 = v41(v40, &mut a1, 0x7FFFFFFF, notificationFlags);
        v43 = v42;
        if v42 < 0 {
            HandleSubsystemError(v42);
        }
        LogTraceEvent(v43);
        if v43 < 0
            && v43 != -1073425643
            && v43 != -1073425660
            && v7 >= 0
        {
            v7 = v43;
        }
        LogTraceEvent(0);
    }

    v11 = v7;
    if v7 < 0 {
        v12 = v7;
        HandleSubsystemError(v12);
    }

    LogTraceEvent(v11);
    v11
}

pub unsafe fn SppUpdateCryptoHash(
    pCryptoCtx: *mut SppCryptoContext,
    pbData: *mut u8,
    cbData: u32,
) -> i32 {
    let p_hHashInstance = &mut (*pCryptoCtx).hHashInstance as *mut *mut core::ffi::c_void;
    let p_hAlgProvider = &mut (*pCryptoCtx).hAlgProvider as *mut *mut core::ffi::c_void;
    let mut status: i32;
    let mut v9: i32;

    if (*pCryptoCtx).isHashActive != 0 {
        status = x4_bcrypthashdata(*p_hHashInstance, pbData, cbData, 0);
        v9 = status;
        if status < 0 {
            HandleSubsystemError(status);
        }
    } else {
        let alg_id: [u16; 7] = [0x0053, 0x0048, 0x0041, 0x0032, 0x0035, 0x0036, 0x0000]; // L"SHA256"
        status = x4_bcryptopenalgorithmprovider(
            p_hAlgProvider,
            alg_id.as_ptr(),
            core::ptr::null_mut(),
            0,
        );
        v9 = status;

        if status >= 0 {
            status = x4_bcryptcreatehash(
                *p_hAlgProvider,
                p_hHashInstance,
                core::ptr::null_mut(),
                0,
                core::ptr::null_mut(),
                0,
                0,
            );
            v9 = status;

            if status >= 0 {
                (*pCryptoCtx).isHashActive = 1;
                status = x4_bcrypthashdata(*p_hHashInstance, pbData, cbData, 0);
                v9 = status;
                if status < 0 {
                    HandleSubsystemError(status);
                }
            } else {
                HandleSubsystemError(status);
            }
        } else {
            HandleSubsystemError(status);
        }
    }

    if (*pCryptoCtx).isHashActive == 0 {
        if !(*p_hAlgProvider).is_null() {
            x4_bcryptclosealgorithmprovider(*p_hAlgProvider, 0);
        }
        *p_hAlgProvider = core::ptr::null_mut();
        *p_hHashInstance = core::ptr::null_mut();
    }

    LogTraceEvent(v9);
    v9
}

pub unsafe fn SppCreateComponentInstance(
    hNamespaceProvider: i64,
    pBindingContext: *const SppBindingContext,
    ppOutComponent: *mut *mut ISppLicenseComponent,
) -> i32 {
    let mut v26: *mut ISppLicenseComponent = core::ptr::null_mut();
    let v3 = *(0x14046B830 as *const i64); // Global manager instance pointer

    let mut prop: BindingProperty = core::mem::zeroed();

    ProcessReloc_Rva36_Len0(&stru_140437828 as *const _ as *mut _, &dword_14046797C as *const _ as *mut _);

    let v7_vtbl = *(v3 as *const *const usize);
    let v7: unsafe fn(
        i64,
        i64,
        i64,
        *const core::ffi::c_void,
        *mut *mut ISppLicenseComponent,
    ) -> i32 = core::mem::transmute(*v7_vtbl.offset(5)); // offset 40LL -> 5th pointer

    ProcessReloc_Rva8_Len36(&stru_1404390F8 as *const _ as *mut _, &dword_14046882C as *const _ as *mut _);
    let v8 = SppGetCurrentThreadContext(pManager);

    _guard_check_icall_fptr();
    ProcessReloc_Rva33_Len3(&stru_14044B3E8 as *const _ as *mut _, &dword_140466A98 as *const _ as *mut _);

    let mut v9 = v7(
        v3,
        v8 as i64,
        hNamespaceProvider,
        &unk_1403E9730 as *const _ as *const core::ffi::c_void,
        &mut v26,
    );

    ProcessReloc_Rva28_Len0(&stru_140439BA8 as *const _ as *mut _, &dword_140469294 as *const _ as *mut _);

    if v9 >= 0 {
        if !pBindingContext.is_null() {
            let appId = (*pBindingContext).appId;
            ProcessReloc_Rva3_Len31(&stru_140441690 as *const _ as *mut _, &dword_140466D70 as *const _ as *mut _);

            if appId != 0 {
                let name_app_id: [u16; 16] = [
                    0x0053, 0x0070, 0x0070, 0x0042, 0x0069, 0x006E, 0x0064, 0x0069,
                    0x006E, 0x0067, 0x0041, 0x0070, 0x0070, 0x0049, 0x0064, 0x0000,
                ]; // L"SppBindingAppId"

                prop.name = name_app_id.as_ptr();
                ProcessReloc_Rva3_Len31(&stru_140450550 as *const _ as *mut _, &dword_1404616C8 as *const _ as *mut _);
                prop.value = appId;
                let v11 = v26;
                prop.prop_type = 2;

                let v12 = (*(*v11).lpVtbl).Unknown2[1];
                ProcessReloc_Rva36_Len3(&stru_14044B38C as *const _ as *mut _, &dword_1404669B8 as *const _ as *mut _);

                _guard_check_icall_fptr();
                let v13 = v12(v11, &mut prop as *mut _ as *mut core::ffi::c_void, 1);
                v9 = v13;
                if v13 < 0 {
                    HandleSubsystemError(v13);
                    LogTraceEvent(v9);
                    let v21 = v26;
                    if !v26.is_null() {
                        let Release = (*(*v26).lpVtbl).Release.unwrap();
                        _guard_check_icall_fptr();
                        Release(v21);
                    }
                    ProcessReloc_Rva33_Len3(&stru_14043BA58 as *const _ as *mut _, &dword_14046A438 as *const _ as *mut _);
                    return v9;
                }
            }

            let skuId = (*pBindingContext).skuId;
            if skuId != 0 {
                let name_sku_id: [u16; 16] = [
                    0x0053, 0x0070, 0x0070, 0x0042, 0x0069, 0x006E, 0x0064, 0x0069,
                    0x006E, 0x0067, 0x0053, 0x006B, 0x0075, 0x0049, 0x0064, 0x0000,
                ]; // L"SppBindingSkuId"

                prop.prop_type = 2;
                prop.name = name_sku_id.as_ptr();
                prop.value = skuId;

                ProcessReloc_Rva3_Len36(&dword_14043CD58 as *const _ as *mut _, &dword_140461568 as *const _ as *mut _);
                let v15 = v26;
                let v16 = (*(*v26).lpVtbl).Unknown2[1];

                _guard_check_icall_fptr();
                let v13 = v16(v15, &mut prop as *mut _ as *mut core::ffi::c_void, 1);
                v9 = v13;
                if v13 < 0 {
                    HandleSubsystemError(v13);
                    LogTraceEvent(v9);
                    let v21 = v26;
                    if !v26.is_null() {
                        let Release = (*(*v26).lpVtbl).Release.unwrap();
                        _guard_check_icall_fptr();
                        Release(v21);
                    }
                    ProcessReloc_Rva33_Len3(&stru_14043BA58 as *const _ as *mut _, &dword_14046A438 as *const _ as *mut _);
                    return v9;
                }
            }

            let pkeyId = (*pBindingContext).pkeyId;
            if pkeyId != 0 {
                let name_pkey_id: [u16; 17] = [
                    0x0053, 0x0070, 0x0070, 0x0042, 0x0069, 0x006E, 0x0064, 0x0069,
                    0x006E, 0x0067, 0x0050, 0x006B, 0x0065, 0x0079, 0x0049, 0x0064, 0x0000,
                ]; // L"SppBindingPkeyId"

                let v18 = v26;
                prop.name = name_pkey_id.as_ptr();
                prop.value = pkeyId;
                prop.prop_type = 2;
                let lpVtbl = (*v26).lpVtbl;

                ProcessReloc_Rva30_Len0(&stru_14044A9B8 as *const _ as *mut _, &dword_140465F84 as *const _ as *mut _);
                _guard_check_icall_fptr();
                ProcessReloc_Rva2_Len30(&stru_140439B18 as *const _ as *mut _, &dword_140469224 as *const _ as *mut _);

                let v13 = ((*lpVtbl).Unknown2[1])(v18, &mut prop as *mut _ as *mut core::ffi::c_void, 1);
                v9 = v13;
                if v13 < 0 {
                    HandleSubsystemError(v13);
                } else {
                    let v20 = v26;
                    v26 = core::ptr::null_mut();
                    if !ppOutComponent.is_null() {
                        *ppOutComponent = v20;
                    }
                }
            } else {
                let v20 = v26;
                v26 = core::ptr::null_mut();
                if !ppOutComponent.is_null() {
                    *ppOutComponent = v20;
                }
            }
        } else {
            let v20 = v26;
            v26 = core::ptr::null_mut();
            if !ppOutComponent.is_null() {
                *ppOutComponent = v20;
            }
        }
    } else {
        HandleSubsystemError(v9);
        ProcessReloc_Rva0_Len36(&stru_140449720 as *const _ as *mut _, &dword_1404616B4 as *const _ as *mut _);
    }

    LogTraceEvent(v9);
    let v21 = v26;
    if !v26.is_null() {
        let Release = (*(*v26).lpVtbl).Release.unwrap();
        _guard_check_icall_fptr();
        Release(v21);
    }

    ProcessReloc_Rva33_Len3(&stru_14043BA58 as *const _ as *mut _, &dword_14046A438 as *const _ as *mut _);
    v9
}

pub unsafe fn SppValidateLicenseComponentState(
    skuId: i64,
    pbIsValid: *mut i32,
) -> i32 {
    let mut result: i32;
    let mut v5: *mut ISppLicenseComponent;
    let mut v6: i32;
    let lpVtbl: *mut ISppLicenseComponentVtbl;
    let v8: i32;
    let v9: i32;
    let v10: *mut ISppLicenseComponentVtbl;
    let mut ppOutComponent: *mut ISppLicenseComponent = core::ptr::null_mut();

    ppOutComponent = core::ptr::null_mut();
    ProcessReloc_Rva28_Len0(&stru_14044F5B8 as *const _ as *mut _, &dword_14046A8E0 as *const _ as *mut _);

    result = SppGetComponentInterface(skuId, &mut ppOutComponent);
    v5 = ppOutComponent;
    v6 = result;

    if result < 0 {
        ProcessReloc_Rva36_Len3(&stru_140443608 as *const _ as *mut _, &dword_140469050 as *const _ as *mut _);
        HandleSubsystemError(v6);
    } else {
        lpVtbl = (*ppOutComponent).lpVtbl;
        ProcessReloc_Rva36_Len0(&stru_14043EFF8 as *const _ as *mut _, &dword_140463F18 as *const _ as *mut _);
        ProcessReloc_Rva0_Len36(&stru_140442260 as *const _ as *mut _, &dword_140467B6C as *const _ as *mut _);
        _guard_check_icall_fptr();
        ProcessReloc_Rva0_Len36(&stru_140440F98 as *const _ as *mut _, &dword_140466570 as *const _ as *mut _);

        v8 = ((*lpVtbl).ValidateState.unwrap())(v5) as i32;
        if v8 == -1073418220 { // 0xC004F014
            v9 = 0;
        } else {
            v6 = v8;
            if v8 < 0 {
                HandleSubsystemError(v8);
                ProcessReloc_Rva30_Len0(&stru_140448D38 as *const _ as *mut _, &dword_140463ECC as *const _ as *mut _);
                LogTraceEvent(v6);
                ProcessReloc_Rva3_Len31(&stru_1404426C8 as *const _ as *mut _, &dword_140467F80 as *const _ as *mut _);
                if !v5.is_null() {
                    ProcessReloc_Rva3_Len36(&dword_14043CB6C as *const _ as *mut _, &dword_140461154 as *const _ as *mut _);
                    v10 = (*v5).lpVtbl;
                    ProcessReloc_Rva2_Len30(&stru_1404381F0 as *const _ as *mut _, &dword_1404679B0 as *const _ as *mut _);
                    ProcessReloc_Rva33_Len3(&stru_1404456C0 as *const _ as *mut _, &dword_14046A3F8 as *const _ as *mut _);
                    _guard_check_icall_fptr();
                    ((*v10).Release.unwrap())(v5);
                }
                return v6;
            }
            v9 = 1;
            ProcessReloc_Rva36_Len3(&stru_1404366F8 as *const _ as *mut _, &dword_140465CA4 as *const _ as *mut _);
        }

        if !pbIsValid.is_null() {
            *pbIsValid = v9;
        }
        ProcessReloc_Rva8_Len36(&stru_1404411B0 as *const _ as *mut _, &dword_1404667E0 as *const _ as *mut _);
    }

    LogTraceEvent(v6);
    ProcessReloc_Rva3_Len31(&stru_1404426C8 as *const _ as *mut _, &dword_140467F80 as *const _ as *mut _);
    if !v5.is_null() {
        ProcessReloc_Rva3_Len36(&dword_14043CB6C as *const _ as *mut _, &dword_140461154 as *const _ as *mut _);
        v10 = (*v5).lpVtbl;
        ProcessReloc_Rva2_Len30(&stru_1404381F0 as *const _ as *mut _, &dword_1404679B0 as *const _ as *mut _);
        ProcessReloc_Rva33_Len3(&stru_1404456C0 as *const _ as *mut _, &dword_14046A3F8 as *const _ as *mut _);
        _guard_check_icall_fptr();
        ((*v10).Release.unwrap())(v5);
    }

    v6
}
