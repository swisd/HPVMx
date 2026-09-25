use core::ffi::c_void;
use crate::x4::error::{HandleSubsystemError, LogTraceEvent};
use crate::x4::externals::{GetCurrentThreadId, GetProcessHeap, HeapAlloc, HeapFree, LeaveCriticalSection, RaiseException, ReleaseSemaphore, SetEvent, Sleep, WaitForSingleObject};
use crate::x4::types::{SppCustomLock, SppCustomLockV2, ThreadSlot, CRITICAL_SECTION, HANDLE, DWORD, SIZE_T};

pub unsafe fn SppCustomLockAcquireExclusive(lock: *mut SppCustomLockV2) {
    if lock.is_null() {
        return;
    }

    let current_thread_id = GetCurrentThreadId();
    EnterCriticalSection(&mut (*lock).internalLock);

    let mut acquired = false;

    if (*lock).isWriterMode == 0 {
        (*lock).targetStateMarker = 1;
        (*lock).owningThreadId = current_thread_id;
        (*lock).recursionCount = 1;
        (*lock).isWriterMode = 1;
        acquired = true;
    } else if (*lock).targetStateMarker == 0 || (*lock).owningThreadId != current_thread_id {
        let thread_id = GetCurrentThreadId();
        let mut recursion_count = 0;

        if (*lock).isWriterMode != 0 {
            if (*lock).owningThreadId == thread_id {
                recursion_count = (*lock).recursionCount;
            } else if (*lock).targetStateMarker == 0 {
                EnterCriticalSection(&mut (*lock).internalLock);
                let mut index: usize = 0;

                if (*lock).maxSlotsCount > 0 {
                    let p_thread_slots = (*lock).pThreadSlotsArray;
                    while index < (*lock).maxSlotsCount as usize {
                        if (*p_thread_slots.add(index)).threadId == thread_id {
                            let slot_ptr = if !p_thread_slots.is_null() {
                                (*lock).pThreadSlotsArray
                            } else {
                                core::ptr::null_mut()
                            };

                            if !slot_ptr.is_null() {
                                recursion_count = (*slot_ptr.add(index)).slotRecursionCount;
                            }
                            break;
                        }
                        index += 1;
                    }
                }
                LeaveCriticalSection(&mut (*lock).internalLock);
            }

            if recursion_count != 0 {
                // STATUS_POSSIBLE_DEADLOCK
                RaiseException(0xC0000194, 0, 0, core::ptr::null());
            }
        }

        (*lock).queuedWaitersCount += 1;
    } else {
        (*lock).recursionCount += 1;
        acquired = true;
    }

    LeaveCriticalSection(&mut (*lock).internalLock);

    if !acquired {
        if WaitForSingleObject((*lock).hEvent, 0x337F9800) != 0 {
            RaiseException(0xC0000194, 0, 0, core::ptr::null());
        }
        (*lock).owningThreadId = current_thread_id;
    }
}

pub unsafe fn SppCustomLockAcquire(
    lock: *mut SppCustomLock,
    isExclusiveRequested: i32,
) -> bool {
    if lock.is_null() {
        return false;
    }

    let mut v4: *mut ThreadSlot = core::ptr::null_mut();
    let mut v5 = 0;
    let current_thread_id = GetCurrentThreadId();
    let mut v7 = true;

    EnterCriticalSection(&mut (*lock).internalLock);

    loop {
        if (*lock).isWriterMode == 0 {
            (*(*lock).inlineSlot).threadId = current_thread_id;
            (*(*lock).inlineSlot).slotRecursionCount = 1;
            v5 = 1;
            (*lock).isWriterMode = 1;
            break;
        }

        let inline_thread_id = (*(*lock).inlineSlot).threadId;

        if (*lock).recursionCount == 0 {
            let owning_thread_id = (*lock).owningThreadId;
            let mut v13: *mut ThreadSlot = core::ptr::null_mut();
            v4 = core::ptr::null_mut();

            if inline_thread_id == current_thread_id {
                v13 = (*lock).inlineSlot;
            }

            if owning_thread_id == 0 && inline_thread_id == 0 {
                v4 = (*lock).inlineSlot;
            }

            let slots_ptr = (*lock).pThreadSlotsArray;
            if !slots_ptr.is_null() {
                if v13.is_null() {
                    let mut v16: usize = 0;
                    let max_slots = (*lock).maxSlotsCount as usize;

                    if max_slots > 0 {
                        while (*slots_ptr.add(v16)).threadId != current_thread_id {
                            if v4.is_null() && (*slots_ptr.add(v16)).threadId == 0 {
                                v4 = slots_ptr.add(v16);
                            }
                            v16 += 1;
                            if v16 >= max_slots {
                                break;
                            }
                        }
                        if v16 < max_slots {
                            v13 = slots_ptr.add(v16);
                        }
                    }
                }
            }

            if !v13.is_null() {
                v4 = v13;
            }

            if v4.is_null() {
                SppCustomLockResizeSlots(lock);
                continue;
            }

            if (*v4).threadId == current_thread_id {
                (*v4).slotRecursionCount += 1;
            } else {
                if owning_thread_id != 0 {
                    break;
                }
                (*v4).threadId = current_thread_id;
                (*v4).slotRecursionCount = 1;
                (*lock).isWriterMode += 1;
            }

            v5 = 1;
            break;
        }

        if inline_thread_id == current_thread_id {
            (*(*lock).inlineSlot).slotRecursionCount += 1;
            v5 = 1;
            break;
        }

        let slots_ptr = (*lock).pThreadSlotsArray;
        v4 = core::ptr::null_mut();

        if !slots_ptr.is_null() {
            let mut v11: usize = 0;
            let max_slots = (*lock).maxSlotsCount as usize;

            if max_slots > 0 {
                while (*slots_ptr.add(v11)).threadId != 0 {
                    v11 += 1;
                    if v11 >= max_slots {
                        break;
                    }
                }
                if v11 < max_slots {
                    v4 = slots_ptr.add(v11);
                    if !v4.is_null() {
                        break;
                    }
                }
            }
        }

        SppCustomLockResizeSlots(lock);
    }

    if v5 == 0 {
        if isExclusiveRequested != 0 {
            if !v4.is_null() {
                (*v4).threadId = current_thread_id;
                (*v4).slotRecursionCount = 1;
            }
            (*lock).unknownFlag += 1;
        } else {
            v7 = false;
            v5 = 1;
        }
    }

    LeaveCriticalSection(&mut (*lock).internalLock);

    if v5 == 0 {
        if WaitForSingleObject((*lock).activeWritersCount as HANDLE, 0x337F9800) != 0 {
            // STATUS_POSSIBLE_DEADLOCK
            RaiseException(0xC0000194, 0, 0, core::ptr::null());
        }
    }

    v7
}



pub unsafe fn SppCustomLockResizeSlots(lock: *mut SppCustomLock) {
    if lock.is_null() {
        return;
    }

    let v1 = (*lock).pThreadSlotsArray as *const c_void;
    let mut v2: u32 = 0;
    let mut v3: i32 = 0;
    let v5: u32;
    let v6: u32;

    if v1.is_null() {
        v5 = 0;
        v6 = 3;
        let v7 = 24;
        v2 = v7;
    } else {
        v5 = (*lock).maxSlotsCount as u32;
        v6 = v5.wrapping_add(4);

        if v5 != 0xFFFFFFFC {
            let v7 = 8 * v6;
            if (v6 & 0x1FFFFFFF) == v6 {
                v2 = v7;
            } else {
                v3 = -2147024362; // INTSAFE_E_ARITHMETIC_OVERFLOW (0x80070216)
                HandleSubsystemError(-2147024362);
            }
        }
    }

    LogTraceEvent(v3);

    if v3 < 0 {
        // STATUS_INTEGER_OVERFLOW (0xC0000095)
        RaiseException(0xC0000095, 0, 0, core::ptr::null());
        return;
    }

    LeaveCriticalSection(&mut (*lock).internalLock);

    let process_heap = GetProcessHeap();
    let v9 = HeapAlloc(process_heap, 0, v2 as usize as SIZE_T) as *mut u8;

    if v9.is_null() {
        Sleep(10);
    } else {
        core::ptr::write_bytes(
            v9.add((8 * v5) as usize),
            0,
            (8 * (v6 - v5)) as usize,
        );

        EnterCriticalSection(&mut (*lock).internalLock);

        if v5 == (*lock).maxSlotsCount as u32 {
            if !v1.is_null() && v5 > 0 {
                core::ptr::copy_nonoverlapping(v1 as *const u8, v9, (8 * v5) as usize);
            }

            (*lock).pThreadSlotsArray = v9 as *mut ThreadSlot;
            (*lock).maxSlotsCount = v6 as i32;

            LeaveCriticalSection(&mut (*lock).internalLock);

            if !v1.is_null() {
                let heap_to_free = GetProcessHeap();
                HeapFree(heap_to_free, 0, v1 as *mut c_void);
            }
        } else {
            LeaveCriticalSection(&mut (*lock).internalLock);
            let heap_to_free = GetProcessHeap();
            HeapFree(heap_to_free, 0, v9 as *mut c_void);
        }
    }

    EnterCriticalSection(&mut (*lock).internalLock);
}


pub unsafe fn SppCustomLockRelease(lock: *mut SppCustomLock) {
    if lock.is_null() {
        return;
    }

    let current_thread_id = GetCurrentThreadId();
    EnterCriticalSection(&mut (*lock).internalLock);
    let mut should_leave_lock = true;

    if (*lock).recursionCount == 0 {
        let slot_ptr: *mut ThreadSlot;

        if (*(*lock).inlineSlot).threadId == current_thread_id {
            slot_ptr = (*lock).inlineSlot;
        } else {
            let slots_ptr = (*lock).pThreadSlotsArray;
            if slots_ptr.is_null() {
                // STATUS_RESOURCE_NOT_OWNED
                RaiseException(0xC0000264, 0, 0, core::ptr::null());
            }

            let max_slots = (*lock).maxSlotsCount as usize;
            if max_slots == 0 {
                RaiseException(0xC0000264, 0, 0, core::ptr::null());
            }

            let mut found_slot: *mut ThreadSlot = core::ptr::null_mut();
            let mut index: usize = 0;

            while index < max_slots {
                if (*slots_ptr.add(index)).threadId == current_thread_id {
                    found_slot = slots_ptr.add(index);
                    break;
                }
                index += 1;
            }

            if found_slot.is_null() {
                RaiseException(0xC0000264, 0, 0, core::ptr::null());
            }

            slot_ptr = found_slot;
        }

        let prev_recursion = (*slot_ptr).slotRecursionCount;
        (*slot_ptr).slotRecursionCount -= 1;

        if prev_recursion == 1 {
            (*slot_ptr).threadId = 0;
            let prev_writer_mode = (*lock).isWriterMode;
            (*lock).isWriterMode -= 1;

            if prev_writer_mode == 1 {
                let owning_thread_id = (*lock).owningThreadId;
                if owning_thread_id != 0 {
                    (*lock).recursionCount = 1;
                    (*lock).owningThreadId = owning_thread_id - 1;
                    (*(*lock).inlineSlot).threadId = 1;
                    (*(*lock).inlineSlot).slotRecursionCount = 1;
                    (*lock).isWriterMode = 1;

                    LeaveCriticalSection(&mut (*lock).internalLock);
                    SetEvent((*lock).hEvent);
                    return;
                }
            }
        }
    } else {
        let prev_recursion = (*(*lock).inlineSlot).slotRecursionCount;
        (*(*lock).inlineSlot).slotRecursionCount -= 1;

        if prev_recursion == 1 {
            (*(*lock).inlineSlot).threadId = 0;
            (*lock).isWriterMode -= 1;
            let unknown_flag = (*lock).unknownFlag;

            if unknown_flag != 0 {
                (*lock).recursionCount = 0;
                (*lock).unknownFlag = 0;
                should_leave_lock = false;
                (*lock).isWriterMode = unknown_flag as u32 as i32;

                LeaveCriticalSection(&mut (*lock).internalLock);
                ReleaseSemaphore((*lock).activeWritersCount, unknown_flag, core::ptr::null_mut());
            } else {
                let owning_thread_id = (*lock).owningThreadId;
                if owning_thread_id != 0 {
                    (*(*lock).inlineSlot).threadId = 1;
                    (*(*lock).inlineSlot).slotRecursionCount = 1;
                    (*lock).isWriterMode = 1;
                    should_leave_lock = false;
                    (*lock).owningThreadId = owning_thread_id - 1;

                    LeaveCriticalSection(&mut (*lock).internalLock);
                    SetEvent((*lock).hEvent);
                } else {
                    (*lock).recursionCount = 0;
                }
            }
        }
    }

    if should_leave_lock {
        LeaveCriticalSection(&mut (*lock).internalLock);
    }
}

fn EnterCriticalSection(p0: &mut CRITICAL_SECTION) {
    todo!()
}



