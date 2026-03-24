use crate::Color;
use crate::hpvm_log;
use elf::{ElfBytes, endian::AnyEndian};
use uefi::prelude::*;
use uefi::boot as boot_t;
use uefi::boot::AllocateType;
use uefi_raw::table::boot::MemoryType;
use crate::{hpvm_info, FileSystem};


#[allow(unsafe_code, unsafe_op_in_unsafe_fn)]
pub unsafe fn load_and_jump_os(path: &str) -> ! {
    // 1. Get Framebuffer info (unchanged)
    let gop_handle = uefi::boot::get_handle_for_protocol::<uefi::proto::console::gop::GraphicsOutput>().unwrap();
    let mut gop = uefi::boot::open_protocol_exclusive::<uefi::proto::console::gop::GraphicsOutput>(gop_handle).unwrap();
    let fb_ptr = gop.frame_buffer().as_mut_ptr();
    let fb_size = gop.frame_buffer().size();
    hpvm_info!("loader", "{:?}  {:?}  {:?}", gop_handle, fb_ptr, fb_size);

    // 2. Load ELF data
    let data = FileSystem::read_file(path).expect("OS file not found");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&data).expect("Invalid ELF format");

    // We need to track the actual load address to calculate the jump later
    let mut load_base: Option<u64> = None;
    hpvm_info!("loader", "load base {:?}", load_base);

    // 3. Allocate and Copy segments
    for phdr in file.segments().unwrap().iter().filter(|p| p.p_type == elf::abi::PT_LOAD) {
        // --- PAGE ALIGNMENT MATH ---
        // 1. Round the virtual address down to the nearest 4KB page
        let vaddr = phdr.p_vaddr;
        let aligned_vaddr = vaddr & !0xFFF;
        let offset_within_page = vaddr - aligned_vaddr;

        // 2. Adjust page count to include the offset and the full memory size
        let total_size = offset_within_page + phdr.p_memsz;
        let pages = (total_size + 0xFFF) / 0x1000;

        // 3. Let UEFI find ANY free memory (Avoids the NOT_FOUND conflict)
        let allocated_addr = uefi::boot::allocate_pages(
            uefi::boot::AllocateType::AnyPages, // "Give me any free RAM"
            uefi::boot::MemoryType::LOADER_CODE,
            pages as usize
        ).expect("Failed to allocate memory for OS segment");

        // The actual destination for the data is the allocated start + the offset
        let dest_ptr = (allocated_addr.as_ptr() as u64 + offset_within_page) as *mut u8;

        // Save the base address for the entry point calculation (assuming first segment is base)
        if load_base.is_none() {
            load_base = Some(allocated_addr.as_ptr() as u64 - aligned_vaddr);
        }

        // --- COPY DATA ---
        let dest_slice = core::slice::from_raw_parts_mut(dest_ptr, phdr.p_memsz as usize);
        dest_slice.fill(0); // Zero BSS

        let src_start = phdr.p_offset as usize;
        let src_end = src_start + phdr.p_filesz as usize;
        dest_slice[..phdr.p_filesz as usize].copy_from_slice(&data[src_start..src_end]);
        hpvm_info!("loader", "vaddr {:?} aligned_vaddr {:?} page_int_offset {:?} total_sz {:?} pages {:?} alloc_addr {:?} dest_ptr {:?}", vaddr, aligned_vaddr, offset_within_page, total_size, pages, allocated_addr, dest_ptr);
    }

    // 4. Calculate Dynamic Entry Point
    // entry_point = (Original Entry) + (New Base - Original Base)
    let entry_point = file.ehdr.e_entry + load_base.expect("No loadable segments found");
    // let entry_offset: u64 = file.ehdr.e_entry - file.ehdr.;   // Fix this to add offset
    let actual_jump_address = entry_point; // + entry_offset;

    // 5. Exit Boot Services (Safety: Disable interrupts first)
    // Note: You should ideally pass the memory map to the kernel here!
    let _mmap = unsafe {
        x86_64::instructions::interrupts::disable(); // Recommended if available
        boot::exit_boot_services(Some(MemoryType::LOADER_DATA))
    };

    // 6. Hand over to OS
    let entry_fn: extern "C" fn(fb: *mut u32, size: usize) -> ! = core::mem::transmute(actual_jump_address);
    entry_fn(fb_ptr as *mut u32, fb_size);
}