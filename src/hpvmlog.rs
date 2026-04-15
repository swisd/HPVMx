use uefi::proto::console::text::Color;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use crate::filesystem::State;
use crate::state::Persistable;

#[derive(Clone)]
pub struct LogEntry {
    pub level: Color,
    pub tag: String,
    pub message: String,
}

const MAX_LOGS: usize = 4096;
pub static mut LOG_BUFFER: Option<Vec<LogEntry>> = None;
static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
pub static mut LOGGING_SILENCED: bool = false;

pub fn init_log_buffer() {
    unsafe {
        LOG_BUFFER = Some(Vec::with_capacity(MAX_LOGS));
    }
}

pub fn push_log(level: Color, tag: &str, msg: &str) {
    unsafe {
        if let Some(ref mut buffer) = LOG_BUFFER {
            if buffer.len() >= MAX_LOGS {
                buffer.remove(0);
            }
            buffer.push(LogEntry {
                level,
                tag: String::from(tag),
                message: String::from(msg),
            });
            LOG_COUNT.fetch_add(1, Ordering::SeqCst);
        }
    }
}

pub fn get_logs() -> Vec<(Color, String, String)> {
    unsafe {
        if let Some(ref buffer) = LOG_BUFFER {
            buffer.iter().map(|e| (e.level, e.tag.clone(), e.message.clone())).collect()
        } else {
            Vec::new()
        }
    }
}

#[allow(static_mut_refs)]
pub unsafe fn get_log_buffer() -> &'static Option<Vec<LogEntry>> {
    unsafe {
        let option = &LOG_BUFFER;
        option
    }
}

impl Persistable for &'static Option<Vec<LogEntry>> {
    fn magic() -> u32 { 0x474F4C48 } // "HLOG" in hex

    fn get_heap_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let size = size_of::<Option<Vec<LogEntry>>>();
        let ptr = &self as *const _ as *const u8;
        unsafe {
            data.extend_from_slice(core::slice::from_raw_parts(ptr, size));
        }
        data
    }
}



#[macro_export] macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        {
            unsafe {
            let msg = alloc::format!($($arg)*);
            $crate::hpvmlog::push_log($color, $prefix, &msg);

            if !crate::hpvmlog::LOGGING_SILENCED {
            uefi::system::with_stdout(|stdout| {
                use core::fmt::Write;
                let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
                let _ = write!(stdout, "[{}] ", $prefix);
                match $color {
                    uefi::proto::console::text::Color::Yellow => {}
                    uefi::proto::console::text::Color::Red => {}
                    _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
                }
                let _ = write!(stdout, "{}", msg);
                let _ = write!(stdout, "\n");
                let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            })
                }
        }
            }
    };
}

#[macro_export] macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        {
            let msg = alloc::format!($($arg)*);
            $crate::hpvmlog::push_log(uefi::proto::console::text::Color::White, "", &msg);

            unsafe {
                if !crate::hpvmlog::LOGGING_SILENCED {
            uefi::system::with_stdout(|stdout| {
                use core::fmt::Write;
                let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                let _ = write!(stdout, $start);
                let _ = write!(stdout, "{}", msg);
                let _ = write!(stdout, "\n");
            })
        }
                }
            }
    }
}

#[macro_export] macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

#[macro_export] macro_rules! hpvm_warn {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Yellow, $tag, $($arg)*) };
}

// Added this to stop the "unused macro" warning
#[macro_export] macro_rules! hpvm_error {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Red, $tag, $($arg)*) };
}
