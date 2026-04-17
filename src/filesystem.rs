//! File system and device management.
//!
//! This module provides an abstraction over the UEFI file system,
//! including path resolution, file operations, and device mapping.

use crate::Color;
use crate::hpvm_log;
use crate::hpvm_info;
use crate::message;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::boot::SearchType;
use uefi::data_types::CStr16;
use uefi::{Handle, Identify};
use uefi::proto::device_path::DevicePath;
use uefi::proto::device_path::text::{AllowShortcuts, DisplayOnly};
use uefi::proto::media::file::{File, FileMode, FileAttribute, FileInfo};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi_raw::protocol::device_path::{DeviceSubType, DeviceType};
use crate::hpvmlog::LogEntry;
use crate::state::{KernelState, Persistable};

/// Global file system state.
///
/// Holds the current working directory, device mappings, and drive information.
#[derive(Clone)]
pub struct State {
    cwd: String,
    pub device_map: Vec<(String, String)>,
    pub root_handle: Option<uefi::Handle>,
    pub drive_handles: Vec<(String, uefi::Handle)>,
}


impl Persistable for &mut State {
    fn magic() -> u32 { 0x54535346 } // "FSST" in hex

    fn get_heap_bytes(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let size = size_of::<State>();
        let ptr = &self as *const _ as *const u8;
        unsafe {
            data.extend_from_slice(core::slice::from_raw_parts(ptr, size));
        }
        data
    }
}

static mut STATE: Option<State> = None;


/// Entry point for file system operations.
pub struct FileSystem;

impl FileSystem {
    pub fn is_handle() -> bool {
        match uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().map_err(|_| "No FS handle") {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

#[allow(dead_code)]
impl FileSystem {
    #[allow(static_mut_refs)]
    /// Internal helper to access global state
    pub fn get_state() -> &'static mut State {
        unsafe {
            if STATE.is_none() {
                STATE = Some(State {
                    cwd: String::from("\\"),
                    device_map: Vec::new(),
                    root_handle: None,
                    drive_handles: Vec::new(),
                });
            }
            STATE.as_mut().unwrap()
        }
    }

    pub fn set_root_handle(handle: Option<Handle>) {
        let state = Self::get_state();
        state.root_handle = Some(handle.expect("Could not set root fs handle"));
    }

    /// Resolves path based on Aliases (dev0:), Root-relative (/), or CWD
    fn resolve_path(path: &str) -> String {
        let state = Self::get_state();
        let input = path.replace('/', "\\");
        let mut base_path: String;

        // 1. Determine the starting point (Base)
        if let Some(colon_idx) = input.find(':') {
            // Handle Device Aliases (ex: dsk0:\path)
            let drive_name = &input[..colon_idx];
            let sub_path = &input[colon_idx + 1..];
            
            base_path = input.clone();
            
            // Check drive_handles first for dskX:
            let mut found = false;
            for (alias, _handle) in &state.drive_handles {
                if alias == drive_name {
                    // Ensure sub_path starts with a backslash if it's not empty
                    let formatted_sub = if !sub_path.starts_with('\\') && !sub_path.is_empty() {
                        format!("\\{}", sub_path)
                    } else if sub_path.is_empty() {
                        String::from("\\")
                    } else {
                        sub_path.to_string()
                    };
                    base_path = format!("{}:{}", alias, formatted_sub);
                    found = true;
                    break;
                }
            }

            if !found {
                for (alias, full_path) in &state.device_map {
                    let prefix = format!("{}:", alias);
                    if base_path.starts_with(&prefix) {
                        base_path = base_path.replace(&prefix, full_path);
                        break;
                    }
                }
            }
        } else if input.starts_with('\\') {
            // Root-relative
            base_path = input.clone();
        } else {
            // Relative to CWD
            base_path = state.cwd.clone();
            if !base_path.ends_with('\\') {
                base_path.push('\\');
            }
            base_path.push_str(&input);
        }

        // 2. Normalize the path (Handle ".." and ".")
        let mut components = Vec::new();
        // Split by backslash and filter out empty strings (caused by double slashes)
        let parts_source = if let Some(colon_idx) = base_path.find(':') {
            &base_path[colon_idx + 1..]
        } else {
            &base_path
        };

        for part in parts_source.split('\\') {
            if part.is_empty() || part == "." {
                continue;
            } else if part == ".." {
                // Pop the last element to "jump up" a level
                components.pop();
            } else {
                components.push(part);
            }
        }

        // 3. Reconstruct the final UEFI string
        let mut final_path = if let Some(colon_idx) = base_path.find(':') {
             format!("{}:", &base_path[..colon_idx])
        } else {
            String::new()
        };
        final_path.push('\\');
        
        for (i, comp) in components.iter().enumerate() {
            final_path.push_str(comp);
            if i < components.len() - 1 {
                final_path.push('\\');
            }
        }

        final_path
    }

    /// Change current directory
    pub fn cd(path: &str) {
        let state = Self::get_state();
        state.cwd = Self::resolve_path(path);
    }

    /// Scans all drives and writes "alias -> path" to a file
    pub fn scan_and_map_devices(map_file_path: &str) -> Result<(), &'static str> {
        // 1. ONLY locate handles that explicitly support the DevicePath protocol.
        // This skips "ghost" PCI handles and internal firmware handles that cause hangs.
        let handles = uefi::boot::locate_handle_buffer(SearchType::ByProtocol(&DevicePath::GUID))
            .map_err(|_| "Failed to locate device path handles")?;

        let mut map_contents = String::new();
        let state = Self::get_state();
        state.device_map.clear();
        state.drive_handles.clear();

        let mut dsk_i = 0; // Start from 1 as per typical disk naming
        let mut net_i = 0;
        let mut usb_i = 0;
        let mut com_i = 0;
        let mut pci_i = 0;

        for handle in handles.as_slice() {
            // Check if this handle supports SimpleFileSystem
            let has_fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(*handle).is_ok();

            // Use open_protocol with GetProtocol attribute - safest non-locking method
            let device_path_res = unsafe {
                uefi::boot::open_protocol::<DevicePath>(
                    uefi::boot::OpenProtocolParams {
                        handle: *handle,
                        agent: uefi::boot::image_handle(),
                        controller: None,
                    },
                    uefi::boot::OpenProtocolAttributes::GetProtocol,
                )
            };

            if let Ok(device_path) = device_path_res {
                let full_path: String = device_path
                    .to_string(DisplayOnly(false), AllowShortcuts(false))
                    .map_err(|_| "Path string error")?
                    .to_string();

                let mut alias = String::new();

                for node in device_path.node_iter() {
                    let d_type = node.device_type();
                    let d_sub = node.sub_type();

                    match (d_type, d_sub) {
                        (DeviceType::MEDIA, DeviceSubType::MEDIA_HARD_DRIVE) => {
                            alias = format!("dsk{}", dsk_i);
                        }
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_MAC_ADDRESS) => {
                            alias = format!("net{}", net_i);
                        }
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB) => {
                            alias = format!("usb{}", usb_i);
                        }
                        (DeviceType::MESSAGING, DeviceSubType::MESSAGING_UART) => {
                            alias = format!("com{}", com_i);
                        }
                        (DeviceType::HARDWARE, DeviceSubType::HARDWARE_PCI) if alias.is_empty() => {
                            alias = format!("pci{}", pci_i);
                        }
                        _ => continue,
                    }
                }

                if alias.is_empty() {
                    alias = format!("dev{}", state.device_map.len());
                }

                if has_fs && alias.starts_with("dsk") {
                    state.drive_handles.push((alias.clone(), *handle));
                    dsk_i += 1;
                } else if alias.starts_with("net") {
                    net_i += 1;
                } else if alias.starts_with("usb") {
                    usb_i += 1;
                } else if alias.starts_with("com") {
                    com_i += 1;
                } else if alias.starts_with("pci") {
                    pci_i += 1;
                }

                state.device_map.push((alias.clone(), full_path.clone()));
                map_contents.push_str(&format!("{} -> {}\n", alias, full_path));
            }
        }

        // Now write_to_file can execute because no handles are locked
        Self::write_to_file(map_file_path, &map_contents, 'w')
    }

    pub fn mkdir(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root(None)?;
        let path_cstr = Self::path_to_cstr16(path)?;

        root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::DIRECTORY)
            .map_err(|_| "Failed to create directory")?;
        Ok(())
    }

    pub fn touch(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root(None)?;
        let path_cstr = Self::path_to_cstr16(path)?;

        root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Failed to create file")?;
        Ok(())
    }

    pub fn copy(src: &str, dst: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root(None)?;

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
        let mut root = Self::get_root(None)?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Open for delete failed")?;
        let file = handle.into_regular_file().ok_or("Not a file")?;

        file.delete().map_err(|_| "Delete failed")?;
        Ok(())
    }

    pub fn cat(path: &str) -> Result<(), &'static str> {
        let mut root = Self::get_root(None)?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::Read, FileAttribute::empty()).map_err(|_| "Open failed")?;
        let mut file = handle.into_regular_file().ok_or("Not a file")?;

        let mut info_buf = [0u8; 512];
        let info = file.get_info::<FileInfo>(&mut info_buf).map_err(|_| "Info error")?;
        let mut buffer = Vec::new();
        buffer.resize(info.file_size() as usize, 0u8);
        file.read(&mut buffer).map_err(|_| "Read error")?;

       
       if let Ok(text) = core::str::from_utf8(&buffer) {
           message!("\n", "{}", text);
       } else {
           for (i, byte) in buffer.iter().enumerate() {
               if i % 16 == 0 { message!("\n", "\n{:08X}: ", i); }
               message!("\n", "{:02X} ", byte);
           }
       }
        Ok(())
    }

    pub fn clone_dir(src: &str, dst: &str) -> Result<(), &'static str> {
        Self::mkdir(dst)?;
        let mut root = Self::get_root(None)?;
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
        let mut root = Self::get_root(None)?;
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

    pub fn write_to_file_bytes(path: &str, data: &[u8], mode: char) -> Result<(), &'static str> {
        let mut root = Self::get_root(None)?;
        let path_cstr = Self::path_to_cstr16(path)?;

        let handle = root.open(path_cstr, FileMode::CreateReadWrite, FileAttribute::empty())
            .map_err(|_| "Open for write failed")?;
        let mut file = handle.into_regular_file().ok_or("Not a file")?;

        if mode == 'a' {
            file.set_position(0xFFFFFFFFFFFFFFFF).map_err(|_| "Seek error")?;
        }
        file.write(data).map_err(|_| "Write error")?;
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

    fn get_root(drive_name: Option<&str>) -> Result<uefi::proto::media::file::Directory, &'static str> {
        let state = Self::get_state();
        
        let handle = if let Some(name) = drive_name {
            state.drive_handles.iter()
                .find(|(alias, _)| alias == name)
                .map(|(_, handle)| *handle)
                .ok_or("Drive not found")?
        } else {
            state.root_handle.or_else(|| {
                // Fallback to first drive handle if root_handle is not set
                state.drive_handles.first().map(|(_, handle)| *handle)
            }).or_else(|| {
                // Last fallback: use the first handle found for SimpleFileSystem protocol
                uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().ok()
            }).ok_or("No FS handle")?
        };

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle).map_err(|_| "FS open err")?;
        fs.open_volume().map_err(|_| "Volume open err")
    }

    pub fn get_drives(drive_ref_file_path: &str) {
        Self::cat(drive_ref_file_path).unwrap()
    }

    pub fn list_files() {
        let state = Self::get_state();
        let current_path = &state.cwd;

        let drive_name = if let Some(colon_idx) = current_path.find(':') {
            Some(&current_path[..colon_idx])
        } else {
            None
        };

        let mut root_dir = match Self::get_root(drive_name) {
            Ok(root) => root,
            Err(_) => {
                message!("\n", "Failed to open drive: {:?}", drive_name);
                return;
            }
        };

        message!("\n", "ROOT {:?}", root_dir);
        message!("", "CONTENTS OF: {}\n", current_path);

        let sub_path = if let Some(colon_idx) = current_path.find(':') {
            &current_path[colon_idx + 1..]
        } else {
            current_path
        };

        let mut target_dir = if sub_path == "\\" || sub_path == "/" || sub_path.is_empty() {
            root_dir
        } else {
            let path_u16 = Self::path_to_cstr16(current_path);
            let path_cstr = match path_u16 {
                Ok(cstr) => cstr,
                Err(_) => {
                    message!("", "  Invalid path");
                    return;
                }
            };

            let handle = match root_dir.open(path_cstr, FileMode::Read, FileAttribute::DIRECTORY) {
                Ok(h) => h,
                Err(_) => {
                    message!("", "  Failed to open directory");
                    return;
                }
            };

            match handle.into_directory() {
                Some(dir) => dir,
                None => {
                    message!("", "  Path is not a directory");
                    return;
                }
            }
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

                    message!("\t", "  {}{:<30} ** {} BYTES", name, suffix, size);
                }
                Ok(None) => break, // End of directory
                Err(_) => {
                    message!("", "  Error reading entry");
                    break;
                }
            }
        }
    }

    pub fn get_cwd() -> Result<(String), ()> {
        let state = Self::get_state();
        let current_path = &state.cwd;
        message!("\n", "CWD: {}", current_path);
        Ok(current_path.clone())
    }

    /// Read a file and return its contents as a `Vec<u8>`
    pub fn read_file(path: &str) -> Result<Vec<u8>, &'static str> {
        let drive_name = if let Some(colon_idx) = path.find(':') {
            Some(&path[..colon_idx])
        } else {
            None
        };

        let mut root = Self::get_root(drive_name)?;

        // Open the file
        let mut file = root
            .open(
                Self::path_to_cstr16(path)?,
                FileMode::Read,
                FileAttribute::empty(),
            )
            .map_err(|_| "failed to open file")?;

        // Get file info to determine size
        let mut info_buffer = [0u8; 256];
        let file_info = file
            .get_info::<FileInfo>(&mut info_buffer)
            .map_err(|_| "failed to get file info")?;

        let file_size = file_info.file_size() as usize;

        // Read file contents
        let mut buffer = Vec::with_capacity(file_size);
        buffer.resize(file_size, 0u8);

        let mut regular_file = file.into_regular_file().ok_or("not a regular file")?;
        regular_file.read(&mut buffer)
            .map_err(|_| "failed to read file")?;

        Ok(buffer)
    }

    /// Read a file as a string
    pub fn read_file_to_string(path: &str) -> Result<String, &'static str> {
        let data = Self::read_file(path)?;
        String::from_utf8(data)
            .map_err(|_| "file is not valid UTF-8")
    }
}

