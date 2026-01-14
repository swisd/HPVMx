use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::data_types::CStr16;
use uefi::proto::media::file::{File, FileMode, FileAttribute, FileType};
use uefi::proto::media::fs::SimpleFileSystem;

pub struct FileSystem;

impl FileSystem {
    /// Helper to convert &str to CStr16
    fn to_cstr16(s: &str) -> Result<alloc::vec::Vec<u16>, ()> {
        let mut result = alloc::vec![];
        for c in s.encode_utf16() {
            result.push(c);
        }
        result.push(0);
        Ok(result)
    }

    /// Create a new directory
    pub fn mkdir(path: &str) -> Result<(), &'static str> {
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        let path_u16 = Self::to_cstr16(path).map_err(|_| "Invalid path")?;
        let path_cstr = CStr16::from_u16_with_nul(&path_u16)
            .map_err(|_| "Invalid path string")?;

        let _dir = root.open(
            path_cstr,
            FileMode::CreateReadWrite,
            FileAttribute::DIRECTORY,
        ).map_err(|_| "Failed to create directory")?;

        Ok(())
    }

    /// Create a new empty file
    pub fn touch(path: &str) -> Result<(), &'static str> {
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        let path_u16 = Self::to_cstr16(path).map_err(|_| "Invalid path")?;
        let path_cstr = CStr16::from_u16_with_nul(&path_u16)
            .map_err(|_| "Invalid path string")?;

        let _file = root.open(
            path_cstr,
            FileMode::CreateReadWrite,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to create file")?;

        Ok(())
    }

    /// Copy a file from source to destination
    pub fn copy(src: &str, dst: &str) -> Result<(), &'static str> {

        let mut buffer = [0u8; 4096];
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        // Open source file for reading
        let src_u16 = Self::to_cstr16(src).map_err(|_| "Invalid source path")?;
        let src_cstr = CStr16::from_u16_with_nul(&src_u16)
            .map_err(|_| "Invalid source path string")?;

        let src_handle = root.open(
            src_cstr,
            FileMode::Read,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open source file")?;

        let mut src_file = src_handle.into_regular_file()
            .ok_or("Source is not a regular file")?;

        // Get source file size
        let src_info = src_file.get_info::<uefi::proto::media::file::FileInfo>(&mut buffer)
            .map_err(|_| "Failed to get source file info")?;

        let file_size = src_info.file_size() as usize;

        if file_size == 0 {
            return Ok(());
        }

        // Read source file
        let mut buffer = Vec::new();
        buffer.resize(file_size, 0u8);
        src_file.read(&mut buffer)
            .map_err(|_| "Failed to read source file")?;

        // Open destination file for writing
        let dst_u16 = Self::to_cstr16(dst).map_err(|_| "Invalid destination path")?;
        let dst_cstr = CStr16::from_u16_with_nul(&dst_u16)
            .map_err(|_| "Invalid destination path string")?;

        let dst_handle = root.open(
            dst_cstr,
            FileMode::CreateReadWrite,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open destination file")?;

        let mut dst_file = dst_handle.into_regular_file()
            .ok_or("Destination is not a regular file")?;

        // Write to destination
        dst_file.write(&buffer)
            .map_err(|_| "Failed to write to destination file")?;

        Ok(())
    }

    /// Move/rename a file (copy and delete)
    pub fn move_file(src: &str, dst: &str) -> Result<(), &'static str> {
        Self::copy(src, dst)?;
        Self::remove(src)?;
        Ok(())
    }

    /// Delete a file
    pub fn remove(path: &str) -> Result<(), &'static str> {
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        let path_u16 = Self::to_cstr16(path).map_err(|_| "Invalid path")?;
        let path_cstr = CStr16::from_u16_with_nul(&path_u16)
            .map_err(|_| "Invalid path string")?;

        let file_handle = root.open(
            path_cstr,
            FileMode::CreateReadWrite,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open file for deletion")?;

        let mut file = file_handle.into_regular_file()
            .ok_or("Target is not a regular file")?;

        file.delete()
            .map_err(|_| "Failed to delete file")?;

        Ok(())
    }

    /// Read and display a file's contents
    pub fn cat(path: &str) -> Result<(), &'static str> {
        let mut buffer = [0u8; 4096];
        
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        let path_u16 = Self::to_cstr16(path).map_err(|_| "Invalid path")?;
        let path_cstr = CStr16::from_u16_with_nul(&path_u16)
            .map_err(|_| "Invalid path string")?;

        let file_handle = root.open(
            path_cstr,
            FileMode::Read,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open file")?;

        let mut file = file_handle.into_regular_file()
            .ok_or("Target is not a regular file")?;

        let file_info = file.get_info::<uefi::proto::media::file::FileInfo>(&mut buffer)
            .map_err(|_| "Failed to get file info")?;

        let file_size = file_info.file_size() as usize;

        if file_size == 0 {
            uefi::system::with_stdout(|stdout| {
                let _ = write!(stdout, "\n(empty file)\n");
            });
            return Ok(());
        }

        let mut buffer = Vec::new();
        buffer.resize(file_size, 0u8);
        file.read(&mut buffer)
            .map_err(|_| "Failed to read file")?;

        // Display file contents
        uefi::system::with_stdout(|stdout| {
            let _ = write!(stdout, "\n");
            match core::str::from_utf8(&buffer) {
                Ok(text) => {
                    let _ = write!(stdout, "{}", text);
                }
                Err(_) => {
                    // Display as hex if not valid UTF-8
                    for (i, byte) in buffer.iter().enumerate() {
                        if i % 16 == 0 {
                            let _ = write!(stdout, "\n{:08X}: ", i);
                        }
                        let _ = write!(stdout, "{:02X} ", byte);
                    }
                    let _ = write!(stdout, "\n");
                }
            }
            let _ = write!(stdout, "\n");
        });

        Ok(())
    }

    /// Clone a directory recursively
    pub fn clone_dir(src: &str, dst: &str) -> Result<(), &'static str> {
        // Create destination directory
        Self::mkdir(dst)?;

        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        let src_u16 = Self::to_cstr16(src).map_err(|_| "Invalid source path")?;
        let src_cstr = CStr16::from_u16_with_nul(&src_u16)
            .map_err(|_| "Invalid source path string")?;

        let src_handle = root.open(
            src_cstr,
            FileMode::Read,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open source directory")?;

        let mut src_dir = src_handle.into_directory()
            .ok_or("Source is not a directory")?;

        // Buffer for reading directory entries
        let mut buffer = [0u8; 8192];

        loop {
            match src_dir.read_entry(&mut buffer) {
                Ok(Some(entry)) => {
                    let file_name = entry.file_name();
                    let file_name_str = CStr16::to_string(file_name);

                    if file_name_str == "." || file_name_str == ".." {
                        continue;
                    }

                    // Construct full paths
                    let src_path = alloc::format!("{}/{}", src, file_name_str);
                    let dst_path = alloc::format!("{}/{}", dst, file_name_str);

                    // If directory, recursively clone; if file, copy
                    if entry.is_directory() {
                        Self::clone_dir(&src_path, &dst_path)?;
                    } else {
                        Self::copy(&src_path, &dst_path)?;
                    }
                }
                Ok(None) => break,
                Err(_) => return Err("Failed to read directory entry"),
            }
        }

        Ok(())
    }
}