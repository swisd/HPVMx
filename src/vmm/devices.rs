//! Bootloader support for various OS types

use alloc::string::String;
use alloc::vec::Vec;

#[allow(dead_code)]
pub struct BootLoader {
    pub boot_type: BootType,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum BootType {
    BIOS,
    UEFI,
    Multiboot,
    Multiboot2,
}

#[allow(dead_code)]
impl BootLoader {
    pub fn new(boot_type: BootType) -> Self {
        Self { boot_type }
    }

    pub fn load_efi_firmware(&self, _path: &str) -> Result<Vec<u8>, &str> {
        // Load EFI firmware/bootloader from disk
        Ok(Vec::new())
    }

    pub fn load_kernel(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        crate::kernel::KernelLoader::load_kernel(path)
    }

    pub fn load_initrd(&self, _path: &str) -> Result<Vec<u8>, &str> {
        // Load initial ramdisk
        Ok(Vec::new())
    }

    pub fn prepare_boot_environment(
        &self,
        _kernel: &[u8],
        _initrd: Option<&[u8]>,
        cmdline: &str,
    ) -> Result<BootEnvironment, &str> {
        Ok(BootEnvironment {
            kernel_addr: 0x100000,
            initrd_addr: 0x1000000,
            cmdline: String::from(cmdline),
        })
    }
}

#[allow(dead_code)]
pub struct BootEnvironment {
    pub kernel_addr: u64,
    pub initrd_addr: u64,
    pub cmdline: String,
}