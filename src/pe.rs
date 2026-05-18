use alloc::vec::Vec;
use core::mem::size_of;
use uefi::proto::console::text::Color;
use crate::hpvm_log;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DosHeader {
    pub magic: [u8; 2],      // "MZ"
    pub last_size: u16,
    pub n_blocks: u16,
    pub n_reloc: u16,
    pub hdr_size: u16,
    pub min_alloc: u16,
    pub max_alloc: u16,
    pub ss: u16,
    pub sp: u16,
    pub checksum: u16,
    pub ip: u16,
    pub cs: u16,
    pub reloc_pos: u16,
    pub n_overlay: u16,
    pub reserved1: [u16; 4],
    pub oem_id: u16,
    pub oem_info: u16,
    pub reserved2: [u16; 10],
    pub lfanew: u32,         // Offset to PE header
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CoffeeHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OptionalHeader64 {
    pub magic: u16, // 0x20b for PE32+
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directories: [DataDirectory; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SectionHeader {
    pub name: [u8; 8],
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub size_of_raw_data: u32,
    pub pointer_to_raw_data: u32,
    pub pointer_to_relocations: u32,
    pub pointer_to_linenumbers: u32,
    pub number_of_relocations: u16,
    pub number_of_linenumbers: u16,
    pub characteristics: u32,
}

pub struct PeLoader;

impl PeLoader {
    pub fn load_pe(data: &[u8]) -> Result<u64, &'static str> {
        if data.len() < size_of::<DosHeader>() {
            return Err("Data too small for DOS header");
        }

        let dos_header = unsafe { &*(data.as_ptr() as *const DosHeader) };
        if &dos_header.magic != b"MZ" {
            return Err("Invalid DOS magic");
        }

        let pe_offset = dos_header.lfanew as usize;
        if data.len() < pe_offset + 4 + size_of::<CoffeeHeader>() {
            return Err("Data too small for PE header");
        }

        if &data[pe_offset..pe_offset + 4] != b"PE\0\0" {
            return Err("Invalid PE signature");
        }

        let coffee_header = unsafe { &*(data.as_ptr().add(pe_offset + 4) as *const CoffeeHeader) };
        
        // Machine 0x8664 is x86-64
        if coffee_header.machine != 0x8664 {
            return Err("Not an x86-64 PE file");
        }

        let optional_header_offset = pe_offset + 4 + size_of::<CoffeeHeader>();
        if data.len() < optional_header_offset + size_of::<OptionalHeader64>() {
            return Err("Data too small for Optional header");
        }

        let optional_header = unsafe { &*(data.as_ptr().add(optional_header_offset) as *const OptionalHeader64) };
        if optional_header.magic != 0x20b {
            return Err("Not a PE32+ (64-bit) file");
        }

        crate::hpvm_info!("PE", "Loading PE image: EntryPoint RVA=0x{:x}, ImageBase=0x{:x}, Size=0x{:x}", 
            optional_header.address_of_entry_point, 
            optional_header.image_base,
            optional_header.size_of_image);

        // Allocate memory for the image
        // In UEFI, we use allocate_pages
        let pages = (optional_header.size_of_image as usize + 4095) / 4096;
        let image_mem = uefi::boot::allocate_pages(
            uefi::boot::AllocateType::AnyPages,
            uefi::boot::MemoryType::LOADER_DATA,
            pages
        ).map_err(|_| "Failed to allocate memory for PE image")?;

        let image_ptr = image_mem.as_ptr();
        let image_addr = image_ptr as u64;
        unsafe {
            core::ptr::write_bytes(image_ptr, 0, optional_header.size_of_image as usize);
        }

        // Copy headers
        let headers_size = core::cmp::min(optional_header.size_of_headers as usize, data.len());
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr(), image_ptr, headers_size);
        }

        // Copy sections
        let section_header_offset = optional_header_offset + coffee_header.size_of_optional_header as usize;
        for i in 0..coffee_header.number_of_sections {
            let offset = section_header_offset + (i as usize) * size_of::<SectionHeader>();
            let section = unsafe { &*(data.as_ptr().add(offset) as *const SectionHeader) };
            
            if section.size_of_raw_data > 0 {
                let dest = unsafe { image_ptr.add(section.virtual_address as usize) };
                let src_offset = section.pointer_to_raw_data as usize;
                let copy_size = core::cmp::min(section.size_of_raw_data as usize, data.len().saturating_sub(src_offset));
                
                if copy_size > 0 {
                    let src = unsafe { data.as_ptr().add(src_offset) };
                    unsafe {
                        core::ptr::copy_nonoverlapping(src, dest, copy_size);
                    }
                }
            }
            
            let name = core::str::from_utf8(&section.name).unwrap_or("unknown");
            crate::hpvm_info!("PE", "Section {}: {} mapped to 0x{:x}", i, name, image_addr + section.virtual_address as u64);
        }

        // Handle relocations if image base changed
        if image_addr != optional_header.image_base {
            crate::hpvm_warn!("PE", "Image base mismatch (Allocated: 0x{:x}, Expected: 0x{:x}). Relocations needed.", image_addr, optional_header.image_base);
            Self::apply_relocations(image_ptr, image_addr, &optional_header)?;
        }

        let entry_point = image_addr + optional_header.address_of_entry_point as u64;
        crate::hpvm_info!("PE", "PE loaded successfully. Entry point: 0x{:x}", entry_point);

        Ok(entry_point)
    }

    fn apply_relocations(image_ptr: *mut u8, image_base: u64, optional_header: &OptionalHeader64) -> Result<(), &'static str> {
        let reloc_dir = &optional_header.data_directories[5]; // IMAGE_DIRECTORY_ENTRY_BASERELOC
        if reloc_dir.virtual_address == 0 || reloc_dir.size == 0 {
            crate::hpvm_info!("PE", "No relocations found");
            return Ok(());
        }

        let delta = image_base.wrapping_sub(optional_header.image_base);
        if delta == 0 {
            return Ok(());
        }

        let mut current_offset = 0;
        while current_offset < reloc_dir.size as usize {
            let block_ptr = unsafe { image_ptr.add(reloc_dir.virtual_address as usize + current_offset) };
            let page_rva = unsafe { *(block_ptr as *const u32) };
            let block_size = unsafe { *(block_ptr.add(4) as *const u32) };

            if block_size < 8 { break; }

            let entries_count = (block_size - 8) / 2;
            let entries_ptr = unsafe { block_ptr.add(8) as *const u16 };

            for i in 0..entries_count {
                let entry = unsafe { *entries_ptr.add(i as usize) };
                let reloc_type = entry >> 12;
                let offset = entry & 0xFFF;

                if reloc_type == 10 { // IMAGE_REL_BASED_DIR64
                    let target_ptr = unsafe { image_ptr.add(page_rva as usize + offset as usize) as *mut u64 };
                    unsafe {
                        *target_ptr = (*target_ptr).wrapping_add(delta);
                    }
                } else if reloc_type != 0 { // 0 is IMAGE_REL_BASED_ABSOLUTE (skip)
                    crate::hpvm_warn!("PE", "Unsupported relocation type: {}", reloc_type);
                }
            }

            current_offset += block_size as usize;
        }

        crate::hpvm_info!("PE", "Relocations applied with delta 0x{:x}", delta);
        Ok(())
    }

    pub fn parse_pe(data: &[u8]) -> Result<(), &'static str> {
        if data.len() < size_of::<DosHeader>() {
            return Err("Data too small for DOS header");
        }

        let dos_header = unsafe { &*(data.as_ptr() as *const DosHeader) };
        if &dos_header.magic != b"MZ" {
            return Err("Invalid DOS magic");
        }

        let pe_offset = dos_header.lfanew as usize;
        if data.len() < pe_offset + 4 + size_of::<CoffeeHeader>() {
            return Err("Data too small for PE header");
        }

        if &data[pe_offset..pe_offset + 4] != b"PE\0\0" {
            return Err("Invalid PE signature");
        }

        let coffee_header = unsafe { &*(data.as_ptr().add(pe_offset + 4) as *const CoffeeHeader) };
        
        // Machine 0x8664 is x86-64
        if coffee_header.machine != 0x8664 {
            return Err("Not an x86-64 PE file");
        }

        let optional_header_offset = pe_offset + 4 + size_of::<CoffeeHeader>();
        if data.len() < optional_header_offset + size_of::<OptionalHeader64>() {
            return Err("Data too small for Optional header");
        }

        let optional_header = unsafe { &*(data.as_ptr().add(optional_header_offset) as *const OptionalHeader64) };
        if optional_header.magic != 0x20b {
            return Err("Not a PE32+ (64-bit) file");
        }

        crate::hpvm_info!("PE", "Entry point RVA: 0x{:x}", optional_header.address_of_entry_point);
        crate::hpvm_info!("PE", "Image base: 0x{:x}", optional_header.image_base);
        crate::hpvm_info!("PE", "Number of sections: {}", coffee_header.number_of_sections);

        let section_header_offset = optional_header_offset + coffee_header.size_of_optional_header as usize;
        for i in 0..coffee_header.number_of_sections {
            let offset = section_header_offset + (i as usize) * size_of::<SectionHeader>();
            if data.len() < offset + size_of::<SectionHeader>() {
                return Err("Data too small for Section header");
            }
            let section = unsafe { &*(data.as_ptr().add(offset) as *const SectionHeader) };
            let name = core::str::from_utf8(&section.name).unwrap_or("unknown");
            crate::hpvm_info!("PE", "Section {}: {} (VSize: 0x{:x}, VAddr: 0x{:x})", i, name, section.virtual_size, section.virtual_address);
        }

        Ok(())
    }
}
