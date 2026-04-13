use alloc::string::String;
use alloc::vec::Vec;
use uefi::{data_types, CStr16, CString16};
use uefi::proto::media::file::{File, FileMode, FileAttribute};
use uefi::proto::media::fs::SimpleFileSystem;
use crate::message;

pub struct KernelLoader;

impl KernelLoader {
    /// Load a kernel file from the filesystem
    pub fn load_kernel(path: &str) -> Result<Vec<u8>, &'static str> {
        // Get the filesystem
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;
        
        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;
        
        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        // Convert path to CStr16
        let path_u16: Vec<u16> = path.encode_utf16().collect();
        message!("", "n: {:#?}", String::from_utf16_lossy(&path_u16));
        let path_cstr = CStr16::from_u16_with_nul(&path_u16)
            .map_err(move |_| {
                "Invalid kernel path"
            })?;


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

    pub fn load_kernel_dangerous(path: &str) -> Result<Vec<u8>, &'static str> {
        // Get the filesystem
        let handle = uefi::boot::get_handle_for_protocol::<SimpleFileSystem>()
            .map_err(|_| "Failed to get filesystem handle")?;

        let mut fs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle)
            .map_err(|_| "Failed to open filesystem")?;

        let mut root = fs.open_volume()
            .map_err(|_| "Failed to open root volume")?;

        // Convert path to CStr16
        let path_u16: Vec<u16> = path.encode_utf16().collect();
        message!("", "n: {:#?}", String::from_utf16_lossy(&path_u16));
        let path_cstr = Self::u16_to_cstr16_unsafe(&*path_u16) // danger
            .map_err(move |_| {
                "Invalid kernel path"
            })?;
        message!("\n", "{}", path_cstr);


        // Open the kernel file
        let kernel_handle = root.open(
            &*path_cstr,
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
    pub unsafe fn execute_kernel(_kernel_data: &[u8], entry_point: u64) -> ! { unsafe {
        // Type cast the kernel data pointer to a function pointer
        let kernel_entry: extern "C" fn() -> ! = core::mem::transmute(entry_point as *const ());
        
        // Jump to kernel entry point
        kernel_entry()
    }}

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

    pub fn u16_to_cstr16_unsafe(data: &[u16]) -> Result<CString16, &'static str> {

        // Method 2: Manually building a CStr16 (More direct for slices)
        // CStr16 is essentially a view over a null-terminated &[u16].
        // If your slice *already* has the null, you can cast.


        match CStr16::from_u16_with_nul(data) {
            Ok(..) => {
                let cstr16_direct_cast = unsafe {
                    CStr16::from_ptr(data.as_ptr() as *const _)
                };
                Ok(CString16::from(cstr16_direct_cast))
            }
            Err(..) => {
                // If your slice *doesn't* have the null, you'd need a temporary buffer (or unsafe with careful checks):
                let mut data_with_null: Vec<u16> = data.to_vec();
                if data_with_null.last() != Some(&0u16) {
                    data_with_null.push(0u16); // Add the null terminator
                }
                let cstr16_manual_vec = unsafe {
                    CStr16::from_ptr(data_with_null.as_ptr() as *const _)
                };
                Ok(CString16::from(cstr16_manual_vec))
            }
        }

    }

}

