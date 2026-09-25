use core::ptr;
use crate::x4::externals::{memcpy_s, GetProcessHeap, HeapFree, SetLastError, GetLastError};
use crate::x4::helpers::AllocateFromHeap;
use crate::x4::types::VectorLayout;

pub unsafe fn ReallocVectorBufferWithCacheAlignment64(
    vec: *mut VectorLayout,
    bytesRequested: u64,
) -> i8 {
    let mut v3 = bytesRequested;
    let v4 = ((*vec).pCapacityEnd as u64).saturating_sub((*vec).pBegin as u64);

    if bytesRequested.saturating_add((*vec).pEnd as u64).saturating_sub((*vec).pBegin as u64) < v4 {
        return 1;
    }
    if bytesRequested < 2 * v4 {
        v3 = 2 * v4;
    }
    if v4 >= v3 {
        return 1;
    }

    let v6 = (v3 & 0xFFFFFFFFFFFFFFC0) + 64;
    let LastError = GetLastError();
    let v8 = AllocateFromHeap(0, v6) as *mut u8;
    let v9: i8 = 0;
    let v10 = v8;

    if !v8.is_null() {
        let v11 = ((*vec).pEnd as u64).saturating_sub((*vec).pBegin as u64);

        // Match the behavior of memcpy_s
        memcpy_s(v8 as *mut _, v6 as usize, (*vec).pBegin as *const _, v11 as usize);

        let pAllocation = (*vec).pAllocation;
        (*vec).pAllocation = v10;

        if !pAllocation.is_null() {
            let ProcessHeap = GetProcessHeap();
            HeapFree(ProcessHeap, 0, pAllocation as *mut _);
        }

        (*vec).pBegin = v10;
        (*vec).pEnd = v10.add(v11 as usize);
        (*vec).pCapacityEnd = v10.add(v6 as usize);

        SetLastError(LastError);
        return 1;
    }

    SetLastError(LastError);
    v9
}