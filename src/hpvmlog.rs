use uefi::proto::console::text::Color;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};
use uefi::mem::memory_map::MemoryMap;
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
pub static mut VERBOSE_DEBUG_ENABLED: bool = true;
pub static mut BUSY_TSC: u64 = 0;

pub  static mut LOCK_AUTOPREFIX: bool = true;

pub fn set_verbose_debug(enabled: bool) {
    unsafe {
        VERBOSE_DEBUG_ENABLED = enabled;
    }
}

pub fn verbose_debug_enabled() -> bool {
    unsafe { VERBOSE_DEBUG_ENABLED }
}

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

impl Persistable for Vec<LogEntry> {
    fn magic() -> u32 { 0x474F4C48 } // "HLOG" in hex

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend_from_slice(&(self.len() as u32).to_le_bytes());
        for entry in self {
            data.push(entry.level as u8);
            data.extend_from_slice(&(entry.tag.len() as u32).to_le_bytes());
            data.extend_from_slice(entry.tag.as_bytes());
            data.extend_from_slice(&(entry.message.len() as u32).to_le_bytes());
            data.extend_from_slice(entry.message.as_bytes());
        }
        data
    }
}



#[macro_export] macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        {
            unsafe {
            let msg = alloc::format!($($arg)*);
            if crate::hpvmlog::LOCK_AUTOPREFIX {
                let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                $crate::hpvmlog::push_log($color, &tag, &msg);
            } else {
                $crate::hpvmlog::push_log($color, $prefix, &msg);
            }

            if !crate::hpvmlog::LOGGING_SILENCED {
                if crate::hpvmlog::LOCK_AUTOPREFIX {
                    uefi::system::with_stdout(|stdout| {
                use core::fmt::Write;
                let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
                let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                let _ = write!(stdout, "[{}] ", tag);
                match $color {
                    uefi::proto::console::text::Color::Yellow => {}
                    uefi::proto::console::text::Color::Red => {}
                    _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
                }
                let _ = write!(stdout, "{}", msg);
                let _ = write!(stdout, "\n");
                // let _ = write!(stdout, "{}MB", $crate::hpvmlog::getmem());
                // let _ = write!(stdout, "\n\n");
                let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            })
                } else {
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
                // let _ = write!(stdout, "{}MB", $crate::hpvmlog::getmem());
                // let _ = write!(stdout, "\n\n");
                let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            })
                    }
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

#[macro_export] macro_rules! vdebug {
    ($tag:expr, $($arg:tt)*) => {
        {
            unsafe {
                let msg = alloc::format!($($arg)*);
                if crate::hpvmlog::LOCK_AUTOPREFIX {
                    let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                    $crate::hpvmlog::push_log(uefi::proto::console::text::Color::White, &tag, &msg);
                } else {
                    $crate::hpvmlog::push_log(uefi::proto::console::text::Color::White, $tag, &msg);
                }
                if crate::hpvmlog::VERBOSE_DEBUG_ENABLED && !crate::hpvmlog::LOGGING_SILENCED {
                    if crate::hpvmlog::LOCK_AUTOPREFIX {
                        let msg = alloc::format!($($arg)*);
                    uefi::system::with_stdout(|stdout| {
                        use core::fmt::Write;
                        let _ = stdout.set_color(uefi::proto::console::text::Color::LightBlue, uefi::proto::console::text::Color::Black);
                        let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                        let _ = write!(stdout, "[{}] ", tag);
                        let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "{}", msg);
                        let _ = write!(stdout, "\n");
                    });
                    } else {
                    let msg = alloc::format!($($arg)*);
                    uefi::system::with_stdout(|stdout| {
                        use core::fmt::Write;
                        let _ = stdout.set_color(uefi::proto::console::text::Color::LightBlue, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "[{}] ", $tag);
                        let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "{}", msg);
                        let _ = write!(stdout, "\n");
                    });
                        }
                }
            }
        }
    };
}


pub trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for Color {
    #[inline]
    fn as_color(&self) -> Color {
        *self
    }
}

impl AsColor for u8 {
    #[inline]
    fn as_color(&self) -> Color {
        color_from_index(*self as usize)
    }
}

impl AsColor for u16 {
    #[inline]
    fn as_color(&self) -> Color {
        color_from_index(*self as usize)
    }
}

impl AsColor for u32 {
    #[inline]
    fn as_color(&self) -> Color {
        color_from_index(*self as usize)
    }
}

impl AsColor for usize {
    #[inline]
    fn as_color(&self) -> Color {
        color_from_index(*self)
    }
}

impl AsColor for i32 {
    #[inline]
    fn as_color(&self) -> Color {
        if *self < 0 {
            Color::White
        } else {
            color_from_index(*self as usize)
        }
    }
}

pub const fn color_from_index(index: usize) -> Color {
    match index {
        0 => Color::Black,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Cyan,
        4 => Color::Red,
        5 => Color::Magenta,
        6 => Color::Brown,
        7 => Color::LightGray,
        8 => Color::DarkGray,
        9 => Color::LightBlue,
        10 => Color::LightGreen,
        11 => Color::LightCyan,
        12 => Color::LightRed,
        13 => Color::LightMagenta,
        14 => Color::Yellow,
        15 => Color::White,
        _ => Color::White,
    }
}

#[macro_export] macro_rules! vdebug_autoprefix {
    ($color_idx:literal, $fmt:literal $(, $($arg:tt)*)?) => {
        {
            unsafe {
                use $crate::hpvmlog::AsColor;
                let color = ($color_idx).as_color();
                let msg = alloc::format!($fmt $(, $($arg)*)?);
                let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                $crate::hpvmlog::push_log(color, &tag, &msg);
                if crate::hpvmlog::VERBOSE_DEBUG_ENABLED && !crate::hpvmlog::LOGGING_SILENCED {
                    let msg = alloc::format!($fmt $(, $($arg)*)?);
                    uefi::system::with_stdout(|stdout| {
                        use core::fmt::Write;
                        let _ = stdout.set_color(color, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "[{}] ", tag);
                        match color {
                            uefi::proto::console::text::Color::Yellow
                            | uefi::proto::console::text::Color::Red
                            | uefi::proto::console::text::Color::LightRed
                            | uefi::proto::console::text::Color::LightGreen
                            | uefi::proto::console::text::Color::Green => {}
                            _ => {
                                let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                            }
                        }
                        let _ = write!(stdout, "{}", msg);
                        let _ = write!(stdout, "\n");
                        let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                    });
                }
            }
        }
    };
    ($fmt:literal $(, $($arg:tt)*)?) => {
        {
            unsafe {
                let msg = alloc::format!($fmt $(, $($arg)*)?);
                let tag = module_path!().strip_prefix("HPVMx::").unwrap_or(module_path!()).replace("::", ".");
                $crate::hpvmlog::push_log(uefi::proto::console::text::Color::White, &tag, &msg);
                if crate::hpvmlog::VERBOSE_DEBUG_ENABLED && !crate::hpvmlog::LOGGING_SILENCED {
                    let msg = alloc::format!($fmt $(, $($arg)*)?);
                    uefi::system::with_stdout(|stdout| {
                        use core::fmt::Write;
                        let _ = stdout.set_color(uefi::proto::console::text::Color::LightBlue, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "[{}] ", tag);
                        let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
                        let _ = write!(stdout, "{}", msg);
                        let _ = write!(stdout, "\n");
                    });
                }
            }
        }
    };
}

pub fn getmem() -> u32 {
    let mut free_phys_memory_mb = 0;
    let mut total_phys_memory_mb = 0;
    match uefi::boot::memory_map(uefi::boot::MemoryType::LOADER_DATA) {
        Ok(map) => {
            for entry in map.entries() {
                let size_mb = (entry.page_count * 4096) / (1024 * 1024);
                // If TOTAL_PHYSICAL_MEMORY_MB wasn't captured correctly at boot, accumulate it here as fallback
                if total_phys_memory_mb == 0 {
                    total_phys_memory_mb += size_mb as u32;
                }
                if entry.ty == uefi::boot::MemoryType::CONVENTIONAL {
                    free_phys_memory_mb += size_mb as u32;
                }
            }
        }
        Err(_) => {
            if total_phys_memory_mb == 0 {
                total_phys_memory_mb = 1024; // Fallback
            }
            free_phys_memory_mb = 512;
        }
    }
    free_phys_memory_mb
}