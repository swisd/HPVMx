use elf::{ElfBytes, endian::AnyEndian};
use uefi::prelude::*;
use uefi::boot as boot_t;
use uefi::boot::AllocateType;
use uefi_raw::table::boot::MemoryType;
use crate::FileSystem;

pub unsafe fn load_and_jump_os(path: &str) -> ! {

    // 1. Get Framebuffer info for the OS splash screen
    let gop_handle = uefi::boot::get_handle_for_protocol::<uefi::proto::console::gop::GraphicsOutput>().unwrap();
    let mut gop = uefi::boot::open_protocol_exclusive::<uefi::proto::console::gop::GraphicsOutput>(gop_handle).unwrap();
    let fb_ptr = gop.frame_buffer().as_mut_ptr();
    let fb_size = gop.frame_buffer().size();

    // 2. Load ELF data using HPVMx's filesystem helper
    let data = FileSystem::read_file(path).expect("Monitoring OS file not found");
    let file = ElfBytes::<AnyEndian>::minimal_parse(&data).expect("Invalid ELF format");
    let entry_point = file.ehdr.e_entry;
    // 3. Allocate and Copy segments to physical memory
    for phdr in file.segments().unwrap().iter().filter(|p| p.p_type == elf::abi::PT_LOAD) {
        let pages = (phdr.p_memsz as usize + 0xFFF) / 0x1000;
        let addr = uefi::boot::allocate_pages(
            uefi::boot::AllocateType::Address(phdr.p_paddr),
            uefi::boot::MemoryType::LOADER_DATA,
            pages
        ).expect("Failed to allocate memory for OS segment");

        let dest = unsafe { core::slice::from_raw_parts_mut(addr.as_ptr(), phdr.p_memsz as usize) };
        dest.fill(0); // Zero-initialize .bss
        let src_start = phdr.p_offset as usize;
        let src_end = src_start + phdr.p_filesz as usize;
        dest[..phdr.p_filesz as usize].copy_from_slice(&data[src_start..src_end]);
    }

    // 4. Exit Boot Services using the safe wrapper
    // This returns the memory map, which you should ideally pass to the OS too.
    let _mmap = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

    // 5. Hand over to OS Entry Function
    // Signature: fn(framebuffer_ptr, framebuffer_size)
    let entry_fn: extern "C" fn(fb: *mut u32, size: usize) -> ! = unsafe { core::mem::transmute(entry_point) };
    entry_fn(fb_ptr as *mut u32, fb_size);
}