use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::x4_localfree;
use crate::x4::globals::{GlobalPtrSppNamespace, GlobalSppPacketControlFlags, _guard_check_icall_fptr};
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva30_Len0, ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0, ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31, ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36};
use crate::x4::spp::query::{SppQueryPropertyInternal, SppQuerySystemPolicyBits};
use crate::x4::spp::queue::SppQueueRegistrationPacket;
use crate::x4::spp::{SppCreateComponentInstance, SppIsSubsystemInitialized};
use crate::x4::spp::string::SppDuplicateStringLocal;
use crate::x4::types::{ISppLicenseComponent, ISppNamespace, SppBindingContext, SppTelemetryContext};

pub unsafe fn SppGetOrUpdateControlFlags(
    pTelemetryContext: *mut SppTelemetryContext,
    pOutControlFlags: *mut u32,
) -> *mut u32 {
    let mut queryStatus: i32 = 0;
    let mut policyMaskPayload: i64 = 0;

    // *(_QWORD *)pOutControlFlags = 0;
    *(pOutControlFlags as *mut u64) = 0;

    let mut firstByteFlags = GlobalSppPacketControlFlags[0] as u8;
    *pOutControlFlags = GlobalSppPacketControlFlags[0] as u32;

    if (firstByteFlags & 6) != 6 {
        let threadIdOrStatus = SppIsSubsystemInitialized(/*pTelemetryContext*/);
        queryStatus = 0;
        let activeStatusMetric = threadIdOrStatus;

        let dummyTracker: i64 = 0; // Uninitialized/passed value
        SppQuerySystemPolicyBits(dummyTracker, &mut (policyMaskPayload as u32), queryStatus as i64);

        let mut currentFlags = *pOutControlFlags;

        loop {
            let policyBits = policyMaskPayload as u16;
            let isPolicyOutputNull = queryStatus == 0;
            let mut targetNewFlags = currentFlags | 0x40000;
            *pOutControlFlags = currentFlags | 0x40000;

            if !isPolicyOutputNull && (currentFlags & 2) == 0 {
                targetNewFlags = ((policyBits & 0x9C1) as u32)
                    | (currentFlags & 0xFFFBF63E)
                    | 0x40000
                    | 2;
                *pOutControlFlags = targetNewFlags;
            }

            if (currentFlags & 4) == 0 {
                targetNewFlags = ((policyBits & 0x400) as u32)
                    | (targetNewFlags & 0xFFFFFBFF)
                    | 4;
                *pOutControlFlags = targetNewFlags;
            }

            let actualCompareFlags = core::sync::atomic::AtomicU32::from_ptr(
                GlobalSppPacketControlFlags.as_mut_ptr(),
            )
                .compare_exchange(
                    currentFlags,
                    targetNewFlags,
                    core::sync::atomic::Ordering::SeqCst,
                    core::sync::atomic::Ordering::SeqCst,
                )
                .unwrap_or_else(|x| x);

            if currentFlags == actualCompareFlags {
                break;
            }
            currentFlags = actualCompareFlags;
        }

        if (currentFlags & 4) == 0 {
            SppQueueRegistrationPacket(GlobalSppPacketControlFlags.as_mut_ptr(), 3, activeStatusMetric as i32);
        }

        if (*pOutControlFlags & 2) == 0 {
            *pOutControlFlags = ((policyMaskPayload & 0x9C1) as u32) | (*pOutControlFlags & 0xFFFFF63E);
        }
    }

    pOutControlFlags
}

pub unsafe fn SppGetComponentInterface(
    skuId: i64,
    ppOutComponent: *mut *mut ISppLicenseComponent,
) -> i32 {
    let mut v4: *mut ISppLicenseComponent = core::ptr::null_mut();
    let mut v16: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut v14: *mut ISppLicenseComponent = core::ptr::null_mut();

    ProcessReloc_Rva3_Len31(
        &stru_14044A588 as *const _ as *mut _,
        &dword_140465C18 as *const _ as *mut _,
    );

    let mut v15: [i64; 3] = [0, 0, 0];

    ProcessReloc_Rva28_Len0(
        (&stru_14044E028 as *const _ as *const u8).add(4) as *mut _,
        &dword_140469E78 as *const _ as *mut _,
    );

    let mut hMem: *mut core::ffi::c_void = core::ptr::null_mut();
    let v5 = GlobalPtrSppNamespace;

    ProcessReloc_Rva36_Len0(
        &stru_14044AD78 as *const _ as *mut _,
        &dword_1404662E0 as *const _ as *mut _,
    );

    let OpenNamespace = (*(*v5).lpVtbl).OpenNamespace;
    _guard_check_icall_fptr();

    let mut v7 = OpenNamespace(v5, skuId, &mut hMem);

    ProcessReloc_Rva36_Len0(
        &stru_140437230 as *const _ as *mut _,
        &dword_14046690C as *const _ as *mut _,
    );

    if v7 >= 0 {
        let v8 = GlobalPtrSppNamespace;
        let v9: unsafe fn(
            *mut ISppNamespace,
            *mut core::ffi::c_void,
            *mut *mut core::ffi::c_void,
        ) -> i32 = core::mem::transmute((*(*v8).lpVtbl)._Reserved1[7]);

        ProcessReloc_Rva2_Len30(
            &stru_140449B98 as *const _ as *mut _,
            &dword_140464FB0 as *const _ as *mut _,
        );
        _guard_check_icall_fptr();

        v7 = v9(v8, hMem, &mut v16);
        if v7 < 0 {
            ProcessReloc_Rva3_Len36(
                &dword_1404514C8 as *const _ as *mut _,
                &dword_1404631A8 as *const _ as *mut _,
            );
            HandleSubsystemError(v7);
        }
    } else {
        HandleSubsystemError(v7);
        ProcessReloc_Rva28_Len0(
            &stru_14044CD88 as *const _ as *mut _,
            &dword_1404688C8 as *const _ as *mut _,
        );
    }

    LogTraceEvent(v7);

    if !hMem.is_null() {
        x4_localfree(hMem);
    }

    if v7 >= 0 {
        v15[1] = skuId;
        ProcessReloc_Rva33_Len3(
            &stru_140450F18 as *const _ as *mut _,
            &dword_140462CEC as *const _ as *mut _,
        );

        let v10 = SppCreateComponentInstance(
            v16 as i64,
            v15.as_ptr() as *const SppBindingContext,
            &mut v14,
        );
        v7 = v10;

        if v10 >= 0 {
            let v11 = v14;
            ProcessReloc_Rva0_Len36(
                (&stru_140448948 as *const _ as *const u8).add(4) as *mut _,
                &dword_1404615CC as *const _ as *mut _,
            );
            if !ppOutComponent.is_null() {
                *ppOutComponent = v11;
            }
        } else {
            HandleSubsystemError(v10);
            ProcessReloc_Rva8_Len36(
                &stru_1404390A8 as *const _ as *mut _,
                &dword_140468700 as *const _ as *mut _,
            );
            v4 = v14;
        }
    } else {
        HandleSubsystemError(v7);
    }

    LogTraceEvent(v7);

    if !v16.is_null() {
        x4_localfree(v16);
        v16 = core::ptr::null_mut();
    }

    ProcessReloc_Rva30_Len0(
        (&stru_140439BC8 as *const _ as *const u8).add(4) as *mut _,
        &dword_1404692F4 as *const _ as *mut _,
    );

    if !v4.is_null() {
        let Release = (*(*v4).lpVtbl).Release;
        _guard_check_icall_fptr();
        ProcessReloc_Rva36_Len3(
            &stru_14044B080 as *const _ as *mut _,
            &dword_140461A90 as *const _ as *mut _,
        );
        Release(v4);
    }

    v7
}

pub unsafe fn SppGetProtectedEditionString(
    skuId: i64,
    pwszPropertyName: *const u16,
    ppwszOutString: *mut *mut u16,
) -> i32 {
    let mut v6: *mut u16 = core::ptr::null_mut();
    let mut hr: i32;
    let v8: *mut SppProperty;
    let v9: i32;
    let v10: *mut u16;
    let mut hMem: [*mut core::ffi::c_void; 5] = [core::ptr::null_mut(); 5];
    let mut ppwszDestination: *mut u16 = core::ptr::null_mut();

    hMem[0] = core::ptr::null_mut();
    ProcessReloc_Rva28_Len0(
        &stru_140449A98 as *const _ as *mut _,
        &dword_140464E9C as *const _ as *mut _,
    );
    v6 = core::ptr::null_mut();
    ppwszDestination = core::ptr::null_mut();

    hr = SppQueryPropertyInternal(skuId, pwszPropertyName, hMem.as_mut_ptr());

    ProcessReloc_Rva3_Len31(
        &stru_140447EBC as *const _ as *mut _,
        &dword_140462F18 as *const _ as *mut _,
    );
    ProcessReloc_Rva3_Len36(
        &stru_140442708 as *const _ as *mut _,
        &dword_140468040 as *const _ as *mut _,
    );

    v8 = hMem[0] as *mut SppProperty;

    if hr < 0 {
        HandleSubsystemError(hr);
    } else if (*v8).prop_type != 2 {
        hr = -2147418113; // E_UNEXPECTED (0x8000FFFF)
        HandleSubsystemError(hr);
    } else {
        ProcessReloc_Rva8_Len36(
            &stru_14044AED0 as *const _ as *mut _,
            &dword_1404663F4 as *const _ as *mut _,
        );

        v9 = SppDuplicateStringLocal((*v8).value, &mut ppwszDestination);
        hr = v9;

        if v9 >= 0 {
            v10 = ppwszDestination;
            ProcessReloc_Rva0_Len36(
                (&stru_1404456C0 as *const _ as *const u8).add(4) as *mut _,
                &dword_14046A410 as *const _ as *mut _,
            );
            ProcessReloc_Rva2_Len30(
                &stru_140435278 as *const _ as *mut _,
                &dword_140464AA4 as *const _ as *mut _,
            );
            if !ppwszOutString.is_null() {
                *ppwszOutString = v10;
            }
        } else {
            HandleSubsystemError(v9);
            ProcessReloc_Rva36_Len0(
                (&stru_140449CF8 as *const _ as *const u8).add(4) as *mut _,
                &dword_1404650A8 as *const _ as *mut _,
            );
            v6 = ppwszDestination;
        }
    }

    LogTraceEvent(hr);
    ProcessReloc_Rva33_Len3(
        &stru_140438218 as *const _ as *mut _,
        &dword_1404679B4 as *const _ as *mut _,
    );

    if !v6.is_null() {
        x4_localfree(v6 as *mut core::ffi::c_void);
        ProcessReloc_Rva30_Len0(
            &stru_14043F5B8 as *const _ as *mut _,
            &dword_140464690 as *const _ as *mut _,
        );
    }

    if !v8.is_null() {
        ProcessReloc_Rva3_Len36(
            &dword_14043ECB8 as *const _ as *mut _,
            &dword_140463BF8 as *const _ as *mut _,
        );
        x4_localfree(v8 as *mut core::ffi::c_void);
        ProcessReloc_Rva36_Len3(
            &stru_14043CC78 as *const _ as *mut _,
            &dword_140461398 as *const _ as *mut _,
        );
    }

    ProcessReloc_Rva30_Len0(
        &stru_140445CD8 as *const _ as *mut _,
        &dword_14046033C as *const _ as *mut _,
    );

    hr
}
