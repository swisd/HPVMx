use crate::x4::externals::{x4_getprocessheap, x4_heapfree};

pub unsafe fn SwapVectorBuffersAndFreeOrphans(a1: *mut i32, a2: *mut i32) -> i32 {
    // Offset 8 corresponds to the 64-bit elements pointer in the vector struct 
    // (following two 32-bit integers: size and capacity)
    let p1_elem = (a1 as *mut u8).add(8) as *mut *mut core::ffi::c_void;
    let p2_elem = (a2 as *mut u8).add(8) as *mut *mut core::ffi::c_void;

    let v2 = *p1_elem;
    let v4 = *a1;
    let v6 = a1[1];

    *p1_elem = core::ptr::null_mut();

    *a1 = *a2;
    a1[1] = a2[1];

    let v7 = *p2_elem;
    *p2_elem = core::ptr::null_mut();

    let v8 = *p1_elem;
    if !v8.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v8);
        *p1_elem = core::ptr::null_mut();
    }

    let mut v10: *mut core::ffi::c_void = core::ptr::null_mut();
    if !v7.is_null() {
        v10 = v7;
    }
    *p1_elem = v10;

    *a2 = v4;
    a2[1] = v6;

    let v11 = *p2_elem;
    if !v11.is_null() {
        let process_heap = x4_getprocessheap();
        v10 = x4_heapfree(process_heap, 0, v11) as *mut core::ffi::c_void;
    }

    *p2_elem = v2;

    v10 as i32
}