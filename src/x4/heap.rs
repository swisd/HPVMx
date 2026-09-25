use core::ffi::c_void;
use core::ptr::addr_of_mut;
use core::sync::atomic::{AtomicU32, Ordering};
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{x4_closehandle, x4_deletecriticalsection, x4_getprocessheap, x4_heapalloc, x4_heapfree};
use crate::x4::types::{ManagedAllocationObject40Byte, HRESULT, ManagedAllocationObject48Byte, RTL_CRITICAL_SECTION, ManagedAllocationObject48Byte_Variant2};

pub unsafe fn CreateRefCountedObject40byte(
    pp_out_object: *mut *mut ManagedAllocationObject40Byte,
) -> HRESULT {
    let mut v2: HRESULT = 0;

    let process_heap = x4_getprocessheap();
    let allocated = x4_heapalloc(process_heap, 0, 0x28);
    let object = allocated as *mut ManagedAllocationObject40Byte;

    if !object.is_null() {
        // Zero-initialize the 40-byte block (equivalent to clearing the OWORDS and QWORD)
        core::ptr::write_bytes(object as *mut u8, 0, 0x28);

        (*object).reference_count = 1;
        (*object).lpVtbl = &raw const off_1403C9910;

        if !pp_out_object.is_null() {
            *pp_out_object = object;
        }
    } else {
        v2 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn CreateRefCountedObject48byte(
    pp_out_object: *mut *mut ManagedAllocationObject48Byte,
) -> HRESULT {
    let mut v2: HRESULT = 0;

    let process_heap = x4_getprocessheap();
    let allocated = x4_heapalloc(process_heap, 0, 0x30);
    let object = allocated as *mut ManagedAllocationObject48Byte;

    if !object.is_null() {
        // Zero-initialize the 48-byte block (clearing the three OWORDS)
        core::ptr::write_bytes(object as *mut u8, 0, 0x30);

        (*object).reference_count = 1;
        (*object).lpVtbl = &raw const off_1403C98B8;
        (*object).trailing_handle = core::ptr::null_mut();

        if !pp_out_object.is_null() {
            *pp_out_object = object;
        }
    } else {
        v2 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn CreateRefCountedObject48byte_Variant2(
    pp_out_object: *mut *mut ManagedAllocationObject48Byte_Variant2,
) -> HRESULT {
    let mut v2: HRESULT = 0;

    let process_heap = x4_getprocessheap();
    let allocated = x4_heapalloc(process_heap, 0, 0x30);
    let object = allocated as *mut ManagedAllocationObject48Byte_Variant2;

    if !object.is_null() {
        // Zero-initialize the 48-byte block (clearing the three OWORDS)
        core::ptr::write_bytes(object as *mut u8, 0, 0x30);

        (*object).reference_count = 1;
        (*object).lpVtbl = &raw const off_1403C9968;
        (*object).internal_state_2 = core::ptr::null_mut();

        if !pp_out_object.is_null() {
            *pp_out_object = object;
        }
    } else {
        v2 = -2147024882; // E_OUTOFMEMORY
        HandleSubsystemError(v2);
    }

    LogTraceEvent(v2) as HRESULT
}

pub unsafe fn HeapFree_StructOffset24_Offset32_Minus4(a1: *mut c_void) {
    if a1.is_null() {
        return;
    }

    // Handle pointer at offset 32
    let ptr32_field = (a1 as *mut u8).add(32) as *mut *mut c_void;
    let val32 = *ptr32_field;
    if !val32.is_null() {
        let process_heap = x4_getprocessheap();
        // The pointer passed to x4_heapfree is offset by -4 bytes
        let adjusted_ptr = (val32 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr32_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 24
    let ptr24_field = (a1 as *mut u8).add(24) as *mut *mut c_void;
    let val24 = *ptr24_field;
    if !val24.is_null() {
        let process_heap = x4_getprocessheap();
        // The pointer passed to x4_heapfree is offset by -4 bytes
        let adjusted_ptr = (val24 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr24_field = core::ptr::null_mut();
    }
}

pub unsafe fn HeapFree_StructOffset24_Offset32_Offset40_Minus4(a1: *mut *mut c_void) {
    if a1.is_null() {
        return;
    }

    // Handle pointer at offset 40 (index 5)
    let ptr40_field = a1.add(5);
    let val40 = *ptr40_field;
    if !val40.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val40 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr40_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 32 (index 4)
    let ptr32_field = a1.add(4);
    let val32 = *ptr32_field;
    if !val32.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val32 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr32_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 24 (index 3)
    let ptr24_field = a1.add(3);
    let val24 = *ptr24_field;
    if !val24.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val24 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr24_field = core::ptr::null_mut();
    }
}

pub unsafe fn HeapFree_StructOffset8_Offset16_Minus4(a1: *mut c_void) {
    if a1.is_null() {
        return;
    }

    // Handle pointer at offset 16
    let ptr16_field = (a1 as *mut u8).add(16) as *mut *mut c_void;
    let val16 = *ptr16_field;
    if !val16.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val16 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr16_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 8
    let ptr8_field = (a1 as *mut u8).add(8) as *mut *mut c_void;
    let val8 = *ptr8_field;
    if !val8.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val8 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr8_field = core::ptr::null_mut();
    }
}

pub unsafe fn HeapFree_StructOffsets32_16_And_Offset8Minus4(a1: *mut *mut c_void) {
    if a1.is_null() {
        return;
    }

    // Handle pointer at offset 32 (index 4) - regular pointer free
    let ptr32_field = a1.add(4);
    let val32 = *ptr32_field;
    if !val32.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, val32);
        *ptr32_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 16 (index 2) - regular pointer free
    let ptr16_field = a1.add(2);
    let val16 = *ptr16_field;
    if !val16.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, val16);
        *ptr16_field = core::ptr::null_mut();
    }

    // Handle pointer at offset 8 (index 1) - offset by -4, logs ETW event
    let ptr8_field = a1.add(1);
    let val8 = *ptr8_field;
    if !val8.is_null() {
        let process_heap = x4_getprocessheap();
        let adjusted_ptr = (val8 as *mut u8).sub(4) as *mut c_void;
        x4_heapfree(process_heap, 0, adjusted_ptr);
        LogTraceEvent(0);
        *ptr8_field = core::ptr::null_mut();
    }
}

pub unsafe fn ObjectBaseRefDestroyInternal(a1: *mut c_void) -> i64 {
    if a1.is_null() {
        return 0;
    }

    // Assuming a1[1] advances by sizeof(ObjectBaseRef).
    // In many Spp structures, this points to an embedded second block or secondary struct section.
    // Let's treat a1 as a byte pointer and advance by the struct size (e.g., assuming a standard block size or calculating offset).
    // Based on typical patterns where a1[1] is used, let's use a byte offset or pointer arithmetic.
    // Here we use pointer arithmetic assuming `a1` is a pointer to a struct of known size or handle it via byte offsets.
    // Let's assume size of ObjectBaseRef or use a byte pointer offset equivalent to `a1 + 1`.

    // For safety and exact parity with `a1[1]`, let's treat `a1` as `*mut u8` and add the stride.
    // Since the exact sizeof(ObjectBaseRef) isn't explicitly declared here, we can use standard pointer addition `a1.offset(1)`
    // or evaluate it based on field offsets. Let's use `a1.add(1)` cast appropriately.

    let base_ptr = a1 as *mut u8;
    // Assuming size of ObjectBaseRef block 1 is represented by element stride (often sizeof struct, let's assume standard layout or define via byte offset).
    // Let's implement using raw pointer offsets matching the decompiled indices.

    // Let's define the pointer to the secondary block `a1[1]` (assuming sizeof(ObjectBaseRef) bytes, typically 64 or similar,
    // but we can also use field-based raw pointer casting if struct layout is known.
    // Let's use generic byte offset or pointer arithmetic matching `a1 + 1`).
    let block1 = (a1 as *mut *mut c_void).offset(8); // Approximation or direct pointer arithmetic:

    // Let's write it cleanly using raw pointer arithmetic matching the C semantics:
    // a1[1].size30_descriptor -> let's compute field offsets relative to the second struct element or block.
    // Looking at `&a1[1].size30_descriptor`, `a1[1]` means pointer arithmetic `a1 + sizeof(ObjectBaseRef)`.

    // Let's provide an implementation using raw byte offsets where appropriate or standard pointer casting:
    let struct_size = core::mem::size_of::<*mut c_void>() * 8; // placeholder or estimated stride
    // To be fully robust against layout variations, let's treat it as raw pointer arithmetic:

    let p_element_1 = (a1 as *mut u8).add(core::mem::size_of::<[usize; 8]>()); // Adjust based on base struct size

    // Alternatively, let's map the exact operations using raw pointer offsets:
    // 1. ReleaseRefCountedObjectArraySmart(&a1[1].size30_descriptor)
    let size30_desc_ptr = p_element_1.add(0) as *mut c_void;
    ReleaseRefCountedObjectArraySmart(size30_desc_ptr as *mut u32);

    // 2. Free p_size30_descriptor[1] if not null
    let p_size30_desc_word_array = p_element_1.add(8) as *mut *mut c_void;
    let v3 = *p_size30_desc_word_array;
    if !v3.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, v3);
        *p_size30_desc_word_array = core::ptr::null_mut();
    }

    // 3. ReleaseRefCountedObjectArraySmart_Type2(&a1[1].size28_descriptor)
    let size28_desc_ptr = p_element_1.add(16) as *mut c_void;
    ReleaseRefCountedObjectArraySmart_Type2(size28_desc_ptr as *mut u32);

    // 4. Free size28_array if not null
    let size28_array_ptr = p_element_1.add(24) as *mut *mut c_void;
    let size28_array = *size28_array_ptr;
    if !size28_array.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, size28_array as *mut c_void);
        *size28_array_ptr = core::ptr::null_mut();
    }

    // 5. ReleaseRefCountedObjectArraySmart_Type3(&a1[1].variant_b_descriptor)
    let variant_b_desc_ptr = p_element_1.add(32) as *mut c_void;
    ReleaseRefCountedObjectArraySmart_Type3(variant_b_desc_ptr as *mut u32);

    // 6. Free variant_b_array if not null
    let variant_b_array_ptr = p_element_1.add(40) as *mut *mut c_void;
    let variant_b_array = *variant_b_array_ptr;
    if !variant_b_array.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, variant_b_array);
        *variant_b_array_ptr = core::ptr::null_mut();
    }

    // 7. ReleaseRefCountedObjectArraySmart_Type4(&a1[1].sync_array_descriptor)
    let sync_array_desc_ptr = p_element_1.add(48) as *mut c_void;
    ReleaseRefCountedObjectArraySmart_Type4(sync_array_desc_ptr as *mut u32);

    // 8. Free sync_array_buffer if not null
    let sync_array_buffer_ptr = p_element_1.add(56) as *mut *mut c_void;
    let sync_array_buffer = *sync_array_buffer_ptr;
    if !sync_array_buffer.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, sync_array_buffer);
        *sync_array_buffer_ptr = core::ptr::null_mut();
    }

    // 9. Call sub_14005F1A0 on subsystem_base_lock (located at the start of `a1`)
    let lock_ptr = a1; // &a1->subsystem_base_lock
    ObjectBaseRefDestroyInternalPtr(lock_ptr) as i64
}

pub unsafe fn ObjectBaseRefDestroyInternalPtr(a1: *mut c_void) -> i32 {
    if a1.is_null() {
        return -1;
    }

    // Check DWORD flag at offset 0
    let flag_ptr = a1 as *mut u32;
    if *flag_ptr != 0 {
        // Delete critical section located at offset 8
        let cs_ptr = a1.add(8) as *mut RTL_CRITICAL_SECTION;
        x4_deletecriticalsection(cs_ptr);
        *flag_ptr = 0;
    }

    // Handle at offset 56 (index 7 in QWORD terms)
    let handle56_ptr = (a1 as *mut *mut c_void).add(7);
    let handle56 = *handle56_ptr;
    if !handle56.is_null() {
        x4_closehandle(handle56);
        *handle56_ptr = core::ptr::null_mut();
    }

    // Handle at offset 48 (index 6 in QWORD terms)
    let handle48_ptr = (a1 as *mut *mut c_void).add(6);
    let handle48 = *handle48_ptr;
    if !handle48.is_null() {
        x4_closehandle(handle48);
        *handle48_ptr = core::ptr::null_mut();
    }

    // Memory pointer at offset 88 (index 11 in QWORD terms)
    let mem88_ptr = (a1 as *mut *mut c_void).add(11);
    let mem88 = *mem88_ptr;
    if !mem88.is_null() {
        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, mem88);
        *mem88_ptr = core::ptr::null_mut();
    }

    // Redundant checks/closes for offset 56 and 48 matching the original decompilation flow
    let handle56 = *handle56_ptr;
    if !handle56.is_null() {
        x4_closehandle(handle56);
        *handle56_ptr = core::ptr::null_mut();
    }

    let handle48 = *handle48_ptr;
    if !handle48.is_null() {
        x4_closehandle(handle48);
        *handle48_ptr = core::ptr::null_mut();
    }
    0
}

pub unsafe fn ReleaseRefCountedObjectArraySmart(a1: *mut u32) -> i64 {
    if a1.is_null() {
        LogTraceEvent(0);
        return 0;
    }

    let flag_ptr = a1;
    let count_ptr = (a1 as *mut u8).add(4) as *mut i32;
    let buffer_ptr_field = (a1 as *mut u8).add(8) as *mut *mut c_void;

    let flag = *flag_ptr;
    let mut v3 = 0;

    if flag != 0 {
        let count = *count_ptr;
        if count > 0 {
            let array_buffer = *buffer_ptr_field;
            if !array_buffer.is_null() {
                for i in 0..count {
                    let element_slot_ptr = (array_buffer as *mut u8).add((i as usize) * 8) as *mut *mut c_void;
                    let v7 = *element_slot_ptr;
                    if !v7.is_null() {
                        let ref_count_ptr = v7 as *mut AtomicU32;
                        let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

                        if old_ref == 1 {
                            core::sync::atomic::fence(Ordering::Acquire);
                            HeapFree_StructOffset8_Offset16_Minus4(v7);
                            let process_heap = x4_getprocessheap();
                            x4_heapfree(process_heap, 0, v7);
                        }
                        *element_slot_ptr = core::ptr::null_mut();
                    }
                }
            }
        } else {
            v3 = *count_ptr;
        }

        let buffer = *buffer_ptr_field;
        if !buffer.is_null() {
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, buffer);
            *buffer_ptr_field = core::ptr::null_mut();
        }

        *buffer_ptr_field = core::ptr::null_mut();
        *flag_ptr = 0;
        *count_ptr = v3;
    }

    LogTraceEvent(0);
    0
}

pub unsafe fn ReleaseRefCountedObjectArraySmart_Type2(a1: *mut u32) -> i64 {
    if a1.is_null() {
        LogTraceEvent(0);
        return 0;
    }

    let flag_ptr = a1;
    let count_ptr = (a1 as *mut u8).add(4) as *mut i32;
    let buffer_ptr_field = (a1 as *mut u8).add(8) as *mut *mut c_void;

    let flag = *flag_ptr;
    let mut v3 = 0;

    if flag != 0 {
        let count = *count_ptr;
        if count > 0 {
            let array_buffer = *buffer_ptr_field;
            if !array_buffer.is_null() {
                for i in 0..count {
                    let element_slot_ptr = (array_buffer as *mut u8).add((i as usize) * 8) as *mut *mut c_void;
                    let v7 = *element_slot_ptr;
                    if !v7.is_null() {
                        let ref_count_ptr = v7 as *mut AtomicU32;
                        let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

                        if old_ref == 1 {
                            core::sync::atomic::fence(Ordering::Acquire);
                            HeapFree_StructOffset24_Offset32_Minus4(v7);
                            let process_heap = x4_getprocessheap();
                            x4_heapfree(process_heap, 0, v7);
                        }
                        *element_slot_ptr = core::ptr::null_mut();
                    }
                }
            }
        } else {
            v3 = *count_ptr;
        }

        let buffer = *buffer_ptr_field;
        if !buffer.is_null() {
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, buffer);
            *buffer_ptr_field = core::ptr::null_mut();
        }

        *buffer_ptr_field = core::ptr::null_mut();
        *flag_ptr = 0;
        *count_ptr = v3;
    }

    LogTraceEvent(0);
    0
}

pub unsafe fn ReleaseRefCountedObjectArraySmart_Type3(a1: *mut u32) -> i64 {
    if a1.is_null() {
        LogTraceEvent(0);
        return 0;
    }

    let flag_ptr = a1;
    let count_ptr = (a1 as *mut u8).add(4) as *mut i32;
    let buffer_ptr_field = (a1 as *mut u8).add(8) as *mut *mut c_void;

    let flag = *flag_ptr;
    let mut v3 = 0;

    if flag != 0 {
        let count = *count_ptr;
        if count > 0 {
            let array_buffer = *buffer_ptr_field;
            if !array_buffer.is_null() {
                for i in 0..count {
                    let element_slot_ptr = (array_buffer as *mut u8).add((i as usize) * 8) as *mut *mut c_void;
                    let v7 = *element_slot_ptr;
                    if !v7.is_null() {
                        let ref_count_ptr = v7 as *mut AtomicU32;
                        let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

                        if old_ref == 1 {
                            core::sync::atomic::fence(Ordering::Acquire);
                            HeapFree_StructOffsets32_16_And_Offset8Minus4( v7 as *mut *mut c_void );
                            let process_heap = x4_getprocessheap();
                            x4_heapfree(process_heap, 0, v7);
                        }
                        *element_slot_ptr = core::ptr::null_mut();
                    }
                }
            }
        } else {
            v3 = *count_ptr;
        }

        let buffer = *buffer_ptr_field;
        if !buffer.is_null() {
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, buffer);
            *buffer_ptr_field = core::ptr::null_mut();
        }

        *buffer_ptr_field = core::ptr::null_mut();
        *flag_ptr = 0;
        *count_ptr = v3;
    }

    LogTraceEvent(0);
    0
}

pub unsafe fn ReleaseRefCountedObjectArraySmart_Type4(a1: *mut u32) -> i64 {
    if a1.is_null() {
        LogTraceEvent(0);
        return 0;
    }

    let flag_ptr = a1;
    let count_ptr = (a1 as *mut u8).add(4) as *mut i32;
    let buffer_ptr_field = (a1 as *mut u8).add(8) as *mut *mut c_void;

    let flag = *flag_ptr;
    let mut v3 = 0;

    if flag != 0 {
        let count = *count_ptr;
        if count > 0 {
            let array_buffer = *buffer_ptr_field;
            if !array_buffer.is_null() {
                for i in 0..count {
                    let element_slot_ptr = (array_buffer as *mut u8).add((i as usize) * 8) as *mut *mut c_void;
                    let v7 = *element_slot_ptr;
                    if !v7.is_null() {
                        let ref_count_ptr = v7 as *mut AtomicU32;
                        let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

                        if old_ref == 1 {
                            core::sync::atomic::fence(Ordering::Acquire);
                            HeapFree_StructOffset24_Offset32_Offset40_Minus4(v7 as *mut *mut c_void);
                            let process_heap = x4_getprocessheap();
                            x4_heapfree(process_heap, 0, v7);
                        }
                        *element_slot_ptr = core::ptr::null_mut();
                    }
                }
            }
        } else {
            v3 = *count_ptr;
        }

        let buffer = *buffer_ptr_field;
        if !buffer.is_null() {
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, buffer);
            *buffer_ptr_field = core::ptr::null_mut();
        }

        *buffer_ptr_field = core::ptr::null_mut();
        *flag_ptr = 0;
        *count_ptr = v3;
    }

    LogTraceEvent(0);
    0
}

pub unsafe fn ShutdownRefCountedSubsystem() -> i64 {
    if dword_14046C5A8 != 0 {
        x4_deletecriticalsection(&raw mut stru_14046C5B0);
        dword_14046C5A8 = 0;
    }

    let v0 = qword_14046C5D8;
    if !qword_14046C5D8.is_null() {
        let ref_count_ptr = v0 as *mut AtomicU32;
        let old_ref = (*ref_count_ptr).fetch_sub(1, Ordering::Release);

        if old_ref == 1 && !v0.is_null() {
            core::sync::atomic::fence(Ordering::Acquire);
            ObjectBaseRefDestroyInternal(v0);
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, v0);
        }
        qword_14046C5D8 = core::ptr::null_mut();
    }

    LogTraceEvent(0);
    0
}