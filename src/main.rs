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
mod logiclang_int;
mod devices;
mod hpvmlog;

use alloc::boxed::Box;
use tools::dsk;

extern crate alloc;
use alloc::string::{String, ToString};
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
use uefi_async::*;
use uefi_async::nano_alloc::{executor, Executor, TaskNode};
use uefi_async::nano_alloc::time::_WaitTimer;
//use ui::UI;
use kernel::KernelLoader;
use filesystem::FileSystem;
use vmm::HypervisorManager;
use ui::WinNTShell;
use ui::DashboardUI;
use vmm::bootloader::BootLoader;

use hpvmlog::*;

//#[global_allocator]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();
static mut HEAP_STORAGE: [u8; 2 * 1024 * 1024] = [0; 2 * 1024 * 1024];
static mut VIRT_STACK: [u8; 256 * 1024 * 1024] = [0; 256 * 1024 * 1024];

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
        let parts = &command;

        match command.as_slice()[0] {
            "help" => message!("\n", "commands available: \n(* means command is not in a working state) \n\nFileSystem:\n  help - show this help\n  clear - clear screen\n  ls - list files\n  cd [dir] - change directory*\n  pwd - print working directory\n  mkdir [dir] - make directory\n  touch [file] - create file\n  cpy [src] [dst] - copy file\n  mov [src] [dst] - move file\n  rm [file] - remove file\n  cat [file] - show file contents\n  clon [src] [dst] - clone directory\n  write [file] [data] [mode] - write to file\n\nVM Management:\n  vm create [name] [memory_mb] [vcpus] - create VM\n  vm list - list all VMs\n  vm start [vm_id] - start VM\n  vm stop [vm_id] - stop VM\n  vm delete [vm_id] - delete VM\n  boot [vm_id] [iso|efi|img] - boot VM with media\n  console [vm_id] - attach to VM console\n  run-efi [path] [args...] - run EFI application\n  dashboard - show management dashboard\n\nHypervisor:\n  vmm info - show hypervisor stats\n  vmm info-adv - show advanced stats\n\nNetworking:\n  net status - show NIC status (SNP)\n  net up - initialize NIC via UEFI SNP\n  ping [ip] - test reachability (placeholder)\n  lanscan [x.y.z.] - scan /24 network (placeholder)\n  httpd start [port] - start HTTP management server (placeholder)\n  httpd stop - stop HTTP server\n\nOther:\n  devs - list drives\n  info - show system info\n  start [kernel] - load kernel*\n  shutdown [s|r] - shutdown(s) or reboot(r)\n  BIOS - exit to BIOS"),
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
                if command.len() > 0 {
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

            // New: Boot ISO/EFI files
            "boot" => {
                if command.len() < 3 {
                    message!("\n", "Usage: boot [vm_id] [iso_path|efi_path]");
                } else {
                    let vm_id: u32 = match command[1].parse() {
                        Ok(id) => id,
                        Err(_) => {
                            hpvm_error!("Boot", "invalid VM ID");
                            22
                        }
                    };
                    let path = command[2];
                    boot_vm_with_media(vm_id, path);
                }
            }

            // New: Run EFI applications
            "run-efi" => {
                if command.len() < 2 {
                    message!("\n", "Usage: run-efi [efi_path] [args...]");
                } else {
                    run_efi_application(command[1], &command[2..]);
                }
            }

            // New: Dashboard UI
            "dashboard" => {
                show_dashboard_ui();
            }

            // New: Console access to running VM
            "console" => {
                if command.len() < 2 {
                    message!("\n", "Usage: console [vm_id]");
                } else {
                    let vm_id: u32 = match command[1].parse() {
                        Ok(id) => id,
                        Err(_) => {
                            hpvm_error!("Console", "invalid VM ID");
                            22
                        }
                    };
                    attach_vm_console(vm_id);
                }
            }
            "vmm" => {
                handle_vmm_command(&command);
            }
            ,
            "ping" => {
                if parts.len() < 2 { message!("\n", "Usage: ping [ip]"); } else {
                    let _ = devices::net::ping(parts[1], 1, 1000);
                }
            }
            ,
            "lanscan" => {
                if parts.len() < 2 { message!("\n", "Usage: lanscan [x.y.z.]"); } else {
                    devices::net::lanscan(parts[1]);
                }
            }
            ,
            "httpd" => {
                if parts.len() < 2 { message!("\n", "Usage: httpd [start port|stop]"); }
                else if parts[1] == "start" {
                    let port: u16 = if parts.len() >= 3 { parts[2].parse().unwrap_or(8080) } else { 8080 };
                    devices::net::httpd_start(port);
                } else if parts[1] == "stop" {
                    devices::net::httpd_stop();
                } else {
                    message!("\n", "Usage: httpd [start port|stop]");
                }
            }
            ,
            "net" => {
                if parts.len() < 2 {
                    message!("\n", "Usage: net [status|up]");
                } else if parts[1] == "status" {
                    devices::net::status();
                } else if parts[1] == "up" {
                    match devices::net_hw::init() {
                        Ok(()) => hpvm_info!("net", "NIC initialized (SNP)"),
                        Err(e) => hpvm_warn!("net", "NIC init failed: {}", e),
                    }
                } else {
                    message!("\n", "Usage: net [status|up]");
                }
            }
            "logiclang" => {
                let mut cmdbuf: String = String::new();
                let state = read_line_int(&mut cmdbuf);
                if state != 1 {
                    let unclean = cmdbuf.trim();

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
                    let text = cmdbuf.clone();
                    let output = logiclang_int::interpreter::LogicInterpreter::interpret(text);
                    message!("\n", "{:#?}", output);
                    cmdbuf.clear();
                } else {
                    //do nothing
                }
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
                    if (ch == '\r' || ch == '\n') {
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

// ============================================
// NEW FUNCTIONS FOR VM BOOT AND EFI SUPPORT
// ============================================

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
                        match hv.boot_vm_with_media(vm_id, &media_data) {
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


#[allow(static_mut_refs)]
/// Display the dashboard UI
fn show_dashboard_ui() {
    let mut dashboard = DashboardUI::new();

    // Populate dashboard with VM information
    unsafe {
        if let Some(ref hv) = HYPERVISOR {
            let vms = hv.list_vms();

            for (id, name, state) in vms {
                let vm_info = ui::VmDisplayInfo {
                    id,
                    name: alloc::string::String::from(name),
                    state: state.to_string(),
                    cpu_usage: 25,  // Placeholder
                    memory_usage_mb: 512,  // Placeholder
                    disk_usage_mb: 10240,  // Placeholder
                    uptime_seconds: 3600,  // Placeholder
                };
                dashboard.add_vm(vm_info);
            }

            // Get system resources
            let stats = hv.get_stats();
            dashboard.set_resources(ui::SystemResources {
                total_memory_mb: stats.total_memory_mb,
                used_memory_mb: stats.total_memory_mb / 2,  // Approximate
                cpu_count: 8,  // Placeholder
                cpu_usage: 35,  // Placeholder
            });
        }
    }

    // Enter dashboard interaction loop
    loop {
        dashboard.draw();

        let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
        uefi::boot::wait_for_event(&mut events).unwrap();

        if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
            dashboard.handle_input(key);

            // Check if user wants to exit dashboard
            if let uefi::proto::console::text::Key::Printable(c) = key {
                if char::from(c).to_ascii_lowercase() == 'q' {
                    hpvm_info!("Dashboard", "exiting dashboard");
                    uefi::system::with_stdout(|s| {
                        let _ = s.clear();
                    });
                    break;
                }
            }
        }
    }
}

#[allow(static_mut_refs)]
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
                    uefi::system::with_stdout(|s| {
                        let _ = core::fmt::Write::write_fmt(s,
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
fn read_boot_file(path: &str) -> Result<Vec<u8>, &'static str> {
    use alloc::vec::Vec;

    // Use KernelLoader's file loading capability
    match KernelLoader::load_kernel(path) {
        Ok(data) => Ok(data),
        Err(_) => Err("file not found"),
    }
}

// async fn draw(){
//     let mut pacer = Pacer::new(60); // Target 60 FPS
//     let mut cursor = graphics::Cursor::new();
//     loop {
//         cursor.update_from_mouse();
//         uefi::system::with_stdout(|stdout| {
//             cursor.render(stdout);
//         });
//     }
// }