use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{GetProcessHeap, HeapAlloc, HeapFree};
use crate::x4::globals::E_OUTOFMEMORY;
use crate::x4::spp::safe::SppSafeIntAdd;
use crate::x4::types::{PointerVector, HRESULT, SIZE_T};

pub unsafe fn PointerVectorResize(a1: *mut PointerVector, a2: i32) -> HRESULT {
    let mut v2: HRESULT = 0;
    let v3 = a2 as i64;
    let mut v5: *mut *mut core::ffi::c_void = core::ptr::null_mut();

    if (*a1).capacity != a2 {
        let mut size = (*a1).size;
        if a2 < size {
            size = a2;
        }

        if a2 > 0 {
            LogTraceEvent(0);
            let process_heap = GetProcessHeap();
            let v8 = HeapAlloc(process_heap, 0, (8 * v3) as usize as SIZE_T) as *mut *mut core::ffi::c_void;
            v5 = v8;

            if v8.is_null() {
                v2 = E_OUTOFMEMORY;
                HandleSubsystemError(E_OUTOFMEMORY);
                LogTraceEvent(v2);
                return v2;
            }

            if size > 0 {
                core::ptr::copy_nonoverlapping((*a1).elements, v8, size as usize);
            }
        }

        if v3 < (*a1).size as i64 {
            let mut counter = v3;
            let mut v10 = v3;
            let old_size = (*a1).size as i64;

            while counter < old_size {
                let elements = (*a1).elements;
                let v13 = elements.offset(v10 as isize);

                if !v13.is_null() && !(*v13).is_null() {
                    let v14 = *v13 as *mut u8;
                    // Replicates the `v14 - 4` header offset from the C code
                    let adjusted_ptr = v14.offset(-4) as *mut core::ffi::c_void;
                    let process_heap = GetProcessHeap();
                    HeapFree(process_heap, 0, adjusted_ptr);
                    LogTraceEvent(0);
                    *v13 = core::ptr::null_mut();
                }

                counter += 1;
                v10 += 1;
            }
        }

        let v16 = (*a1).elements;
        if !v16.is_null() {
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, v16 as *mut core::ffi::c_void);
            (*a1).elements = core::ptr::null_mut();
        }

        if v5.is_null() {
            v5 = core::ptr::null_mut();
        }

        (*a1).elements = v5;
        (*a1).size = size;
        (*a1).capacity = a2;
    }

    LogTraceEvent(v2);
    v2
}

pub unsafe fn PointerVectorCalculateGrowthCapacity(
    current_capacity: i32,
    p_out_new_capacity: *mut i32,
) -> HRESULT {
    let mut v3: HRESULT;
    let mut v4: i32 = 0;

    if current_capacity == 0 {
        if !p_out_new_capacity.is_null() {
            *p_out_new_capacity = 32;
        }
        v3 = 0;
        LogTraceEvent(v3);
        return v3;
    }

    if current_capacity < 0 {
        v3 = -2147024809; // E_INVALIDARG
    } else {
        v3 = 0;
        let u_curr = current_capacity as u32;
        // Replicates: (unsigned int)(2 * currentCapacity) >> 1 == currentCapacity
        let doubled = 2_u32.wrapping_mul(u_curr);
        if (doubled >> 1) == u_curr {
            v4 = doubled as i32;
        } else {
            v3 = -2147024362;
            HandleSubsystemError(v3);
        }

        LogTraceEvent(v3);

        if v3 >= 0 {
            if v4 >= 0 {
                if !p_out_new_capacity.is_null() {
                    *p_out_new_capacity = v4;
                }
                // Corresponds to jumping to LABEL_13
                LogTraceEvent(v3);
                if v3 < 0 {
                    HandleSubsystemError(v3);
                }
                LogTraceEvent(v3);
                return v3;
            }
            v3 = -2147024362;
        }
    }

    HandleSubsystemError(v3);
    // Corresponds to LABEL_13 and LABEL_15 flows
    LogTraceEvent(v3);
    if v3 < 0 {
        HandleSubsystemError(v3);
    }
    LogTraceEvent(v3);
    v3
}

pub unsafe fn PointerVectorPushBack(
    p_vec: *mut PointerVector,
    pp_source_element: *mut *mut core::ffi::c_void,
) -> HRESULT {
    let size = (*p_vec).size as i64;
    let mut p_out_result: i32 = 0;
    let mut v5: HRESULT = 0;
    let mut v6: HRESULT = 0;

    if size < 0 {
        v5 = -2147418113; // E_UNEXPECTED
        v6 = -2147418113;
        HandleSubsystemError(v6);
        LogTraceEvent(v5);
        return v5;
    }

    let v7 = SppSafeIntAdd(size as i32, 1, &mut p_out_result);
    v5 = v7;
    if v7 < 0 {
        v6 = v7;
        HandleSubsystemError(v6);
        LogTraceEvent(v5);
        return v5;
    }

    let capacity = (*p_vec).capacity;
    let mut v9: HRESULT = 0;

    if capacity >= 0 {
        let v11 = p_out_result;
        if p_out_result <= capacity {
            v9 = 0;
        } else {
            let mut v13 = 0;
            let mut current_cap = capacity;
            loop {
                v13 = PointerVectorCalculateGrowthCapacity(current_cap, &mut p_out_result);
                v9 = v13;
                if v13 < 0 {
                    break;
                }
                current_cap = p_out_result;
                if p_out_result >= v11 {
                    v13 = PointerVectorResize(p_vec, current_cap);
                    v9 = v13;
                    if v13 >= 0 {
                        break;
                    }
                }
            }
            if v13 < 0 {
                HandleSubsystemError(v13);
            }
        }
    } else {
        v9 = -2147418113;
        HandleSubsystemError(v9);
    }

    LogTraceEvent(v9);
    v5 = v9;
    if v9 < 0 {
        v6 = v9;
        HandleSubsystemError(v6);
        LogTraceEvent(v5);
        return v5;
    }

    let v14 = (*p_vec).size;
    if (size as i32) < v14 {
        let elements = (*p_vec).elements;
        if !elements.is_null() {
            let src = elements.offset(size as isize);
            let dst = elements.offset((size + 1) as isize);
            let count = (v14 - size as i32) as usize;
            core::ptr::copy(src, dst, count);
        }
    }

    let v17 = (*p_vec).elements;
    *v17.offset(size as isize) = core::ptr::null_mut();

    let v18 = if !pp_source_element.is_null() {
        let val = *pp_source_element;
        *pp_source_element = core::ptr::null_mut();
        val
    } else {
        core::ptr::null_mut()
    };

    let target_slot = *v17.offset(size as isize);
    if !target_slot.is_null() {
        let v19 = target_slot as *mut u8;
        let adjusted_ptr = v19.offset(-4) as *mut core::ffi::c_void;
        let process_heap = GetProcessHeap();
        HeapFree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
    }

    v5 = 0;
    let final_v18 = if v18.is_null() { core::ptr::null_mut() } else { v18 };
    (*p_vec).elements.offset(size as isize).write(final_v18);
    (*p_vec).size += 1;

    LogTraceEvent(v5);
    v5
}