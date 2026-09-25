use core::ffi::c_void;
use crate::x4::dxgi::buffer::ReallocVectorBufferWithCacheAlignment64;
use crate::x4::externals::{memcpy_s, AcquireSRWLockExclusive, ReleaseSRWLockExclusive};
use crate::x4::globals::pQueueMgr;

pub unsafe fn SppQueueRegistrationPacket(
    pControlFlags: *mut i32,
    actionType: i32,
    subsystemStatus: i32,
) {
    if pQueueMgr.status_flag != 0 {
        AcquireSRWLockExclusive(&mut pQueueMgr.lock as *mut usize);

        let hi_dword_config = (pQueueMgr.unk_config_flags >> 32) as i32;

        if subsystemStatus != 0 && subsystemStatus == hi_dword_config {
            let registration_packet = RegistrationPacket {
                action_type: actionType,
                reserved: 0,
                p_control_flags: pControlFlags,
            };

            let vector_ptr = pQueueMgr.pad_64_87.as_mut_ptr() as *mut c_void;

            if ReallocVectorBufferWithCacheAlignment64(vector_ptr, 0x10) {
                let vec_begin = *(&pQueueMgr.pad_64_87[8] as *const _ as *const *mut c_void);
                let vec_end = *(&pQueueMgr.pad_64_87[16] as *const _ as *const *mut c_void);

                let available_capacity = if (vec_begin as usize) < (vec_end as usize) {
                    (vec_end as usize) - (vec_begin as usize)
                } else {
                    0
                };

                memcpy_s(
                    vec_begin,
                    available_capacity,
                    &registration_packet as *const _ as *const c_void,
                    0x10,
                );

                let p_vec_begin = &mut pQueueMgr.pad_64_87[8] as *mut _ as *mut usize;
                *p_vec_begin += 16;
            }
        } else if !pControlFlags.is_null() {
            let mask = if actionType != 0 { -5 } else { -2111 };
            core::intrinsics::atomic_and_seqcst(pControlFlags, mask);
        }

        ReleaseSRWLockExclusive(&mut pQueueMgr.lock as PSRWLOCK);
    }
}