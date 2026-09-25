use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{GetProcessHeap, HeapAlloc, HeapFree, LocalFree};
use crate::x4::spp::pointer::PointerVectorCalculateGrowthCapacity;
use crate::x4::spp::safe::SppSafeIntAdd;
use crate::x4::types::{VectorLayoutBuffered, HRESULT, SIZE_T, HLOCAL, SppVector};

pub unsafe fn SppVectorErase(
    p_vector: *mut SppVector,
    index_to_remove: u32,
) -> HRESULT {
    if p_vector.is_null() {
        let hr = -2147024809; // E_INVALIDARG
        HandleSubsystemError(hr);
        return LogTraceEvent(hr) as HRESULT;
    }

    let vector = &mut *p_vector;
    let old_last_index = vector.count - 1u32;
    let index = index_to_remove;
    vector.count = old_last_index;

    if index < old_last_index {
        let p_elements_array = vector.p_elements_array as *mut u8 as *mut u32;
        if !p_elements_array.is_null() {
            let dest = p_elements_array.offset((16 * index) as isize);
            let src = p_elements_array.offset((16 * index + 16) as isize);
            let count_bytes = (16 * (old_last_index - index)) as usize;
            core::ptr::copy(src, dest, count_bytes);
        }
    }

    let mut count = vector.count;
    let mut new_capacity: u32 = 0;
    let capacity = vector.capacity;
    let mut hr: HRESULT = 0;

    if count <= 32 {
        new_capacity = 32;
    } else {
        let mut doubled_count = 0;
        // Check for integer overflow during multiplication by 2
        let unsigned_count = count as u32;
        let multiplied = 2_i32.wrapping_mul(unsigned_count as i32);
        if (multiplied >> 1) == unsigned_count as i32 {
            doubled_count = multiplied as i32;
        } else {
            hr = -2147024362;
            HandleSubsystemError(hr);
        }

        LogTraceEvent(hr);

        if hr >= 0 {
            if doubled_count < 0 {
                hr = -2147024362;
                HandleSubsystemError(hr);
            } else {
                count = doubled_count as u32;
            }
        }
        LogTraceEvent(hr);

        if hr >= 0 {
            if count >= capacity {
                new_capacity = capacity as u32;
            } else {
                new_capacity = (capacity / 2) as u32;
                hr = 0;
            }
        } else {
            HandleSubsystemError(hr);
        }
    }

    LogTraceEvent(hr);

    if hr < 0 || (new_capacity as i32 != vector.capacity as i32 && {
        hr = SppVectorResize(p_vector, new_capacity);
        hr < 0
    }) {
        HandleSubsystemError(hr);
    }

    LogTraceEvent(hr) as HRESULT
}

pub unsafe fn SppVectorPushBack(
    layout: *mut VectorLayoutBuffered,
    pp_in_out_token: *mut *mut c_void,
) -> HRESULT {
    if layout.is_null() || pp_in_out_token.is_null() {
        let hr = -2147024809; // E_INVALIDARG
        HandleSubsystemError(hr);
        return LogTraceEvent(hr) as HRESULT;
    }

    let l = &mut *layout;
    let count = l.count;
    let mut p_out_result = 0;

    if count < 0 {
        let v5 = -2147418113; // E_UNEXPECTED
        HandleSubsystemError(v5);
        LogTraceEvent(v5);
        return v5;
    }

    let mut v5;
    let mut v9 = 0;
    let mut v10 = 0;

    let v7 = SppSafeIntAdd(count, 1, &mut p_out_result);
    v5 = v7;
    if v7 < 0 {
        HandleSubsystemError(v7);
        LogTraceEvent(v5);
        return v5;
    }

    let mut capacity = l.capacity;
    if capacity >= 0 {
        let v11 = p_out_result;
        if p_out_result > capacity {
            p_out_result = capacity;
            if v11 <= capacity {
                let v13 = SppVectorReallocateHeap(layout, capacity);
                v9 = v13;
                v10 = v13;
                if v13 < 0 {
                    HandleSubsystemError(v10);
                }
            } else {
                let mut v13 = 0;
                loop {
                    v13 = PointerVectorCalculateGrowthCapacity(capacity, &mut p_out_result);
                    v9 = v13;
                    if v13 < 0 {
                        v10 = v13;
                        break;
                    }
                    capacity = p_out_result;
                    if p_out_result >= v11 {
                        v13 = SppVectorReallocateHeap(layout, capacity);
                        v9 = v13;
                        v10 = v13;
                        break;
                    }
                }
                if v13 < 0 {
                    HandleSubsystemError(v10);
                }
            }
        }
    } else {
        v9 = -2147418113;
        v10 = -2147418113;
        HandleSubsystemError(v10);
    }

    LogTraceEvent(v9);
    v5 = v9;
    if v9 < 0 {
        HandleSubsystemError(v5);
        LogTraceEvent(v5);
        return v5;
    }

    let current_count = l.count;
    if count < current_count {
        let pp_buffer = l.ppBuffer;
        if !pp_buffer.is_null() {
            let dest = pp_buffer.offset((count + 1) as isize);
            let src = pp_buffer.offset(count as isize);
            let count_bytes = (8 * (current_count - count)) as usize;
            core::ptr::copy(src, dest, count_bytes);
        }
    }

    let pp_buffer = l.ppBuffer;
    if !pp_buffer.is_null() {
        *pp_buffer.offset(count as isize) = core::ptr::null_mut();
        let v18 = *pp_in_out_token;
        *pp_in_out_token = core::ptr::null_mut();

        let target_slot = pp_buffer.offset(count as isize);
        let existing = *target_slot;
        if !existing.is_null() {
            LocalFree(existing);
        }
        *target_slot = v18;
    }

    l.count += 1;
    v5 = 0;
    LogTraceEvent(v5) as HRESULT
}

pub unsafe fn SppVectorReallocateHeap(
    layout: *mut VectorLayoutBuffered,
    target_capacity: i32,
) -> HRESULT {
    let mut v2: HRESULT = 0;
    if layout.is_null() {
        let hr = -2147024809; // E_INVALIDARG
        HandleSubsystemError(hr);
        LogTraceEvent(hr);
        return hr;
    }

    let l = &mut *layout;
    let v3 = target_capacity;
    let mut new_buf: *mut *mut c_void = core::ptr::null_mut();

    if l.capacity != target_capacity {
        let mut count = l.count;
        if target_capacity < count {
            count = target_capacity;
        }

        if target_capacity > 0 {
            LogTraceEvent(0);
            let process_heap = GetProcessHeap();
            let bytes_to_alloc = match (v3 as usize).checked_mul(8) {
                Some(val) => val,
                None => {
                    v2 = -2147024362; // E_OUTOFMEMORY / INTSAFE_E_ARITHMETIC_OVERFLOW
                    HandleSubsystemError(v2);
                    LogTraceEvent(v2);
                    return v2;
                }
            };

            let allocated = HeapAlloc(process_heap, 0, bytes_to_alloc as SIZE_T);
            new_buf = allocated as *mut *mut c_void;

            if new_buf.is_null() {
                v2 = -2147024882; // E_OUTOFMEMORY
                HandleSubsystemError(v2);
                LogTraceEvent(v2);
                return v2;
            }

            if count > 0 && !l.ppBuffer.is_null() {
                core::ptr::copy_nonoverlapping(
                    l.ppBuffer,
                    new_buf,
                    count as usize,
                );
            }
        }

        // Free elements if new capacity truncates the list
        if v3 < l.count {
            let mut v9 = v3;
            let mut v10 = v3 as usize;
            while v9 < l.count {
                if !l.ppBuffer.is_null() {
                    let item_ptr = l.ppBuffer.add(v10);
                    if !item_ptr.is_null() && !(*item_ptr).is_null() {
                        LocalFree(*item_ptr as HLOCAL);
                        *item_ptr = core::ptr::null_mut();
                    }
                }
                v9 += 1;
                v10 += 1;
            }
        }

        // Free old buffer
        let old_buffer = l.ppBuffer;
        if !old_buffer.is_null() {
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, old_buffer as *mut c_void);
            l.ppBuffer = core::ptr::null_mut();
        }

        l.ppBuffer = new_buf;
        l.count = count;
        l.capacity = v3;
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn SppVectorResize(
    p_vector: *mut SppVector,
    new_capacity: u32,
) -> HRESULT {
    let mut v2: HRESULT = 0;
    if p_vector.is_null() {
        let hr = -2147024809; // E_INVALIDARG
        HandleSubsystemError(hr);
        LogTraceEvent(hr);
        return hr;
    }

    let vector = &mut *p_vector;
    let v3 = new_capacity as i32;
    let mut new_ptr: *mut c_void = core::ptr::null_mut();

    if vector.capacity != v3 as u32 {
        let mut count = vector.count;
        if v3 < count as i32 {
            count = v3 as u32;
        }

        if v3 > 0 {
            LogTraceEvent(0);
            let process_heap = GetProcessHeap();

            let bytes_to_alloc = match (v3 as usize).checked_mul(16) {
                Some(val) => val,
                None => {
                    v2 = -2147024362; // E_OUTOFMEMORY / Arithmetic overflow
                    HandleSubsystemError(v2);
                    LogTraceEvent(v2);
                    return v2;
                }
            };

            let allocated = HeapAlloc(process_heap, 0, bytes_to_alloc as SIZE_T);
            new_ptr = allocated;

            if new_ptr.is_null() {
                v2 = -2147024882; // E_OUTOFMEMORY
                HandleSubsystemError(v2);
                LogTraceEvent(v2);
                return v2;
            }

            if count > 0 && !vector.p_elements_array.is_null() {
                core::ptr::copy_nonoverlapping(
                    vector.p_elements_array as *const u8,
                    new_ptr as *mut u8,
                    (16 * count) as usize,
                );
            }
        }

        let old_elements_array = vector.p_elements_array;
        if !old_elements_array.is_null() {
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, old_elements_array);
        }

        vector.p_elements_array = if new_ptr.is_null() {
            core::ptr::null_mut()
        } else {
            new_ptr
        };
        vector.count = count;
        vector.capacity = v3 as u32;
    }

    LogTraceEvent(v2) as HRESULT
}