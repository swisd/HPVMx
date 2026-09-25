use alloc::borrow::ToOwned;
use core::cmp::PartialEq;
use core::ffi::c_void;
use core::sync::atomic::Ordering;
use crate::x4::types::{ConfigHierarchyNode, MappedViewVectorHeader, HRESULT, MemoryMappedViewEntry, PVOID, ProtectedSubsystemContext, LPVOID, __int64, HANDLE};

pub unsafe fn BuildSubsystemPathRecursive(
    p_context_out: *mut c_void,
    p_current_node: *mut ConfigHierarchyNode,
    p_out_path_buffer: *mut u8,
    max_buffer_length: u64,
) -> u8 {
    if p_out_path_buffer.is_null() {
        return 0;
    }
    *p_out_path_buffer = 0;

    if p_current_node.is_null() {
        return 0;
    }

    let current_node = &mut *p_current_node;
    let _result = BuildSubsystemPathRecursive(
        p_context_out,
        current_node.next_traverse_node,
        p_out_path_buffer,
        max_buffer_length,
    );

    let path_data_ptr = current_node.path_data;
    if path_data_ptr.is_null() {
        return _result;
    }

    let path_data = &mut *path_data_ptr;

    if path_data.node_tracking_id == 0 {
        path_data.node_tracking_id = dword_14045FCD0.fetch_add(1, Ordering::SeqCst) as u32 + 1;
    }

    // Check *((_DWORD *)pContextOut + 20) -> offset 80 bytes
    let context_u32_ptr = (p_context_out as *mut u8).add(80) as *mut u32;
    if *context_u32_ptr == 0 {
        // *((_OWORD *)pContextOut + 5) = *(_OWORD *)&path_data->node_tracking_id;
        let dest_oword = (p_context_out as *mut u8).add(80) as *mut u64;
        *dest_oword = path_data.node_tracking_id as u64;
        *dest_oword.add(1) = 0;

        // *((_QWORD *)pContextOut + 12) = path_data->unk_metric_0;
        let dest_qword12 = (p_context_out as *mut u8).add(96) as *mut u64;
        *dest_qword12 = path_data.unk_metric_0;
    }

    // *(_OWORD *)((char *)pContextOut + 104) = *(_OWORD *)&path_data->node_tracking_id;
    let dest_104 = (p_context_out as *mut u8).add(104) as *mut u64;
    *dest_104 = path_data.node_tracking_id as u64;
    *dest_104.add(1) = 0;

    // *((_QWORD *)pContextOut + 15) = path_data->unk_metric_0;
    let dest_120 = (p_context_out as *mut u8).add(120) as *mut u64;
    *dest_120 = path_data.unk_metric_0;

    let v10 = p_out_path_buffer.add(max_buffer_length as usize);

    let mut v12 = 0;
    while *p_out_path_buffer.add(v12) != 0 {
        v12 += 1;
    }
    let v13 = p_out_path_buffer.add(v12);

    if (v10 as usize) - (v13 as usize) > 2 {
        *v13 = 92; // ASCII backslash ('\\')
        let v14 = v13.add(1);
        let node_element_name = path_data.node_element_name;

        if !node_element_name.is_null() {
            let mut v11 = 0;
            while *node_element_name.add(v11) != 0 {
                v11 += 1;
            }

            let v16 = (v10 as usize) - (v14 as usize);
            let mut v17 = v11 + 1;
            if v17 >= v16 {
                v17 = v16;
            }

            memcpy_s(
                v14 as *mut c_void,
                v16,
                node_element_name as *const c_void,
                v17,
            );
            *v14.add(v17 - 1) = 0;
        }
    }

    1
}

pub unsafe fn InitializeProtectedSubsystem() -> HRESULT {
    let mut v0: HRESULT = 0;
    let process_heap = x4_getprocessheap();
    let context_ptr = x4_heapalloc(process_heap, 0, 16) as *mut ProtectedSubsystemContext;
    let old_context = context_ptr;
    let mut context_to_clean: *mut ProtectedSubsystemContext = core::ptr::null_mut();

    if !context_ptr.is_null() {
        (*context_ptr).allocated_capacity = 0;
        (*context_ptr).current_element_count = 0;
        (*context_ptr).pMappedViewsArray = core::ptr::null_mut();

        let cs_ptr = stru_14046B4F8;
        if x4_initializecriticalsectionandspincount(cs_ptr, 0) != 0 {
            core::ptr::addr_of_mut!(dword_14046B4DC).write_volatile(1);
            pContext = *old_context;
            v0 = 0;
        } else {
            let last_error = x4_getlasterror() as i32;
            v0 = last_error;
            if last_error != 0 {
                if last_error > 0 {
                    v0 = (last_error as u16 as i32) | 0x80070000;
                }
            } else {
                v0 = -2147467259; // E_FAIL
            }
            context_to_clean = old_context;
            HandleSubsystemError(v0);
        }
    } else {
        v0 = -2147024882; // E_OUTOFMEMORY (ERROR_NOT_ENOUGH_MEMORY)
        HandleSubsystemError(v0);
    }

    LogTraceEvent(v0);

    if !context_to_clean.is_null() {
        ProtectedSubsystemTeardownAndSanitize(context_to_clean);
    }

    v0
}

pub unsafe fn ProtectedSubsystemTeardownAndSanitize(
    p_context: *mut ProtectedSubsystemContext,
) -> *mut ProtectedSubsystemContext {
    if p_context.is_null() {
        return p_context;
    }

    ResizeMemoryMappedViewArray((*p_context).pMappedViewsArray, 0);

    ProcessReloc_Rva3_Len36(
        pRelocBlockRva,
        pTargetStateRva,
    );
    ProcessReloc_Rva36_Len3(
        pRelocHeader,
        pRvaTargetState,
    );

    let p_mapped_views_array = (*p_context).pMappedViewsArray;

    ProcessReloc_Rva28_Len0(
        stru_14044FE38,
        dword_140460968,
    );

    if !p_mapped_views_array.is_null() {
        ProcessReloc_Rva36_Len0(
            stru_140444E38,
            dword_140465D98,
        );

        let process_heap = x4_getprocessheap();

        ProcessReloc_Rva33_Len3(
            stru_140436B60,
            dword_140466218,
        );

        x4_heapfree(process_heap, 0, p_mapped_views_array as LPVOID);

        ProcessReloc_Rva3_Len31(
            stru_14044E300,
            dword_140465618,
        );

        (*p_context).pMappedViewsArray = core::ptr::null_mut();

        ProcessReloc_Rva8_Len36(
            stru_140436E98,
            dword_140466500,
        );
    }

    let v4 = x4_getprocessheap();
    ProcessReloc_Rva0_Len36(
        stru_14044F7D8,
        pTargetState,
    );

    x4_heapfree(v4, 0, p_context as *mut c_void);

    ProcessReloc_Rva2_Len30(
        stru_140435FA8,
        dword_14046564C,
    );

    p_context
}

pub unsafe fn RegisterProtectedSubsystemCallbacks() -> PVOID {
    let node_ptr = core::ptr::addr_of_mut!(GlobalSecureRegistrationNode) as *mut c_void;
    let master_head_ptr = core::ptr::addr_of_mut!(GlobalMasterCallbackListHead);

    // Link the new node into the master callback list head
    GlobalSecureRegistrationNode = *master_head_ptr as *mut c_void;
    *master_head_ptr = node_ptr as usize as __int64;

    // Encode pointers for secure execution tracking
    let encoded_init = x4_encodepointer(InitializeProtectedSubsystem as *mut c_void);
    core::ptr::addr_of_mut!(GlobalSecureRegistrationNode_pEncodedInitFunc)
        .write_volatile(encoded_init as usize as __int64);

    let result = x4_encodepointer(ShutdownProtectedSubsystem as *mut c_void);

    core::ptr::addr_of_mut!(GlobalSecureRegistrationNode_registration_flags).write_volatile(0);
    core::ptr::addr_of_mut!(GlobalSecureRegistrationNode_pEncodedShutdownFunc)
        .write_volatile(result as usize as __int64);

    result
}

pub unsafe fn ResizeMemoryMappedViewArray(
    p_vector: *mut MappedViewVectorHeader,
    target_size: i32,
) -> HRESULT {
    if p_vector.is_null() {
        return 0;
    }

    let vector = &mut *p_vector;
    let mut v2: HRESULT = 0;
    let v3 = target_size as usize;
    let mut v5: *mut MemoryMappedViewEntry = core::ptr::null_mut();

    if vector.allocated_capacity != target_size {
        let mut current_element_count = vector.current_element_count;
        if target_size < current_element_count {
            current_element_count = target_size;
        }

        if target_size > 0 {
            LogTraceEvent(0);
            let process_heap = x4_getprocessheap();
            let v8 = x4_heapalloc(process_heap, 0, 16 * v3) as *mut MemoryMappedViewEntry;
            v5 = v8;

            if v8.is_null() {
                v2 = -2147024882; // E_OUTOFMEMORY
                HandleSubsystemError(-2147024882);
                goto_cleanup!(v2);
            }

            if current_element_count > 0 && !vector.pArrayBuffer.is_null() {
                memcpy(
                    v8 as *mut c_void,
                    vector.pArrayBuffer as *const c_void,
                    (16 * current_element_count) as usize,
                );
            }
        }

        if target_size < vector.current_element_count {
            let mut v9 = target_size;
            let mut v10 = target_size as usize;

            while v9 < vector.current_element_count {
                let v11 = vector.pArrayBuffer.add(v10);
                if !v11.is_null() {
                    let h_file_mapping_object = (*v11).hFileMappingObject;
                    // Check if handle is valid (not null and not INVALID_HANDLE_VALUE -1)
                    let val = h_file_mapping_object as usize;
                    if val != 0 && val != usize::MAX {
                        x4_closehandle(h_file_mapping_object);
                        (*v11).hFileMappingObject = (-1isize) as HANDLE;
                    }

                    if !(*v11).pMappedBaseAddress.is_null() {
                        x4_unmapviewoffile((*v11).pMappedBaseAddress);
                        (*v11).pMappedBaseAddress = core::ptr::null_mut();
                    }
                }
                v9 += 1;
                v10 += 1;
            }
        }

        let p_array_buffer = vector.pArrayBuffer;
        if !p_array_buffer.is_null() {
            let v14 = x4_getprocessheap();
            x4_heapfree(v14, 0, p_array_buffer as *mut c_void);
        }

        if v5.is_null() {
            v5 = core::ptr::null_mut();
        }

        vector.pArrayBuffer = v5;
        vector.current_element_count = current_element_count;
        vector.allocated_capacity = target_size;
    }

    LogTraceEvent(v2);
    return v2;

    // Macro-like block matching goto LABEL_22
    // Rust uses labeled blocks or direct falls through
}

macro_rules! goto_cleanup {
    ($v2:expr) => {
        LogTraceEvent($v2);
        return $v2;
    };
}
use goto_cleanup;
use crate::devices::audio::mute;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{memcpy, x4_getprocessheap, x4_encodepointer, x4_heapfree, memcpy_s, x4_heapalloc, x4_getlasterror, x4_initializecriticalsectionandspincount, x4_deletecriticalsection, x4_closehandle, x4_unmapviewoffile};
use crate::x4::globals::{pContext, GlobalSecureRegistrationNode, GlobalMasterCallbackListHead, GlobalSecureRegistrationNode_pEncodedInitFunc, GlobalSecureRegistrationNode_registration_flags, GlobalSecureRegistrationNode_pEncodedShutdownFunc};
use crate::x4::reloc::{ProcessReloc_Rva0_Len36, ProcessReloc_Rva28_Len0, ProcessReloc_Rva2_Len30, ProcessReloc_Rva33_Len3, ProcessReloc_Rva36_Len0, ProcessReloc_Rva36_Len3, ProcessReloc_Rva3_Len31, ProcessReloc_Rva3_Len36, ProcessReloc_Rva8_Len36};



pub unsafe fn ShutdownProtectedSubsystem() -> i64 {
    let mut context = core::ptr::addr_of!(pContext).read_volatile();
    if !context.is_null() {
        ProtectedSubsystemTeardownAndSanitize(context);
        core::ptr::addr_of_mut!(pContext).write_volatile(*core::ptr::null_mut());
    }

    let dword_val = core::ptr::addr_of!(dword_14046B4DC).read_volatile();
    if dword_val != 0 {
        let cs_ptr = core::ptr::addr_of_mut!(stru_14046B4F8) as *mut c_void;
        x4_deletecriticalsection(cs_ptr);
        core::ptr::addr_of_mut!(dword_14046B4DC).write_volatile(0);
    }

    LogTraceEvent(0);
    0
}