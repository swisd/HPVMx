use alloc::vec::Vec;
use crate::hpvm_log;
use uefi::proto::console::text::Color;

pub struct BinaryLoader;

impl BinaryLoader {
    /// Loads a raw binary at a fixed address and executes it.
    /// Many micro-c targets use ORG 0x100000.
    pub fn load_and_run(data: &[u8], load_addr: u64) -> Result<(), &'static str> {
        crate::hpvm_info!("ASM", "Loading raw binary of size {} at 0x{:x}", data.len(), load_addr);

        // In UEFI, we might not be able to allocate at EXACTLY 0x100000 if it's already used.
        // But we can try to allocate pages and copy it there if it's free, 
        // or just allocate anywhere and hope the code is position independent (unlikely for raw bin with ORG).
        
        // For raw binaries with fixed ORG, we really need that specific address.
        let pages = (data.len() + 4095) / 4096;
        
        let allocated_ptr = match uefi::boot::allocate_pages(
            uefi::boot::AllocateType::Address(load_addr),
            uefi::boot::MemoryType::LOADER_DATA,
            pages
        ) {
            Ok(ptr) => ptr,
            Err(_) => {
                crate::hpvm_warn!("ASM", "Failed to allocate at 0x{:x}, trying AnyPages", load_addr);
                uefi::boot::allocate_pages(
                    uefi::boot::AllocateType::AnyPages,
                    uefi::boot::MemoryType::LOADER_DATA,
                    pages
                ).map_err(|_| "Failed to allocate any memory for binary")?
            }
        };

        let dest = allocated_ptr.as_ptr();
        crate::hpvm_info!("ASM", "Binary allocated at 0x{:x}", dest as u64);

        if dest as u64 != load_addr {
            crate::hpvm_warn!("ASM", "Binary loaded at 0x{:x} but expected 0x{:x}. Code might fail if not PIC.", dest as u64, load_addr);
        }

        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), dest, data.len());
        }

        crate::hpvm_info!("ASM", "Executing binary...");
        
        unsafe {
            let entry_point: extern "C" fn() = core::mem::transmute(dest);
            entry_point();
        }

        Ok(())
    }
}
