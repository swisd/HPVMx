use alloc::format;
use alloc::string::{String, ToString};
use crate::error::error;

pub fn open_file_or_lib(path: &str) -> String {
    error(&format!("cant open file {}", path));
    "".to_string()
}