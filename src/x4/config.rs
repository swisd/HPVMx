use core::ffi::c_void;
use core::ops::Add;
use core::sync::atomic::{AtomicI32, Ordering};
use crate::x4::dxgi::CloseContextSyncHandles;
use crate::x4::dxgi::pipeline::GetThreadLocalPipelineContext;
use crate::x4::externals::{memcpy_s, x4_heapfree, x4_getprocessheap, x4_getcurrentthreadid, x4_closehandle, x4_waitforsingleobjectex, x4_releasemutex, x4_getlasterror, x4_setlasterror};
use crate::x4::globals::_guard_check_icall_fptr;
use crate::x4::helpers::{AllocateFromHeap, NormalizingExitRegisterWrapper};
use crate::x4::ops::memset;
use crate::x4::types::{ConfigHashNode, ConfigHashNodeSimple, ConfigManager, ConfigVectorHeader, BOOL, SIZE_T, ConfigHierarchyNode, ThreadContextNode};
use crate::x4::spp::subsystem::BuildSubsystemPathRecursive;
use crate::x4::threading::{HandleHandleCloseError, ReportLockFailureException};

pub unsafe fn CleanupConfigManagerResources(p_manager_context: *mut ConfigManager) -> i32 {
    if p_manager_context.is_null() {
        return 0;
    }

    let mut result: i32 = 1; // Default Win32 success state (TRUE)

    // Destroy the hash map first
    let hash_buckets = (*p_manager_context).hash_buckets;
    DestroyConfigHashMap(hash_buckets);

    // Clean up completion_event_1
    let completion_event_1 = (*p_manager_context).completion_event_1;
    if !completion_event_1.is_null() {
        result = x4_closehandle(completion_event_1);
        if result == 0 {
            // Passing 0 for retaddr placeholder as raw stack return addresses aren't exposed in Rust
            HandleHandleCloseError(0, 2525, 0, 0);
        }
    }

    // Clean up completion_event_0
    let completion_event_0 = (*p_manager_context).completion_event_0;
    if !completion_event_0.is_null() {
        result = x4_closehandle(completion_event_0);
        if result == 0 {
            HandleHandleCloseError(0, 2525, 0, 0);
        }
    }

    // Clean up session_mutex
    let session_mutex = (*p_manager_context).session_mutex;
    if !session_mutex.is_null() {
        result = x4_closehandle(session_mutex);
        if result == 0 {
            HandleHandleCloseError(0, 2525, 0, 0);
        }
    }

    result
}

pub unsafe fn ConfigManagerRelease(p_manager_context: *mut ConfigManager) -> i32 {
    if p_manager_context.is_null() {
        return 0;
    }

    let disable_bypass = core::ptr::addr_of!(g_DisableBypassCheck).read_volatile();
    if disable_bypass != 0 {
        (*p_manager_context).reference_count -= 1;
        return (*p_manager_context).reference_count;
    }

    let condition_check = core::ptr::addr_of!(g_pfnConditionCheck).read_volatile();
    if let Some(check_fn) = condition_check {
        _guard_check_icall_fptr();
        if check_fn() != 0 {
            (*p_manager_context).reference_count -= 1;
            return (*p_manager_context).reference_count;
        }
    }

    let mut session_mutex = (*p_manager_context).session_mutex;
    let wait_result = x4_waitforsingleobjectex(session_mutex, 0xFFFFFFFF, 0);

    if wait_result == 258 {
        // WAIT_TIMEOUT
        session_mutex = core::ptr::null_mut(); // Note: core::ptr::null_mut()
    } else if (wait_result & 0xFFFFFF7F) != 0 {
        ReportLockFailureException(0);
    }

    (*p_manager_context).reference_count -= 1;
    let result = (*p_manager_context).reference_count;

    if result != 0 {
        if !session_mutex.is_null() {
            let release_res = x4_releasemutex(session_mutex);
            if release_res == 0 {
                HandleHandleCloseError(0, 2535, 0, 0);
            }
        }
    } else {
        // Reference count reached zero: teardown resources
        let sync_handles_ptr = &raw mut (*p_manager_context).completion_event_0 as *mut c_void;
        CloseContextSyncHandles(sync_handles_ptr);

        if !session_mutex.is_null() {
            let last_error = x4_getlasterror();
            if x4_releasemutex(session_mutex) == 0 {
                HandleHandleCloseError(0, 2535, 0, 0);
            }
            x4_setlasterror(last_error);
        }

        CleanupConfigManagerResources(p_manager_context);

        let process_heap = x4_getprocessheap();
        x4_heapfree(process_heap, 0, p_manager_context as *mut c_void);
        return 0;
    }

    result
}

pub unsafe fn DestroyConfigHashMap(pp_bucket_array: *mut *mut ConfigHashNode) {
    if pp_bucket_array.is_null() {
        return;
    }

    // The loop processes 10 hash buckets (ppBucketArray + 10)
    for i in 0..10 {
        let bucket_ptr = pp_bucket_array.add(i);
        let mut node = *bucket_ptr;

        while !node.is_null() {
            let old_node = node;
            node = (*old_node).next_node;

            // element_vector is located right after next_node (offset 8)
            let element_vector_ptr = (old_node as *mut u8).add(8) as *mut c_void;
            FreeConfigElementVector(old_node.element_vector);

            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, old_node as *mut c_void);
        }

        // Clear the bucket head pointer
        *bucket_ptr = core::ptr::null_mut();
    }
}

pub unsafe fn DestroyConfigHashMapShallow(pp_bucket_array: *mut *mut ConfigHashNodeSimple) {
    if pp_bucket_array.is_null() {
        return;
    }

    // Iterates through 10 hash buckets (ppBucketArray + 10)
    for i in 0..10 {
        let bucket_ptr = pp_bucket_array.add(i);
        let mut v3 = *bucket_ptr;

        while !v3.is_null() {
            let v4 = v3;
            v3 = (*v4).next_node;

            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, v4 as *mut c_void);
        }

        *bucket_ptr = core::ptr::null_mut();
    }
}

pub unsafe fn DispatchConfigurationPipeline(
    p_parser_workspace: *mut c_void,
    p_path_out_buffer: *mut u8,
    max_buffer_len: u64,
) -> i32 {
    if p_path_out_buffer.is_null() || max_buffer_len == 0 {
        return 0;
    }

    *p_path_out_buffer = 0;
    let mut v3: u8 = 0;

    let base_ptr: usize = core::ptr::addr_of!(qword_14046B2A8).read_volatile();
    if !base_ptr.is_null() {
        let current_thread_id = x4_getcurrentthreadid();
        let bucket_index = (current_thread_id % 10) as usize;
        let mut i_ptr = *base_ptr.add(bucket_index);

        while !i_ptr.is_null() {
            let thread_id_val = *(i_ptr as *mut u32);
            if thread_id_val == current_thread_id {
                i_ptr = (i_ptr as *mut u8).add(16) as *mut c_void;
                break;
            }
            i_ptr = *(i_ptr.add(1) as *mut *mut c_void); // Assuming next node pointer is at offset 8
        }

        if !i_ptr.is_null() {
            let root_node = *(i_ptr as *mut *mut ConfigHierarchyNode);
            if !root_node.is_null() {
                *p_path_out_buffer = 0;
                if BuildSubsystemPathRecursive(p_parser_workspace, root_node, p_path_out_buffer, max_buffer_len) != 0 {
                    let workspace_slot = (p_parser_workspace as *mut u8).add(72) as *mut *mut u8;
                    *workspace_slot = p_path_out_buffer;
                }

                let mut v10 = root_node;
                while !v10.is_null() {
                    let state_flags_ptr = &mut (*v10).unk_state_flags;
                    let unk_state_flags = *state_flags_ptr;
                    *state_flags_ptr = 1;

                    if unk_state_flags == 0 {
                        let unk_sibling_ptr = (*v10).unk_sibling_ptr as *mut *mut u8;
                        if !unk_sibling_ptr.is_null() {
                            let v13: unsafe fn(*mut *mut u8, *mut c_void) -> u8 =
                                core::mem::transmute(*(*unk_sibling_ptr as *const *const c_void));
                            _guard_check_icall_fptr();
                            v3 |= v13(unk_sibling_ptr, p_parser_workspace);
                        }
                        (*v10).unk_state_flags = 0;
                    }
                    v10 = (*v10).next_traverse_node;
                }
            }
        }
    }

    let global_callback = core::ptr::addr_of!(qword_14046B2B8).read_volatile();
    if !global_callback.is_null() {
        let workspace_byte = *((p_parser_workspace as *mut u8).add(4));
        let v15 = v3 != 0 || (workspace_byte & 2) != 0;
        let v14: unsafe fn(usize, *mut c_void) = core::mem::transmute(global_callback);
        _guard_check_icall_fptr();
        v14(v15 as usize, p_parser_workspace);
    }

    let current_thread_id = x4_getcurrentthreadid() as i32;
    let mut thread_local_pipeline_context: *mut ThreadContextNode = core::ptr::null_mut();

    let current_lock_thread = core::ptr::addr_of!(dword_14046B35C).read_volatile();
    if current_lock_thread != current_thread_id {
        let incremented_val = dword_14046B370.fetch_add(1, Ordering::SeqCst) + 1;
        if incremented_val < 4 {
            core::ptr::addr_of_mut!(dword_14046B35C).write_volatile(current_thread_id);
            thread_local_pipeline_context = GetThreadLocalPipelineContext();
            if !thread_local_pipeline_context.is_null() {
                DCP_sub1(thread_local_pipeline_context, p_parser_workspace);
            }
            core::ptr::addr_of_mut!(dword_14046B35C).write_volatile(0);
        }
        dword_14046B370.fetch_sub(1, Ordering::SeqCst);
    }

    thread_local_pipeline_context as i32
}

pub unsafe fn DCP_sub1(a1: *mut u8, a2: i64) -> i64 {
    if a1.is_null() {
        return 0;
    }

    let v3_ptr = a1.add(16) as *mut i32;
    let v3 = *v3_ptr;

    let ptr24_field = a1.add(24) as *mut *mut u8;
    let mut result_ptr = *ptr24_field;

    if result_ptr.is_null() {
        if v3 != 0 {
            // AllocateFromHeap(8, 400) -> 0x190 hex is 400 bytes
            result_ptr = AllocateFromHeap(8, 400) as *mut u8;
            *ptr24_field = result_ptr;
            if !result_ptr.is_null() {
                let capacity_ptr = a1.add(32) as *mut u16;
                *capacity_ptr = 5;

                // Initialization loop: 5 entries of 80 bytes each, setting the first WORD to 80
                let mut cur = result_ptr;
                let end = result_ptr.add(400);
                while cur != end {
                    *(cur as *mut u16) = 80;
                    cur = cur.add(80);
                }
            }
        }
    }

    let v7 = *ptr24_field;
    if !v7.is_null() {
        let capacity_ptr = a1.add(32) as *mut u16;
        let capacity = *capacity_ptr;
        let v8 = v7.add((20 * capacity as usize) * 4);

        if v3 == 0 || v7 == v8 {
            return fallback_round_robin(a1, v7, a2);
        } else {
            // v9 starts at v7 + 8 bytes (v7 + 2 dwords)
            let mut v9 = v7.add(8) as *mut i32;
            let v8_dwords = v8 as *mut i32;

            loop {
                // *(v9 - 1) corresponds to the dword immediately preceding v9
                let prev_val = *v9.sub(1);
                let mut result_val = 0;

                if prev_val > v3 {
                    result_val = *(a2 as *const i32).add(2); // offset 8
                    if *v9 == result_val {
                        break;
                    }
                }

                v9 = v9.add(20); // advance by 20 dwords (80 bytes)
                // v9 - 2 in dwords corresponds to byte pointer check (v9.sub(2) as *mut u8 == v8)
                if v9.sub(2) as *mut u8 == v8 {
                    return fallback_round_robin(a1, v7, a2);
                }
            }

            // If broken out of loop successfully, return result_val or appropriate handle
            return *(a2 as *const i32).add(2) as i64;
        }
    }

    a1 as i64
}

unsafe fn fallback_round_robin(a1: *mut u8, v7: *mut u8, a2: i64) -> i64 {
    let index_ptr = a1.add(34) as *mut u16;
    let capacity_ptr = a1.add(32) as *mut u16;
    let capacity = *capacity_ptr;

    let v10 = ((*index_ptr as u32 + 1) % capacity as u32) as u16;
    *index_ptr = v10;

    let slot_ptr = v7.add((20 * v10 as usize) * 4);

    let ref_ptr_field = a1.add(8) as *mut *mut AtomicI32;
    let ref_val = if !(*ref_ptr_field).is_null() {
        (**ref_ptr_field).fetch_add(1, Ordering::SeqCst) + 1
    } else {
        0
    };

    DCP_sub2(slot_ptr, a2, ref_val) as i64
}

pub unsafe fn DCP_sub2(a1: *mut u8, a2: i64, a3: i32) -> i32 {
    if a1.is_null() || a2.is_null() {
        return 0;
    }

    // Set header fields
    *(a1.add(4) as *mut i32) = a3;
    *(a1.add(8) as *mut i32) = *(a2.add(8) as *const i32);
    *(a1.add(16) as *mut *mut c_void) = core::ptr::null_mut();
    *(a1.add(24) as *mut u16) = *(a2.add(64) as *const u16);
    *(a1.add(26) as *mut u8) = a2 as u8;
    *(a1.add(32) as *mut *mut c_void) = core::ptr::null_mut();
    *(a1.add(40) as *mut u64) = *(a2.add(136) as *const u64);
    *(a1.add(48) as *mut u64) = *(a2.add(144) as *const u64);
    *(a1.add(56) as *mut *mut c_void) = core::ptr::null_mut();

    // Calculate string 1 length (at a2 + 56)
    let s1_ptr = *(a2.add(56) as *const *const u8);
    let v7 = if !s1_ptr.is_null() {
        let mut len = 0;
        while *s1_ptr.add(len) != 0 {
            len += 1;
        }
        len + 1
    } else {
        1
    };

    // Calculate string 2 length (at a2 + 128)
    let s2_ptr = *(a2.add(128) as *const *const u8);
    let v10 = if !s2_ptr.is_null() {
        let mut len = 0;
        while *s2_ptr.add(len) != 0 {
            len += 1;
        }
        len + 1
    } else {
        1
    };

    // Calculate wide string length (at a2 + 24)
    let ws_ptr = *(a2.add(24) as *const *const u16);
    let v13 = if !ws_ptr.is_null() {
        let mut len = 0;
        while *ws_ptr.add(len) != 0 {
            len += 1;
        }
        (len + 1) * 2
    } else {
        2
    };

    let required_size = v7 + v13 + v10;
    let capacity_ptr = a1.add(72) as *mut usize;
    let buffer_ptr_field = a1.add(64) as *mut *mut u8;

    if (*buffer_ptr_field).is_null() || *capacity_ptr < required_size {
        let new_mem = AllocateFromHeap(8, required_size as SIZE_T);
        if !new_mem.is_null() {
            let old_mem = *buffer_ptr_field;
            let heap = x4_getprocessheap();
            x4_heapfree(heap, 0, old_mem as *mut c_void);
            *buffer_ptr_field = new_mem as *mut u8;
            *capacity_ptr = required_size;
        }
    }

    let buf = *buffer_ptr_field;
    if buf.is_null() {
        return 0;
    }

    let capacity = *capacity_ptr;
    let buf_end = buf.add(capacity);
    let mut current = buf;

    // Copy string 1
    let s1 = *(a2.add(56) as *const *const u8);
    let s1_dest = a1.add(16) as *mut *mut u8;
    if !s1.is_null() && *s1 != 0 {
        let mut s1_len = 0;
        while *s1.add(s1_len) != 0 {
            s1_len += 1;
        }
        let s1_total = s1_len + 1;
        if capacity >= s1_total {
            memcpy_s(current as *mut c_void, capacity, s1 as *const c_void, s1_total);
            *s1_dest = current;
            current = current.add(s1_total);
        } else {
            *s1_dest = core::ptr::null_mut();
        }
    } else {
        *s1_dest = core::ptr::null_mut();
    }

    // Copy string 2
    let s2 = *(a2.add(128) as *const *const u8);
    let s2_dest = a1.add(32) as *mut *mut u8;
    if current != buf_end && !s2.is_null() && *s2 != 0 {
        let mut s2_len = 0;
        while *s2.add(s2_len) != 0 {
            s2_len += 1;
        }
        let s2_total = s2_len + 1;
        if (buf_end as usize - current as usize) >= s2_total {
            memcpy_s(current as *mut c_void, buf_end as usize - current as usize, s2 as *const c_void, s2_total);
            *s2_dest = current;
            current = current.add(s2_total);
        } else {
            *s2_dest = core::ptr::null_mut();
        }
    } else {
        *s2_dest = core::ptr::null_mut();
    }

    // Copy wide string
    let ws = *(a2.add(24) as *const *const u16);
    let ws_dest = a1.add(56) as *mut *mut u8;
    if current != buf_end && !ws.is_null() && *ws != 0 {
        let mut ws_len = 0;
        while *ws.add(ws_len) != 0 {
            ws_len += 1;
        }
        let ws_total = (ws_len + 1) * 2;
        if (buf_end as usize - current as usize) >= ws_total {
            memcpy_s(current as *mut c_void, buf_end as usize - current as usize, ws as *const c_void, ws_total);
            *ws_dest = current;
            current = current.add(ws_total);
        } else {
            *ws_dest = core::ptr::null_mut();
        }
    } else {
        *ws_dest = core::ptr::null_mut();
    }

    // Zero out remaining allocated buffer space
    let remaining = buf_end as usize - current as usize;
    if remaining > 0 {
        memset(current as *mut c_void, 0, remaining as u64);
    }

    1
}

pub unsafe fn FreeConfigElementVector(p_vector_header: *mut ConfigVectorHeader) -> BOOL {
    if p_vector_header.is_null() {
        return 0;
    }

    let header = &mut *p_vector_header;
    let array_base_pointer = header.array_base_pointer;

    if array_base_pointer.is_null() {
        header.element_count = 0;
        return 1;
    }

    let count = header.element_count as usize;
    // Each ConfigArrayElement is 80 bytes (10 pointers × 8 bytes) as referenced in previous routines
    let element_size = 80;

    for i in 0..count {
        let elem_ptr = (array_base_pointer as *mut u8).add(i * element_size);
        let ptr_field = elem_ptr as *mut *mut c_void;

        // Free the sub-allocated buffer pointer
        let sub_buf = *ptr_field;
        if !sub_buf.is_null() {
            let process_heap = x4_getprocessheap();
            x4_heapfree(process_heap, 0, sub_buf);
        }

        // Zero out the first two pointers in the element block (matching p_sub_allocated_buffer assignment patterns)
        *ptr_field = core::ptr::null_mut();
        *ptr_field.add(1) = core::ptr::null_mut();
    }

    // Free the main array base buffer
    let process_heap = x4_getprocessheap();
    x4_heapfree(process_heap, 0, array_base_pointer);
    let result = 0;

    // Reset header state
    header.element_count = 0;
    header.array_base_pointer = core::ptr::null_mut();

    result
}

pub unsafe fn RegisterConfigSystemShutdown() -> i64 {
    NormalizingExitRegisterWrapper(ShutdownConfigManagerSystem)
}

pub unsafe fn RegisterShutdownShallowConfigHashMapSystem() -> i64 {
    NormalizingExitRegisterWrapper(ShutdownShallowConfigHashMapSystem)
}

pub unsafe fn ShutdownConfigManagerSystem() -> i64 {
    let context = core::ptr::addr_of!(pManagerContext).read_volatile();
    if !context.is_null() {
        return ConfigManagerRelease(context) as i64;
    }
    0
}

pub unsafe fn ShutdownShallowConfigHashMapSystem() -> i32 {
    let bucket_ptr_addr = core::ptr::addr_of_mut!(ppBucketArray);
    DestroyConfigHashMapShallow(bucket_ptr_addr);
    0
}