use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use uefi::proto::media::file::FileHandle;
use crate::filesystem::FileSystem;
use crate::{vdebug, vdebug_autoprefix};

#[macro_export] macro_rules! println {
    ($($arg:tt)*) => ({
        use crate::vdebug;
        vdebug!("xdb", $($arg)*)
    })
 }

#[macro_export] macro_rules! eprintln {
    ($($arg:tt)*) => ({
        use crate::vdebug;
        vdebug!("xdb", $($arg)*)
    })
}

pub fn file_create(path: &str) -> Result<FileHandle, &'static str> {
    FileSystem::create(path)
}

pub fn file_write_append_bytes(path: &str, bytes: &[u8]) -> Result<(), ()> {
    FileSystem::write_to_file_bytes(path, bytes, 'a');
    Ok(())
}

pub fn file_open(path: &str) -> Vec<u8> {
    if let Ok(data) = FileSystem::read_file(path) {
        vdebug_autoprefix!(0, "File Open OK {path}");
        data
    } else {
        vdebug_autoprefix!(4, "File Open Failed {path}");
        vec![0]
    }
}

pub fn local_format_timestamp_string() -> String {
    let time = uefi::runtime::get_time().unwrap();
    format!("{}-{}-{}@{}:{}:{}", time.year(), time.month(), time.day(), time.hour(), time.minute(), time.second())
}