use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::String;
use core::arch::x86_64::_rdtsc;
use core::ffi::c_void;
use core::ptr::null;
use core::time::Duration;
use log::error;
use crate::x4;
use crate::x4::ops::to_u16s;

// #[derive(Debug)]
// pub struct Thread {
//     handle: Handle,
// }
//
// type Handle = u64;
//
// impl Thread {
//
//     pub unsafe fn new(stack: usize, init: Box<ThreadInit>) -> Result<Thread, String> {
//         /*
//
//         let data = Box::into_raw(init);
//
//         let ret = unsafe {
//             let ret = c::CreateThread(
//                 ptr::null_mut(),
//                 stack,
//                 Some(thread_start),
//                 data as *mut _,
//                 c::STACK_SIZE_PARAM_IS_A_RESERVATION,
//                 ptr::null_mut(),
//             );
//
//             HandleOrNull::from_raw_handle(ret)
//
//             null()
//         };
//         return if let Ok(handle) = ret.try_into() {
//             Ok(Thread { handle: Handle::from_inner(handle) })
//         } else {
//             // The thread failed to start and as a result data was not consumed. Therefore, it is
//             // safe to reconstruct the box so that it gets deallocated.
//             unsafe { drop(Box::from_raw(data)) };
//             Err("failed to create thread".to_owned())
//         };
//
//         unsafe extern "system" fn thread_start(data: *mut c_void) -> u32 {
//             // SAFETY: we are simply recreating the box that was leaked earlier.
//             let init = unsafe { Box::from_raw(data as *mut ThreadInit) };
//             let rust_start = init.init();
//
//             // Reserve some stack space for if we otherwise run out of stack.
//             stack_overflow::reserve_stack();
//
//             rust_start();
//             Ok(0)
//         }
//          */
//         Err(format!("stack: {}, init: {:?}", stack, init))
//     }
//
//     pub fn join(self) {
//         let rc = unsafe { c::WaitForSingleObject(self.handle.as_raw_handle(), c::INFINITE) };
//         if rc == c::WAIT_FAILED {
//             panic!("failed to join on thread: {}", io::Error::last_os_error());
//         }
//     }
//
//     pub fn handle(&self) -> &Handle {
//         &self.handle
//     }
//
//     pub fn into_handle(self) -> Handle {
//         self.handle
//     }
// }
//
// #[derive(Debug)]
// pub(crate) struct ThreadInit {
//     pub handle: Thread,
//     pub _start: Box<dyn FnOnce() + Send>,
// }
//
// impl ThreadInit {
//     pub fn init(self: Box<Self>) -> Box<dyn FnOnce() + Send> {
//
//         if let Err(_thread) = set_current(self.handle.clone()) {
//             // rtabort!("current thread handle already set during thread spawn");
//         }
//
//         if let Some(name) = self.handle.cname() {
//             set_name(name);
//         }
//
//         self._start
//     }
// }
//
// pub fn set_current(thread: Thread) -> Result<(), Thread> {
//     if CURRENT.get() != NONE {
//         return Err(thread);
//     }
//
//     match id::get() {
//         Some(id) if id == thread.id() => {}
//         None => id::set(thread.id()),
//         _ => return Err(thread),
//     }
//
//     // Make sure that `crate::rt::thread_cleanup` will be run, which will
//     // call `drop_current`.
//     crate::sys::thread_local::guard::enable();
//     CURRENT.set(thread.into_raw().cast_mut());
//     Ok(())
// }


pub fn current_os_id() -> Option<u64> {
    // SAFETY: FFI call with no preconditions.
    let id: u32 = unsafe { /*get_current_thread_id()*/ 0};

    // A return value of 0 indicates failed lookup.
    if id == 0 { None } else { Some(id.into()) }
}

pub fn set_name(name: String) {
    if let Ok(utf16) = to_u16s(name) {
        unsafe {
            // SAFETY: the vec returned by `to_u16s` ends with a zero value
            set_name_u16(&utf16)
        }
    };
}

/// # Safety
///
/// `name` must end with a zero value
pub unsafe fn set_name_u16(name: &[u16]) {
    unsafe { /*far_fn set_thread_description(far_fn get_current_thread(), name.as_ptr())*/ };
}

pub fn sleep(dur: Duration) {
    fn high_precision_sleep(dur: Duration) -> Result<(), String> {
        let timer: Option<Generic> = /*far_fn waitable_timer::high_resolution()?*/None;
        // timer.set(dur)?;
        // timer.wait()
        Err("unable to sleep".to_owned())
    }
    if dur.is_zero() {
        unsafe { /*far_fn Sleep(0)*/ };
    } else if high_precision_sleep(dur).is_err() {
        let start = /*Instant::now();*/unsafe { _rdtsc() };
        unsafe { /*far_fn Sleep*/(/*dur2timeout(dur)*/); 0 };

        if start/*unk .elapsed()*/.saturating_sub(unsafe{_rdtsc()}) < dur.as_nanos() as u64 {
            unsafe { /*far_fn Sleep(1)*/ };
        }
    }
}

pub fn yield_now() {
    unsafe {
        /*far_fn SwitchTo */
    }
}


pub type Generic = c_void;