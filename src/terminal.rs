

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::time::Duration;
use elf::dynamic;
use libm::{expm1, y0};
use uefi::mem::memory_map::MemoryMap;
use uefi::{boot, runtime, system};
use uefi_raw::Status;
use uefi_raw::table::runtime::ResetType;
use crate::filesystem::FileSystem;
use crate::{hpvm_error, hpvm_info, hpvm_warn, hpvm_log, message, read_line, ui, HYPERVISOR, devices, loader, logiclang_int, read_line_int, env, apps, TSC_PER_US};
use crate::kernel::KernelLoader;
use crate::rng::XorShiftRng;
use crate::ui::DashboardUI;
use uefi::proto::console::text::{Color, Key};
use crate::env::{Application, /*ApplicationContext*/};
use crate::pm::PackageManager;
use crate::apps::simple_app::SimpleApp;
use crate::hpvmlog::LOGGING_SILENCED;
use crate::ui::pixel_graphics::PixelGraphics;

pub fn cmd(command: Vec<&str>, parts: &Vec<&str>, body: Vec<&str>, package_manager: &mut PackageManager) {
    match command.as_slice()[0] {
        "help" => {
            if parts.len() < 2 {
                message!("\n", "commands sets available: \n\nhelp fs - FileSystem Help\nhelp vm - VM Help\nhelp hv - Hypervisor Help\nhelp net - Network Help\nhelp misc - Misc. Help\nhelp prog [command] - Command-Specific Help\n\n")
            } else {
                match parts[1] {
                    "fs" => {
                        message!("\n", "\nFileSystem:\n \n  clear - clear screen\n  ls - list files\n  cd [dir] - change directory*\n  pwd - print working directory\n  mkdir [dir] - make directory\n  touch [file] - create file\n  cpy [src] [dst] - copy file\n  mov [src] [dst] - move file\n  rm [file] - remove file\n  cat [file] - show file contents\n  clon [src] [dst] - clone directory\n  write [file] [data] [mode] - write to file\n")
                    }
                    "vm" => {
                        message!("\n", "\nVM Management:\n  vm create [name] [memory_mb] [vcpus] - create VM\n  vm list - list all VMs\n  vm start [vm_id] - start VM\n  vm stop [vm_id] - stop VM\n  vm delete [vm_id] - delete VM\n  vm boot [vm_id] [iso|efi|img] - boot VM with media\n  boot vm [vm_id] [iso|efi|img] - boot VM with media\n  console [vm_id] - attach to VM console\n")
                    }
                    "hv" => {
                        message!("\n", "\nHypervisor:\n  vmm info - show hypervisor stats\n  vmm info-adv - show advanced stats\n\n")
                    }
                    "net" => {
                        message!("\n", "\nNetworking:\n  net status - show NIC status (SNP)\n  net up - initialize NIC via UEFI SNP\n  ping [ip] - test reachability (placeholder)\n  lanscan [x.y.z.] - scan /24 network (placeholder)\n  httpd start [port] - start HTTP management server (placeholder)\n  httpd stop - stop HTTP server\n\n")
                    }
                    "misc" => {
                        message!("\n", "\n Misc\nOther:\n  devs - list drives\n  info - show system info\n  sysinfo - show detailed system information\n  start [kernel] - load kernel*\n  shutdown [s|r] - shutdown(s) or reboot(r)\n  BIOS - exit to BIOS\n  mouse-debug - debug mouse protocols and data\nrun-efi [path] [args...] - run EFI application\n  dashboard - show management dashboard")
                    }
                    "prog" => {
                        message!("\n", "no help for '{}' (yet)", parts[2])
                    }
                    _ => {
                        message!("\n", "no help for this")
                    }
                }
            }
        },
        "clear" => { system::with_stdout(|s| s.clear().unwrap()); }
        "ls" => FileSystem::list_files(),
        "cd" => {
            if (command.len() == 2) {
                FileSystem::cd(command[1])
            } else {
                message!("\n", "Usage: cd [directory]")
            }
        }
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
        // "sysinfo" => {
        //     message!("\n", "=== System Information ===");
        //     message!("", "Memory Total: {} MB", sysinfo::);
        //     message!("", "Memory Used: {} MB", info.memory_used_mb);
        //     message!("", "Memory Free: {} MB", info.memory_total_mb - info.memory_used_mb);
        //     message!("", "CPU Count: {}", info.cpu_count);
        //     message!("", "CPU Usage: {}%", info.cpu_usage_percent);
        //     message!("", "CPU Frequency: {} MHz", info.cpu_frequency_mhz);
        //     message!("", "GPU Usage: {}%", info.gpu_usage_percent);
        //     message!("", "Disk Read Operations: {}", info.disk_read_ops);
        //     message!("", "Disk Write Operations: {}", info.disk_write_ops);
        //     message!("", "Disk Read Bytes: {} MB", info.disk_read_bytes / (1024 * 1024));
        //     message!("", "Disk Write Bytes: {} MB", info.disk_write_bytes / (1024 * 1024));
        // }
        "devs" => {
            if command.len() > 0 {
                FileSystem::get_drives("DEVICELIST");
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
        "BIOS" => {
            hpvm_warn!("bios", "unavailable");
            return;
        },
        "mouse-debug" => crate::graphics::Cursor::debug_mouse(),

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
                let vm_id: u32 = command[1].parse().unwrap_or_else(|_| {
                    hpvm_error!("Boot", "invalid VM ID");
                    22
                });
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
            unsafe {
                show_dashboard_ui(package_manager);
            }
        }

        // New: Console access to running VM
        "console" => {
            if command.len() < 2 {
                message!("\n", "Usage: console [vm_id]");
            } else {
                let vm_id: u32 = command[1].parse().unwrap_or_else(|_| {
                    hpvm_error!("Console", "invalid VM ID");
                    22
                });
                attach_vm_console(vm_id);
            }
        }
        "vmm" => {
            handle_vmm_command(&command);
        }
        ,
        "ping" => {
            if parts.len() < 2 { message!("\n", "Usage: ping [ip]"); } else {
                let _ = devices::net::ping(parts[1], 4, 250);
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
        "load-into" => unsafe {
            if command.len() == 2 {
                loader::load_and_jump_os(command[1])
            } else {
                message!("\n", "Usage: load-into [efi path]")
            }

        }

        "pm" => {
            crate::pm::command(parts, package_manager);
        }

        "micro-c" => {
            if parts.len() >= 3 {
                if parts[1] == "compile" {
                    let data = crate::micro_c::compile_from_file_to_asm(parts[2].parse().unwrap());
                    let newpath = parts[2].split(".").next().unwrap().to_owned() + ".asm";
                    FileSystem::touch(&*newpath);
                    FileSystem::write_to_file(&*newpath, &*data, 'w');
                }
            } else {
                message!("\n", "Usage micro-c [args]")
            }
        }

        "run-app" => unsafe {
            // if parts.len() >= 2 {
            //     let app_name = parts[1];
            //
            //     if let Some(mut context) = ApplicationContext::from_name(app_name) {
            //         // If you want the app to take over the whole screen immediately:
            //         context.run();
            //     } else {
            //         hpvm_error!("Shell", "Unknown application: {}", app_name);
            //     }
            // }
            message!("\n", "run-app is deprecated. use the Apps tab in dashboard instead")
        }


        _ => message!("\n", "unknown command: {:?}", command),
    }
    // let mut cursor: Cursor = Cursor::new();
    //
    // cursor.update_from_mouse();
    // with_stdout(|stdout| {cursor.render(stdout)});
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
            let mmap = unsafe { boot::exit_boot_services(None) };

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
                match crate::load_boot_media(media_path) {
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

    match crate::load_boot_media(efi_path) {
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
unsafe fn show_dashboard_ui(package_manager: &PackageManager) {
    let mut dashboard = DashboardUI::new(package_manager.clone());
    LOGGING_SILENCED = true;
    dashboard.refresh_storage();
    dashboard.refresh_devices();

    // Populate dashboard with VM information
    unsafe {
        if let Some(ref hv) = HYPERVISOR {
            let vms = hv.list_vms();

            for (id, name, state) in vms {
                let memory_usage_mb = if let Some(hv) = unsafe { &HYPERVISOR } {
                    hv.vms.get(&id).map(|v| v.memory_mb).unwrap_or(128)
                } else {
                    128
                };

                let vm_info = ui::VmDisplayInfo {
                    id,
                    name: String::from(name),
                    state: state.to_string(),
                    cpu_usage: 25,  // Placeholder
                    memory_usage_mb,
                    disk_usage_mb: 10240,  // Placeholder
                    uptime_seconds: 3600,  // Placeholder
                };
                dashboard.add_vm(vm_info);
            }

            // Get system resources
            let stats = hv.get_stats();
            let net_stats = devices::net_stack::stats();
            let mut core_usage = Vec::new();
            for i in 0..8 { core_usage.push(25 + i); }
            devices::net_stack::poll_tick();

            dashboard.set_resources(ui::SystemResources {
                total_memory_mb: stats.total_physical_memory_mb,
                used_memory_mb: stats.used_physical_memory_mb,
                cpu_count: 3,  // Placeholder
                cpu_usage: 35,  // Placeholder
                cpu_core_usage: core_usage,
                disk_read_kbps: 0,
                disk_write_kbps: 0,
                net_rx_kbps: net_stats.rx_bytes / 1024,
                net_tx_kbps: net_stats.tx_bytes / 1024,
                gpu_usage: 0,
                cpu_history: alloc::vec![],
                mem_history: alloc::vec![],
                disk_read_history: alloc::vec![],
                disk_write_history: alloc::vec![],
                net_rx_history: alloc::vec![],
                net_tx_history: alloc::vec![],
                gpu_history: alloc::vec![],
                fps: 0,
                frame_ms: 0,
            });
        }
    }

    // Enter dashboard interaction loop
    let mut last_refresh = 0;
    let refresh_rate = 15;

    let mut RNG: XorShiftRng = XorShiftRng::new(20);


    let mut frame_count = 0;
    let mut last_second_time = uefi::runtime::get_time().unwrap();
    let mut current_fps = 0;
    let mut current_frame_ms = 0;
    let mut current_cpu_usage = 0;

    loop {
        unsafe { crate::hpvmlog::BUSY_TSC = 0; }
        let frame_start_tsc = unsafe { core::arch::x86_64::_rdtsc() };

        frame_count += 1;

        let now = uefi::runtime::get_time().unwrap();

        // Check if 1 second has passed (simplistic check)
        if now.second() != last_second_time.second() {
            current_fps = frame_count;
            frame_count = 0;
            last_second_time = now;

            // Optional: Log it or update the dashboard
            // dashboard.set_fps(current_fps);
        }

        // Periodically refresh data from hypervisor
        last_refresh += 1;
        if last_refresh >= refresh_rate { // Refresh roughly every second
            let mut buffer = [0u8; 32768];
            unsafe {
                if let Some(ref hv) = HYPERVISOR {
                    dashboard.vms.clear();
                    let vms = hv.list_vms();
                    // We need memory usage for VMs too
                    let hv_vms = &hv.vms;

                    for (id, name, state) in vms {
                        let memory_usage_mb = hv_vms.get(&id).map(|v| v.memory_mb).unwrap_or(0);

                        dashboard.add_vm(ui::VmDisplayInfo {
                            id,
                            name: name.to_string(),
                            state: state.to_string(),
                            cpu_usage: RNG.rand_range(20, 50) as u32,
                            memory_usage_mb,
                            disk_usage_mb: 10240,
                            uptime_seconds: 3600,
                        });
                    }
                    let stats = hv.get_stats();
                    let net_stats = devices::net_stack::stats();

                    // Real per-core usage isn't available easily in UEFI without timers/interrupts tracking,
                    // so we simulate it based on total cpu_usage or random jitter for "realism".
                    let mut core_usage = Vec::new();
                    for i in 0..8 {
                        core_usage.push((RNG.rand_range(0, 2) + current_cpu_usage as u64) as u32);
                    }


                    // these values need to actually be measured (implement soon)
                    dashboard.set_resources(ui::SystemResources {
                        total_memory_mb: stats.total_physical_memory_mb,
                        used_memory_mb: stats.used_physical_memory_mb,
                        cpu_count: 3,
                        cpu_usage: current_cpu_usage as u32,
                        cpu_core_usage: core_usage,
                        disk_read_kbps: RNG.rand_range(50, 250), // Mocked
                        disk_write_kbps: RNG.rand_range(50, 250), // Mocked
                        net_rx_kbps: if RNG.rand_range(0, 500) < 400 { 0 } else { 150 }, //(net_stats.rx_bytes / 1024) % 1000, // Very rough
                        net_tx_kbps: if RNG.rand_range(0, 500) < 400 { 0 } else { 150 }, //(net_stats.tx_bytes / 1024) % 1000, // Very rough
                        gpu_usage: RNG.rand_range(1, 30) as u32, // Mocked
                        cpu_history: alloc::vec![],
                        mem_history: alloc::vec![],
                        disk_read_history: alloc::vec![],
                        disk_write_history: alloc::vec![],
                        net_rx_history: alloc::vec![],
                        net_tx_history: alloc::vec![],
                        gpu_history: alloc::vec![],
                        fps: current_fps,
                        frame_ms: current_frame_ms,
                    });
                }
            }
            dashboard.refresh_storage();
            dashboard.refresh_devices();
            last_refresh = 0;
        }

        dashboard.draw();

        dashboard.active_apps.retain_mut(|ctx| {
            // Logic: Give the app CPU time
            let mut vars = Vec::new();
            ctx.application.logic(&mut vars);

            // Draw: Give the app a reference to the screen
            // Note: You can pass a 'Viewport' or 'Offset' here so
            // the app knows where its "window" is located.
            if let Some(pg) = PixelGraphics::new() {
                let mut pg = pg.with_backbuffer();
                let (width, height) = pg.resolution();
                ctx.application.draw(&mut pg, &vars, 200, 200);
                pg.app_context_border(&ctx.application.name);
            }

            // Input: Forward the key if this app is focused
            // This reads key from stdin, which might consume it for others.
            // But we only do it if the app is focused.
            // For now, let's keep it simple.
            
            !ctx.exit_requested
        });


        let key = system::with_stdin(|i| {
            match i.read_key() {
                Ok(Some(key)) => Some(key),
                _ => None,
            }
        });

        if let Some(key) = key {
            let mut old_tab = dashboard.get_tab();
            // Forward input to focused app if any
            if let Some(focused_idx) = dashboard.focused_process_idx {
                if focused_idx < dashboard.active_apps.len() {
                    let app = &mut dashboard.active_apps[focused_idx];
                    app.handle_input(key);
                    if app.exit_requested {
                        dashboard.focused_process_idx = None;
                    }
                }
            } else {
                dashboard.handle_input(key);
            }

            // Handle VM Actions
            let is_enter = matches!(key, Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A);
            if matches!(old_tab, ui::DashboardTab::VirtualMachines) && is_enter {
                if let Some(vm_id) = dashboard.get_selected_vm_id() {
                    let action = dashboard.get_selected_action();
                    unsafe {
                        if let Some(hv) = HYPERVISOR.as_mut() {
                            match action {
                                0 => { let _ = hv.start_vm(vm_id); }
                                1 => { let _ = hv.stop_vm(vm_id); }
                                2 => { let _ = hv.reset_vm(vm_id); }
                                3 => { let _ = hv.zero_vm(vm_id); }
                                4 => { let _ = hv.delete_vm(vm_id); }
                                _ => {}
                            }
                        }
                    }
                    last_refresh = refresh_rate; // Force refresh
                }
            }

            // Handle Create VM Confirmation
            if matches!(old_tab, ui::DashboardTab::CreateVM) && dashboard.is_create_vm_requested() {
                let (name, mem, vcpus) = dashboard.get_create_vm_data();
                unsafe {
                    if let Some(hv) = HYPERVISOR.as_mut() {
                        if let Ok(vm_id) = hv.create_vm(&name, mem, vcpus) {
                            hpvm_info!("VMM", "Created VM '{}' via UI", name);
                        }
                    }
                }
                dashboard.reset_create_vm_data();
                dashboard.set_tab(ui::DashboardTab::VirtualMachines);
                last_refresh = refresh_rate; // Force refresh
            }

            if dashboard.exit_requested() {
                hpvm_info!("Dashboard", "exiting dashboard");
                if let Some(mut pg) = PixelGraphics::new() {
                    let mut pg = pg.with_backbuffer();
                    let (width, height) = pg.resolution();
                    //pg.exit()
                }
                break;
            }
        }

        let frame_end_tsc = unsafe { core::arch::x86_64::_rdtsc() };

        let cycles = frame_end_tsc.saturating_sub(frame_start_tsc);
        
        // Calculate CPU usage for this frame
        if cycles > 0 {
            let busy = unsafe { crate::hpvmlog::BUSY_TSC };
            current_cpu_usage = ((busy * 100) / cycles) as usize;
            if current_cpu_usage > 100 { current_cpu_usage = 100; }
        }

        // Limit frame rate to ~60Hz (16,666 microseconds)
        // 22_222 for 45 fps
        // currently at 32
        let us_elapsed = unsafe {
            if TSC_PER_US > 0 { cycles / TSC_PER_US } else { 0 }
        };

        current_frame_ms = (us_elapsed / 1000) as usize;

        if us_elapsed < 22_222 {
            let remaining = 22_222 - us_elapsed;
            boot::stall(Duration::from_micros(remaining));
        }
    }


    uefi::system::with_stdout(|stdout| {
        // Reset() is the most effective way to tell UEFI "ignore previous pixels, start over"
        let _ = stdout.reset(false);
        let _ = stdout.clear();
    });
    LOGGING_SILENCED = false;

    return;
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




