use core::arch::x86_64::_rdtsc;
use crate::terminal::shutdown_handler;
use crate::{vdebug, vdebug_autoprefix};

pub fn with_modifier<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    vdebug_autoprefix!("pre-exec");
    let r = f();
    vdebug_autoprefix!("post-exec");
    r
}

pub unsafe fn time_execution<F, R>(n: &str, a: F) -> R
where
    F: FnOnce() -> R,
{
    let s = _rdtsc();
    let r = a();
    let d = _rdtsc().saturating_sub(s);
    vdebug_autoprefix!("process '{n}' took {d} cycles");
    r
}


