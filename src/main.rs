#![allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused, unused_must_use, unused_features)]
#![feature(str_as_str)]
#![feature(abi_x86_interrupt)]
#![feature(core_float_math)]
#![feature(generic_atomic)]
#![no_std]
#![no_main]




mod ui;
mod kernel;
mod filesystem;
mod graphics;
mod interrupts;
mod gdt;
mod imx;
mod paging;
mod tools;
mod vmm;
mod hardware;
mod logiclang_int;
mod devices;
mod hpvmlog;
mod consts;
mod types;
mod rng;
mod state;
mod loader;
mod terminal;
mod pm;
mod micro_c;

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::ptr::addr_of_mut;
use uefi::prelude::*;
use uefi::Char16;
use log::error;
use uefi::boot;
use buddy_system_allocator::LockedHeap;
use uefi::boot::{MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::console::text::Color;
use uefi::runtime::ResetType;
//use uefi::system::with_stdout;
use uefi_raw::table::system::SystemTable;
use uefi::proto::console::pointer::Pointer as SimplePointer;
use uefi::proto::device_path::DevicePath;
//use ui::UI;
use kernel::KernelLoader;
use filesystem::FileSystem;
use vmm::HypervisorManager;
//use ui::WinNTShell;
use ui::DashboardUI;
//use sysinfo;
use types::*;
use crate::paging::PagingManager;
use crate::rng::XorShiftRng;
use crate::ui::DashboardTab;
use pm::PackageManager;



//#[global_allocator]
#[allow(dead_code, unused)]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();

#[allow(dead_code, unused)]
static mut HEAP_STORAGE: [u8; 2 * 1024 * 1024] = [0; 2 * 1024 * 1024];

#[allow(dead_code, unused)]
static mut VIRT_STACK: [u8; 256 * 1024 * 1024] = [0; 256 * 1024 * 1024];


//use crate::graphics::Cursor;



static mut HYPERVISOR: Option<HypervisorManager> = None;

#[allow(dead_code, unused, unused_must_use, non_camel_case_types, nonstandard_style)]
#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    hpvm_info!("UEFI", "init uefi helpers");
    crate::hpvmlog::init_log_buffer();

    // FIXED: Using addr_of_mut! to avoid static_mut_refs errors
    unsafe {
        let heap_ptr = addr_of_mut!(HEAP_STORAGE) as *mut u8;
        let heap_size = core::mem::size_of_val(&&raw const HEAP_STORAGE);
        ALLOCATOR.lock().init(heap_ptr as usize, heap_size);
    }

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.clear();
        let _ = stdout.enable_cursor(true);
    });

    hpvm_info!("HPVMx", "HPVMx version is {}", env!("CARGO_PKG_VERSION"));
    hpvm_info!("malloc", "heap initialized. retrieving memory map...");

    // 2. In uefi 0.36.1 with 'alloc' feature, use boot::memory_map()
    // This returns a MemoryMapOwned object automatically using the heap.
    //let size = uefi::boot::memory_map_size().map_size;
    let size = uefi::boot::PAGE_SIZE;
    hpvm_info!("page", "system required buffer of {} bytes", size);

    // 16KB is usually enough for most servers; 32KB is safe for high-end systems.
    let mut map_buffer = [0u8; 32768];
    hpvm_info!("page", "set map buffer to [0u8; 32768]");


    let SYSTEM_TABLE: *mut SystemTable = uefi::table::system_table_raw().unwrap().as_ptr();



    // 2. Use get_memory_map_static instead of the alloc version
    // This does NOT use your LockedHeap; it uses the array above.
    // let memory_map = uefi::boot::get_boot_services().memory_map(&mut map_buffer)
    //     .expect("Failed to get memory map. Buffer might be too small.");

    // let memory_map = boot::memory_map(MemoryType::CONVENTIONAL)
    //     .expect("failed to retrieve memory map. ensure 'alloc' feature is enabled in Cargo.toml");

    match boot::memory_map(MemoryType::LOADER_DATA) {
        Ok(map) => {
            hpvm_info!("malloc", "retrieved memory map with {} entries.  OMT (bsc/bsd)", map.entries().count());

            // Iterate and filter for free RAM
            for entry in map.entries() {
                match entry.ty {
                    MemoryType::BOOT_SERVICES_CODE => {}
                    MemoryType::BOOT_SERVICES_DATA => {}

                    _ => hpvm_info!("malloc",
                         "AREA {:#?}  START {:#x}  PAGE {}",
                         entry.ty,
                         entry.phys_start,
                         entry.page_count,
                     )
                }
            }
        }
        Err(e) => {
            error!("Failed to retrieve memory map: {:?}", e.status());
        }
    };

    hpvm_info!("GDT", "initializing gdt");
    gdt::init();
    hpvm_info!("IDT", "initializing idt");
    interrupts::init_idt();

    hpvm_info!("page", "setting active paging mapper");
    let mut mapper = unsafe { PagingManager::get_active_mapper(x86_64::VirtAddr::new(16384)) };


    hpvm_info!("fs", "building devicelist");

    while !(FileSystem::is_handle()) {
        // wait
        hpvm_warn!("fs", "waiting for file handle")
    }

    FileSystem::scan_and_map_devices("DEVICELIST").unwrap();

    unsafe {
        HYPERVISOR = Some(HypervisorManager::new());
        if let Some(ref mut hv) = HYPERVISOR {
            match hv.initialize() {
                Ok(_) => hpvm_info!("VMM", "hypervisor initialized"),
                Err(e) => hpvm_warn!("VMM", "hypervisor init failed: {}", e),
            }
        }
    }

    // 1. Get all handles
    let handles = boot::find_handles::<DevicePath>().unwrap();

    // 2. Force UEFI to connect drivers to every handle it finds
    for handle in handles {
        let _ = boot::connect_controller(handle, None, None, true);
    }

    // 3. Now check for SimplePointer again
    let mouse_handles = boot::find_handles::<SimplePointer>().unwrap_or_default();
    hpvm_info!("mouse", "Now found {} pointer handles", mouse_handles.len());







    init_mouse_deep_scan();

    hpvm_info!("HPVMx", "init sequence complete.");
    let mut PACKAGE_MANAGER: PackageManager = PackageManager::new();
    PACKAGE_MANAGER.load_registry();

    hpvm_info!("HPVMx", "ready");
    hpvm_warn!("HPVMx", "within spinloop");
    //Graphics::get_graphics_info();

    const icon_ascii: &str = "\n
       __ _____ _   ____  ___
      / // / _ \\ | / /  |/  /_ __
     / _  / ___/ |/ / /|_/ /\\ \\ /
    /_//_/_/   |___/_/  /_//_\\_\\      \n\n";

    message!("", "{}", icon_ascii);
    hpvm_info!("HPVMx", "HPVMx Shell v0.1.0");
    hpvm_info!("HPVMx", "Type 'help' for commands.");

    let mut input_buffer = String::new();

    // let mut executor = Executor::new();
    // executor
    //     .add(&mut TaskNode::new(Box::pin(draw()), 60))
    //     .run_forever();

    loop {
        // drive network timers (loopback stack)
        crate::devices::net_stack::poll_tick();

        // Print Prompt
        uefi::system::with_stdout(|s| core::fmt::Write::write_str(s, "\nHPVMx> ").unwrap());

        // Simple line reader
        input_buffer.clear();
        read_line(&mut input_buffer);

        let unclean = input_buffer.trim();

        // Handle backspaces by removing previous char(s)
        let mut command = String::with_capacity(unclean.len());
        let mut consecutive_backspaces = 0;

        for c in unclean.chars() {
            if c == '\u{8}' {
                consecutive_backspaces += 1;
                if consecutive_backspaces >= 2 {
                    command.pop(); // Remove additional char for consecutive backspaces
                }
                command.pop(); // Remove char before backspace
            } else {
                consecutive_backspaces = 0;
                command.push(c);
            }
        }

        let body = command.split(" ").collect::<Vec<&str>>();
        let body = command.split(" ").collect::<Vec<&str>>();
        if command.is_empty() { continue; }
        let command = command.split(" ").collect::<Vec<&str>>();
        let parts = command.clone();

        terminal::cmd(command, &parts, body, &mut PACKAGE_MANAGER);

    }

    Status::SUCCESS
}

fn read_line(buf: &mut String) {
    loop {
        let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
        uefi::boot::wait_for_event(&mut events).unwrap();

        if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    buf.remove(buf.len() - 1);
                    // if buf.pop().is_some() {
                    //     uefi::system::with_stdout(|s| core::fmt::Write::write_str(s, "\u{0008} \u{0008}").unwrap());
                    // }
                    uefi::system::with_stdout(|s| {
                        s.clear().unwrap(); // Clear current line
                        s.write_str("HPVMx> ").unwrap(); // Rewrite prompt
                        s.write_str(&buf[..buf.len() - 1]).unwrap(); // Rewrite buffer without last char
                    });
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, ch).unwrap());
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, "\n".parse().unwrap()).unwrap());
                        break;
                    }
                    if ch != '`' /* != '\u{8}' */ {
                        buf.push(ch);
                        // Echo to screen
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, ch).unwrap());
                    }
                }
                _ => {}
            }
        }
    }
}

fn read_line_int(buf: &mut String) -> i32 {
    loop {
        let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
        uefi::boot::wait_for_event(&mut events).unwrap();

        if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    buf.remove(buf.len() - 1);
                    // if buf.pop().is_some() {
                    //     uefi::system::with_stdout(|s| core::fmt::Write::write_str(s, "\u{0008} \u{0008}").unwrap());
                    // }
                    uefi::system::with_stdout(|s| {
                        s.clear().unwrap(); // Clear current line
                        s.write_str("> ").unwrap(); // Rewrite prompt
                        s.write_str(&buf[..buf.len() - 1]).unwrap(); // Rewrite buffer without last char
                    });
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, ch).unwrap());
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, "\n".parse().unwrap()).unwrap());
                        if buf.contains("run") {
                            break 0;
                        }
                        if buf.contains("exit") {
                            return 1;
                        }
                    }
                    if ch != '`' /* != '\u{8}' */ {
                        buf.push(ch);
                        // Echo to screen
                        uefi::system::with_stdout(|s| core::fmt::Write::write_char(s, ch).unwrap());
                    }
                }
                _ => {}
            }
        }
    }
}


fn enter(itm: &str) {
    match itm {
        "ui" => {
            hpvm_warn!("ui", "NTSHELL is deprecated, use dashboard instead");
            // let mut shell = WinNTShell::new();
            // shell.init_desktop();
            // shell.draw();
            //
            // loop {
            //
            //     let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
            //     uefi::boot::wait_for_event(&mut events).unwrap();
            //
            //     if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
            //         shell.handle_input(key);
            //         shell.draw();
            //     }
            // }
        }
        _ => {}
    }
}

fn start_kernel(path: &str) {
    hpvm_info!("kernel", "attempting to load kernel from: {}", path);

    match KernelLoader::load_kernel(path) {
        Ok(kernel_data) => {
            hpvm_info!("kernel", "kernel loaded, {} bytes", kernel_data.len());

            match KernelLoader::validate_kernel(&kernel_data) {
                Ok(entry_point) => {
                    hpvm_info!("kernel", "kernel validated, entry point: {:#x}", entry_point);
                    hpvm_warn!("kernel", "jumping to kernel... goodbye!");

                    unsafe {
                        KernelLoader::execute_kernel(&kernel_data, entry_point);
                    }
                }
                Err(e) => {
                    hpvm_error!("kernel", "kernel validation failed: {}", e);
                }
            }
        }
        Err(e) => {
            hpvm_error!("FileIO", "failed to load file '{}': {}", path, e);
            hpvm_warn!("FileIO", "attempting to load in dangerous mode '{}'", path);

            match KernelLoader::load_kernel_dangerous(path) {
                Ok(kernel_data) => {
                    hpvm_info!("kernel", "kernel loaded, {} bytes", kernel_data.len());

                    match KernelLoader::validate_kernel(&kernel_data) {
                        Ok(entry_point) => {
                            hpvm_info!("kernel", "kernel validated, entry point: {:#x}", entry_point);
                            hpvm_warn!("kernel", "jumping to kernel... goodbye!");

                            unsafe {
                                KernelLoader::execute_kernel(&kernel_data, entry_point);
                            }
                        }
                        Err(e) => {
                            hpvm_error!("kernel", "kernel validation failed: {}", e);
                            hpvm_warn!("kernel", "loading kernel in dangerous mode -- invalidated elf header");
                            unsafe {
                                KernelLoader::execute_kernel(&kernel_data, 5);
                            }
                        }
                    }
                }
                Err(e) => {
                    hpvm_error!("FileIO", "failed to load file '{}': {}", path, e);
                }
            }
        }
    }
}


fn shutdown(mode: char) {
    match mode {
        's' => {
            hpvm_info!("HPVMx", "shutting down...");
            let mmap = unsafe { uefi::boot::exit_boot_services(None) };

            hpvm_info!("malloc", "Memory Map:");
            for desc in mmap.entries() {
                hpvm_info!("malloc",
            "start=0x{:016x} size=0x{:016x} type={:?}, attr={:?}",
            desc.phys_start,
            desc.page_count * 4096,
            desc.ty,
            desc.att
        );
            }
            runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, Some(&[0]))
        }
        'r' => {
            hpvm_info!("HPVMx", "restarting...");
            runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, Some(&[255]))
        }
        _ => { hpvm_info!("x", "incorrect, command") }
    }
}

#[allow(static_mut_refs)]
fn handle_vm_command(command: &[&str]) {
    unsafe {
        match HYPERVISOR.as_mut() {
            Some(hv) => {
                match command.get(1) {
                    Some(&"create") => {
                        if command.len() < 5 {
                            message!("\n", "Usage: vm create [name] [memory_mb] [vcpus]");
                            return;
                        }
                        let name = command[2];
                        let memory_mb: u32 = match command[3].parse() {
                            Ok(m) => m,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid memory size");
                                return;
                            }
                        };
                        let vcpus: u32 = match command[4].parse() {
                            Ok(v) => v,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid vCPU count");
                                return;
                            }
                        };

                        match hv.create_vm(name, memory_mb, vcpus) {
                            Ok(vm_id) => hpvm_info!("VMM", "VM '{}' created with ID: {}", name, vm_id),
                            Err(e) => hpvm_error!("VMM", "failed to create VM: {}", e),
                        }
                    }
                    Some(&"list") => {
                        let vms = hv.list_vms();
                        if vms.is_empty() {
                            message!("\n", "No VMs created");
                        } else {
                            message!("\n", "Virtual Machines:");
                            for (id, name, state) in vms {
                                message!("", "  ID: {}, Name: {}, State: {}", id, name, state);
                            }
                        }
                    }
                    Some(&"start") => {
                        if command.len() < 3 {
                            message!("\n", "Usage: vm start [vm_id]");
                            return;
                        }
                        let vm_id: u32 = match command[2].parse() {
                            Ok(id) => id,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid VM ID");
                                return;
                            }
                        };
                        match hv.start_vm(vm_id) {
                            Ok(_) => hpvm_info!("VMM", "VM {} started", vm_id),
                            Err(e) => hpvm_error!("VMM", "failed to start VM: {}", e),
                        }
                    }
                    Some(&"stop") => {
                        if command.len() < 3 {
                            message!("\n", "Usage: vm stop [vm_id]");
                            return;
                        }
                        let vm_id: u32 = match command[2].parse() {
                            Ok(id) => id,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid VM ID");
                                return;
                            }
                        };
                        match hv.stop_vm(vm_id) {
                            Ok(_) => hpvm_info!("VMM", "VM {} stopped", vm_id),
                            Err(e) => hpvm_error!("VMM", "failed to stop VM: {}", e),
                        }
                    }
                    Some(&"delete") => {
                        if command.len() < 3 {
                            message!("\n", "Usage: vm delete [vm_id]");
                            return;
                        }
                        let vm_id: u32 = match command[2].parse() {
                            Ok(id) => id,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid VM ID");
                                return;
                            }
                        };
                        match hv.delete_vm(vm_id) {
                            Ok(_) => hpvm_info!("VMM", "VM {} deleted", vm_id),
                            Err(e) => hpvm_error!("VMM", "failed to delete VM: {}", e),
                        }
                    }
                    Some(&"boot") => {
                        if command.len() < 4 {
                            message!("\n", "Usage: vm boot [vm_id] [iso|efi|img]");
                            return;
                        }
                        let vm_id: u32 = match command[2].parse() {
                            Ok(id) => id,
                            Err(_) => {
                                hpvm_error!("Boot", "invalid VM ID");
                                return;
                            }
                        };
                        let path = command[3];
                        match hv.boot_vm_with_media(vm_id, path) {
                            Ok(_) => hpvm_info!("Boot", "VM {} boot process initiated", vm_id),
                            Err(e) => hpvm_error!("Boot", "failed to boot VM: {}", e),
                        }
                    }
                    Some(&"simulate-violation") => {
                        if command.len() < 4 {
                            message!("\n", "Usage: vm simulate-violation [vm_id] [error_code]");
                            return;
                        }
                        let vm_id: u32 = match command[2].parse() {
                            Ok(id) => id,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid VM ID");
                                return;
                            }
                        };
                        let error_code: u32 = match command[3].parse() {
                            Ok(code) => code,
                            Err(_) => {
                                hpvm_error!("VMM", "invalid error code");
                                return;
                            }
                        };
                        match hv.trigger_autolytic_response(vm_id, error_code) {
                            Ok(_) => hpvm_info!("VMM", "Autolytic response triggered for VM {}", vm_id),
                            Err(e) => hpvm_error!("VMM", "failed to trigger response: {}", e),
                        }
                    }
                    _ => message!("\n", "Usage: vm [create|list|start|stop|delete|boot|simulate-violation]"),
                }
            }
            None => hpvm_error!("VMM", "hypervisor not initialized"),
        }
    }
}

#[allow(static_mut_refs)]
fn handle_vmm_command(command: &[&str]) {
    unsafe {
        match HYPERVISOR.as_mut() {
            Some(hv) => {
                match command.get(1) {
                    Some(&"info") => {
                        let stats = hv.get_stats();
                        message!("\n", "--- Hypervisor Statistics ---");
                        message!("", "Initialized: {}", stats.initialized);
                        message!("", "Total VMs: {}", stats.total_vms);
                        message!("", "Running VMs: {}", stats.running_vms);
                        message!("", "Total Memory: {} MB", stats.total_memory_mb);
                    }
                    Some(&"info-adv") => {
                        let stats = hv.get_stats_advanced();
                        message!("\n", "--- Hypervisor Statistics ---");
                        message!("", "Initialized: {}", stats.0.initialized);
                        message!("", "Total VMs: {}", stats.0.total_vms);
                        message!("", "Running VMs: {}", stats.0.running_vms);
                        message!("", "Total Memory: {} MB", stats.0.total_memory_mb);
                        message!("\n", "INDIVIDUAL VM STATS\n{}", stats.1)
                    }
                    _ => message!("\n", "Usage: vmm [info]"),
                }
            }
            None => hpvm_error!("VMM", "hypervisor not initialized"),
        }
    }
}


// NEW FUNCTIONS FOR VM BOOT AND EFI SUPPORT

#[allow(static_mut_refs)]
/// Boot a VM with an ISO, EFI file, or disk image
fn boot_vm_with_media(vm_id: u32, media_path: &str) {
    unsafe {
        match HYPERVISOR.as_mut() {
            Some(hv) => {
                // Determine media type from file extension
                let media_type = if media_path.ends_with(".iso") {
                    hpvm_info!("Boot", "detected ISO image");
                    "ISO"
                } else if media_path.ends_with(".efi") {
                    hpvm_info!("Boot", "detected EFI executable");
                    "EFI"
                } else if media_path.ends_with(".img") {
                    hpvm_info!("Boot", "detected disk image");
                    "IMG"
                } else {
                    hpvm_warn!("Boot", "unknown media type, assuming disk image");
                    "IMG"
                };

                // Load the media file
                match load_boot_media(media_path) {
                    Ok(media_data) => {
                        hpvm_info!("Boot", "loaded {} bytes from '{}'", media_data.len(), media_path);

                        // Attempt to boot the VM with the media
                        match hv.boot_vm_with_media(vm_id, media_path) {
                            Ok(_) => {
                                hpvm_info!("Boot", "VM {} booted with {}", vm_id, media_type);
                            }
                            Err(e) => {
                                hpvm_error!("Boot", "failed to boot VM {}: {}", vm_id, e);
                            }
                        }
                    }
                    Err(e) => {
                        hpvm_error!("Boot", "failed to load media '{}': {}", media_path, e);
                    }
                }
            }
            None => hpvm_error!("Boot", "hypervisor not initialized"),
        }
    }
}

#[allow(static_mut_refs)]
/// Run a standalone EFI application
fn run_efi_application(efi_path: &str, args: &[&str]) {
    hpvm_info!("EFI", "loading EFI application from '{}'", efi_path);

    match load_boot_media(efi_path) {
        Ok(efi_data) => {
            hpvm_info!("EFI", "loaded {} bytes", efi_data.len());

            // Parse EFI header
            if efi_data.len() < 64 {
                hpvm_error!("EFI", "invalid EFI file: too small");
                return;
            }

            // Check for MZ signature (EFI is PE format)
            if efi_data[0] != 0x4D || efi_data[1] != 0x5A {
                hpvm_warn!("EFI", "file doesn't start with MZ signature");
            }

            hpvm_info!("EFI", "executing application with {} arguments", args.len());

            // In a real implementation, you would:
            // 1. Set up page tables and memory mapping
            // 2. Load sections from the EFI file
            // 3. Call the entry point
            // For now, we'll just log the arguments
            for (i, arg) in args.iter().enumerate() {
                hpvm_info!("EFI", "  arg[{}]: {}", i, arg);
            }

            hpvm_warn!("EFI", "EFI execution not fully implemented in this build");
        }
        Err(e) => {
            hpvm_error!("EFI", "failed to load EFI file: {}", e);
        }
    }
}



#[allow(static_mut_refs, dead_code)]
/// Attach to a VM's console for interaction
fn attach_vm_console(vm_id: u32) {
    hpvm_info!("Console", "attaching to VM {} console", vm_id);

    unsafe {
        match HYPERVISOR.as_mut() {
            Some(hv) => {
                // Check if VM exists and is running
                let vms = hv.list_vms();
                let vm_exists = vms.iter().any(|(id, _, state)| {
                    *id == vm_id && (state.to_string().contains("running") || state.to_string().contains("paused"))
                });

                if !vm_exists {
                    hpvm_error!("Console", "VM {} not found or not running", vm_id);
                    return;
                }

                hpvm_info!("Console", "connected to VM {} console (type 'exit' to disconnect)", vm_id);
                hpvm_warn!("Console", "use Ctrl+Alt+D to disconnect");

                // Simple console loop
                let mut console_input = String::new();
                loop {
                    system::with_stdout(|s| {
                        let _ = Write::write_fmt(s,
                                                 format_args!("vm{}> ", vm_id)).ok();
                    });

                    console_input.clear();
                    read_line(&mut console_input);

                    let trimmed = console_input.trim();

                    // Check exit conditions
                    if trimmed == "exit" || trimmed == "quit" {
                        hpvm_info!("Console", "disconnecting from VM console");
                        break;
                    }

                    // In a real implementation, send input to VM's serial port/console
                    hpvm_info!("Console", "sent to VM: {}", trimmed);
                }
            }
            None => hpvm_error!("Console", "hypervisor not initialized"),
        }
    }
}


/// Load boot media (ISO, EFI, IMG files) from filesystem
fn load_boot_media(path: &str) -> Result<Vec<u8>, &'static str> {
    // Since FileSystem doesn't expose a read_file method, we use the kernel loader
    // which already has file loading capability
    match KernelLoader::load_kernel(path) {
        Ok(data) => {
            hpvm_info!("FileIO", "loaded {} bytes from '{}'", data.len(), path);
            Ok(data)
        }
        Err(e) => {
            hpvm_error!("FileIO", "failed to load file '{}': {}", path, e);
            hpvm_warn!("FileIO", "attempting to load in dangerous mode '{}'", path);

            match KernelLoader::load_kernel_dangerous(path) {
                Ok(data) => {
                    hpvm_info!("FileIO", "loaded {} bytes from '{}' in dangerous mode", data.len(), path);
                    Ok(data)
                }
                Err(e) => {
                    hpvm_error!("FileIO", "failed to load file '{}': {}", path, e);
                    Err(e)
                }
            }
        }
    }
}

/// Helper function to read a file from the filesystem
#[allow(dead_code)]
fn read_boot_file(path: &str) -> Result<Vec<u8>, &'static str> {
    

    // Use KernelLoader's file loading capability
    match KernelLoader::load_kernel(path) {
        Ok(data) => Ok(data),
        Err(_) => Err("file not found"),
    }
}

fn init_mouse() {
    if let Ok(handle) = boot::get_handle_for_protocol::<SimplePointer>() {
        let _ = boot::connect_controller(handle, None, None, true);
        if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<SimplePointer>(handle) {


            // This is the "magic" line for VirtualBox PS/2
            // We try non-extended first, then extended if it doesn't fail but isn't working
            let rx = mouse.reset(false);
            hpvm_info!("usbhid", "mouse reset ev=false  {:?}", rx);
            // Some firmwares require extended verification
            let ry = mouse.reset(true);
            hpvm_info!("usbhid", "mouse reset ev=true  {:?}", ry);
        }
    }

    // Also try to reset AbsolutePointer if present
    // Since it's a manual protocol in graphics.rs, we use its GUID directly or just skip here
    // as we don't have easy access to the type without importing it.
    // Actually, let's keep it simple for now as the Dashboard loop will call update_from_mouse
    // which will open it.
}

fn init_mouse_deep_scan() {


    // 1. Force UEFI to connect every device it sees on the PCI/USB bus
    // This is critical because passed-through USB devices aren't always auto-started
    if let Ok(all_handles) = boot::find_handles::<uefi::proto::device_path::DevicePath>() {
        for handle in all_handles {
            let _ = boot::connect_controller(handle, None, None, true);
        }
    }

    // 2. Now find ALL SimplePointer handles (expecting 2: Virtual and Physical)
    if let Ok(handles) = boot::find_handles::<SimplePointer>() {
        hpvm_info!("usbhid", "Found {} SimplePointer handles", handles.len());

        for (i, handle) in handles.iter().enumerate() {
            if let Ok(mut mouse) = boot::open_protocol_exclusive::<SimplePointer>(*handle) {
                // Reset is mandatory for Logitech receivers to begin reporting
                let r = mouse.reset(false);
                hpvm_info!("usbhid", "Handle [{}]: Reset result {:?}", i, r);

                // Read the resolution - physical mice usually have small numbers (1, 2, 4)
                // unlike the virtual tablet's 65536
                #[allow(irrefutable_let_patterns)]
                if let mode = mouse.mode() {
                    hpvm_info!("usbhid", "Handle [{}]: Res X={}", i, mode.resolution[0]);
                }
            }
        }
    }
}


