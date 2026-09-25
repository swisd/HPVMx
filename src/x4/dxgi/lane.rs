use crate::x4::error::{ReportAlignmentAssertionFailure, TraceProviderEvent};
use crate::x4::externals::{x4_acquiresrwlockexclusive, x4_closehandle, x4_etwwritetransfer, x4_getlasterror, x4_getprocessheap, x4_heapfree, x4_opensemaphorew, x4_releasesrwlockexclusive};
use crate::x4::helpers::StringCchCatW;
use crate::x4::threading::{CreateOrOpenSemaphoreW, HandleHandleCloseError, QueryAndValidateSemaphoreCount};
use crate::x4::types::{LaneSnapshotData, SharedContextBlock, HANDLE, PSRWLOCK, __int64, LONG};

pub unsafe fn SyncBuffers(pContextBlock: *mut SharedContextBlock) -> i64 {
    // Allocate localized contiguous blocks on the stack continuously
    // matching the exact _BYTE mapping boundaries
    let mut laneFrame0 = [0u8; 64];
    let mut v4 = [0u8; 64];
    let mut v5 = [0u8; 72];

    // InitContextInternalPools((struct SharedContextBlock *)laneFrame0);
    InitContextInternalPools(laneFrame0.as_mut_ptr() as *mut SharedContextBlock);

    // Acquire exclusive ownership via Slim Reader/Writer lock interface
    x4_acquiresrwlockexclusive(pContextBlock as PSRWLOCK);

    // 1. Process Lane Index 0
    let lane_0_ptr = core::ptr::addr_of!((*pContextBlock).lanes[0]);
    let check_byte_0 = *(core::ptr::addr_of!((*lane_0_ptr).reservedPool) as *const u8);
    if check_byte_0 != 0 {
        SwapLaneSnapshots(laneFrame0.as_mut_ptr() as *mut _, lane_0_ptr as *mut _);
    }

    // 2. Process Lane Index 1
    let lane_1_ptr = core::ptr::addr_of!((*pContextBlock).lanes[1]);
    let check_byte_1 = *(core::ptr::addr_of!((*lane_1_ptr).reservedPool) as *const u8);
    if check_byte_1 != 0 {
        SwapLaneSnapshots(v4.as_mut_ptr() as *mut _, lane_1_ptr as *mut _);
    }

    // 3. Process Lane Index 2
    let lane_2_ptr = core::ptr::addr_of!((*pContextBlock).lanes[2]);
    let check_byte_2 = *(core::ptr::addr_of!((*lane_2_ptr).reservedPool) as *const u8);
    if check_byte_2 != 0 {
        SwapLaneSnapshots(v5.as_mut_ptr() as *mut _, lane_2_ptr as *mut _);
    }

    // Release synchronization barrier
    x4_releasesrwlockexclusive(pContextBlock as PSRWLOCK);

    // Clean up frame metadata and free trailing allocations
    CleanupFrameTracking(laneFrame0.as_mut_ptr() as *mut _);
    FreeLanePayloadBuffers(laneFrame0.as_mut_ptr() as *mut _) as i64
}

pub unsafe fn SwapLaneSnapshots(
    pDestSlot: *mut LaneSnapshotData,
    pSrcSlot: *mut LaneSnapshotData,
) -> i8 {
    // 1. Take snapshot of destination properties to local registers/stack frames
    // tempPoolSnapshot0 = *(_OWORD *)&pDestSlot->dynamic_payload_bytes[24];
    let dest_bytes_ptr = (*pDestSlot).dynamic_payload_bytes.as_mut_ptr();
    let tempPoolSnapshot0 = *(dest_bytes_ptr.add(24) as *const u128);

    // tempPoolSnapshot1 = *(_QWORD *)&pDestSlot->dynamic_payload_bytes[40];
    let tempPoolSnapshot1 = *(dest_bytes_ptr.add(40) as *const i64);

    // oldDestBuffer = *(_QWORD *)&pDestSlot->dynamic_payload_bytes[48];
    let oldDestBuffer = *(dest_bytes_ptr.add(48) as *const i64);

    // *(_QWORD *)&pDestSlot->dynamic_payload_bytes[48] = 0;
    *(dest_bytes_ptr.add(48) as *mut i64) = 0;

    // 2. Transmit source properties over to destination targets
    let src_bytes_ptr = (*pSrcSlot).dynamic_payload_bytes.as_mut_ptr();

    // *(_OWORD *)&pDestSlot->dynamic_payload_bytes[24] = *(_OWORD *)&pSrcSlot->dynamic_payload_bytes[24];
    *(dest_bytes_ptr.add(24) as *mut u128) = *(src_bytes_ptr.add(24) as *const u128);

    // *(_QWORD *)&pDestSlot->dynamic_payload_bytes[40] = *(_QWORD *)&pSrcSlot->dynamic_payload_bytes[40];
    *(dest_bytes_ptr.add(40) as *mut i64) = *(src_bytes_ptr.add(40) as *const i64);

    // newSrcBuffer = *(_QWORD *)&pSrcSlot->dynamic_payload_bytes[48];
    let newSrcBuffer = *(src_bytes_ptr.add(48) as *const i64);

    // *(_QWORD *)&pSrcSlot->dynamic_payload_bytes[48] = 0;
    *(src_bytes_ptr.add(48) as *mut i64) = 0;

    // 3. Inspect destination pointer tracking block modifications for orphan states
    // pOrphanedMemory0 = *(void **)&pDestSlot->dynamic_payload_bytes[48];
    let pOrphanedMemory0 = *(dest_bytes_ptr.add(48) as *const *mut core::ffi::c_void);

    // *(_QWORD *)&pDestSlot->dynamic_payload_bytes[48] = newSrcBuffer;
    *(dest_bytes_ptr.add(48) as *mut i64) = newSrcBuffer;

    if !pOrphanedMemory0.is_null() {
        let ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, pOrphanedMemory0);
    }

    // 4. Restore recorded destination properties over to source targets
    // *(_OWORD *)&pSrcSlot->dynamic_payload_bytes[24] = tempPoolSnapshot0;
    *(src_bytes_ptr.add(24) as *mut u128) = tempPoolSnapshot0;

    // *(_QWORD *)&pSrcSlot->dynamic_payload_bytes[40] = tempPoolSnapshot1;
    *(src_bytes_ptr.add(40) as *mut i64) = tempPoolSnapshot1;

    // 5. Inspect source pointer tracking block modifications for orphan states
    // pOrphanedMemory1 = *(void **)&pSrcSlot->dynamic_payload_bytes[48];
    let pOrphanedMemory1 = *(src_bytes_ptr.add(48) as *const *mut core::ffi::c_void);

    // *(_QWORD *)&pSrcSlot->dynamic_payload_bytes[48] = oldDestBuffer;
    *(src_bytes_ptr.add(48) as *mut i64) = oldDestBuffer;

    if !pOrphanedMemory1.is_null() {
        let procHeapHandle = x4_getprocessheap();
        x4_heapfree(procHeapHandle, 0, pOrphanedMemory1);
    }

    // 6. Synchronize trailing state identifiers and flags
    let oldTrailerId = (*pDestSlot).is_dirty_or_active;
    (*pDestSlot).is_dirty_or_active = (*pSrcSlot).is_dirty_or_active;

    let result = (*pSrcSlot).alignment_padding[0];
    (*pSrcSlot).is_dirty_or_active = oldTrailerId;

    let oldCleanupByte = (*pDestSlot).alignment_padding[0];
    (*pDestSlot).alignment_padding[0] = result;
    (*pSrcSlot).alignment_padding[0] = oldCleanupByte;

    result as i8
}


pub unsafe fn FreeLanePayloadBuffers(pFrameArray: *mut u64) -> i8 {
    let mut pLane2Buffer: *mut core::ffi::c_void;
    let mut hHeap2: HANDLE;
    let mut pLane1Buffer: *mut core::ffi::c_void;
    let mut hHeap1: HANDLE;
    let mut pLane0Buffer: *mut core::ffi::c_void;
    let mut hHeap0: HANDLE;

    // 1. Clean up Lane 2 Tracking Slot (Index 22)
    pLane2Buffer = *pFrameArray.add(22) as *mut core::ffi::c_void;
    *pFrameArray.add(22) = 0;
    if !pLane2Buffer.is_null() {
        hHeap2 = x4_getprocessheap();
        x4_heapfree(hHeap2, 0, pLane2Buffer);
    }

    // 2. Clean up Lane 1 Tracking Slot (Index 14)
    pLane1Buffer = *pFrameArray.add(14) as *mut core::ffi::c_void;
    *pFrameArray.add(14) = 0;
    if !pLane1Buffer.is_null() {
        hHeap1 = x4_getprocessheap();
        x4_heapfree(hHeap1, 0, pLane1Buffer);
    }

    // 3. Clean up Lane 0 Tracking Slot (Index 6)
    pLane0Buffer = *pFrameArray.add(6) as *mut core::ffi::c_void;
    *pFrameArray.add(6) = 0;
    if !pLane0Buffer.is_null() {
        hHeap0 = x4_getprocessheap();
        x4_heapfree(hHeap0, 0, pLane0Buffer);
    }
    0
}

pub unsafe fn CleanupFrameTracking(pFrameArray: *mut u8) -> i32 {
    let mut result: i32 = 0; // Uninitialized storage fallback to match decompilation path

    // Stack allocation mimicking your [rsp+20h] contiguous tracking frame block
    let mut telemetryHash = [0i64; 6];

    // 1. Process First Frame Lane Target (Offset 56)
    if *pFrameArray.add(56) != 0 {
        telemetryHash[0] = 0x418A073AA3BC1C75;
        telemetryHash[1] = 0x418A073AA3BC2475;
        telemetryHash[2] = 0x418A073AA3BC2C75;

        result = x4_etwwritetransfer(
            telemetryHash.as_ptr() as u64 as __int64,
            3,
            pFrameArray as i64,
        );
    }

    // 2. Process Second Frame Lane Target (Offset 120)
    if *pFrameArray.add(120) != 0 {
        telemetryHash[0] = 0x418A073AA3BC3475;
        telemetryHash[1] = 0x418A073AA3BC3C75;
        telemetryHash[2] = 0x418A073AA3BC4475;

        result = x4_etwwritetransfer(
            telemetryHash.as_ptr() as u64 as __int64,
            3,
            pFrameArray.add(64) as i64,
        );
    }

    // 3. Process Third Frame Lane Target (Offset 184)
    if *pFrameArray.add(184) != 0 {
        telemetryHash[0] = 0x418A073AA3BC4C75;
        telemetryHash[1] = 0x418A073AA3BC5475;
        telemetryHash[2] = 0x418A073AA3BC5C75;
        telemetryHash[3] = 0x418A073AA3BC6475;
        telemetryHash[4] = 0x418A073AA3BC6C75;
        telemetryHash[5] = 0x418A073AA3BC7475;

        return x4_etwwritetransfer(
            telemetryHash.as_ptr() as u64 as __int64,
            6,
            pFrameArray.add(128) as i64,
        );
    }

    result
}

pub unsafe fn InitContextInternalPools(pContext: *mut SharedContextBlock) -> *mut SharedContextBlock {
    // Treat the top-level pointer as a raw byte tracker to cleanly mirror unaligned decompiled macros
    let p_byte = pContext as *mut u8;

    // pContext->lanes = 0x40000;
    // Assuming 'lanes' sits at the very beginning offset 0 of your struct (adjust if your layout differs)
    *(p_byte as *mut u32) = 0x40000;

    // LOBYTE(pContext->?) = 1;
    // LOBYTE(pContext->?) = 0;
    // These structural bits and scalar layouts sitting past lanes map to immediate index offsets
    *p_byte.add(4) = 1;
    *p_byte.add(5) = 0;

    // LOWORD(pContext->?[0].reservedPool[0]) = 0;
    *(p_byte.add(6) as *mut u16) = 0;

    // BYTE2(pContext->?[0].reservedPool[0]) = 0;
    *p_byte.add(8) = 0;

    // HIWORD(pContext->?) = 4;
    *(p_byte.add(10) as *mut u16) = 4;

    // pContext->? = (void *)4;
    *(p_byte.add(12) as *mut *mut core::ffi::c_void) = 4 as *mut _;

    // pContext->? = nullptr;
    *(p_byte.add(20) as *mut *mut core::ffi::c_void) = core::ptr::null_mut();

    // *(_QWORD *)&pContext->?[0].version = 0;
    *(p_byte.add(28) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[0].subStatus = 0;
    *(p_byte.add(36) as *mut u64) = 0;

    // pContext->?[0].capacity = 0;
    *(p_byte.add(44) as *mut u32) = 0;

    // HIWORD(pContext->?[0].reservedPool[1]) = 4;
    *(p_byte.add(48) as *mut u16) = 4;

    // LODWORD(pContext->?[0].reservedPool[1]) = 0x40000;
    *(p_byte.add(50) as *mut u32) = 0x40000;

    // BYTE4(pContext->?[0].reservedPool[1]) = 1;
    *p_byte.add(54) = 1;

    // LOBYTE(pContext->?[0].reservedPool[2]) = 2;
    *p_byte.add(55) = 2;

    // *(_QWORD *)&pContext->?[0].trailerId = 0;
    *(p_byte.add(56) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[1].version = 0;
    *(p_byte.add(64) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[1].subStatus = 0;
    *(p_byte.add(72) as *mut u64) = 0;

    // pContext->?[1].capacity = 0;
    *(p_byte.add(80) as *mut u32) = 0;

    // pContext->?[0].reservedPool[3] = 8;
    *(p_byte.add(84) as *mut u64) = 8;

    // LOWORD(pContext->?[1].reservedPool[0]) = 0;
    *(p_byte.add(92) as *mut u16) = 0;

    // BYTE2(pContext->?[1].reservedPool[0]) = 0;
    *p_byte.add(94) = 0;

    // LODWORD(pContext->?[1].reservedPool[1]) = 0x40000;
    *(p_byte.add(95) as *mut u32) = 0x40000;

    // BYTE4(pContext->?[1].reservedPool[1]) = 1;
    *p_byte.add(99) = 1;

    // HIWORD(pContext->?[1].reservedPool[1]) = 0;
    *(p_byte.add(100) as *mut u16) = 0;

    // LOBYTE(pContext->?[1].reservedPool[2]) = 1;
    *p_byte.add(102) = 1;

    // *(_QWORD *)&pContext->?[1].trailerId = 0;
    *(p_byte.add(103) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[2].version = 0;
    *(p_byte.add(111) as *mut u64) = 0;

    // *(_QWORD *)&pContext->?[2].subStatus = 0;
    *(p_byte.add(119) as *mut u64) = 0;

    // pContext->?[2].capacity = 0;
    *(p_byte.add(127) as *mut u32) = 0;

    // pContext->?[1].reservedPool[3] = 0;
    *(p_byte.add(131) as *mut u64) = 0;

    // LOWORD(pContext->?[2].reservedPool[0]) = 0;
    *(p_byte.add(139) as *mut u16) = 0;

    // BYTE2(pContext->?[2].reservedPool[0]) = 0;
    *p_byte.add(141) = 0;

    pContext
}

pub unsafe fn InitSharedMapping(
    phOutHandles: i64,
    pszBaseName: i64,
    allocationSize: u64,
) -> i64 {
    let primaryCapacity: u64;
    let v5: i64;
    let mut v6: *mut u16;
    let mut v7: i64;
    let mut secondaryInitialCount: u32;
    let mut v9: u16;
    let mut v10: *mut u16;
    let mut primaryInitialCount: i64;
    let secondaryCapacity: u64;
    let primaryCapacityClone: u32;
    let mut status: i32;
    let v15: i64 = 0; // Uninitialized/passed context in original frame
    let v17: i64;
    let mut pszDest: [u16; 264] = [0; 264];
    let retaddr: i64 = 0; // Local context placeholder for trace return address

    if (allocationSize & 3) != 0 {
        ReportAlignmentAssertionFailure(/*phOutHandles, pszBaseName, pszBaseName*/);
    }

    primaryCapacity = allocationSize >> 2;
    v5 = pszBaseName - (pszDest.as_ptr() as i64);
    v6 = pszDest.as_mut_ptr();
    v7 = 260;
    secondaryInitialCount = 1;

    loop {
        if v7 == -2147483386 {
            break;
        }
        v9 = *((v6 as *const u8).offset(v5 as isize) as *const u16);
        if v9 == 0 {
            break;
        }
        *v6 = v9;
        v6 = v6.offset(1);
        v7 -= 1;
        if v7 == 0 {
            break;
        }
    }

    v10 = v6.offset(-1);
    if v7 != 0 {
        v10 = v6;
    }
    *v10 = 0;

    // Direct StringCchCatW equivalent call (UTF-16 wide string literal)
    let p0_suffix: [u16; 4] = [0x005F, 0x0070, 0x0030, 0x0000]; // L"_p0"
    StringCchCatW(pszDest.as_mut_ptr(), 0x104, p0_suffix.as_ptr());

    primaryInitialCount = 1;
    secondaryCapacity = primaryCapacity >> 31;
    primaryCapacityClone = (primaryCapacity & 0x7FFFFFFF) as u32;

    if primaryCapacityClone != 0 {
        primaryInitialCount = primaryCapacityClone as i64;
    }

    status = CreateOrOpenSemaphoreW(
        phOutHandles,
        primaryCapacityClone as u64 as LONG,
        primaryInitialCount as u64 as LONG,
        pszDest.as_ptr() as i64,
    );

    if status >= 0 {
        let h_suffix: [u16; 2] = [0x0068, 0x0000]; // L"h"
        StringCchCatW(pszDest.as_mut_ptr(), 0x104, h_suffix.as_ptr());

        if (secondaryCapacity as u32) != 0 {
            secondaryInitialCount = secondaryCapacity as u32;
        }

        status = CreateOrOpenSemaphoreW(
            phOutHandles + 8,
            secondaryCapacity as LONG,
            secondaryInitialCount as u64 as LONG,
            pszDest.as_ptr() as i64,
        );

        if status >= 0 {
            return 0;
        }
        v17 = 141;
    } else {
        v17 = 136;
    }

    TraceProviderEvent(retaddr as u64, v17 as u64, v15 as u64, status as u32 as u64);
    status as u32 as i64
}

pub unsafe fn LookupExistingSharedContext(
    pszBaseName: i64,
    unused_param: i64,
    ptpOutCachedAddress: *mut u64,
) -> i64 {
    let mut v3: i64;
    let mut v4: *mut u16;
    let v5: i64;
    let mut v7: u16;
    let mut v8: *mut u16;
    let mut hPrimarySemaphore: *mut core::ffi::c_void;
    let v11: i32 = 0;
    let v12: i32 = 0;
    let mut primaryStatus: i32;
    let v15: i32 = 0;
    let mut hSecondarySemaphore: *mut core::ffi::c_void;
    let v18: i32 = 0;
    let v19: i32 = 0;
    let mut secondaryStatus: i32;
    let v22: i32 = 0;
    let mut v24: i32 = 0;
    let mut v25: [i32; 3] = [0; 3];
    let mut semNameBuffer: [u16; 264] = [0; 264];
    let retaddr: *mut core::ffi::c_void = core::ptr::null_mut();

    if !ptpOutCachedAddress.is_null() {
        *ptpOutCachedAddress = 0;
    }

    v3 = 260;
    v4 = semNameBuffer.as_mut_ptr();
    v5 = pszBaseName - (semNameBuffer.as_ptr() as i64);

    loop {
        if v3 == -2147483386 {
            break;
        }
        v7 = *((v4 as *const u8).offset(v5 as isize) as *const u16);
        if v7 == 0 {
            break;
        }
        *v4 = v7;
        v4 = v4.offset(1);
        v3 -= 1;
        if v3 == 0 {
            break;
        }
    }

    v8 = v4.offset(-1);
    if v3 != 0 {
        v8 = v4;
    }
    *v8 = 0;

    let p0_suffix: [u16; 4] = [0x005F, 0x0070, 0x0030, 0x0000]; // L"_p0"
    StringCchCatW(semNameBuffer.as_mut_ptr(), 0x104, p0_suffix.as_ptr());

    hPrimarySemaphore = x4_opensemaphorew(0x1F0003, 0, semNameBuffer.as_ptr());
    if hPrimarySemaphore.is_null() {
        if x4_getlasterror() != 2 {
            return LogDiagnosticEventWithStatus(retaddr as i32, 205, v11, v12);
        }
        return 0;
    }

    v25[0] = 0;
    v24 = 0;
    primaryStatus = QueryAndValidateSemaphoreCount(hPrimarySemaphore, v25.as_mut_ptr()) as i32;
    if primaryStatus < 0 {
        TraceProviderEvent(retaddr as i32 as u64, 211, v15 as u64, primaryStatus as u64);
        if x4_closehandle(hPrimarySemaphore) == 0 {
            HandleHandleCloseError(retaddr as u64 as i32, 2525, 0, 0);
        }
        return primaryStatus as u32 as i64;
    }

    let h_suffix: [u16; 2] = [0x0068, 0x0000]; // L"h"
    StringCchCatW(semNameBuffer.as_mut_ptr(), 0x104, h_suffix.as_ptr());

    hSecondarySemaphore = x4_opensemaphorew(0x1F0003, 0, semNameBuffer.as_ptr());
    if hSecondarySemaphore.is_null() {
        primaryStatus = LogDiagnosticEventWithStatus(retaddr as i32, 217, v18, v19);
        if x4_closehandle(hPrimarySemaphore) == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        return primaryStatus as u32 as i64;
    }

    secondaryStatus = QueryAndValidateSemaphoreCount(hSecondarySemaphore, &mut v24) as i32;
    if secondaryStatus >= 0 {
        if x4_closehandle(hSecondarySemaphore) == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        if !ptpOutCachedAddress.is_null() {
            *ptpOutCachedAddress = (v25[0] as u32 as u64) | ((v24 as i64) << 31) as u64;
        }
        if x4_closehandle(hPrimarySemaphore) == 0 {
            HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
        }
        return 0;
    }

    TraceProviderEvent(retaddr as i32 as u64, 219, v22 as u64, secondaryStatus as u64);
    if x4_closehandle(hSecondarySemaphore) == 0 {
        HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
    }
    if x4_closehandle(hPrimarySemaphore) == 0 {
        HandleHandleCloseError(retaddr as i32, 2525, 0, 0);
    }

    secondaryStatus as u32 as i64
}