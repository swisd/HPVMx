use core::ffi::c_void;
use crate::x4::externals::x4_getcurrentthreadid;
use crate::x4::spp::lock::{SppCustomLockAcquire, SppCustomLockRelease};
use crate::x4::types::{ThreadMap, DWORD, SppThreadContextBody, SppThreadContextManager};

pub unsafe fn SppGetCurrentThreadContext(
    pManager: *mut SppThreadContextManager,
) -> *mut c_void {
    if pManager.is_null() {
        return core::ptr::null_mut();
    }

    let body = (*pManager).body;
    let mut v2 = 0;
    let mut p_out_index: i32 = 0;

    let invalid_sentinel = -16isize as *mut SppThreadContextBody;
    let is_valid = !body.is_null() && body != invalid_sentinel;

    let p_lock = if is_valid {
        core::ptr::addr_of_mut!((*body).lock)
    } else {
        core::ptr::null_mut()
    };

    if is_valid {
        v2 = 1;
        SppCustomLockAcquire(p_lock, 1);
    }

    let current_thread_id = x4_getcurrentthreadid();

    let context = if is_valid {
        if SppThreadMapLookupIndex(
            core::ptr::addr_of!((*body).threadIdMap),
            current_thread_id,
            &mut p_out_index,
        ) != false {
            (*(*body).threadIdMap.array.offset(p_out_index as isize)).context
        } else {
            (*body).globalContext
        }
    } else {
        core::ptr::null_mut()
    };

    if !p_lock.is_null() && v2 != 0 {
        SppCustomLockRelease(p_lock);
    }

    context
}

pub unsafe fn SppThreadMapLookupIndex(
    map: *const ThreadMap,
    targetThreadId: DWORD,
    pOutIndex: *mut i32,
) -> bool {
    if map.is_null() {
        if !pOutIndex.is_null() {
            *pOutIndex = 0;
        }
        return false;
    }

    let element_count = (*map).elementCount;
    let array = (*map).array;

    if element_count <= 0 {
        if !pOutIndex.is_null() {
            *pOutIndex = 0;
        }
        return false;
    }

    let mut v5 = 0;
    let mut v6 = element_count as u32;

    while v6 > 0 {
        let v7 = v5 + (v6 >> 1) as i32;
        let entry_id = (*array.offset(v7 as isize)).threadId;

        // Equivalent to the unsigned 64-bit comparison check in MSVC output
        if entry_id >= targetThreadId {
            v6 >>= 1;
        } else {
            v5 = v7 + 1;
            v6 = v6.wrapping_add((-1i32 as u32).wrapping_sub(v6 >> 1));
        }
    }

    let result = v5 < element_count
        && (*array.offset(v5 as isize)).threadId == targetThreadId;

    if !pOutIndex.is_null() {
        *pOutIndex = v5;
    }

    result
}