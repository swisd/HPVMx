use core::ffi::c_void;
use core::ops::Sub;
use crate::x4::error::LogTraceEvent;
use crate::x4::externals::{GetProcessHeap, HeapFree};

struct FPTR(u64);

impl FPTR {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[repr(C)]
pub struct FivePointerArray {
    pub(crate) elements: [FPTR; 5],
}

pub unsafe fn MoveAssignFivePointerArray(
    p_dst: *mut FivePointerArray,
    p_src: *mut FivePointerArray,
) -> *mut FivePointerArray {
    if p_dst.is_null() || p_src.is_null() {
        return p_dst;
    }

    let dst = &mut *p_dst;
    let src = &mut *p_src;

    for i in 0..5 {
        let temp = &src.elements[i];
        src.elements[i] = *core::ptr::null_mut().clone();

        let current_dst = &dst.elements[i];
        if !current_dst.is_null() {
            // Adjust pointer back by 4 bytes as seen in the decompiled snippet (v5 - 4)
            let adjusted = (current_dst.as_u64()).sub(4) as *mut c_void;
            let process_heap = GetProcessHeap();
            HeapFree(process_heap, 0, adjusted);
            LogTraceEvent(0);
        }

        dst.elements[i] = *temp;
    }

    p_dst
}
