#![allow(unsafe_code, dead_code, non_camel_case_types, non_snake_case, unused, unused_must_use, unused_features, static_mut_refs, unsafe_op_in_unsafe_fn)]
#![feature(str_as_str)]
#![feature(abi_x86_interrupt)]
#![feature(core_float_math)]
#![feature(generic_atomic)]
#![feature(c_variadic)]
#![no_std]
#![no_main]
#![feature(const_heap)]
#![feature(try_trait_v2)]
#![feature(core_intrinsics)]
#![allow(internal_features)]
#![allow(stable_features)]



extern crate alloc;

mod ui;
mod kernel;
mod pe;
mod binary_loader;
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
mod traits;
mod loader;
mod terminal;
mod pm;
mod micro_c;
mod micro_c_externs;
mod cpucheck;
mod env;
mod apps;
mod input;
mod page;
mod modules;
mod c_stems;
mod registry;
mod dls;
mod backgrounds;
#[cfg(feature = "xmlui")]
mod xmlui;
mod multipar;
mod string_utils;
#[cfg(feature = "x4")]
mod x4;
mod version;
mod hosting;
mod testmodules;
mod localmodules;
mod xdb;
pub mod externals;
pub mod disk_executable;
#[cfg(feature = "crypt")]
mod crypto;
mod neural;
mod keys;
mod network;

pub use crate::micro_c::lexer;
pub use crate::micro_c::parser;
pub use crate::micro_c::ast;
pub use crate::micro_c::error;
pub use crate::micro_c::arch;
pub use crate::micro_c::codegen_ir;
pub use crate::micro_c::ir;
pub use crate::micro_c::regalloc;
pub use crate::micro_c::stackframe;
pub use crate::micro_c::fs;
pub use crate::xdb::interface;
pub use crate::xdb::database;
pub use crate::xdb::manager;
pub use crate::xdb::cache;
pub use crate::xdb::server;
pub use crate::xdb::cmd_parser;

static SYSTEMNAME: &str = "us-east-hypervisor-01";

pub fn get_total_physical_memory_mb() -> u32 {
    unsafe { TOTAL_PHYSICAL_MEMORY_MB }
}


//extern crate alloc;
use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use core::fmt::Write;
use core::ptr::addr_of_mut;
use core::time::Duration;
use uefi::prelude::*;
use uefi::Identify;
use uefi::Char16;
use log::error;
use uefi::boot;
use buddy_system_allocator::LockedHeap;
use uefi::boot::{MemoryType, ScopedProtocol};
use uefi::mem::memory_map::MemoryMap;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::console::text::Color;
use uefi::runtime::{self, ResetType};
//use uefi::system::with_stdout;
use uefi_raw::table::system::SystemTable;
use uefi::proto::console::pointer::{Pointer as SimplePointer, Pointer};
use uefi::proto::device_path::DevicePath;
use uefi::proto::unsafe_protocol;
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
use crate::ui::{DashboardTab, VectorSyncState};
use pm::PackageManager;
use crate::devices::timer::sleep_ms;
use crate::env::{GlobalEnvironment, GlobalEnvironmentData};
use crate::page::{Pagefile, PagefileHeader};
use crate::terminal::load_boot_media;
use crate::version::VersionGroup;

//#[global_allocator]
#[allow(dead_code, unused)]
static ALLOCATOR: LockedHeap<32> = LockedHeap::<32>::empty();

#[allow(dead_code, unused)]
static mut HEAP_STORAGE: [u8; 64 * 1024 * 1024] = [0; 64 * 1024 * 1024];

#[allow(dead_code, unused)]
static mut VIRT_STACK: [u8; 4 * 1024 * 1024] = [0; 4 * 1024 * 1024];

static mut PAGEFILE: Pagefile = Pagefile { header: PagefileHeader::DefaultHeader() };

static mut MOUSE: Option<ScopedProtocol<Pointer>> = None;

static mut GLOBAL_SYNC_STATE: Option<VectorSyncState> = Some(VectorSyncState::new(0));

static REG_HIVE_PATHS: [&str; 3] = [
    "/reg/hive/hive0.xdb", // Primary hive
    "/reg/hive/hive1.xdb", // Secondary hive
    "/reg/hive/hive2.xdb"];// Backup hive
//use crate::graphics::Cursor;

pub static mut GLOBALENV: Option<GlobalEnvironment> = None;

pub static mut HYPERVISOR: Option<HypervisorManager> = None;
static mut TOTAL_PHYSICAL_MEMORY_MB: u32 = 0;
static mut VERSION_DATA: Option<VersionGroup> = None;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn wcslen(mut s: *const u16) -> usize {
    let mut len = 0;
    while *s != 0 {
        len += 1;
        s = s.add(1);
    }
    len
}


//noinspection RsUnreachableCode
#[allow(dead_code, unused, unused_must_use, non_camel_case_types, nonstandard_style)]
#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    vdebug!("UEFI", "init uefi helpers");
    hpvmlog::init_log_buffer();

    // FIXED: Using addr_of_mut! to avoid static_mut_refs errors
    unsafe {
        let heap_ptr = addr_of_mut!(HEAP_STORAGE) as *mut u8;
        let heap_size = size_of_val(&&raw const HEAP_STORAGE);
        ALLOCATOR.lock().init(heap_ptr as usize, heap_size);
    }

    system::with_stdout(|stdout| {
        let _ = stdout.clear();
        let _ = stdout.enable_cursor(true);
    });

    vdebug!("HPVMx", "HPVMx version is {}", env!("CARGO_PKG_VERSION"));
    unsafe {
        VERSION_DATA = Some(VersionGroup::from_env());
    }
    vdebug!("malloc", "heap initialized. retrieving memory map...");

    // 2. In uefi 0.36.1 with 'alloc' feature, use boot::memory_map()
    // This returns a MemoryMapOwned object automatically using the heap.
    //let size = uefi::boot::memory_map_size().map_size;
    let size = boot::PAGE_SIZE;
    vdebug!("page", "system required buffer of {} bytes", size);

    // 16KB is usually enough for most servers; 32KB is safe for high-end systems.
    let mut map_buffer = [0u8; 32768];
    vdebug!("page", "set map buffer to {}", map_buffer.len());


    let SYSTEM_TABLE: *mut SystemTable = uefi::table::system_table_raw().unwrap().as_ptr();



    // 2. Use get_memory_map_static instead of the alloc version
    // This does NOT use your LockedHeap; it uses the array above.
    // let memory_map = uefi::boot::get_boot_services().memory_map(&mut map_buffer)
    //     .expect("Failed to get memory map. Buffer might be too small.");

    // let memory_map = boot::memory_map(MemoryType::CONVENTIONAL)
    //     .expect("failed to retrieve memory map. ensure 'alloc' feature is enabled in Cargo.toml");

    match boot::memory_map(MemoryType::LOADER_DATA) {
        Ok(map) => {
            vdebug!("malloc", "retrieved memory map with {} entries.  OMT (bsc/bsd)", map.entries().count());

            let mut total_mb = 0;
            // Iterate and filter for free RAM
            for entry in map.entries() {
                let size_mb = (entry.page_count * 4096) / (1024 * 1024);
                total_mb += size_mb;

                match entry.ty {
                    MemoryType::BOOT_SERVICES_CODE => {}
                    MemoryType::BOOT_SERVICES_DATA => {}

                    _ => vdebug!("malloc",
                         "AREA {:#?}  START {:#x}  PAGE {}  ATT: {:?}  VS: {:?}",
                         entry.ty,
                         entry.phys_start,
                         entry.page_count,
                         entry.att,
                         entry.virt_start,
                     )
                }
            }
            unsafe { TOTAL_PHYSICAL_MEMORY_MB = total_mb as u32; }
            vdebug!("malloc", "Total physical memory: {} MB", total_mb);
        }
        Err(e) => {
            error!("Failed to retrieve memory map: {:?}", e.status());
        }
    };

    vdebug!("GDT", "initializing gdt");
    gdt::init();
    vdebug!("IDT", "initializing idt");
    interrupts::init_idt();

    vdebug!("page", "setting active paging mapper");
    let mut mapper = unsafe { PagingManager::get_active_mapper(x86_64::VirtAddr::new(16384)) };


    vdebug!("fs", "building devicelist");

    // Identify boot disk
    let loaded_image = boot::open_protocol_exclusive::<uefi::proto::loaded_image::LoadedImage>(boot::image_handle()).unwrap();
    let boot_device = loaded_image.device();
    FileSystem::set_root_handle(boot_device);

    while !(FileSystem::is_handle()) {
        // wait
        hpvm_warn!("fs", "waiting for file handle")
    }

    FileSystem::scan_and_map_devices("DEVICELIST").unwrap();

    vdebug!("fs", "devicelist mapped");

    unsafe {
        vdebug!("page", "creating pagefile");
        PAGEFILE.create_pagefile(268435456, 8);
    }




    unsafe {
        init_mouse();
    }

    // Initialize XDB before network so saved static IP config can be loaded
    crate::xdb::init_global_db(REG_HIVE_PATHS[0]);
    {
        let dbserver0 = crate::xdb::get_global_db_mut().unwrap();
        // Only create the table if it doesn't already exist — preserves saved data (e.g. network config) across reboots
        if !dbserver0.manager.cache.entries.contains_key(&1) {
            let mut out: String = dbserver0.parse_command(&"CREATE TABLE 1 superhive_meta ( id Int32 key Int32 name Text type Text data Text )".as_bytes().to_vec(), &mut Vec::new());
            out += ";";
            out += &*(dbserver0.parse_command(&"COMMIT".as_bytes().to_vec(), &mut Vec::new()));
        }
        // Always update the version string
        let mut out: String = dbserver0.parse_command(&format!("INSERT INTO 1 ROW 1 VALUES Int32(1) Int32(1) Text(\"version_str\") Text(\"String\") Text(\"{}\")", VersionGroup::from_env().format_to_string()).as_bytes().to_vec(), &mut Vec::new());
        out += ";";
        out += &*(dbserver0.parse_command(&"COMMIT".as_bytes().to_vec(), &mut Vec::new()));
    }
    let dbserver1 = crate::xdb::open_db(REG_HIVE_PATHS[1]);
    let dbserver2 = crate::xdb::open_db(REG_HIVE_PATHS[2]);

    // Initialize network hardware - this MUST succeed for the HTTP server to work
    // Note: httpd_start() is deferred until after MP/async init so the AP poll worker can spawn correctly
    match devices::net_hw::init() {
        Ok(_) => {
            vdebug_autoprefix!(10, "Network hardware initialized successfully");
            devices::net::status();
        }
        Err(e) => {
            vdebug_autoprefix!(12, "Network hardware init failed: {} - HTTP server will NOT work", e);
        }
    }

    vdebug!("HPVMx", "init sequence complete.");
    let mut PACKAGE_MANAGER: PackageManager = PackageManager::new();
    PACKAGE_MANAGER.load_registry();

    vdebug!("CPU", "calibrating tsc");
    calibrate_tsc();
    vdebug!("CPU", "tsc cyc/us {}", TSC_PER_US);
    let _cpu_info = hardware::cpu::CpuInfo::detect();

    unsafe {
        HYPERVISOR = Some(HypervisorManager::new());
        if let Some(ref mut hv) = HYPERVISOR {
            match hv.initialize() {
                Ok(_) => { vdebug!("VMM", "hypervisor initialized");
                           hv.cpu_props_from_info(&_cpu_info)
                },
                Err(e) => hpvm_warn!("VMM", "hypervisor init failed: {}", e),
            }
        }
    }

    let _ = boot::set_watchdog_timer(0, 0, None);

    vdebug!("env", "creating globalenv");
    unsafe {
        GLOBALENV = Some(GlobalEnvironment::new());
    }

    if env::run_async_tests() {
        vdebug_autoprefix!(10, "async/await multitasking verified");
    } else {
        vdebug_autoprefix!(12, "async multitasking self-test failed");
    }

    if multipar::run_multipar_tests() {
        vdebug_autoprefix!(10, "multi-core async executor & ArcWaker verified");
    } else {
        vdebug_autoprefix!(12, "multi-core async executor self-test failed");
    }

    if hardware::cpu::mp::run_mp_tests() {
        vdebug_autoprefix!(10, "MP topology verification passed");
    } else {
        vdebug_autoprefix!(12, "MP topology verification failed");
    }

    if testmodules::boot::run_all_boot_tests() {
        vdebug_autoprefix!(10, "startup, VMM/CVM and hosting self-tests passed");
    } else {
        vdebug_autoprefix!(12, "startup, VMM/CVM and hosting self-tests failed");
    }

    multipar::init_global_executor();
    vdebug_autoprefix!(11, "global async executor initialized");

    // Start HTTP server AFTER MP and async executor are ready so the AP poll worker can spawn
    if devices::net::is_initialized() {
        devices::net::httpd_start(8080);
        vdebug_autoprefix!(10, "HTTP management server started on port 8080");
    }

    vdebug!("HPVMx", "ready");
    hpvm_warn!("HPVMx", "within spinloop");
    //Graphics::get_graphics_info();

    const icon_ascii: &str = "\n\n
       __ _____ _   ____  ___
      / // / _ \\ | / /  |/  /_ __
     / _  / ___/ |/ / /|_/ /\\ \\ /
    /_//_/_/   |___/_/  /_//_\\_\\      \n\n";

    message!("", "{}", icon_ascii);


    // Auto-start dashboard if configured in config.cfg or if hypervisor is available (default behavior)
    FileSystem::cd("/");
    let mut auto_start_dashboard = false;
    if let Ok(config_data) = FileSystem::read_file_to_string("config.cfg") {
        if config_data.contains("onstart=\"dashboard\"") {
            auto_start_dashboard = true;
            vdebug!("HPVMx", "Auto-starting dashboard from config.cfg");
        }
    }
    // Default: start dashboard when hypervisor is available
    if !auto_start_dashboard {
        unsafe {
            if crate::HYPERVISOR.is_some() {
                auto_start_dashboard = true;
                vdebug!("HPVMx", "Auto-starting dashboard (hypervisor available)");
            }
        }
    }
    if auto_start_dashboard {
        unsafe {
            terminal::show_dashboard_ui(&PACKAGE_MANAGER);
        }
    }

    vdebug!("HPVMx", "HPVMx Shell v1.3.2");
    vdebug!("HPVMx", "Type 'help' for commands.");

    let mut input_buffer = String::new();

    // let mut executor = Executor::new();
    // executor
    //     .add(&mut TaskNode::new(Box::pin(draw()), 60))
    //     .run_forever();

    // Create a timer event for non-blocking input polling
    let mut timer_event = match unsafe { boot::create_event(uefi::boot::EventType::TIMER, uefi::boot::Tpl(0), None, None) } {
        Ok(e) => e,
        Err(_) => {
            hpvm_warn!("main", "Failed to create timer event, falling back to blocking input");
            loop {
                multipar::task::poll_global_ready();
                terminal::tick_disk_executables();
                devices::net_stack::poll_tick();
                devices::audio::update();
                if !devices::hda::is_available() { let _ = devices::hda::init(); }
                system::with_stdout(|s| Write::write_str(s, "\nHPVMx> ").unwrap());
                input_buffer.clear();
                read_line(&mut input_buffer);
                let unclean = input_buffer.trim();
                let mut command = String::with_capacity(unclean.len());
                let mut consecutive_backspaces = 0;
                for c in unclean.chars() {
                    if c == '\u{8}' { consecutive_backspaces += 1; if consecutive_backspaces >= 2 { command.pop(); } command.pop(); }
                    else { consecutive_backspaces = 0; command.push(c); }
                }
                if command.is_empty() { continue; }
                let parts = command.split(" ").collect::<Vec<&str>>();
                terminal::cmd(command.split(" ").collect(), &parts, command.split(" ").collect(), &mut PACKAGE_MANAGER, None);
            }
        }
    };

    // Set timer to fire every 1ms for responsive network polling.
    // The `net-multicore` feature is disabled by default, so the BSP handles
    // all network polling via this fast timer instead of a dedicated AP worker.
    let _ = boot::set_timer(&timer_event, uefi::boot::TimerTrigger::Periodic(Duration::from_millis(1)));

    let key_event = system::with_stdin(|i| i.wait_for_key_event().unwrap());

    loop {
        // Wait for either a key press or the timer
        let triggered_index = unsafe {
            let events: [*const uefi::Event; 2] = [core::ptr::addr_of!(key_event), core::ptr::addr_of!(timer_event)];
            let events_slice: &mut [uefi::Event] = core::slice::from_raw_parts_mut(events.as_ptr() as *mut uefi::Event, 2);
            boot::wait_for_event(events_slice)
        };
        let triggered_index = match triggered_index {
            Ok(idx) => idx,
            Err(_) => continue,
        };

        let triggered_is_key = triggered_index == 0;

        run_polling();

        // Check if we have a key to read
        if let Some(key) = system::with_stdin(|i| i.read_key().unwrap()) {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    if !input_buffer.is_empty() {
                        input_buffer.pop();
                        system::with_stdout(|s| {
                            let _ = core::fmt::Write::write_str(s, "\u{0008} \u{0008}");
                        });
                    }
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        // Execute the command
                        system::with_stdout(|s| {
                            let _ = Write::write_char(s, ch);
                            let _ = Write::write_char(s, "\n".parse::<char>().unwrap());
                        });

                        if !input_buffer.is_empty() {
                            let command = input_buffer.clone();
                            let parts = command.split(" ").collect::<Vec<&str>>();
                            terminal::cmd(command.split(" ").collect(), &parts, command.split(" ").collect(), &mut PACKAGE_MANAGER, None);
                        }
                        input_buffer.clear();
                        // Print new prompt
                        system::with_stdout(|s| {
                            let _ = Write::write_str(s, "\nHPVMx> ");
                        });
                    } else if ch != '`' {
                        input_buffer.push(ch);
                        system::with_stdout(|s| {
                            let _ = Write::write_char(s, ch);
                        });
                    }
                }
                _ => {}
            }
        }
        // Timer fired, no key to read — just continue polling
    }

    Status::SUCCESS
}

pub fn run_polling() {
    // Poll ready asynchronous tasks and disk-loaded background executables.
    multipar::task::poll_global_ready();
    terminal::tick_disk_executables();

    // drive network timers - poll frequently for real-time TCP handling
    for _ in 0..10 {
        devices::net_stack::poll_tick();
    }

    // update non-blocking audio
    devices::audio::update();

    // init HDA if not initialized
    if !devices::hda::is_available() {
        let _ = devices::hda::init();
    }
}

fn read_line(buf: &mut String) {
    loop {
        let mut events = [system::with_stdin(|i| i.wait_for_key_event().unwrap())];
        boot::wait_for_event(&mut events).unwrap();

        if let Some(key) = system::with_stdin(|i| i.read_key().unwrap()) {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    buf.remove(buf.len() - 1);
                    // if buf.pop().is_some() {
                    //     uefi::system::with_stdout(|s| core::fmt::Write::write_str(s, "\u{0008} \u{0008}").unwrap());
                    // }
                    system::with_stdout(|s| {
                        s.clear().unwrap(); // Clear current line
                        s.write_str("HPVMx> ").unwrap(); // Rewrite prompt
                        s.write_str(&buf[..buf.len() - 1]).unwrap(); // Rewrite buffer without last char
                    });
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        system::with_stdout(|s| Write::write_char(s, ch).unwrap());
                        system::with_stdout(|s| Write::write_char(s, "\n".parse().unwrap()).unwrap());
                        break;
                    }
                    if ch != '`' /* != '\u{8}' */ {
                        buf.push(ch);
                        // Echo to screen
                        system::with_stdout(|s| Write::write_char(s, ch).unwrap());
                    }
                }
                _ => {}
            }
        }
    }
}

fn read_line_int(buf: &mut String) -> i32 {
    loop {
        let mut events = [system::with_stdin(|i| i.wait_for_key_event().unwrap())];
        boot::wait_for_event(&mut events).unwrap();

        if let Some(key) = system::with_stdin(|i| i.read_key().unwrap()) {
            match key {
                Key::Special(ScanCode::DELETE) => {
                    buf.remove(buf.len() - 1);
                    // if buf.pop().is_some() {
                    //     uefi::system::with_stdout(|s| core::fmt::Write::write_str(s, "\u{0008} \u{0008}").unwrap());
                    // }
                    system::with_stdout(|s| {
                        s.clear().unwrap(); // Clear current line
                        s.write_str("> ").unwrap(); // Rewrite prompt
                        s.write_str(&buf[..buf.len() - 1]).unwrap(); // Rewrite buffer without last char
                    });
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        system::with_stdout(|s| Write::write_char(s, ch).unwrap());
                        system::with_stdout(|s| Write::write_char(s, "\n".parse().unwrap()).unwrap());
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
                        system::with_stdout(|s| Write::write_char(s, ch).unwrap());
                    }
                }
                _ => {}
            }
        }
    }
}

/// Run a micro-c compiled binary
fn run_asm_application(asm_path: &str) {
    crate::vdebug!("ASM", "loading micro-c binary from '{}'", asm_path);

    match load_boot_media(asm_path) {
        Ok(asm_data) => {
            crate::vdebug!("ASM", "loaded {} bytes", asm_data.len());

            // By default, raw x86_64 binaries from micro-c use ORG 0x100000
            match binary_loader::BinaryLoader::load_and_run(&asm_data, 0x100000) {
                Ok(_) => {
                    crate::vdebug!("ASM", "execution finished");
                }
                Err(e) => {
                    hpvm_error!("ASM", "failed to load/run ASM: {}", e);
                }
            }
        }
        Err(e) => {
            hpvm_error!("ASM", "failed to load binary file: {}", e);
        }
    }
}

pub fn init_mouse() {
    unsafe {
        MOUSE = init_mouse_deep_scan();
    }
}

pub unsafe fn init_mouse_deep_scan() -> Option<ScopedProtocol<Pointer>> {
    // 1. Multi-pass connect all controllers (EDK2 BdsConnectAll style)
    // Connecting PCI controllers creates child USB controller handles;
    // connecting USB controllers creates child USB device (mouse/tablet) handles.
    for pass in 0..4 {
        if let Ok(all_handles) = boot::locate_handle_buffer(boot::SearchType::AllHandles) {
            vdebug!("usbhid", "ConnectAll pass {}: scanning {} handles", pass, all_handles.len());
            for handle in all_handles.iter() {
                let _ = boot::connect_controller(*handle, &[None], None, true);
            }
        }
    }

    // 2. Scan & reset SimplePointer handles
    if let Ok(handles) = boot::locate_handle_buffer(boot::SearchType::ByProtocol(&SimplePointer::GUID)) {
        vdebug!("usbhid", "Found {} SimplePointer handles", handles.len());

        for (i, handle) in handles.iter().enumerate() {
            if let Ok(mut mouse) = boot::open_protocol::<SimplePointer>(
                boot::OpenProtocolParams {
                    handle: *handle,
                    agent: boot::image_handle(),
                    controller: None,
                },
                boot::OpenProtocolAttributes::GetProtocol,
            ) {
                let r = mouse.reset(false);
                vdebug!("usbhid", "Handle [{}]: Reset result {:?}", i, r);
                #[allow(irrefutable_let_patterns)]
                if let mode = mouse.mode() {
                    vdebug!("usbhid", "Handle [{}]: Res X={}, Y={}, Z={}", i, /*mode.resolution_x, mode.resolution_y, mode.resolution_z*/ mode.resolution[0], mode.resolution[1], mode.resolution[2]);
                }
                return Some(mouse)
            }
        }
    }
    None

    // 3. Scan & reset AbsolutePointer handles
    // if let Ok(handles) = boot::locate_handle_buffer(boot::SearchType::ByProtocol(&uefi_raw::protocol::console::AbsolutePointerProtocol::GUID)) {
    //     vdebug!("usbhid", "Found {} AbsolutePointer handles", handles.len());
    //
    //     for (i, handle) in handles.iter().enumerate() {
    //         if let Ok(mut mouse) = boot::open_protocol::<uefi_raw::protocol::console::AbsolutePointerProtocol>(
    //             boot::OpenProtocolParams {
    //                 handle: *handle,
    //                 agent: boot::image_handle(),
    //                 controller: None,
    //             },
    //             boot::OpenProtocolAttributes::GetProtocol,
    //         ) {
    //             let r = mouse.reset(false);
    //             vdebug!("usbhid", "Abs Handle [{}]: Reset result {:?}", i, r);
    //             if let Some(mode) = mouse.mode() {
    //                 vdebug!("usbhid", "Abs Handle [{}]: Max X={}, Max Y={}", i, mode.absolute_max_x, mode.absolute_max_y);
    //             }
    //             return Some(mouse)
    //
    //         }
    //     }
    //     None
    // }
}

static mut TSC_PER_US: u64 = 0;

pub fn calibrate_tsc() {
    let start_tsc = unsafe { core::arch::x86_64::_rdtsc() };

    // Stall for 50,000 microseconds (50ms)
    boot::stall(Duration::from_micros(50_000));
    // somehow one-core operation messes up the time measurement

    let end_tsc = unsafe { core::arch::x86_64::_rdtsc() };
    let total_ticks = end_tsc.saturating_sub(start_tsc);

    unsafe {
        // Ticks per microsecond
        TSC_PER_US = total_ticks / 50_000;
        if TSC_PER_US < 200 {
            TSC_PER_US += 240;
        }
    }
}


