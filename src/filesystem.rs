use crate::Color;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::ffi::CStr;
use core::fmt::Write;
use uefi::boot::{ScopedProtocol, SearchType};
use uefi::data_types::CStr16;
use uefi::{Handle, Identify};
use uefi::proto::device_path::DevicePath;
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::media::file::{File, FileMode, FileAttribute, FileType, FileInfo};
use uefi::proto::media::fs::SimpleFileSystem;

macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            // Bring the trait into scope INSIDE the closure
            //use uefi::proto::console::text::Output;
            use core::fmt::Write;

            // let old_attribute = stdout.get_attribute().ok();

            // Set prefix color
            let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, "[{}] ", $prefix);

            // Reset to white for message
            match $color {
                Color::Yellow => {}
                Color::Red => {}
                _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
            }
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);

            // Restore original attributes if they existed
            // if let Some(attr) = old_attribute {
            //     let _ = stdout.set_attribute(attr);
            // }
        })
    };
}

macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            use core::fmt::Write;
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, $start);
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
        })
    }
}


/// Internal global state for CWD and Device Aliases
struct State {
    cwd: String,
    device_map: Vec<(String, String)>,
}

static mut STATE: Option<State> = None;

pub struct FileSystem;

impl FileSystem {
    #[allow(static_mut_refs)]
    /// Internal helper to access global state
    fn get_state() -> &'static mut State {
        unsafe {
            if STATE.is_none() {
                STATE = Some(State {
                    cwd: String::from("\\"),
                    device_map: Vec::new(),
                });
            }
            STATE.as_mut().unwrap()
        }
    }

    /// Resolves path based on Aliases (dev0:), Root-relative (/), or CWD
    fn resolve_path(path: &str) -> String {
        let state = Self::get_state();
        let mut resolved = path.to_string();

        // 1. Handle Device Aliases (ex: dev0:/file)
        for (alias, full_path) in &state.device_map {
            let prefix = format!("{}:", alias);
            if resolved.starts_with(&prefix) {
                return resolved.replace(&prefix, full_path).replace('/', "\\");
            }
        }

        // 2. Handle Root-relative (starts with /)
        if resolved.starts_with('/') || resolved.starts_with('\\') {
            return resolved.replace('/', "\\");
        }

        // 3. Handle Relative Path (append to CWD)
        let mut new_path = state.cwd.clone();
        if !new_path.ends_with('\\') {
            new_path.push('\\');
        }
        new_path.push_str(&resolved.replace('/', "\\"));
        new_path
    }

    /// Change current directory
    pub fn cd(path: &str) {
        let state = Self::get_state();
        state.cwd = Self::resolve_path(path);
    }

    /// Scans all drives and writes "alias -> path" to a file
    pub fn scan_and_map_devices(map_file_path: &str) -> Result<(), &'static str> {
        let handles = uefi::boot::locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID))
            .map_err(|_| "Failed to locate FS handles")?;

        let mut map_contents = String::new();
        let state = Self::get_state();
        state.device_map.clear();

        for (i, handle) in handles.as_slice().iter().enumerate() {
            let device_path_res: Result<ScopedProtocol<DevicePath>, _> = uefi::boot::open_protocol_exclusive(*handle);

            if let Ok(device_path) = device_path_res {
                let full_path = device_path
                    .to_string(DisplayOnly(false), AllowShortcuts(false))
                    .map_err(|_| "Path string error")?
                    .to_string();

                let alias = format!("dev{}", i);
                state.device_map.push((alias.clone(), full_path.clone()));
                map_contents.push_str(&format!("{} -> {}\n", alias, full_path));
                hpvm_info!("fs", "mapped device {} at {}", alias, full_path);
            }
        }

        Self::write_to_file(map_file_path, &map_contents, 'w')
    }

    pub fn mkdir(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;
        let path_cstr = Self::path_to_cstr16(path)?;

        root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::DIRECTORY)
            .map_err(|_| "Failed to create directory")?;
        Ok(())
    }

    pub fn touch(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;
        let path_cstr = Self::path_to_cstr16(path)?;

        root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Failed to create file")?;
        Ok(())
    }

    pub fn copy(src: &str, dst: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;

        // Open and read source
        let src_cstr = Self::path_to_cstr16(src)?;
        let src_handle = root.open(src_cstr, FileMode::Read, FileAttribute::empty()).map_err(|_| "Source not found")?;
        let mut src_file = src_handle.into_regular_file().ok_or("Source is not a file")?;

        let mut info_buf = [0u8; 512];
        let info = src_file.get_info::<FileInfo>(&mut info_buf).map_err(|_| "Info error")?;
        let mut buffer = Vec::new();
        buffer.resize(info.file_size() as usize, 0u8);
        src_file.read(&mut buffer).map_err(|_| "Read error")?;

        // Write to destination
        Self::write_to_file(dst, core::str::from_utf8(&buffer).unwrap_or(""), 'w')
    }

    pub fn move_file(src: &str, dst: &str) -> Result<(), &'static str> {
        Self::copy(src, dst)?;
        Self::remove(src)?;
        Ok(())
    }

    pub fn remove(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Open for delete failed")?;
        let mut file = handle.into_regular_file().ok_or("Not a file")?;

        file.delete().map_err(|_| "Delete failed")?;
        Ok(())
    }

    pub fn cat(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::Read, FileAttribute::empty()).map_err(|_| "Open failed")?;
        let mut file = handle.into_regular_file().ok_or("Not a file")?;

        let mut info_buf = [0u8; 512];
        let info = file.get_info::<FileInfo>(&mut info_buf).map_err(|_| "Info error")?;
        let mut buffer = Vec::new();
        buffer.resize(info.file_size() as usize, 0u8);
        file.read(&mut buffer).map_err(|_| "Read error")?;

        uefi::system::with_stdout(|stdout| {
            if let Ok(text) = core::str::from_utf8(&buffer) {
                let _ = write!(stdout, "{}", text);
            } else {
                for (i, byte) in buffer.iter().enumerate() {
                    if i % 16 == 0 { let _ = write!(stdout, "\n{:08X}: ", i); }
                    let _ = write!(stdout, "{:02X} ", byte);
                }
            }
        });
        Ok(())
    }

    pub fn clone_dir(src: &str, dst: &str) -> Result<(), &'static str> {
        Self::mkdir(dst)?;
        let mut root = Self::get_root()?;
        let src_cstr = Self::path_to_cstr16(src)?;

        let src_handle = root.open(src_cstr, FileMode::Read, FileAttribute::empty()).map_err(|_| "Open src dir failed")?;
        let mut src_dir = src_handle.into_directory().ok_or("Not a directory")?;

        let mut buffer = [0u8; 8192];
        loop {
            match src_dir.read_entry(&mut buffer) {
                Ok(Some(entry)) => {
                    let file_name = CStr16::to_string(entry.file_name());
                    if file_name == "." || file_name == ".." { continue; }

                    let src_path = format!("{}/{}", src, file_name);
                    let dst_path = format!("{}/{}", dst, file_name);

                    if entry.is_directory() {
                        Self::clone_dir(&src_path, &dst_path)?;
                    } else {
                        Self::copy(&src_path, &dst_path)?;
                    }
                }
                Ok(None) => break,
                Err(_) => return Err("Read entry error"),
            }
        }
        Ok(())
    }

    pub fn write_to_file(path: &str, data: &str, mode: char) -> Result<(), &'static str> {
        let mut root = Self::get_root()?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Open for write failed")?;
        let mut file = handle.into_regular_file().ok_or("Not a file")?;

        if mode == 'a' {
            file.set_position(0xFFFFFFFFFFFFFFFF).map_err(|_| "Seek error")?;
        }
        file.write(data.as_bytes()).map_err(|_| "Write error")?;
        Ok(())
    }

    // --- Private Helpers ---

    fn path_to_cstr16(path: &str) -> Result<&CStr16, &'static str> {
        let resolved = Self::resolve_path(path);
        let mut u16_path: Vec<u16> = resolved.encode_utf16().collect();
        u16_path.push(0);
        // Leak the vector to get a 'static reference (common in UEFI no_std logic for CStr16)
        let leaked = Box::leak(u16_path.into_boxed_slice());
        CStr16::from_u16_with_nul(leaked).map_err(|_| "Invalid CStr16 conversion")
    }

    fn get_root() -> Result<uefi::proto::media::file::Directory, &'static str> {
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().map_err(|_| "No FS handle")?;
        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle).map_err(|_| "FS open err")?;
        fs.open_volume().map_err(|_| "Volume open err")
    }

    pub fn get_drives(drive_ref_file_path: &str) {
        Self::cat(drive_ref_file_path).unwrap()
    }

    pub fn list_files() {
        // 1. Get the current resolved path from our state
        let state = Self::get_state();
        let current_path = &state.cwd;

        // 2. Locate the filesystem protocol
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().unwrap();
        let mut sfs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle).unwrap();
        let mut root_dir = sfs.open_volume().unwrap();

        // Keep the original root volume message at the top
        message!("\n", "ROOT {:?}", root_dir);
        message!("", "CONTENTS OF: {}\n", current_path);

        // 3. If CWD is not the root, we need to open the specific subdirectory
        let mut target_dir = if current_path == "\\" || current_path == "/" {
            root_dir
        } else {
            let path_u16 = Self::path_to_cstr16(current_path);
            let path_cstr = path_u16.unwrap();

            let handle = root_dir.open(path_cstr, FileMode::Read, FileAttribute::DIRECTORY)
                .expect("Failed to open current directory");

            handle.into_directory().expect("Path is not a directory")
        };

        // 4. Buffer for directory entries
        let mut buffer = [0u8; 32768];
        loop {
            match target_dir.read_entry(&mut buffer) {
                Ok(Some(entry)) => {
                    let name = entry.file_name();
                    let size = entry.file_size();
                    let attr = entry.attribute();

                    // Add a trailing slash to directories for clarity
                    let is_dir = attr.contains(FileAttribute::DIRECTORY);
                    let suffix = if is_dir { "/" } else { "" };

                    message!("\t", "  {}{:<20} ** {} BYTES", name, suffix, size);
                }
                Ok(None) => break, // End of directory
                Err(_) => {
                    message!("", "  Error reading entry");
                    break;
                }
            }
        }
    }

    pub fn get_cwd() -> Result<(), ()> {
        let state = Self::get_state();
        let current_path = &state.cwd;
        message!("\n", "CWD: {}", current_path);
        Ok(())
    }
}