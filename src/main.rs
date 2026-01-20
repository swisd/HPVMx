#![feature(str_as_str)]
#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod ui;
mod kernel;
mod filesystem;
mod graphics;
mod interrupts;
mod imx;
mod paging;

mod tools;
mod vmm;
mod hardware;

use tools::dsk;

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::addr_of_mut;
use uefi::prelude::*;
use log::{error, info, warn};
use uefi::boot;
use buddy_system_allocator::LockedHeap;
use uefi::boot::{MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::console::text::{Color, Output as TextOutputTrait};
use uefi::runtime::ResetType;
use uefi_raw::table::system::SystemTable;
//use ui::UI;
use kernel::KernelLoader;
use filesystem::FileSystem;
use vmm::HypervisorManager;
use ui::WinNTShell;




macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            // Bring the trait into scope INSIDE the closure
            //use uefi::proto::console::text::Output;
            use core::fmt::Write;

            // let old_attribute = stdout.get_attribute().ok();

            // Set prefix color
            let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, "[{}] ", $prefix);

            // Reset to white for message
            match $color {
                Color::Yellow => {}
                Color::Red => {}
                _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
            }
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);

            // Restore original attributes if they existed
            // if let Some(attr) = old_attribute {
            //     let _ = stdout.set_attribute(attr);
            // }
        })
    };
}

macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            use core::fmt::Write;
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, $start);
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
        })
    }
}

macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

macro_rules! hpvm_warn {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Yellow, $tag, $($arg)*) };
}

// Added this to stop the "unused macro" warning
macro_rules! hpvm_error {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Red, $tag, $($arg)*) };
}


//#[global_allocator]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();
static mut HEAP_STORAGE: [u8; 2 * 1024 * 1024] = [0; 2 * 1024 * 1024];
static mut VIRT_STACK: [u8; 1024 * 1024 * 1024] = [0; 1024 * 1024 * 1024];

use paging::PagingManager;
use crate::graphics::Graphics;

static mut HYPERVISOR: Option<HypervisorManager> = None;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    hpvm_info!("UEFI", "init uefi helpers");

    // FIXED: Using addr_of_mut! to avoid static_mut_refs errors
    // unsafe {
    //     let heap_ptr = addr_of_mut!(HEAP_STORAGE) as *mut u8;
    //     let heap_size = core::mem::size_of_val(&&raw const HEAP_STORAGE);
    //     ALLOCATOR.lock().init(heap_ptr as usize, heap_size);
    // }

    uefi::system::with_stdout(|stdout| {
        let _ = stdout.clear();
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

    hpvm_info!("IDT", "initializing idt");
    hpvm_info!("page", "setting active paging mapper");
    let mut mapper = unsafe { PagingManager::get_active_mapper(x86_64::VirtAddr::new(16384)) };

    hpvm_info!("fs", "building drivelist");
    FileSystem::scan_and_map_devices("DRIVELIST").expect("TODO: panic message");

    //interrupts::init_idt();

    unsafe {
        HYPERVISOR = Some(HypervisorManager::new());
        if let Some(ref mut hv) = HYPERVISOR {
            match hv.initialize() {
                Ok(_) => hpvm_info!("VMM", "hypervisor initialized"),
                Err(e) => hpvm_warn!("VMM", "hypervisor init failed: {}", e),
            }
        }
    }

    hpvm_info!("HPVMx", "init sequence complete.");

    hpvm_info!("HPVMx", "ready");
    hpvm_warn!("HPVMx", "within spinloop");
    //Graphics::get_graphics_info();

    hpvm_info!("HPVMx", "HPVMx Shell v0.1.0");
    hpvm_info!("HPVMx", "Type 'help' for commands.");

    let mut input_buffer = String::new();

    loop {
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
        let parts = &command;

        match command.as_slice()[0] {
            "help" => message!("\n", "commands available: \n(* means command is not in a working state) \nhelp \nclear \nls \n*cd [directory] \npwd \nmkdir [directory] \ntouch [file] \ncpy [source] [destination] \nmov [source] [destination] \nrm [file] \ncat [file] \nclon [args] \nwrite [file] [data] [mode] \n*upd [**args] [disk] \ninfo \ndevs \nstart [kernel filepath] \nBIOS"),
            "clear" => { uefi::system::with_stdout(|s| s.clear().unwrap()); }
            "ls" => FileSystem::list_files(),
            "cd" => { FileSystem::cd(command[1]) }
            "pwd" => { FileSystem::get_cwd(); }
            "mkdir" => {
                if parts.len() < 2 {
                    message!("\n", "Usage: mkdir [directory]");
                } else {
                    match FileSystem::mkdir(parts[1]) {
                        Ok(_) => hpvm_info!("fs", "directory '{}' created", parts[1]),
                        Err(e) => hpvm_error!("fs", "failed to create directory: {}", e),
                    }
                }
            }
            "touch" => {
                if parts.len() < 2 {
                    message!("\n", "Usage: touch [file]");
                } else {
                    match FileSystem::touch(parts[1]) {
                        Ok(_) => hpvm_info!("fs", "file '{}' created", parts[1]),
                        Err(e) => hpvm_error!("fs", "failed to create file: {}", e),
                    }
                }
            }
            "cpy" => {
                if parts.len() < 3 {
                    message!("\n", "Usage: cpy [source] [destination]");
                } else {
                    match FileSystem::copy(parts[1], parts[2]) {
                        Ok(_) => hpvm_info!("fs", "file '{}' copied to '{}'", parts[1], parts[2]),
                        Err(e) => hpvm_error!("fs", "failed to copy file: {}", e),
                    }
                }
            }
            "mov" => {
                if parts.len() < 3 {
                    message!("\n", "Usage: mov [source] [destination]");
                } else {
                    match FileSystem::move_file(parts[1], parts[2]) {
                        Ok(_) => hpvm_info!("fs", "file '{}' moved to '{}'", parts[1], parts[2]),
                        Err(e) => hpvm_error!("fs", "failed to move file: {}", e),
                    }
                }
            }
            "rm" => {
                if parts.len() < 2 {
                    message!("\n", "Usage: rm [file]");
                } else {
                    match FileSystem::remove(parts[1]) {
                        Ok(_) => hpvm_info!("fs", "file '{}' deleted", parts[1]),
                        Err(e) => hpvm_error!("fs", "failed to delete file: {}", e),
                    }
                }
            }
            "cat" => {
                if parts.len() < 2 {
                    message!("\n", "Usage: cat [file]");
                } else {
                    if let Err(e) = FileSystem::cat(parts[1]) {
                        hpvm_error!("fs", "failed to read file: {}", e);
                    }
                }
            }
            "clon" => {
                if parts.len() < 3 {
                    message!("\n", "Usage: clon [source] [destination]");
                } else {
                    match FileSystem::clone_dir(parts[1], parts[2]) {
                        Ok(_) => hpvm_info!("fs", "directory '{}' cloned to '{}'", parts[1], parts[2]),
                        Err(e) => hpvm_error!("fs", "failed to clone directory: {}", e),
                    }
                }
            }
            "write" => {
                if command.len() > 3 {
                    FileSystem::write_to_file(command[1], command[2], command[3].parse().unwrap());
                } else {
                    message!("\n", "Usage: write [file] [data] [mode]")
                }
            }
            "upd" => {}
            "info" => {}
            "devs" => {
                if command.len() > 1 {
                    FileSystem::get_drives("DRIVELIST");
                } else {
                    message!(".", "text")
                }
            }
            "start" => {
                if command.len() < 2 {
                    message!("\n", "Usage: start [kernel_path]");
                } else {
                    start_kernel(command[1]);
                }
            }
            "enter" => {
                enter(body.as_slice()[1])
            }
            "BIOS" => break,

            "shutdown" => {
                if command.len() > 1 {
                    shutdown(command[1].parse().unwrap());
                } else {
                    message!("\n", "Usage: shutdown [s|r]");
                }
            }
            // Hypervisor commands
            "vm" => {
                handle_vm_command(&command);
            }
            "vmm" => {
                handle_vmm_command(&command);
            }

            _ => message!("\n", "unknown command: {:?}", command),
        }
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
                    if (ch == '\r' || ch == '\n') {
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


fn enter(itm: &str) {
    match itm {
        "ui" => {
            let mut shell = WinNTShell::new();
            shell.init_desktop();
            shell.draw();

            loop {

                let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
                uefi::boot::wait_for_event(&mut events).unwrap();

                if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
                    shell.handle_input(key);
                    shell.draw();
                }
            }
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
            hpvm_error!("kernel", "failed to load kernel: {}", e);
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
                    _ => message!("\n", "Usage: vm [create|list|start|stop|delete]"),
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