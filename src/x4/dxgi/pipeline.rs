use crate::x4::dxgi::GetOrCreateRenderContextBlock;
use crate::x4::dxgi::lane::{CleanupFrameTracking, FreeLanePayloadBuffers, InitContextInternalPools, SwapLaneSnapshots};
use crate::x4::externals::{x4_deletecriticalsection, x4_getcurrentthreadid, x4_getprocessheap, x4_heapfree};
use crate::x4::globals::{GlobalPfnPresentFallback, GlobalPfnPresentPrimary, _guard_check_icall_fptr};
use crate::x4::types::{PipelineBindingDescriptor, PipelineCoordinator, RenderContextBlock, SharedContextBlock, ThreadContextNode};

pub unsafe fn PipelineCoordinatorDestroy(a1: *mut PipelineCoordinator) {
    // Allocate pDestSlot on the stack (uninitialized or default-initialized depending on your needs)
    let mut pDestSlot = core::mem::MaybeUninit::<SharedContextBlock>::uninit().assume_init();

    // Initialize the stack-allocated context
    InitContextInternalPools(&mut pDestSlot);

    // lane_array[0]
    if (*a1).lane_array[0].is_dirty_or_active != 0 {
        SwapLaneSnapshots(
            &mut pDestSlot.lanes[0] as *mut _,
            &mut (*a1).lane_array[0] as *mut _
        );
    }

    // lane_array[1]
    if (*a1).lane_array[1].is_dirty_or_active != 0 {
        SwapLaneSnapshots(
            &mut pDestSlot.lanes[1] as *mut _,
            &mut (*a1).lane_array[1] as *mut _
        );
    }

    // lane_array[2]
    if (*a1).lane_array[2].is_dirty_or_active != 0 {
        SwapLaneSnapshots(
            &mut pDestSlot.lanes[2] as *mut _,
            &mut (*a1).lane_array[2] as *mut _
        );
    }

    // Run cleanups on the context
    CleanupFrameTracking(&mut pDestSlot);
    FreeLanePayloadBuffers(&mut pDestSlot);

    // Handle dynamic state array heap allocation
    let dynamic_state_array = (*a1).dynamic_state_array;
    (*a1).dynamic_state_array = core::ptr::null_mut();

    if !dynamic_state_array.is_null() {
        let ProcessHeap = x4_getprocessheap();
        x4_heapfree(ProcessHeap, 0, dynamic_state_array as *mut _);
    }

    // Clean up concurrency lock and bucket buffers
    x4_deletecriticalsection(&mut (*a1).lane_lock);
    FreeLanePayloadBuffers((*a1).thread_buckets);
}

pub unsafe fn GetThreadLocalPipelineContext() -> *mut ThreadContextNode {
    let mut v0 = qword_14046B280;
    let mut v1: *mut ThreadContextNode = core::ptr::null_mut();

    if v0 != 0 {
        // !*(_QWORD *)(qword_14046B280 + 8) -> dereferencing offset +8 bytes
        let second_slot_ptr = (v0 + 8) as *mut *mut RenderContextBlock;

        if (*second_slot_ptr).is_null() {
            let v2 = *(v0 as *mut *const i8);
            let mut ppOutContext: *mut RenderContextBlock = core::ptr::null_mut();

            // Assuming GetOrCreateRenderContextBlock returns an HRESULT or signed int (>= 0 is success)
            if GetOrCreateRenderContextBlock(v2, &mut ppOutContext) >= 0 && (*second_slot_ptr).is_null() {
                *second_slot_ptr = ppOutContext;
            }
        }

        // v3 = (PipelineCoordinator *)((*(_QWORD *)(v0 + 8) + 32LL) & -(__int64)(*(_QWORD *)(v0 + 8) != 0));
        let render_context = *second_slot_ptr;
        let condition_mask = if !render_context.is_null() { -1i64 } else { 0i64 } as u64;
        let v3 = ((render_context as u64).wrapping_add(32) & condition_mask) as *mut PipelineCoordinator;

        if !v3.is_null() {
            let CurrentThreadId = x4_getcurrentthreadid();

            // thread_buckets[CurrentThreadId % 0xAuLL]
            let bucket_index = (CurrentThreadId as u64 % 0xAu64) as usize;
            let mut i = (*v3).thread_buckets[bucket_index];

            while !i.is_null() {
                if (i).thread_id == CurrentThreadId {
                    // C++ checks if the *address* of the field is null: &i->thread_payload_ptr == nullptr
                    // In standard cases this won't be null unless 'i' was offset, but we reproduce it exactly.
                    let thread_payload_ptr_addr = core::ptr::addr_of_mut!((i).thread_payload_ptr);
                    let v6 = thread_payload_ptr_addr.is_null();

                    let v7 = thread_payload_ptr_addr as *mut ThreadContextNode;
                    v1 = v7;

                    if !v6 && (*v7).next_node.is_null() {
                        // ((char *)&v3->unk_flags_or_id + 4)
                        let unk_flags_addr = core::ptr::addr_of!((*v3).unk_flags_or_id) as *const i8;
                        (*v7).next_node = unk_flags_addr.add(4) as *mut ThreadContextNode;
                    }
                    return v1;
                }
                i = *(i).next_node;
            }
        }
    }
    v1
}

pub unsafe fn BindPipelineShaderResources(
    pArray: *mut PipelineBindingDescriptor,
    resourceCount: i64,
) -> i64 {
    let mut result: i64 = 0;

    if resourceCount != 0 {
        let mut v2 = resourceCount;

        // ResourceCount = &pArray->ResourceCount;
        // We track it as a raw `*mut u16` to strictly preserve the exact pointer math (+4 loops)
        let mut resource_count_ptr = core::ptr::addr_of_mut!((*pArray).ResourceCount);

        loop {
            // v4 = GlobalPfnPresentPrimary;
            // Transmuting raw data or globally shared variables to our required function footprint
            let mut v4: Option<unsafe extern "fastcall" fn(u32, u32, u32, u32) -> i64> =
                core::mem::transmute(GlobalPfnPresentPrimary);

            // v5 = ResourceCount[1]; 
            // In a u16 array pointer, index [1] sits 2 bytes ahead. 
            // However, the C++ code reads it as a DWORD (unsigned int), so we cast the pointer at that offset.
            let v5 = *(resource_count_ptr.add(1) as *const u32);

            // v6 = *ResourceCount;
            let v6 = *resource_count_ptr as u32;

            // v7 = *((_DWORD *)ResourceCount - 1);
            // Steps backward 4 bytes (size of one DWORD) relative to ResourceCount
            let v7 = *(resource_count_ptr.byte_sub(4) as *const u32);

            if GlobalPfnPresentPrimary != 0 || {
                v4 = core::mem::transmute(GlobalPfnPresentFallback);
                v4.is_some()
            } {
                // Call Windows Control Flow Guard validation target check if available
                _guard_check_icall_fptr();

                if let Some(func) = v4 {
                    result = func(v7, v6, v5, 0);
                }
            }

            // ResourceCount += 4; (Moves forward 4 * sizeof(u16) = 8 bytes)
            resource_count_ptr = resource_count_ptr.add(4);
            v2 -= 1;

            if v2 == 0 {
                break;
            }
        }
    }
    result
}