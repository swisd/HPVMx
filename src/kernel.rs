use alloc::vec::Vec;
use core::fmt::Write;
use uefi::proto::media::file::{File, FileMode, FileAttribute};
use uefi::proto::media::fs::SimpleFileSystem;

pub struct KernelLoader;

impl KernelLoader {
    /// Load a kernel file from the filesystem
    pub fn load_kernel(path: &str) -> Result<alloc::vec::Vec<u8>, &'static str> {
        // Get the filesystem
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;
        
        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;
        
        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        // Convert path to CStr16
        let path_u16: Vec<u16> = path.encode_utf16().collect();
        let path_cstr = uefi::data_types::CStr16::from_u16_with_nul(&path_u16)
            .map_err(|_| "Invalid kernel path")?;

        // Open the kernel file
        let kernel_handle = root.open(
            path_cstr,
            FileMode::Read,
            FileAttribute::empty(),
        ).map_err(|_| "Failed to open kernel file")?;

        // Convert FileHandle to RegularFile
        let mut kernel_file = kernel_handle.into_regular_file()
            .ok_or("Kernel file is not a regular file")?;

        // Read file size
        let mut buffer = [0u8; 4096];
        let file_info = kernel_file.get_info::<uefi::proto::media::file::FileInfo>(&mut buffer)
            .map_err(|_| "Failed to get file info")?;
        
        let file_size = file_info.file_size() as usize;

        if file_size == 0 {
            return Err("Kernel file is empty");
        }

        // Allocate buffer and read
        let mut kernel_data = Vec::new();
        kernel_data.resize(file_size, 0u8);
        kernel_file.read(&mut kernel_data)
            .map_err(|_| "Failed to read kernel file")?;

        Ok(kernel_data)
    }

    /// Execute a loaded kernel (basic entry point jump)
    /// This is a minimal implementation - you may need to:
    /// - Set up GDT/IDT
    /// - Configure memory mappings
    /// - Exit boot services before jumping
    pub unsafe fn execute_kernel(kernel_data: &[u8], entry_point: u64) -> ! {
        // Type cast the kernel data pointer to a function pointer
        let kernel_entry: extern "C" fn() -> ! = core::mem::transmute(entry_point as *const ());
        
        // Jump to kernel entry point
        kernel_entry()
    }

    pub fn validate_kernel(data: &[u8]) -> Result<u64, &'static str> {
        // Basic ELF validation
        if data.len() < 64 {
            return Err("Kernel file too small");
        }

        // Check ELF magic number
        if &data[0..4] != b"\x7FELF" {
            return Err("Invalid ELF header");
        }

        // Check if it's x86-64
        if data[4] != 2 {
            return Err("Not a 64-bit ELF");
        }

        // For now, assume entry point at offset 0x400000
        // You'll need proper ELF parsing for production use
        Ok(0x400000)
    }
}