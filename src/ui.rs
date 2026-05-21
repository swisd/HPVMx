#![allow(dead_code, deprecated)]

//! User Interface and dashboard management.
//!
//! This module contains the core UI logic, including the `DashboardUI`
//! which manages the main display, active applications, and system status.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use crate::{hpvm_error, hpvm_info, hpvm_log, TSC_PER_US};
use alloc::fmt::format;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::any::Any;
use core::char;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::runtime;
use uefi::runtime::VariableKey;
use uefi_raw::Status;
use uefi_raw::table::runtime::ResetType;

mod graphics;
pub mod pixel_graphics;
pub mod graphics3d;


use crate::{handle_vm_command, hpvm_warn, message, terminal};
use crate::pm::{Package, PackageManager, PackageType};
use pixel_graphics::{PixelGraphics, TreeViewNode};
use crate::apps::error::ErrorApp;
use crate::env::{Application, SteppedApplicationContext};
use crate::input::ScanCodeV2;

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[derive(Clone, Debug)]
pub struct DeviceEntry {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug)]
pub struct DeviceCategory {
    pub name: String,
    pub devices: Vec<DeviceEntry>,
    pub expanded: bool,
    pub icon: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilePendingAction {
    Rename,
    Copy,
    Move,
    Delete,
}

/// Main UI manager for the HPVMx system.
///
/// Handles the dashboard, windowing system for applications,
/// and user input routing.
pub struct DashboardUI {
    selected_tab: DashboardTab,
    pub vms: Vec<VmDisplayInfo>,
    pub resources: SystemResources,
    scroll_offset: usize,
    cursor: crate::graphics::Cursor,
    pub current_path: String,
    pub files: Vec<FileEntry>,
    pub selected_file_idx: usize,
    pub categories: Vec<DeviceCategory>,
    pub selected_device_idx: usize,
    pub device_action_idx: usize,
    exit_requested: bool,

    // Fields for Create VM UI
    pub new_vm_name: String,
    pub new_vm_memory_mb: u32,
    pub new_vm_vcpus: u32,
    pub create_vm_focus_idx: usize, 
    pub vm_action_idx: usize, // For VM actions (0: Start, 1: Stop, 2: Reset, 3: Zero, 4: Delete)
    pub selected_vm_idx: usize,
    pub filesys_action_idx: usize,
    pub filesys_pending_action: Option<FilePendingAction>,
    pub filesys_new_counter: usize,
    pub term_selected: bool,
    pub term_buf: String,
    pub editor: Option<TextEditor>,
    pub package_manager: PackageManager,
    pub iter: u64,
    pub active_apps: Vec<SteppedApplicationContext>,
    pub focused_process_idx: Option<usize>, // Which app gets the keyboard?
    pub selected_app_idx: usize,
    pub app_window_position: (usize, usize),
    pub ctrl_mode: bool,
    pub alt_mode: bool,
    pub fn_mode: bool,
    pub selected_package_idx: usize,
    pub package_action_idx: usize,
    pub selected_network_action_idx: usize,
    pub network_target: String,
    pub selected_settings_category_idx: usize,
    pub selected_settings_idx: usize,
    pub settings: UiSettings,
    pub status_line: String,
    pub command_history: Vec<String>,
    pub history_idx: Option<usize>,

    // New functional UI features
    pub notifications: Vec<(String, usize)>, // (message, duration_frames)
    pub command_palette_active: bool,
    pub command_palette_query: String,
    pub command_palette_selected: usize,
    pub command_palette_scroll_offset: usize,



    pub glitch_y: usize,
    pub pci_devices: Vec<crate::hardware::pci::PciDeviceInfo>,
}

#[derive(Clone, Debug)]
pub struct UiSettings {
    pub extra_debug_info: bool,
    pub folder_absolute_sizes: bool,
    pub state_save_restore: bool,
    pub extended_symbol_library: bool,
    pub ring0_udmi_udxi: bool,
    pub controllang_support: bool,
    pub pg_vshaders: bool,
    pub experimental_mem_comp: bool,
    pub auto_refresh_storage: bool,
    pub show_hidden_files: bool,
    pub general_profile: usize,
    pub boot_target: usize,
    pub interface_density: usize,
    pub vm_safety_policy: usize,
    pub network_profile: usize,
    pub storage_policy: usize,
    pub package_policy: usize,
    pub developer_level: usize,
    pub security_policy: usize,
    pub ui_scaling: usize,
    pub terminal_font: usize,
    pub pg_scanlines: bool,
    pub pg_dither: bool,
    pub pg_glitch: bool,
    pub pg_aberration: usize, // 0: off, 1: low, 2: mid, 3: high
}

#[derive(Clone, Copy, Debug)]
/// Available tabs in the dashboard.
pub enum DashboardTab {
    Overview,
    VirtualMachines,
    Resources,
    Storage,
    Network,
    Console,
    Devices,
    Test,
    CreateVM, // New state for VM creation UI
    Editor,
    Settings,
    Packages,
    Apps,
}

const COMMAND_PALETTE_VISIBLE_COUNT: usize = 10;

const TERMINAL_PALETTE_COMMANDS: &[&str] = &[
    "help - Show command help",
    "help fs - FileSystem help",
    "help vm - VM help",
    "help hv - Hypervisor help",
    "help net - Network help",
    "help pm - Package manager help",
    "help micro-c - Micro-C help",
    "help misc - Miscellaneous help",
    "help prog <command> - Command-specific help",
    "clear - Clear screen",
    "ls - List files",
    "cd <dir> - Change directory",
    "pwd - Print working directory",
    "mkdir <dir> - Make directory",
    "touch <file> - Create file",
    "cpy <src> <dst> - Copy file",
    "mov <src> <dst> - Move file",
    "rm <file> - Remove file",
    "cat <file> - Show file contents",
    "clon <src> <dst> - Clone directory",
    "write <file> <data> <mode> - Write to file",
    "devs - List drives",
    "info - Show system info",
    "sysinfo - Show detailed system information",
    "start <kernel> - Load kernel",
    "shutdown s - Shutdown host",
    "shutdown r - Reboot host",
    "BIOS - Exit to BIOS",
    "mouse-debug - Debug mouse",
    "run-efi <path> [args...] - Run EFI application",
    "dashboard - Show dashboard",
    "console <vm_id> - Attach to VM console",
    "boot <vm_id> <media> - Boot VM with media",
    "load-into <path> - Load and jump to EFI path",
    "vm create <name> <mem> <vcpus> - Create VM",
    "vm list - List VMs",
    "vm start <id> - Start VM",
    "vm stop <id> - Stop VM",
    "vm delete <id> - Delete VM",
    "vm boot <id> <media> - Boot VM with media",
    "vm simulate-violation <id> <code> - Simulate violation",
    "vmm info - Show VMM stats",
    "vmm info-adv - Show advanced VMM stats",
    "ping <ip> - Ping host",
    "lanscan <network> - Scan network",
    "httpd start <port> - Start HTTP server",
    "httpd stop - Stop HTTP server",
    "net status - Show NIC status",
    "net up - Initialize NIC",
    "logiclang - Open LogicLang prompt",
    "pm list - List packages",
    "pm reload - Reload registry",
    "pm verify <name> - Verify package",
    "pm version - Show PM version",
    "micro-c compile <file> - Compile Micro-C",
    "micro-c run <file> - Run Micro-C binary",
    "run-app <name> - Run application (deprecated)",
];

const UI_PALETTE_COMMANDS: &[&str] = &[
    "Overview: Show System Summary",
    "Overview: Refresh Resources",
    "VM: List Virtual Machines",
    "VM: Create New VM",
    "VM: Start Selected VM",
    "VM: Stop Selected VM",
    "VM: Delete Selected VM",
    "VM: Boot with Media",
    "VM: Simulate Violation",
    "VM: Refresh List",
    "Resources: View CPU/Memory",
    "Storage: Browse Files",
    "Storage: Refresh Drives",
    "Storage: New File",
    "Storage: New Folder",
    "Storage: Rename Selected",
    "Storage: Copy Selected",
    "Storage: Move Selected",
    "Storage: Delete Selected",
    "Storage: Toggle Hidden Files",
    "Storage: Toggle Auto Refresh",
    "Network: View Connections",
    "Network: Show NIC Status",
    "Network: Initialize SNP",
    "Network: Ping Target",
    "Network: LAN Scan",
    "Network: Start HTTP Server",
    "Network: Stop HTTP Server",
    "Console: Attach to VM",
    "Devices: Manage Hardware",
    "Devices: List Drives",
    "Devices: Refresh Hardware",
    "Apps: Run Applications",
    "Packages: Software Manager",
    "Packages: Reload Registry",
    "Packages: Verify Dependencies",
    "Packages: Show Version",
    "Settings: UI Configuration",
    "Settings: Toggle Extra Debug Info",
    "Settings: Toggle State Save/Restore",
    "Settings: Toggle Scanlines",
    "Settings: Toggle Dither",
    "Settings: Toggle Glitch",
    "Settings: Cycle Aberration",
    "Settings: Cycle Interface Density",
    "Settings: Cycle UI Scaling",
    "Settings: Cycle General Profile",
    "Settings: Cycle Boot Target",
    "Settings: Cycle VM Safety",
    "Settings: Cycle Network Profile",
    "Settings: Cycle Storage Policy",
    "Settings: Cycle Package Policy",
    "Settings: Cycle Developer Level",
    "Settings: Cycle Security Policy",
    "Test: Run Diagnostics",
    "Editor: Text Editor",
    "Editor: New Buffer",
    "VMM: Show Statistics",
    "VMM: Advanced Stats",
    "System: Refresh Storage",
    "System: Refresh Devices",
    "System: Refresh All",
    "System: Clear Logs",
    "System: System Information",
    "System: BIOS (Exit)",
    "System: Reboot Host",
    "System: Shutdown",
    "System: Mouse Debugging",
    "Help: Command Palette Shortcuts",
    "Help: FileSystem Help",
    "Help: VM Help",
    "Help: Hypervisor Help",
    "Help: Network Help",
    "Help: Package Manager Help",
    "Help: Micro-C Help",
    "Help: Misc Help",
];

#[derive(PartialEq)]
pub enum EditorMode {
    Normal,
    Insert,
    Command,
}

pub struct TextEditor {
    pub file_path: String,
    pub buffer: Vec<u8>,
    pub cursor_pos: (usize, usize), // (line, col)
    pub scroll_offset: usize,
    pub mode: EditorMode,
    pub is_hex: bool,
    pub command_buffer: String, // For : commands
}

impl TextEditor {
    pub fn new(path: String, data: Vec<u8>) -> Self {
        // Detect if file is binary/unreadable
        let is_hex = core::str::from_utf8(&data).is_err();
        Self {
            file_path: path,
            buffer: data,
            cursor_pos: (0, 0),
            scroll_offset: 0,
            mode: EditorMode::Normal,
            is_hex: false,
            command_buffer: "".to_string(),
        }
    }
}

pub struct VmDisplayInfo {
    pub id: u32,
    pub name: String,
    pub state: String,
    pub cpu_usage: u32,
    pub memory_usage_mb: u32,
    pub disk_usage_mb: u32,
    pub uptime_seconds: u64,
}

pub struct SystemResources {
    pub total_memory_mb: u32,
    pub used_memory_mb: u32,
    pub cpu_count: u32,
    pub cpu_usage: u32,
    pub cpu_core_usage: Vec<u32>,
    pub disk_read_kbps: u64,
    pub disk_write_kbps: u64,
    pub net_rx_kbps: u64,
    pub net_tx_kbps: u64,
    pub gpu_usage: u32,
    
    // History for graphs
    pub cpu_history: Vec<u32>,
    pub mem_history: Vec<u32>,
    pub disk_read_history: Vec<u64>,
    pub disk_write_history: Vec<u64>,
    pub net_rx_history: Vec<u64>,
    pub net_tx_history: Vec<u64>,
    pub gpu_history: Vec<u32>,

    pub fps_history: Vec<u32>,
    pub ft_ms_history: Vec<u32>,

    pub fps: usize,
    pub frame_ms: usize,
}

impl DashboardUI {
    pub fn new(package_manager: PackageManager) -> Self {
        Self {
            selected_tab: DashboardTab::Overview,
            vms: Vec::new(),
            resources: SystemResources {
                total_memory_mb: 0,
                used_memory_mb: 0,
                cpu_count: 0,
                cpu_usage: 0,
                cpu_core_usage: Vec::new(),
                disk_read_kbps: 0,
                disk_write_kbps: 0,
                net_rx_kbps: 0,
                net_tx_kbps: 0,
                gpu_usage: 0,
                cpu_history: Vec::with_capacity(100),
                mem_history: Vec::with_capacity(100),
                disk_read_history: Vec::with_capacity(100),
                disk_write_history: Vec::with_capacity(100),
                net_rx_history: Vec::with_capacity(100),
                net_tx_history: Vec::with_capacity(100),
                gpu_history: Vec::with_capacity(100),
                fps_history: Vec::with_capacity(100),
                ft_ms_history: Vec::with_capacity(100),
                fps: 0,
                frame_ms: 0,
            },
            scroll_offset: 0,
            cursor: crate::graphics::Cursor::new(),
            current_path: String::from("\\"),
            files: Vec::new(),
            selected_file_idx: 0,
            categories: Vec::new(),
            selected_device_idx: 0,
            device_action_idx: 0,
            exit_requested: false,
            new_vm_name: String::from("NewVM"),
            new_vm_memory_mb: 256,
            new_vm_vcpus: 1,
            create_vm_focus_idx: 0,
            vm_action_idx: 0,
            selected_vm_idx: 0,
            filesys_action_idx: 0,
            filesys_pending_action: None,
            filesys_new_counter: 1,
            term_selected: false,
            term_buf: "".to_string(),
            editor: None,
            package_manager,
            iter: 0,
            active_apps: Vec::new(),
            focused_process_idx: None,
            selected_app_idx: 0,

            app_window_position: (100, 100),
            ctrl_mode: false,
            alt_mode: false,
            fn_mode: false,
            selected_package_idx: 0,
            package_action_idx: 0,
            selected_network_action_idx: 0,
            network_target: String::from("127.0.0.1"),
            selected_settings_category_idx: 0,
            selected_settings_idx: 0,
            settings: UiSettings {
                extra_debug_info: false,
                folder_absolute_sizes: false,
                state_save_restore: true,
                extended_symbol_library: true,
                ring0_udmi_udxi: false,
                controllang_support: false,
                pg_vshaders: true,
                experimental_mem_comp: false,
                auto_refresh_storage: true,
                show_hidden_files: false,
                general_profile: 0,
                boot_target: 0,
                interface_density: 0,
                vm_safety_policy: 0,
                network_profile: 0,
                storage_policy: 0,
                package_policy: 0,
                developer_level: 0,
                security_policy: 0,
                ui_scaling: 1, // 100%
                terminal_font: 0,
                pg_scanlines: false,
                pg_dither: false,
                pg_glitch: false,
                pg_aberration: 0,
            },
            status_line: String::from("Ready"),
            command_history: Vec::new(),
            history_idx: None,
            notifications: Vec::new(),
            command_palette_active: false,
            command_palette_query: String::new(),
            command_palette_selected: 0,
            command_palette_scroll_offset: 0,
            glitch_y: 0,
            pci_devices: Vec::new(),
        }
    }

    pub fn add_vm(&mut self, vm: VmDisplayInfo) {
        self.vms.push(vm);
    }

    fn command_palette_items(&self) -> &'static [&'static str] {
        if self.command_palette_query.starts_with('$') {
            TERMINAL_PALETTE_COMMANDS
        } else {
            UI_PALETTE_COMMANDS
        }
    }

    fn command_palette_filter_text(&self) -> String {
        if self.command_palette_query.starts_with('$') {
            self.command_palette_query[1..].to_lowercase()
        } else {
            self.command_palette_query.to_lowercase()
        }
    }

    fn filtered_command_palette_items(&self) -> Vec<&'static str> {
        let query = self.command_palette_filter_text();
        self.command_palette_items()
            .iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .copied()
            .collect()
    }

    fn command_palette_filtered_count(&self) -> usize {
        let query = self.command_palette_filter_text();
        self.command_palette_items()
            .iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .count()
    }

    fn close_command_palette(&mut self) {
        self.command_palette_active = false;
        self.command_palette_query.clear();
        self.command_palette_selected = 0;
        self.command_palette_scroll_offset = 0;
    }

    fn command_core(command: &str) -> &str {
        if let Some(idx) = command.find(" - ") {
            &command[..idx]
        } else if let Some(idx) = command.find(':') {
            command[idx + 1..].trim()
        } else {
            command
        }
    }

    fn run_terminal_command_string(&mut self, command: &str) {
        let command_parts = command.split_whitespace().collect::<Vec<&str>>();
        if command_parts.is_empty() {
            return;
        }

        let parts = command_parts.clone();
        let body = command_parts.clone();
        terminal::cmd(command_parts, &parts, body, &mut self.package_manager);
        self.command_history.push(command.to_string());
        self.history_idx = None;
        self.term_buf.clear();
        self.notifications.push((alloc::format!("Terminal: {}", command), 120));
    }

    fn cycle_palette_setting(idx: &mut usize, max: usize) {
        *idx = (*idx + 1) % max;
    }

    pub fn set_resources(&mut self, resources: SystemResources) {
        let old_cpu_hist = self.resources.cpu_history.clone();
        let old_mem_hist = self.resources.mem_history.clone();
        let old_disk_read_hist = self.resources.disk_read_history.clone();
        let old_disk_write_hist = self.resources.disk_write_history.clone();
        let old_net_rx_hist = self.resources.net_rx_history.clone();
        let old_net_tx_hist = self.resources.net_tx_history.clone();
        let old_gpu_hist = self.resources.gpu_history.clone();
        let old_fps_hist = self.resources.fps_history.clone();
        let old_ft_hist = self.resources.ft_ms_history.clone();

        self.resources = resources;

        // Restore and update histories
        self.resources.cpu_history = old_cpu_hist;
        self.resources.mem_history = old_mem_hist;
        self.resources.disk_read_history = old_disk_read_hist;
        self.resources.disk_write_history = old_disk_write_hist;
        self.resources.net_rx_history = old_net_rx_hist;
        self.resources.net_tx_history = old_net_tx_hist;
        self.resources.gpu_history = old_gpu_hist;
        self.resources.fps_history = old_fps_hist;
        self.resources.ft_ms_history = old_ft_hist;


        fn push_limit<T>(vec: &mut Vec<T>, val: T, limit: usize) {
            if vec.len() >= limit {
                vec.remove(0);
            }
            vec.push(val);
        }

        push_limit(&mut self.resources.cpu_history, self.resources.cpu_usage, 100);
        let mem_percent = if self.resources.total_memory_mb > 0 {
            (self.resources.used_memory_mb * 100 / self.resources.total_memory_mb)
        } else { 0 };
        push_limit(&mut self.resources.mem_history, mem_percent, 100);
        push_limit(&mut self.resources.disk_read_history, self.resources.disk_read_kbps, 100);
        push_limit(&mut self.resources.disk_write_history, self.resources.disk_write_kbps, 100);
        push_limit(&mut self.resources.net_rx_history, self.resources.net_rx_kbps, 100);
        push_limit(&mut self.resources.net_tx_history, self.resources.net_tx_kbps, 100);
        push_limit(&mut self.resources.gpu_history, self.resources.gpu_usage, 100);
        push_limit(&mut self.resources.fps_history, self.resources.fps as u32, 100);
        push_limit(&mut self.resources.ft_ms_history, self.resources.frame_ms as u32, 100);
    }

    //noinspection GrazieInspectionRunner
    pub unsafe fn draw(&mut self) {
        if let Some(pg) = PixelGraphics::new() {
            self.iter += 1;
            let mut pg = pg.with_backbuffer();
            let (width, height) = pg.resolution();
            pg.fontid = self.settings.terminal_font as u8;
            
            // Draw background
            pg.clear(0x222222);

            // Draw header
            pg.fill_rect(0, 0, width, 48, 0x008080); // Cyan-ish
            pg.draw_text(width / 2 - 160, 16, "HPVMx - Hypervisor Management Console", 0xFFFFFF);

            // Draw clock in top right
            if let Ok(time) = runtime::get_time() {
                let time_str = alloc::format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second());
                pg.draw_text(width - 100, 16, &time_str, 0xFFFF00); // Yellow clock
            }

            pg.draw_text(width - 100, 2, if self.ctrl_mode {"ctrl"} else {""}, 0xFFFFFF);
            pg.draw_text((width - 100) + 33, 2, if self.alt_mode {"alt"} else {""}, 0xFFFFFF);
            pg.draw_text((width - 100) + 60, 2, if self.fn_mode {"fn"} else {""}, 0xFFFFFF);

            pg.draw_text(40, 1, "   __ _____ _   ____  ___", 0xFFFFFF);
            pg.draw_text(40, 11, "  / // / _ \\ | / /  |/  /_ __", 0xFFFFFF);
            pg.draw_text(40, 21, " / _  / ___/ |/ / /|_/ /\\ \\ /", 0xFFFFFF);
            pg.draw_text(40, 31, "/_//_/_/   |___/_/  /_//_\\_\\", 0xFFFFFF);

            // Draw navigation
            pg.fill_rect(0, 48, width, 32, 0x444444); // Dark Gray
            let nav_text = "O Overview | V VMs | R Resources | S Storage | N Network | D Devices | C Console | T Test | Z Settings | P Packages | A Apps";
            pg.draw_text(10, 56, nav_text, 0xFFFFFF);
            let page_y = 100;

            // Layout constants for consistent spacing across tabs
            let header_h = 48usize;
            let nav_h = 32usize;
            let content_top = header_h + nav_h; // 80px from top
            let margin = 16usize; // outer margin
            let gutter = 12usize; // space between widgets/rows
            let line_h = 15usize; // standard text line height

            // Content area based on selected tab
            match self.selected_tab {
                DashboardTab::Overview => {
                    pg.draw_text(20, 100, "System Overview", 0x00FF00);
                    
                    let mut y = 130;
                    pg.draw_text(20, y, "System Health: OK", 0x00FF00);
                    y += 30;
                    pg.draw_text(20, y, &alloc::format!("CPU:   {} Cores, {}% Usage", self.resources.cpu_count, self.resources.cpu_usage), 0xFFFFFF);
                    y += 20;
                    pg.draw_text(20, y, &alloc::format!("Memory: {} / {} MB", self.resources.used_memory_mb, self.resources.total_memory_mb), 0xFFFFFF);
                    y += 30;
                    
                    pg.draw_text(20, y, "I/O Performance:", 0xAAAAAA);
                    y += 20;
                    pg.draw_text(40, y, &alloc::format!("Disk:   Read {} KB/s, Write {} KB/s", self.resources.disk_read_kbps, self.resources.disk_write_kbps), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(40, y, &alloc::format!("Network: RX {} KB/s, TX {} KB/s", self.resources.net_rx_kbps, self.resources.net_tx_kbps), 0xCCCCCC);
                    y += 30;
                    
                    pg.draw_text(20, y, &alloc::format!("Virtualization: {} VMs Running", self.vms.iter().filter(|v| v.state.contains("Running")).count()), 0xFFFFFF);
                    y += 20;
                    pg.draw_text(20, y, &alloc::format!("Total VMs: {}", self.vms.len()), 0xCCCCCC);
                    y += 30;

                    pg.draw_text(20, y, "Hardware Categories:", 0xAAAAAA);
                    y += 20;
                    pg.draw_text(40, y, &alloc::format!("Storage: {} Files in current path", self.files.len()), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(40, y, &alloc::format!("Devices: {} Categories detected", self.categories.len()), 0xCCCCCC);
                    y += 60;
                    pg.draw_text_bg(40, y, "STATE BACKUP", 0xFF7700, 0x444444);
                    y += 20;
                    pg.fill_rect(40, y, 70, 30, 0x553333);
                    pg.draw_text(42, y+2, "SAVE [/]", 0xBBBBAA);


                    y = 100;
                    //pg.draw_rect_outline(420, y, 320, 420, 0xCCCCCC);
                    let time_data_0 = format!("{:?}", runtime::get_time_and_caps().unwrap().0);
                    let time_data_1 = format!("{:?}", runtime::get_time_and_caps().unwrap().1);
                    pg.draw_text(420, y, &*time_data_0, 0xFFFFFF);
                    y += 10;
                    pg.draw_text(420, y, &*time_data_1, 0xFFFFFF);



                    // let text_new_0 = format!("{:?}", uefi::runtime::get_variable(VariableKey::));
                }
                DashboardTab::Apps => {
                    pg.draw_text(margin, content_top + margin, "Application Registry", 0x00FF00);
                    pg.draw_text(margin, content_top + margin + 20, "Select an app to launch it in a stepped context", 0xAAAAAA);

                    let start_y = content_top + margin + 60;
                    let card_w = 100usize;
                    let card_h = 75usize;
                    let cols = (width - margin * 2) / (card_w + gutter);
                    let cols = if cols == 0 { 1 } else { cols };
                    
                    for (idx, (name, _, icon, version)) in crate::apps::APP_REGISTRY.iter().enumerate() {
                        let row = idx / cols;
                        let col = idx % cols;
                        let x = margin + col * (card_w + gutter);
                        let y = start_y + row * (card_h + gutter);

                        let is_selected = idx == self.selected_app_idx;
                        let border_color = if is_selected { 0x00FF00 } else { 0x666666 };
                        let bg_color = if is_selected { 0x334433 } else { 0x333333 };

                        pg.fill_rect(x, y, card_w, card_h, bg_color);
                        pg.draw_rect_outline(x, y, card_w, card_h, border_color);
                        
                        // Icon placeholder
                        pg.draw_icon(x + card_w/2 - 20, y + 20, 32, 32, icon);
                        pg.draw_text(x + 10, y + card_h - 20, name, 0xFFFFFF);
                        
                        // Grid position info
                        let pos_info = alloc::format!("v{}", version);
                        pg.draw_text(x + 10, y + 5, &pos_info, 0x888888);
                        
                        if is_selected {
                            pg.draw_text(x + card_w - 30, y + 5, "[*]", 0xFFFF00);
                        }
                    }
                    
                    pg.draw_text(margin, height - 40, "Use ARROWS to navigate | ENTER to Launch | ESC to close Apps", 0x888888);
                }
                DashboardTab::VirtualMachines => {
                    // Title
                    pg.draw_text(margin, content_top + margin + 4, "Virtual Machines", 0x00FF00);

                    // New VM Button
                    let create_btn_x = width - margin - 120;
                    let create_btn_y = content_top + margin;
                    pg.fill_rect(create_btn_x, create_btn_y, 120, 24, 0x008000);
                    pg.draw_text(create_btn_x + 10, create_btn_y + 4, "[+] Create VM", 0xFFFFFF);

                    // Table frame
                    let table_x = margin;
                    let table_y = content_top + margin + 32;
                    let table_w = core::cmp::min(width - margin * 2, 760);
                    let table_h = core::cmp::min(height - table_y - 120, 260);
                    pg.draw_rect_outline(table_x, table_y, table_w, table_h, 0x888888);

                    // Header background
                    pg.fill_rect(table_x + 1, table_y + 1, table_w - 2, line_h, 0x333333);
                    pg.draw_text(table_x + 8, table_y + 4, "ID  NAME             STATE       CPU  MEM    UPTIME", 0xCCCCCC);

                    // Rows
                    let mut y = table_y + line_h + gutter;
                    for (idx, vm) in self.vms.iter().enumerate() {
                        if y + line_h > table_y + table_h - 2 { break; }
                        let is_selected = idx == self.selected_vm_idx;
                        let text_color = if is_selected { 0xFFFF00 } else { 0xFFFFFF };
                        if is_selected {
                            pg.fill_rect(table_x + 2, y - 2, table_w - 4, line_h, 0x444400);
                        }
                        let uptime = if vm.uptime_seconds < 60 {
                            alloc::format!("{}s", vm.uptime_seconds)
                        } else if vm.uptime_seconds < 3600 {
                            alloc::format!("{}m {}s", vm.uptime_seconds / 60, vm.uptime_seconds % 60)
                        } else {
                            alloc::format!("{}h {}m", vm.uptime_seconds / 3600, (vm.uptime_seconds % 3600) / 60)
                        };
                        let info = alloc::format!("{:<3} {:<16} {:<11} {:>3}% {:>5}MB  {:>10}",
                            vm.id, vm.name, vm.state, vm.cpu_usage, vm.memory_usage_mb, uptime);
                        pg.draw_text(table_x + 8, y, &info, text_color);
                        y += line_h;
                    }

                    // VM Details / Properties Panel
                    let props_x = table_x + table_w + gutter;
                    let props_w = width.saturating_sub(props_x + margin);
                    if props_w > 150 {
                        let props_h = table_h;
                        pg.draw_rect_outline(props_x, table_y, props_w, props_h, 0x888888);
                        pg.draw_text_bg(props_x + 10, table_y - 4, "VM Properties", 0x00FF00, 0x222222);
                        
                        if let Some(vm) = self.vms.get(self.selected_vm_idx) {
                            let mut py = table_y + 10;
                            pg.draw_text(props_x + 10, py, &alloc::format!("Name: {}", vm.name), 0xFFFFFF);
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("ID:   {}", vm.id), 0xCCCCCC);
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("State: {}", vm.state), if vm.state.contains("Running") { 0x00FF00 } else { 0xFFFFFF });
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("vCPUs: {}", vm.cpu_usage), 0xCCCCCC); // Actually usage, but good to show
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("RAM:   {} MB", vm.memory_usage_mb), 0xCCCCCC);
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("Disk:  {} MB", vm.disk_usage_mb), 0xCCCCCC);
                            py += 20;
                            pg.draw_text(props_x + 10, py, &alloc::format!("Uptime: {}s", vm.uptime_seconds), 0x888888);
                        } else {
                            pg.draw_text(props_x + 10, table_y + 10, "No VM selected", 0x888888);
                        }
                    }

                    // VM Actions Bar
                    if !self.vms.is_empty() {
                        let actions_y = table_y + table_h + gutter;
                        pg.draw_text(margin, actions_y, "Actions for Selected VM:", 0xCCCCCC);
                        let actions = ["Start", "Stop", "Reset", "Zero", "Delete", "Save", "Restore"];
                        let mut action_x = margin;
                        let action_y = actions_y + 20;
                        for (idx, action) in actions.iter().enumerate() {
                            let is_focused = idx == self.vm_action_idx;
                            let color = if is_focused { 0x00AA00 } else { 0x444444 };
                            pg.fill_rect(action_x, action_y, 78, 24, color);
                            pg.draw_text(action_x + 8, action_y + 4, action, 0xFFFFFF);
                            action_x += 88;
                        }
                        pg.draw_text(margin, action_y + 32, "Press ENTER to execute action | SPACE to Create VM", 0x888888);
                    } else {
                        pg.draw_text(margin, table_y + table_h + gutter, "No VMs. Press SPACE to Create VM", 0x888888);
                    }
                }
                DashboardTab::CreateVM => {
                    pg.draw_text(margin, content_top + margin, "Create New Virtual Machine", 0x00FF00);
                    
                    let form_x = margin + 20;
                    let mut form_y = content_top + margin + 40;
                    
                    // Name Field
                    pg.draw_text(form_x, form_y, "Name:", 0xFFFFFF);
                    let name_focus = self.create_vm_focus_idx == 0;
                    pg.draw_rect_outline(form_x + 100, form_y - 4, 200, 24, if name_focus { 0xFFFF00 } else { 0x888888 });
                    pg.draw_text(form_x + 105, form_y, &self.new_vm_name, 0xFFFFFF);
                    
                    form_y += 40;
                    // CPU Cores Field
                    pg.draw_text(form_x, form_y, "vCPUs:", 0xFFFFFF);
                    let cpu_focus = self.create_vm_focus_idx == 1;
                    pg.draw_rect_outline(form_x + 100, form_y - 4, 100, 24, if cpu_focus { 0xFFFF00 } else { 0x888888 });
                    pg.draw_text(form_x + 105, form_y, &alloc::format!("{}", self.new_vm_vcpus), 0xFFFFFF);
                    pg.draw_text(form_x + 210, form_y, "(Use + / - to change)", 0x888888);
                    
                    form_y += 40;
                    // Memory Field
                    pg.draw_text(form_x, form_y, "Memory (MB):", 0xFFFFFF);
                    let mem_focus = self.create_vm_focus_idx == 2;
                    pg.draw_rect_outline(form_x + 100, form_y - 4, 100, 24, if mem_focus { 0xFFFF00 } else { 0x888888 });
                    pg.draw_text(form_x + 105, form_y, &alloc::format!("{}", self.new_vm_memory_mb), 0xFFFFFF);
                    pg.draw_text(form_x + 210, form_y, "(Use + / - to change)", 0x888888);
                    
                    form_y += 60;
                    // Buttons
                    let create_focused = self.create_vm_focus_idx == 3;
                    pg.fill_rect(form_x, form_y, 120, 32, if create_focused { 0x00AA00 } else { 0x006600 });
                    pg.draw_text(form_x + 20, form_y + 8, "CREATE", 0xFFFFFF);
                    
                    let cancel_focused = self.create_vm_focus_idx == 4;
                    pg.fill_rect(form_x + 140, form_y, 120, 32, if cancel_focused { 0xAA0000 } else { 0x660000 });
                    pg.draw_text(form_x + 20 + 140, form_y + 8, "CANCEL", 0xFFFFFF);
                    
                    pg.draw_text(margin, form_y + 50, "TAB to switch fields | ENTER to confirm | ESC to cancel", 0x888888);
                }
                DashboardTab::Resources => {
                    // Left info panel
                    let panel_x = margin;
                    let panel_y = content_top + margin;
                    let panel_w = 360usize;
                    let panel_h = 480usize;
                    pg.draw_rect_outline(panel_x, panel_y, panel_w, panel_h, 0x888888);
                    pg.draw_text_bg(panel_x, panel_y - 4, "Resource Monitor", 0x20FF20, 0x222222);

                    pg.draw_text(panel_x + 10, panel_y + 16, &alloc::format!("CPU Cores: {}", self.resources.cpu_count), 0xFFFFFF);
                    pg.draw_text(panel_x + 10, panel_y + 16 + line_h, &alloc::format!("Total Memory: {} MB", self.resources.total_memory_mb), 0xFFFFFF);
                    pg.draw_text(panel_x + 10, panel_y + 16 + line_h * 2, &alloc::format!("Used Memory: {} MB", self.resources.used_memory_mb), 0xFFFFFF);

                    // Memory usage bar and graph
                    let bar_y = panel_y + 16 + line_h * 3 + gutter;
                    pg.draw_text(panel_x + 10, bar_y, "Memory History (10s):", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &self.resources.mem_history, 100, 0x00FF00, 60);

                    // I/O Stats and Graphs
                    let io_y = bar_y + 80 + gutter * 2;
                    pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, io_y + 20, 165, 50, &self.resources.net_rx_history, 1024, 0x00FFFF, 60);
                    pg.draw_line_graph(panel_x + 185, io_y + 20, 165, 50, &self.resources.net_tx_history, 1024, 0xFFFF00, 60);
                    
                    let disk_y = io_y + 80;
                    pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Red)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, disk_y + 20, 165, 50, &self.resources.disk_read_history, 1024, 0xFFFFFF, 60);
                    pg.draw_line_graph(panel_x + 185, disk_y + 20, 165, 50, &self.resources.disk_write_history, 1024, 0xFF0000, 60);

                    let gpu_y = disk_y + 80;
                    pg.draw_text(panel_x+ 10, gpu_y, "GPU Usage:", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, gpu_y+20, 165, 50, &self.resources.gpu_history, 100, 0xFF7700, 60);




                    // Right CPU core list panel or Total CPU Graph
                    let right_x = panel_x + panel_w + gutter * 2;
                    let right_y = panel_y;
                    let right_w = core::cmp::min(width - right_x - margin, 360);
                    let right_h = core::cmp::min(height - right_y - 100, 260);
                    pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x888888);
                    pg.draw_text_bg(right_x + 10, right_y - 4, "Total CPU Usage History:", 0xFFFFFF, 0x222222);
                    pg.draw_line_graph(right_x + 10, right_y + 10, right_w - 20, 80, &self.resources.cpu_history, 100, 0x00FF00, 60);
                    
                    pg.draw_text(right_x + 10, right_y + 100, "CPU Usage per Core:", 0xFFFFFF);
                    for i in 0..self.resources.cpu_count {
                        let row_y = right_y + 120 + (i as usize * (line_h + 4));
                        if row_y + line_h > right_y + right_h - 8 { break; }
                        let usage = if i < self.resources.cpu_core_usage.len() as u32 { self.resources.cpu_core_usage[i as usize] } else { 0 };
                        pg.draw_text(right_x + 10, row_y, &alloc::format!("C{}:{:>2}%", i, usage), 0xCCCCCC);
                        pg.draw_progress_bar(right_x + 70, row_y, right_w - 80, 12, usage as usize, 100, 0x00FF00);
                    }

                    pg.draw_text_bg(right_x + 10, right_y + 300, "FPS History:", 0xFFFFFF, 0x222222);
                    pg.draw_line_graph(right_x + 10, right_y + 300, right_w - 20, 80, &self.resources.fps_history, 75, 0xFF44FF, 60);
                    pg.draw_text_bg(right_x + 10, right_y + 400, "Frame MS History:", 0xFFFFFF, 0x222222);
                    pg.draw_line_graph(right_x + 10, right_y + 400, right_w - 20, 80, &self.resources.ft_ms_history, 750, 0xFFAAFF, 60);

                    // Heatmap for CPU Core usage
                    let hm_y = right_y + 500;
                    pg.draw_text(right_x + 10, hm_y, "CPU Heatmap (Real-time Core Stress):", 0xFFFFFF);
                    let mut hm_data = [0.0f32; 16];
                    for i in 0..core::cmp::min(self.resources.cpu_core_usage.len(), 16) {
                        hm_data[i] = self.resources.cpu_core_usage[i] as f32 / 100.0;
                    }
                    pg.draw_heatmap(right_x + 10, hm_y + 20, right_w - 20, 80, 4, 4, &hm_data);

                    // draw u64 le text for all stats

                }
                DashboardTab::Network => {

                    let x = 20;
                    let mut y = 100;
                    pg.draw_text(x, y, "Network Status", 0x00FF00);
                    let net_stats = crate::devices::net_stack::stats();
                    y += 30;
                    pg.draw_text(x, y, &alloc::format!("Backend: {}", crate::devices::net_stack::backend_name()), 0xFFFFFF);
                    y += 30;
                    pg.draw_text(x, y, "Statistics:", 0xAAAAAA);

                    let x = 40;
                    let mut y = 180;
                    pg.draw_text(x, y, &alloc::format!("RX Packets: {}", net_stats.rx_pkts), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("TX Packets: {}", net_stats.tx_pkts), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("RX Bytes:   {}", net_stats.rx_bytes), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("TX Bytes:   {}", net_stats.tx_bytes), 0xCCCCCC);

                    y += 100;
                    let state = crate::devices::net_stack::get_state();
                    pg.draw_text(x, y, &alloc::format!("IP: {}.{}.{}.{}", state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("GW: {}.{}.{}.{}", state.gateway[0], state.gateway[1], state.gateway[2], state.gateway[3]), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("MASK: {}.{}.{}.{}", state.subnet_mask[0], state.subnet_mask[1], state.subnet_mask[2], state.subnet_mask[3]), 0xCCCCCC);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", state.mac_addr[0], state.mac_addr[1], state.mac_addr[2], state.mac_addr[3], state.mac_addr[4], state.mac_addr[5]), 0xCCCCCC);
                    y += 40;
                    let is_init = crate::devices::net_stack::is_initialized();
                    pg.draw_text(x, y, &alloc::format!("Initialized: {is_init}", ), 0xFFFFFF);
                    y += 35;

                    pg.draw_text(x, y, &alloc::format!("Target: {}", self.network_target), 0xCCCCCC);
                    y += 28;

                    pg.draw_text(x, y, "Traffic Monitor (10s):", 0x00FFFF);
                    y += 20;
                    pg.draw_text(x, y, &alloc::format!("RX: {} KB/s", self.resources.net_rx_kbps), 0x00FF00);
                    pg.draw_line_graph(x + 120, y - 10, 200, 40, &self.resources.net_rx_history, 1024, 0x00FF00, 60);
                    y += 50;
                    pg.draw_text(x, y, &alloc::format!("TX: {} KB/s", self.resources.net_tx_kbps), 0xFF0000);
                    pg.draw_line_graph(x + 120, y - 10, 200, 40, &self.resources.net_tx_history, 1024, 0xFF0000, 60);
                    y += 60;

                    let actions = ["Net Up", "Status", "Ping", "LAN Scan", "HTTP On", "HTTP Off"];
                    let mut action_x = x;
                    for (idx, action) in actions.iter().enumerate() {
                        let is_focused = idx == self.selected_network_action_idx;
                        pg.fill_rect(action_x, y, 88, 24, if is_focused { 0x00AA00 } else { 0x444444 });
                        pg.draw_text(action_x + 8, y + 4, action, 0xFFFFFF);
                        action_x += 96;
                    }
                    y += 36;
                    pg.draw_text(x, y, "LEFT/RIGHT chooses action, ENTER runs it, +/- cycles ping target", 0x888888);
                    y += 20;
                    pg.draw_text(x, y, &self.status_line, 0xFFFF00);




                }
                DashboardTab::Console => {
                    pg.draw_text(20, 100, "Hypervisor Real-time Log", 0x00FF00);
                    let logs = crate::hpvmlog::get_logs();
                    pg.draw_log_viewer(margin, 130, width - margin * 2, height - 135 - margin * 8, &logs);
                    
                    let y_msg = height - margin * 6;
                    pg.draw_text(margin, y_msg, "Use PgUp/PgDn to scroll, C to clear", 0x888888);

                    pg.draw_rect_outline(margin, height-95, width - margin * 8, 35, 0x999999);
                    if self.term_selected {
                        pg.draw_rect_outline_adv(margin - 1, height-96, (width - margin * 8)+2, 37, 0x888844, 3, 0x0F0F0F0F);
                    }
                    pg.draw_text(margin + 5, height - 60, "press enter to send, end to enter type mode, and esc to exit", 0x888888);
                    pg.draw_text(margin + 5, height - 85, alloc::format!("HPVMx> {}", self.term_buf).as_str(), 0xDDDDDD);
                }
                DashboardTab::Devices => {
                    pg.draw_text(20, 100, "Device Manager", 0x00FF00);
                    
                    let mut y = 130;
                    let mut current_idx = 0;
                    
                    for cat in &self.categories {
                        let expanded_icon = if cat.expanded { "[-] " } else { "[+] " };
                        let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xAAAAAA };
                        pg.draw_text(20, y, &alloc::format!("{}{}{} ({})", expanded_icon, cat.icon, cat.name, cat.devices.len()), color);
                        y += 20;
                        current_idx += 1;
                        
                        if cat.expanded {
                            for dev in &cat.devices {
                                let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xFFFFFF };
                                pg.draw_icon(35, y - 2, 16, 16, if cat.name == "Network Adapters" {&pixel_graphics::icons::PCI_GREEN_ICON_DATA} else { &pixel_graphics::icons::PCI_BLUE_ICON_DATA });
                                
                                // Split path after third '/' for the list view as well
                                let path = &dev.path;
                                let mut slash_count = 0;
                                let mut split_idx = None;
                                for (i, c) in path.char_indices() {
                                    if c == '/' {
                                        slash_count += 1;
                                        if slash_count == 3 {
                                            split_idx = Some(i + 1);
                                            break;
                                        }
                                    }
                                }

                                if let Some(idx) = split_idx {
                                    pg.draw_text(45, y, &alloc::format!(" {}: {}", dev.name, &path[..idx]), color);
                                    y += 18;
                                    pg.draw_text(65, y, &path[idx..], 0x888888);
                                    y += 22;
                                } else {
                                    pg.draw_text(45, y, &alloc::format!(" {}: {}", dev.name, dev.path), color);
                                    y += 20;
                                }

                                current_idx += 1;
                                if y > height - 60 { break; }
                            }
                        }
                        if y > height - 60 { break; }
                    }

                    // Device Details Panel
                    let detail_x = width / 2;
                    let detail_y = 130;
                    let detail_w = (width / 2) - 20;
                    let detail_h = height - 200;
                    pg.draw_rect_outline(detail_x, detail_y, detail_w, detail_h, 0x888888);
                    pg.draw_text_bg(detail_x + 10, detail_y - 4, "Device Properties", 0x00FF00, 0x222222);

                    let mut current_search_idx = 0;
                    let mut selected_device = None;
                    for cat in &self.categories {
                        if current_search_idx == self.selected_device_idx {
                            // Category selected, not a device
                            break;
                        }
                        current_search_idx += 1;
                        if cat.expanded {
                            for dev in &cat.devices {
                                if current_search_idx == self.selected_device_idx {
                                    selected_device = Some((dev, &cat.name));
                                    break;
                                }
                                current_search_idx += 1;
                            }
                        }
                        if selected_device.is_some() { break; }
                    }

                    if let Some((dev, cat_name)) = selected_device {
                        let mut dy = detail_y + 10;
                        pg.draw_text(detail_x + 10, dy, &alloc::format!("Name: {}", dev.name), 0xFFFFFF);
                        dy += 20;
                        pg.draw_text(detail_x + 10, dy, &alloc::format!("Category: {}", cat_name), 0xCCCCCC);
                        dy += 20;
                        pg.draw_text(detail_x + 10, dy, "UEFI Path:", 0xAAAAAA);
                        dy += 15;
                        
                        // Shorten, wrap and indent path after the third '/'
                        let path = &dev.path;
                        let mut parts = alloc::vec::Vec::new();
                        
                        let mut slash_count = 0;
                        let mut last_split = 0;
                        for (i, c) in path.char_indices() {
                            if c == '/' {
                                slash_count += 1;
                                if slash_count == 3 {
                                    parts.push(&path[..i+1]);
                                    last_split = i + 1;
                                    break;
                                }
                            }
                        }
                        
                        if last_split > 0 {
                            parts.push(&path[last_split..]);
                        } else {
                            parts.push(path);
                        }

                        let chunk_size = (detail_w - 30) / 8; // slightly smaller to account for indentation
                        if chunk_size > 0 {
                            for (i, part) in parts.iter().enumerate() {
                                let indent = if i > 0 { 20 } else { 0 };
                                let current_chunk_size = if i > 0 { chunk_size.saturating_sub(3) } else { chunk_size };
                                
                                if part.len() <= current_chunk_size {
                                    pg.draw_text(detail_x + 10 + indent, dy, part, 0x888888);
                                    dy += 15;
                                } else {
                                    for chunk in part.as_bytes().chunks(current_chunk_size) {
                                        if let Ok(s) = core::str::from_utf8(chunk) {
                                            pg.draw_text(detail_x + 10 + indent, dy, s, 0x888888);
                                            dy += 15;
                                        }
                                        if dy > detail_y + detail_h - 20 { break; }
                                    }
                                }
                                if dy > detail_y + detail_h - 20 { break; }
                            }
                        }

                        dy += 20;
                        if cat_name.contains("PCI") {
                            pg.draw_text(detail_x + 10, dy, "PCI Information:", 0x00FFFF);
                            dy += 20;
                            // Search for more detailed info in self.pci_devices
                            if let Some(pci) = self.pci_devices.iter().find(|p| {
                                format!("{:02X}:{:02X}.{}", p.bus, p.device, p.function) == dev.name
                            }) {
                                pg.draw_text(detail_x + 10, dy, &format!("Vendor:   {}", pci.vendor_name()), 0xFFFFFF);
                                dy += 16;
                                pg.draw_text(detail_x + 10, dy, &format!("Device:   0x{:04X}", pci.device_id), 0xFFFFFF);
                                dy += 16;
                                pg.draw_text(detail_x + 10, dy, &format!("Class:    {}", pci.class_name()), 0xFFFFFF);
                                dy += 16;
                                pg.draw_text(detail_x + 10, dy, &format!("Revision: 0x{:02X}", pci.revision_id), 0xCCCCCC);
                                dy += 16;
                                pg.draw_text(detail_x + 10, dy, &format!("Interface: 0x{:02X}", pci.interface_id), 0xCCCCCC);
                                dy += 20;
                                pg.draw_text(detail_x + 10, dy, "Hardware Status: Online", 0x55FF55);
                            } else {
                                pg.draw_text(detail_x + 10, dy, "Scanning for PCI Vendor/Device IDs...", 0x666666);
                            }
                        }
                    } else {
                        pg.draw_text(detail_x + 10, detail_y + 10, "Select a device to view properties", 0x888888);
                    }

                    // Device Actions
                    let action_y = detail_y + detail_h + 20;
                    let actions = ["Refresh List", "Scan PCI Bus", "Diagnostics", "Toggle Expanded"];
                    let mut ax = 20;
                    for (idx, action) in actions.iter().enumerate() {
                        let is_focused = idx == self.device_action_idx && !self.term_selected;
                        pg.fill_rect(ax, action_y, 140, 26, if is_focused { 0x00AA00 } else { 0x444444 });
                        pg.draw_text(ax + 8, action_y + 5, action, 0xFFFFFF);
                        ax += 150;
                    }
                    if self.device_action_idx == 1 {
                        pg.draw_text(20, action_y + 35, "Scans the PCI bus using Port IO (0xCF8/0xCFC) to detect hardware", 0x00AAAA);
                    }
                }
                DashboardTab::Storage => {
                    // Title and path
                    let base_y = content_top + margin;
                    pg.draw_text(margin, base_y - 4, "File Explorer", 0x00FF00);
                    pg.draw_text(margin, base_y + 8, &alloc::format!("Path: {}", self.current_path), 0xAAAAAA);

                    // Table area
                    let list_x = margin;
                    let list_y = base_y + 28;
                    let list_w = core::cmp::min(width - margin * 2, 720);
                    let list_h = core::cmp::min(height - list_y - 90, 460);
                    pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);

                    // Header row with better spacing and column guides
                    pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, line_h, 0x333333);
                    pg.draw_text(list_x + 8, list_y + 4, "TYPE  NAME                                 SIZE (BYTES)  ATTR", 0xCCCCCC);
                    // Optional column separators
                    pg.draw_line(list_x + 48, list_y + 1, list_x + 48, list_y + list_h - 1, 0x444444);
                    pg.draw_line(list_x + 340, list_y + 1, list_x + 340, list_y + list_h - 1, 0x444444);

                    // Rows
                    let mut y = list_y + line_h + gutter;
                    for (i, entry) in self.files.iter().enumerate() {
                        if y + line_h > list_y + list_h - 2 { break; }
                        let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
                        let icon = if entry.is_dir { pixel_graphics::icons::FOLDER_ICON_DATA } else {
                            let dec_syn = ["json", "xml", "toml", "yaml", "yml"];
                            let sys_syn = ["sys", "efi", "asm"];
                            let prog_syn = ["micro", "ufe", "dmx", "bin", "rs"];


                            let ext = entry.name.split(".").last().unwrap();
                            if dec_syn.contains(&ext) {
                                pixel_graphics::icons::JSON_ICON_DATA
                            } else if sys_syn.contains(&ext) {
                                pixel_graphics::icons::EXECUTABLE_ICON_DATA
                            } else if prog_syn.contains(&ext) {
                                pixel_graphics::icons::CODE_ICON_DATA
                            } else {
                                pixel_graphics::icons::FILE_ICON_DATA
                            }
                        };

                        let size: String = if entry.size < 10000 {
                            format!("{}", entry.size)
                        } else if entry.size/1024 < 10000 {
                            format!("{}K", (entry.size/1024))
                        } else {
                            format!("{}M", (entry.size/1024)/1024)
                        };


                        let background = if i == self.selected_file_idx { 0x333333 } else { 0x222222 };
                        pg.draw_icon(list_x + 16, y, 16, 16, &icon);
                        pg.draw_text_bg(list_x + 56, y, &alloc::format!("{:<32}", entry.name), color, background);
                        pg.draw_text_bg(list_x + 348, y, &alloc::format!("{:>12}", size), 0xCCCCCC, background);
                        pg.draw_text_bg(list_x + 470, y, if entry.is_dir { "DIR" } else { "FILE" }, 0x6666FF, background);
                        y += line_h;
                    }

                    let props_x = list_x + list_w + gutter;
                    let props_w = core::cmp::min(width.saturating_sub(props_x + margin), 360);
                    if props_w > 120 {
                        pg.draw_rect_outline(props_x, list_y, props_w, list_h, 0x777777);
                        pg.draw_text(props_x + 10, list_y + 10, "Properties", 0x00FF00);
                        if let Some(entry) = self.files.get(self.selected_file_idx) {
                            let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                            let full_path = format!("{}{}{}", self.current_path, sep, entry.name);
                            pg.draw_text(props_x + 10, list_y + 40, &format!("Name: {}", entry.name), 0xFFFFFF);
                            pg.draw_text(props_x + 10, list_y + 60, &format!("Type: {}", if entry.is_dir { "Directory" } else { "File" }), 0xCCCCCC);
                            pg.draw_text(props_x + 10, list_y + 80, &format!("Size: {} bytes", entry.size), 0xCCCCCC);
                            pg.draw_text(props_x + 10, list_y + 100, &format!("Path: {}", full_path), 0x888888);
                            pg.draw_text(props_x + 10, list_y + 130, &format!("Index: {} / {}", self.selected_file_idx + 1, self.files.len()), 0x888888);
                        } else {
                            pg.draw_text(props_x + 10, list_y + 40, "No item selected", 0x888888);
                        }

                        if let Some(action) = self.filesys_pending_action {
                            let confirm_y = list_y + list_h - 90;
                            pg.fill_rect(props_x + 8, confirm_y, props_w - 16, 72, 0x332222);
                            pg.draw_rect_outline(props_x + 8, confirm_y, props_w - 16, 72, 0xFFAA00);
                            pg.draw_text(props_x + 16, confirm_y + 10, "Confirm Operation", 0xFFAA00);
                            pg.draw_text(props_x + 16, confirm_y + 30, &format!("{:?}", action), 0xFFFFFF);
                            pg.draw_text(props_x + 16, confirm_y + 50, "END confirms, ESC cancels", 0xCCCCCC);
                        }
                    }

                    let actions_y = list_h + margin*8;
                    pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
                    let actions = ["Open", "Props", "New File", "New Dir", "Rename", "Copy", "Move", "Delete"];
                    let mut action_x = margin;
                    let action_y = actions_y + 20;
                    for (idx, action) in actions.iter().enumerate() {
                        let is_focused = idx == self.filesys_action_idx;
                        let color = if is_focused { 0x00AA00 } else { 0x444444 };
                        pg.fill_rect(action_x, action_y, 92, 24, color);
                        pg.draw_text(action_x + 6, action_y + 4, action, 0xFFFFFF);
                        action_x += 100;
                    }
                    pg.draw_text(margin, action_y + 34, "LEFT/RIGHT chooses action, END runs it; rename/copy/move/delete ask for confirmation", 0x888888);
                    pg.draw_text(margin, action_y + 52, &self.status_line, 0xFFFF00);
                }
                DashboardTab::Test => {
                    pg.draw_text(20, 100, &alloc::format!("UI Components Test Bed (Qt6 Style)  res: {}x{}", width, height), 0x00FF00);
                    
                    // Column 1
                    let mut y = 130;
                    pg.draw_text(20, y, "Buttons & Inputs:", 0xAAAAAA); y += 25;
                    pg.fill_rect(20, y, 100, 25, 0x444444); pg.draw_text(25, y+5, "Push Button", 0xFFFFFF);
                    pg.fill_rect(130, y, 30, 25, 0x444444); pg.draw_text(138, y+5, "?", 0xFFFFFF); // ToolButton
                    y += 35;
                    
                    pg.draw_checkbox(20, y, true, false, false, "CheckBox (Checked)"); y += 25;
                    pg.draw_checkbox(20, y, false, false, false, "CheckBox (Unchecked)");  y += 25;
                    pg.draw_checkbox(20, y, false, true, false,"CheckBox (Blocked/Denied)");  y += 25;
                    pg.draw_checkbox(20, y, true, false, true, "CheckBox (Disabled)"); y += 25;
                    
                    pg.draw_radio_button(20, y, true); pg.draw_text(40, y, "RadioButton 1", 0xFFFFFF); y += 25;
                    pg.draw_radio_button(20, y, false); pg.draw_text(40, y, "RadioButton 2", 0xFFFFFF); y += 35;
                    
                    pg.draw_text(20, y, "LineEdit:", 0xAAAAAA); y += 20;
                    pg.draw_rect_outline(20, y, 150, 20, 0x888888); pg.fill_rect(21, y+1, 148, 18, 0xFFFFFF);
                    pg.draw_text(25, y+2, "Editable text..ſ", 0x000000); y += 30;
                    
                    pg.draw_text(20, y, "SpinBox / DoubleSpinBox:", 0xAAAAAA); y += 15;
                    pg.draw_spinbox(20, y, 60, 42, "int");
                    pg.draw_double_spinbox(120, y, 60, 3.14, 2);

                    y+= 30;

                    // Column 2
                    let mut y = 130;
                    let x2 = 250;
                    pg.draw_text(x2, y, "Sliders & Progress:", 0xAAAAAA); y += 25;
                    pg.draw_slider(x2, y, 150, 40, 100, false); y += 25; // Horizontal Slider
                    pg.draw_slider(x2 + 160, 130, 100, 30, 100, true); // Vertical Slider
                    
                    pg.draw_text(x2, y, "Progress Bar:", 0xAAAAAA); y += 20;
                    pg.draw_progress_bar(x2, y, 150, 20, 65, 100, 0x00FF00); y += 35;
                    
                    pg.draw_text(x2, y, "LCD Number:", 0xAAAAAA); y += 20;
                    pg.draw_lcd_number(x2, y, "123.45"); y += 40;
                    
                    pg.draw_text(x2, y, "ScrollBars:", 0xAAAAAA); y += 20;
                    pg.draw_rect_outline(x2, y, 150, 15, 0x444444); pg.fill_rect(x2 + 40, y + 1, 30, 13, 0x888888); // H Scroll
                    y += 25;
                    
                    pg.draw_text(x2, y, "Date/Time Edits:", 0xAAAAAA); y += 20;
                    pg.draw_text(x2, y, "2026-02-23 10:25", 0x00FFFF); //y += 30;

                    // Column 3
                    let mut y = 130;
                    let x3 = 500;
                    pg.draw_text(x3, y, "Complex Views (Mock):", 0xAAAAAA); y += 25;
                    pg.draw_rect_outline(x3, y, 200, 60, 0x888888); // ListView
                    pg.draw_text(x3 + 5, y + 5, "ListView Item A", 0xFFFFFF);
                    pg.draw_text(x3 + 5, y + 25, "ListView Item B", 0xFFFF00);
                    pg.draw_text(x3 + 5, y + 45, "ListView Item C", 0xFFFFFF);
                    y += 70;
                    
                    pg.draw_rect_outline(x3, y, 200, 60, 0x888888); // TreeView
                    pg.draw_text(x3 + 5, y + 5, "[-] Root", 0xFFFFFF);
                    pg.draw_text(x3 + 20, y + 25, " └─ Child 1", 0xAAAAAA);
                    pg.draw_text(x3 + 20, y + 45, " └─ Child 2", 0xAAAAAA);
                    y += 70;
                    
                    pg.draw_rect_outline(x3, y, 200, 60, 0x888888); // TableView
                    pg.draw_line(x3, y + 20, x3 + 200, y + 20, 0x888888);
                    pg.draw_line(x3 + 60, y, x3 + 60, y + 60, 0x888888);
                    pg.draw_text(x3 + 5, y + 2, "H1", 0xAAAAAA); pg.draw_text(x3 + 65, y + 2, "Header 2", 0xAAAAAA);
                    pg.draw_text(x3 + 5, y + 25, "Val 1", 0xFFFFFF); pg.draw_text(x3 + 65, y + 25, "Data 2", 0xFFFFFF);
                    //y += 70;


                    // Group Box & ToolBox
                    let y = 450;
                    pg.draw_rect_outline(20, y, 200, 100, 0x888888);
                    pg.fill_rect(30, y - 8, 80, 16, 0x222222);
                    pg.draw_text(35, y - 8, "GroupBox", 0xAAAAAA);
                    pg.draw_text(40, y + 20, "Internal content", 0x888888);
                    
                    pg.draw_rect_outline(240, y, 200, 100, 0x888888);
                    pg.fill_rect(240, y, 200, 20, 0x444444); pg.draw_text(245, y + 2, "ToolBox Tab 1 [v]", 0xFFFFFF);
                    pg.fill_rect(240, y + 80, 200, 20, 0x444444); pg.draw_text(245, y + 82, "ToolBox Tab 2 [>]", 0xFFFFFF);
                    
                    pg.draw_rect_outline(460, y, 200, 100, 0x888888); // ScrollArea
                    pg.draw_rect_outline(645, y, 15, 100, 0x444444); pg.fill_rect(646, y + 10, 13, 30, 0x888888); // V Scroll
                    pg.draw_text(470, y + 10, "Scroll Area Content...", 0xFFFFFF);
                    pg.draw_text(470, y + 30, "That is clipped", 0xFFFFFF);
                    
                    // Lines

                    let y = 600;
                    pg.draw_line(20, y, 780, y, 0x555555); // Horizontal Line
                    pg.draw_line(400, y + 5, 400, y + 50, 0x555555); // Vertical Line
                    
                    pg.draw_text(20, y + 10, "Labels & Browser:", 0xAAAAAA);
                    pg.draw_text(20, y + 30, "Standard Label", 0xFFFFFF);
                    pg.draw_rect_outline(150, y + 10, 230, 40, 0x444444);
                    pg.draw_text(155, y + 15, "Text Browser with <b>rich</b> content", 0xAAAAAA);
                    
                    pg.draw_text(420, y + 10, "Dial & Key Sequence:", 0xAAAAAA);
                    // Mock Dial
                    pg.draw_dial(420, y + 30, 12, 25, 100);
                    pg.draw_rect_outline(550, y + 30, 100, 20, 0x888888); pg.draw_text(555, y + 32, "Ctrl+Alt+Del", 0xFFFF00);


                    let y = 130;
                    let x = 750;

                    // Table Data (3D setup)
                    let headers = ["ID", "Name", "Status"];
                    let row1 = ["01", "Kernel", "Running"];
                    let row2 = ["02", "GOP", "Active"];
                    let rows = [&row1[..], &row2[..]];
                    pg.draw_table_view(x, y, 250, 100, &headers, &rows);

                    // Tree Data (Nested JSON-style)
                    let children = [
                        TreeViewNode { label: "bin", children: &[], expanded: false },
                        TreeViewNode { label: "boot", children: &[], expanded: false },
                    ];
                    let root = TreeViewNode {
                        label: "Root (/) ",
                        children: &children,
                        expanded: true,
                    };
                    pg.draw_tree_view(x, y + 200, 200, 150, &root);

                    pg.draw_icon(x, y + 400, 16, 16, &pixel_graphics::icons::RAM_ICON_DATA);
                    pg.draw_icon(x + 24, y + 400, 16, 16, &pixel_graphics::icons::PCI_GREEN_ICON_DATA);
                    pg.draw_icon(x + 48, y + 400, 16, 16, &pixel_graphics::icons::PCI_BLUE_ICON_DATA);
                    pg.draw_icon(x + 72, y + 400, 16, 16, &pixel_graphics::icons::CPU_ICON_DATA);
                    pg.draw_icon(x + 96, y + 400, 16, 16, &pixel_graphics::icons::HOURGLASS_ICON_DATA);
                    pg.draw_icon(x + 120, y + 400, 16, 16, &pixel_graphics::icons::ETHERNET_ICON_DATA);
                    pg.draw_icon(x + 144, y + 400, 16, 16, &pixel_graphics::icons::HDD_INTERNAL_ICON_DATA);
                    pg.draw_icon(x + 168, y + 400, 16, 16, &pixel_graphics::icons::SETTINGS_ICON_DATA);
                    pg.draw_icon(x + 200, y + 400, 32, 32, &pixel_graphics::icons::GTK_CUBE_32_ICON_DATA);
                    pg.draw_icon(x, y + 450, 32, 32, &pixel_graphics::icons::CD_DISK_32_ICON_DATA);
                    pg.draw_icon(x + 50, y + 450, 32, 32, &pixel_graphics::icons::SCRIPT_YELLOW_32_ICON_DATA);
                    pg.draw_icon(x + 100, y + 450, 32, 32, &pixel_graphics::icons::TAPE_WRITE_32_ICON_DATA);
                    pg.draw_icon(x + 150, y + 450, 32, 32, &pixel_graphics::icons::CUBE_TREE_32_ICON_DATA);
                    pg.draw_icon(x + 200, y + 450, 32, 32, &pixel_graphics::icons::GEAR_WINDOW_SETTINGS_32_ICON_DATA);
                    pg.draw_icon(x + 250, y + 450, 32, 32, &pixel_graphics::icons::GRAPHICS_2D_32_ICON_DATA);
                    pg.draw_icon(x, y + 500, 32, 32, &pixel_graphics::icons::BLADE_NETWORK_32_ICON_DATA);
                    pg.draw_icon(x + 50, y + 500, 32, 32, &pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA);
                    pg.draw_icon(x + 100, y + 500, 32, 32, &pixel_graphics::icons::WINOBJ_SEMAPHORE_32_ICON_DATA);
                    pg.draw_icon(x + 150, y + 500, 32, 32, &pixel_graphics::icons::REGEDIT_CUBES_32_ICON_DATA);
                    pg.draw_icon(x + 200, y + 500, 32, 32, &pixel_graphics::icons::REGISTRY_HIVE_32_ICON_DATA);
                    pg.draw_icon(x + 250, y + 500, 32, 32, &pixel_graphics::icons::DATABASE_CLUSTER_32_ICON_DATA);
                }
                DashboardTab::Editor => {
                    if let Some(ref ed) = self.editor {
                        // Draw Header with Mode
                        let mode_text = if ed.mode == EditorMode::Insert { "-- INSERT --" } else if ed.mode == EditorMode::Command {"-- COMMAND --"} else { "-- NORMAL --" };
                        let view_type = if ed.is_hex { "[HEX VIEW]" } else { "[TEXT VIEW]" };
                        pg.draw_text(margin, content_top + 5, &format!("Editing: {} {}", ed.file_path, view_type), 0x00FF00);
                        pg.draw_text(width - 150, content_top + 5, mode_text, 0xFFFF00);

                        let edit_y_start = content_top + 30;
                        let visible_lines = (height - edit_y_start - 60) / 20;

                        if ed.is_hex {
                            let mut y = content_top + 40;
                            let line_height = 20;
                            let hex_start_x = margin + 110;
                            let ascii_start_x = margin + 600;

                            for (i, chunk) in ed.buffer.chunks(16).enumerate().skip(ed.scroll_offset) {
                                if y > height - 80 { break; }

                                let offset = i * 16;
                                // Draw Offset in Gray
                                pg.draw_text(margin + 10, y, &format!("{:08X}", offset), 0x888888);

                                for (j, &byte) in chunk.iter().enumerate() {
                                    let color = match byte {
                                        0..=31 | 127 => 0x5555FF,   // Blue: Control
                                        32..=126 => 0xFFFFFF,      // White: ASCII
                                        _ => 0xFF00FF,             // Purple: Other/Extended
                                    };

                                    // Draw Hex Byte
                                    pg.draw_text(hex_start_x + (j * 30), y, &format!("{:02X}", byte), color);

                                    // Draw ASCII Char on the side
                                    let ascii_char = if byte >= 32 && byte <= 126 { byte as char } else { '.' };
                                    pg.draw_text(ascii_start_x + (j * 12), y, &ascii_char.to_string(), color);
                                }
                                y += line_height;
                            }
                        } else {
                            // Text Editor Rendering
                            let content = core::str::from_utf8(&ed.buffer).unwrap_or("");
                            for (i, line) in content.lines().skip(ed.scroll_offset).enumerate() {
                                if i >= visible_lines { break; }
                                pg.draw_text(margin + 40, edit_y_start + (i * 20), line, 0xFFFFFF);
                                // Line numbers
                                pg.draw_text(margin, edit_y_start + (i * 20), &format!("{:3}", ed.scroll_offset + i + 1), 0x666666);
                            }
                        }

                        pg.draw_text(margin, height - 70, ":w - Save | :q - Quit | i - Insert | Esc - Normal", 0x888888);
                        pg.draw_text(margin + 600, height - 70, &*format!(":{}", ed.command_buffer), 0xFFFFFF);
                    }
                }
                DashboardTab::Settings => {

                    pg.draw_text(10, page_y - 15, "SYSTEM SETTINGS", 0x00FF00);


                    let left_x = 10;
                    let left_y = page_y + 2;
                    let left_w = (width/3)-20;
                    let left_h = height - left_y - 20;
                    pg.draw_rect_outline(left_x, left_y, left_w, left_h, 0x444444);
                    pg.fill_rect(left_x + 1, left_y + 1, left_w - 2, 28, 0x222222);
                    pg.draw_text(left_x + 10, left_y + 8, "Categories", 0xAAAAAA);

                    let categories = [
                        ("General", "Runtime defaults and global behavior"),
                        ("Boot", "Startup, watchdog, and state restore"),
                        ("Interface", "Dashboard display and visual features"),
                        ("Virtual Machines", "VM lifecycle and safety defaults"),
                        ("Network", "NIC, ping, LAN scan, and HTTP controls"),
                        ("Storage", "File explorer and filesystem behavior"),
                        ("Packages", "Package index and verification policy"),
                        ("Developer", "Language, debug, and toolchain flags"),
                        ("Security", "Protected and experimental ring0 options"),
                        ("About", "Build and environment information"),
                    ];

                    let mut left_row_y = left_y + 38;
                    for (idx, (name, summary)) in categories.iter().enumerate() {
                        if left_row_y + 35 > left_y + left_h - 8 { break; }
                        let selected = idx == self.selected_settings_category_idx;
                        if selected {
                            pg.fill_rect(left_x + 4, left_row_y - 4, left_w - 8, 32, 0x334433);
                            pg.draw_rect_outline(left_x + 4, left_row_y - 4, left_w - 8, 32, 0x00AA00);
                        }
                        pg.draw_text(left_x + 12, left_row_y, name, if selected { 0xFFFF00 } else { 0xFFFFFF });
                        pg.draw_text(left_x + 12, left_row_y + 14, summary, 0x666666);
                        left_row_y += 38;
                    }

                    let right_x = (width/3) + 5;
                    let right_y = page_y + 2;
                    let right_w = (width*2/3) - 15;
                    let right_h = height - right_y - 20;
                    
                    pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x444444);
                    pg.fill_rect(right_x + 1, right_y + 1, right_w - 2, 28, 0x222222);

                    let selected_category = categories
                        .get(self.selected_settings_category_idx)
                        .map(|(name, _)| *name)
                        .unwrap_or("General");
                    pg.draw_text(right_x + 15, right_y + 8, selected_category, 0xFFFFFF);
                    
                    let mut x = right_x + 20;
                    let mut y = right_y + 45;

                    let settings = self.settings_rows();
                    for (idx, (label, value, blocked, disabled)) in settings.iter().enumerate() {
                        if idx == self.selected_settings_idx {
                            pg.fill_rect(right_x + 5, y - 4, right_w - 10, 24, 0x333333);
                        }
                        
                        let label_color = if *disabled { 0x555555 } else if idx == self.selected_settings_idx { 0xFFFF00 } else { 0xDDDDDD };
                        
                        if value.as_str() == "on" || value.as_str() == "off" {
                            pg.draw_checkbox(x, y, value.as_str() == "on", *blocked, *disabled, label);
                        } else {
                            pg.draw_text(x, y, label, label_color);
                            let val_color = if *blocked { 0xAA5555 } else { 0x00DCDC };
                            pg.draw_text(right_x + (right_w/2), y, value, val_color);
                        }
                        y += 28;
                    }

                    // Bottom info bar
                    let info_y = right_y + right_h - 140;
                    pg.draw_rect_outline(right_x + 10, info_y, right_w - 20, 100, 0x333333);
                    
                    pg.draw_text(right_x + 20, info_y + 10, "NAVIGATION", 0x888888);
                    pg.draw_text(right_x + 20, info_y + 30, "UP/DOWN: Select setting   LEFT/RIGHT: Change category", 0x666666);
                    pg.draw_text(right_x + 20, info_y + 50, "ENTER:   Toggle or cycle through options", 0x666666);
                    
                    if !self.status_line.is_empty() {
                        pg.draw_text(right_x + 20, info_y + 75, &format!("STATUS: {}", self.status_line), 0x00AAAA);
                    }

                    if selected_category == "About" {
                        let env_y = right_y + 180;
                        pg.draw_text(right_x + 20, env_y, "ENVIRONMENT SNAPSHOT", 0xAAAAAA);
                        let mut ey = env_y + 25;
                        for (key, value) in crate::env::global_vars_snapshot().iter().rev().take(8) {
                            pg.draw_text(right_x + 30, ey, &format!("{:<20} = {}", key, value), 0x777777);
                            ey += 18;
                        }
                    }
                }
                DashboardTab::Packages => {
                    pg.draw_text(20, 100, "Packages", 0x00FF00);

                    let package_names = self.package_names();
                    let list_x = 40;
                    let list_y = 140;
                    let list_w = 360;
                    let list_h = 420;
                    pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);
                    pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, 18, 0x333333);
                    pg.draw_text(list_x + 8, list_y + 4, "NAME                         TYPE", 0xCCCCCC);

                    let mut y = list_y + 28;
                    for (idx, name) in package_names.iter().enumerate() {
                        if y > list_y + list_h - 20 { break; }
                        let Some(pkg) = self.package_manager.registry.get(name) else { continue; };
                        if idx == self.selected_package_idx {
                            pg.fill_rect(list_x + 2, y - 2, list_w - 4, 16, 0x444400);
                        }
                        pg.draw_text(list_x + 8, y, &format!("{:<28} {:?}", pkg.name, pkg.package_type), if idx == self.selected_package_idx { 0xFFFF00 } else { 0xFFFFFF });
                        pg.draw_package_icon(list_x + list_w - 24, y - 1, true);
                        y += 18;
                    }

                    let detail_x = list_x + list_w + 30;
                    let detail_w = 520;
                    pg.draw_rect_outline(detail_x, list_y, detail_w, 420, 0x888888);
                    pg.fill_rect(detail_x + 1, list_y + 1, detail_w - 2, 18, 0x333333);
                    pg.draw_text(detail_x + 8, list_y + 4, "PACKAGE DETAILS", 0x00FF00);
                    
                    if let Some(name) = self.selected_package_name() {
                        if let Some(pkg) = self.package_manager.registry.get(&name) {
                            let mut dy = list_y + 30;
                            pg.draw_text(detail_x + 10, dy, &format!("Name:      {}", pkg.name), 0xFFFFFF);
                            dy += 20;
                            pg.draw_text(detail_x + 10, dy, &format!("Version:   {}", pkg.version), 0x00FFFF);
                            dy += 20;
                            pg.draw_text(detail_x + 10, dy, &format!("Type:      {:?}", pkg.package_type), 0xAAAAAA);
                            dy += 20;
                            pg.draw_text(detail_x + 10, dy, &format!("Author:    {}", pkg.author), 0xFFFFFF);
                            dy += 20;
                            
                            if let Some(ref url) = pkg.repo_url {
                                pg.draw_text(detail_x + 10, dy, &format!("Repo:      {}", url), 0x5555FF);
                                dy += 20;
                            }

                            let status_color = if pkg.has_compilation_issues { 0xFF5555 } else { 0x55FF55 };
                            let status_text = if pkg.has_compilation_issues { "FAILED / ISSUES" } else { "READY / OK" };
                            pg.draw_text(detail_x + 10, dy, &format!("Status:    {}", status_text), status_color);
                            dy += 30;

                            pg.draw_text(detail_x + 10, dy, "Dependencies:", 0x00FF00);
                            dy += 20;
                            if pkg.deps.is_empty() {
                                pg.draw_text(detail_x + 20, dy, "none", 0x888888);
                                dy += 20;
                            } else {
                                for dep in &pkg.deps {
                                    pg.draw_text(detail_x + 20, dy, &format!("- {}", dep), 0xCCCCCC);
                                    dy += 16;
                                }
                            }
                            dy += 10;
                            
                            pg.draw_text(detail_x + 10, dy, "Description:", 0x00FF00);
                            dy += 20;
                            // Basic wrapping for description
                            let desc = &pkg.description;
                            let words: Vec<&str> = desc.split_whitespace().collect();
                            let mut line = String::new();
                            for word in words {
                                if line.len() + word.len() > 60 {
                                    pg.draw_text(detail_x + 20, dy, &line, 0xAAAAAA);
                                    dy += 16;
                                    line.clear();
                                }
                                if !line.is_empty() { line.push(' '); }
                                line.push_str(word);
                            }
                            if !line.is_empty() {
                                pg.draw_text(detail_x + 20, dy, &line, 0xAAAAAA);
                            }
                        }
                    } else {
                        pg.draw_text(detail_x + 10, list_y + 40, "No packages loaded", 0xAAAAAA);
                    }

                    let actions = ["Refresh", "Verify", "Uninstall", "Update", "Download", "Autocompile"];
                    let mut action_x = 40;
                    let action_y = list_y + list_h + 24;
                    for (idx, action) in actions.iter().enumerate() {
                        let is_focused = idx == self.package_action_idx;
                        pg.fill_rect(action_x, action_y, 110, 26, if is_focused { 0x00AA00 } else { 0x444444 });
                        pg.draw_text(action_x + 8, action_y + 5, action, 0xFFFFFF);
                        action_x += 120;
                    }
                    if self.package_action_idx == 1 {
                        pg.draw_text(40, action_y + 30, "Verifies package dependencies and integrity", 0x00AAAA);
                    }
                    pg.draw_text(40, action_y + 40, "UP/DOWN selects package, LEFT/RIGHT chooses action, ENTER runs it", 0x888888);
                    pg.draw_text(40, action_y + 60, &self.status_line, 0xFFFF00);
                }
                _ => {
                    pg.draw_text(5, page_y - 15, "this page is unavailable", 0xFFFFFF)
                }
            }

            // Draw footer
            pg.fill_rect(0, height - 48, width, 48, 0x000080); // Blue
            pg.draw_text(10, height - 32, " Use keys O, V, R, S, N, D, C, T, Z to switch tabs | X to shutdown", 0xFFFFFF);
            pg.draw_text(width - 60, height - 13, &format!("{} fps", self.resources.fps), 0xFFFFFF);
            pg.draw_text(width - 120, height - 13, &format!("{} ms", self.resources.frame_ms), 0xFFFFFF);
            pg.draw_text(width - 200, height - 13, &format!("{} MHz", TSC_PER_US), 0xFFFFFF);
            pg.draw_text(width - 250, height - 13, &format!("{}%", self.resources.cpu_usage), 0xFFFFFF);
            pg.draw_text(width - 330, height - 13, &format!("{} MB", self.resources.used_memory_mb), 0xFFFFFF);
            pg.draw_rect_outline(width - 340, height - 15, 338, 14, 0xCCCCCC);

            // Update and draw cursor
            if self.iter % 20 == 0 {
                unsafe {
                    //self.cursor.update_from_mouse(width, height);
                }
            }



            // apply settings to these items
            //pg.app_context_border("");





            //pg.flip();

            // Handle active apps (Windows)
            let mut apps_to_remove = Vec::new();
            for (idx, app_ctx) in self.active_apps.iter_mut().enumerate() {
                let is_focused = self.focused_process_idx == Some(idx);
                
                // For now, let's just step them.
                // In a real windowing system, we'd only step if they are "active" or always.
                // We'll pass None for key if not focused, or handle it in handle_input.
                // Wait, if we want them to be "stepped", we should call step().
                
                // Let's draw a window for the app
                let win_x = self.app_window_position.0 + idx * 60;
                let win_y = self.app_window_position.1 + idx * 60;
                let win_w = app_ctx.application.dimensions()[0] + 2;
                let win_h = app_ctx.application.dimensions()[1] + 20;
                
                pg.fill_rect(win_x, win_y, win_w, win_h, 0x111111);
                pg.draw_rect_outline(win_x, win_y, win_w, win_h, if is_focused { 0x00FFFF } else { 0x888888 });
                pg.fill_rect(win_x, win_y, win_w, 20, if is_focused { 0x008080 } else { 0x444444 });
                pg.fill_rect(win_x+win_w-20, win_y, 20, 20, if is_focused { 0xAA0000 } else { 0x440000 });
                pg.draw_text(win_x+win_w-15, win_y+2, "X", 0xFFFFFF);
                pg.draw_text(win_x + 5, win_y + 2, &app_ctx.application.name, 0xFFFFFF);
                
                // App content - the app's draw() takes x, y.
                // Note: app_ctx.step() currently draws to a NEW PixelGraphics if it can.
                // We need to be careful here because SteppedApplicationContext::step() 
                // creates its own PixelGraphics internally which clears the screen/backbuffer.
                
                // If we want it to draw INTO our UI, we might need to modify step() 
                // or handle drawing here.
                
                // For now, let's just run logic.
                if !app_ctx.step(None) {
                    apps_to_remove.push(idx);
                }
                
                // And manually call draw if we want it in a window
                let mut app_vars = Vec::new();
                app_ctx.application.draw(&mut pg, &app_vars, win_x + 2, win_y + 20);
            }
            
            for idx in apps_to_remove.into_iter().rev() {
                self.active_apps.remove(idx);
                if self.focused_process_idx == Some(idx) {
                    self.focused_process_idx = None;
                } else if let Some(fidx) = self.focused_process_idx {
                    if fidx > idx {
                        self.focused_process_idx = Some(fidx - 1);
                    }
                }
            }

            pg.draw_cursor(self.cursor.x as usize, self.cursor.y as usize);

            // Draw functional UI layers
            let mut ypos = 0;
            for (msg, duration) in &mut self.notifications {
                pg.draw_toast(msg, duration, ypos);
                ypos += 50;
            }
            self.notifications.retain(|(_, d)| *d > 0);

            if self.command_palette_active {
                let filtered = self.filtered_command_palette_items();
                pg.draw_command_palette(&self.command_palette_query, &filtered, self.command_palette_selected, self.command_palette_scroll_offset);
            }

            if self.settings.pg_scanlines { pg.apply_scanlines(); }
            if self.settings.pg_dither { pg.apply_dither(); }
            if self.settings.pg_glitch { self.glitch_y = pg.apply_glitch(self.glitch_y); }

            match self.settings.pg_aberration {
                1 => pg.apply_edge_aberration(0.2),
                2 => pg.apply_edge_aberration(0.5),
                3 => pg.apply_edge_aberration(0.8),
                4 => pg.apply_edge_aberration(1.6),
                5 => pg.apply_edge_aberration(3.0),
                _ => {}
            }

            pg.flip();

        } else {
            // self.draw_header();
            // self.draw_navigation_bar();
            //
            // match self.selected_tab {
            //     DashboardTab::Overview => self.draw_overview(),
            //     DashboardTab::VirtualMachines => self.draw_vms_list(),
            //     DashboardTab::Resources => self.draw_resources(),
            //     DashboardTab::Storage => self.draw_storage(),
            //     DashboardTab::Network => self.draw_network(),
            //     DashboardTab::Console => self.draw_console(),
            //     DashboardTab::Devices => {},
            //     DashboardTab::Test => {},
            // }
            //
            // self.draw_footer();
            message!("", "dashboard unavailable")
        }
    }

    fn count_running_vms(&self) -> usize {
        self.vms.iter()
            .filter(|vm| vm.state.contains("running"))
            .count()
    }

    pub fn refresh_devices(&mut self) {
        self.pci_devices = crate::hardware::pci::scan_bus();
        let device_map = &crate::filesystem::FileSystem::get_state().device_map;
        
        // Group devices by categories
        let mut disks = Vec::new();
        let mut nets = Vec::new();
        let mut usbs = Vec::new();
        let mut coms = Vec::new();
        let mut pcis = Vec::new();
        let mut others = Vec::new();
        
        for (alias, path) in device_map {
            let entry = DeviceEntry { name: alias.clone(), path: path.clone() };
            if alias.starts_with("dsk") { disks.push(entry); }
            else if alias.starts_with("net") { nets.push(entry); }
            else if alias.starts_with("usb") { usbs.push(entry); }
            else if alias.starts_with("com") { coms.push(entry); }
            else if alias.starts_with("pci") { pcis.push(entry); }
            else { others.push(entry); }
        }
        
        // Preserve expansion state if categories already exist
        let mut expanded_map = BTreeMap::new();
        for cat in &self.categories {
            expanded_map.insert(cat.name.clone(), cat.expanded);
        }
        
        self.categories.clear();
        if !disks.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("Disk Drives"),
                devices: disks,
                expanded: *expanded_map.get("Disk Drives").unwrap_or(&true),
                icon: String::from("[D] "),
            });
        }
        if !nets.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("Network Adapters"),
                devices: nets,
                expanded: *expanded_map.get("Network Adapters").unwrap_or(&true),
                icon: String::from("[N] "),
            });
        }
        if !usbs.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("USB Controllers"),
                devices: usbs,
                expanded: *expanded_map.get("USB Controllers").unwrap_or(&true),
                icon: String::from("[U] "),
            });
        }
        if !coms.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("Serial Ports"),
                devices: coms,
                expanded: *expanded_map.get("Serial Ports").unwrap_or(&true),
                icon: String::from("[C] "),
            });
        }
        if !pcis.is_empty() || !self.pci_devices.is_empty() {
            let mut pci_entries = pcis;
            for pci in &self.pci_devices {
                pci_entries.push(DeviceEntry {
                    name: format!("{:02X}:{:02X}.{}", pci.bus, pci.device, pci.function),
                    path: format!("{} [0x{:04X}:0x{:04X}]", pci.class_name(), pci.vendor_id, pci.device_id),
                });
            }
            self.categories.push(DeviceCategory {
                name: String::from("PCI Devices"),
                devices: pci_entries,
                expanded: *expanded_map.get("PCI Devices").unwrap_or(&false),
                icon: String::from("[P] "),
            });
        }
        if !others.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("Other Devices"),
                devices: others,
                expanded: *expanded_map.get("Other Devices").unwrap_or(&false),
                icon: String::from("[?] "),
            });
        }
    }

    pub fn refresh_storage(&mut self) {
        use uefi::proto::media::file::{File, FileMode, FileAttribute};
        use uefi::proto::media::fs::SimpleFileSystem;

        self.files.clear();

        let handle = match uefi::boot::get_handle_for_protocol::<SimpleFileSystem>() {
            Ok(h) => h,
            Err(_) => {
                self.ui_error(27);
                return;
            },
        };
        let mut sfs = match uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle) {
            Ok(s) => s,
            Err(_) => {
                self.ui_error(19);
                return;
            },
        };
        let mut root_dir = match sfs.open_volume() {
            Ok(d) => d,
            Err(_) => {
                self.ui_error(28);
                return;
            },
        };

        let mut target_dir = if self.current_path == "\\" || self.current_path == "/" {
            root_dir
        } else {
            let mut u16_path: Vec<u16> = self.current_path.encode_utf16().collect();
            u16_path.push(0);
            let path_cstr = match uefi::data_types::CStr16::from_u16_with_nul(&u16_path) {
                Ok(c) => c,
                Err(_) => {
                    self.ui_error(24);
                    return;
                },
            };

            let handle = match root_dir.open(path_cstr, FileMode::Read, FileAttribute::DIRECTORY) {
                Ok(h) => h,
                Err(_) => {
                    self.ui_error(10);
                    return;
                },
            };

            match handle.into_directory() {
                Some(d) => d,
                None => {
                    self.ui_error(28);
                    return;
                },
            }
        };

        let mut buffer = [0u8; 4096];
        loop {
            match target_dir.read_entry(&mut buffer) {
                Ok(Some(entry)) => {
                    let name = entry.file_name().to_string();
                    let size = entry.file_size();
                    let is_dir = entry.attribute().contains(FileAttribute::DIRECTORY);
                    
                    self.files.push(FileEntry {
                        name,
                        size,
                        is_dir,
                    });
                }
                _ => break,
            }
        }

        // Clamp selected index to new list size
        if !self.files.is_empty() {
            if self.selected_file_idx >= self.files.len() {
                self.selected_file_idx = self.files.len() - 1;
            }
        } else {
            self.selected_file_idx = 0;
        }
    }

    pub fn get_create_vm_data(&self) -> (String, u32, u32) {
        (self.new_vm_name.clone(), self.new_vm_memory_mb, self.new_vm_vcpus)
    }

    pub fn reset_create_vm_data(&mut self) {
        self.new_vm_name = String::from("NewVM");
        self.new_vm_memory_mb = 1024;
        self.new_vm_vcpus = 1;
        self.create_vm_focus_idx = 0;
    }

    fn option_value(options: &[&'static str], idx: usize) -> String {
        options.get(idx).copied().unwrap_or(options[0]).to_string()
    }

    pub fn settings_rows(&self) -> Vec<(String, String, bool, bool)> {
        match self.selected_settings_category_idx {
            0 => alloc::vec![
                (String::from("HPVMX_PROFILE"), Self::option_value(&["balanced", "diagnostic", "performance"], self.settings.general_profile), false, false),
                (String::from("Extra Debug Info"), if self.settings.extra_debug_info { "on" } else { "off" }.to_string(), false, false),
                (String::from("HPVMX_USER"), String::from("operator"), false, false),
                (String::from("Experimental Mem Comp"), if self.settings.experimental_mem_comp { "on" } else { "off" }.to_string(), false, false),
            ],
            1 => alloc::vec![
                (String::from("HPVMX_BOOT_TARGET"), Self::option_value(&["dashboard", "shell", "last-vm"], self.settings.boot_target), false, false),
                (String::from("State Save/Restore"), if self.settings.state_save_restore { "on" } else { "off" }.to_string(), false, false),
                (String::from("HPVMX_WATCHDOG"), String::from("disabled"), false, false),
            ],
            2 => alloc::vec![
                (String::from("HPVMX_UI_DENSITY"), Self::option_value(&["normal", "compact", "wide"], self.settings.interface_density), false, false),
                (String::from("HPVMX_UI_SCALING"), Self::option_value(&["50%", "100%", "150%", "200%"], self.settings.ui_scaling), false, false),
                (String::from("Extended Symbol Library"), if self.settings.extended_symbol_library { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG VShaders"), if self.settings.pg_vshaders { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Scanlines"), if self.settings.pg_scanlines { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Dither"), if self.settings.pg_dither { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Glitch"), if self.settings.pg_glitch { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Aberration"), Self::option_value(&["off", "low", "mid", "high", "super", "extreme"], self.settings.pg_aberration), false, false),
            ],
            3 => alloc::vec![
                (String::from("HPVMX_VM_SAFETY"), Self::option_value(&["prompt", "auto-save", "strict"], self.settings.vm_safety_policy), false, false),
                (String::from("HPVMX_VM_DEFAULT_MEM"), format!("{}MB", self.new_vm_memory_mb), false, false),
                (String::from("HPVMX_VM_DEFAULT_CPUS"), format!("{}", self.new_vm_vcpus), false, false),
            ],
            4 => alloc::vec![
                (String::from("HPVMX_NET_PROFILE"), Self::option_value(&["dhcp", "static", "loopback"], self.settings.network_profile), false, false),
                (String::from("HPVMX_NET_TARGET"), self.network_target.clone(), false, false),
                (String::from("HPVMX_HTTPD_PORT"), String::from("8080"), false, false),
            ],
            5 => alloc::vec![
                (String::from("HPVMX_STORAGE_POLICY"), Self::option_value(&["preserve", "confirm-delete", "developer"], self.settings.storage_policy), false, false),
                (String::from("Folder Absolute Sizes"), if self.settings.folder_absolute_sizes { "on" } else { "off" }.to_string(), false, false),
                (String::from("Auto-refresh Storage"), if self.settings.auto_refresh_storage { "on" } else { "off" }.to_string(), false, false),
                (String::from("Show Hidden Files"), if self.settings.show_hidden_files { "on" } else { "off" }.to_string(), false, false),
            ],
            6 => alloc::vec![
                (String::from("HPVMX_PM_VERIFY"), Self::option_value(&["standard", "quick", "full"], self.settings.package_policy), false, false),
                (String::from("HPVMX_PM_AUTOHEAL"), String::from("off"), false, false),
                (String::from("HPVMX_PM_INDEX"), self.package_manager.package_path.clone(), false, false),
            ],
            7 => alloc::vec![
                (String::from("HPVMX_DEV_LEVEL"), Self::option_value(&["normal", "verbose", "toolchain"], self.settings.developer_level), false, false),
                (String::from("Terminal Font"), Self::option_value(&["8x16", "dualscale (experimental)"], self.settings.terminal_font), false, false),
                (String::from("ControlLang Support"), if self.settings.controllang_support { "on" } else { "off" }.to_string(), false, true),
                (String::from("HPVMX_MICRO_C_TARGET"), String::from("x86_64"), false, false),
            ],
            8 => alloc::vec![
                (String::from("HPVMX_SECURITY_POLICY"), Self::option_value(&["standard", "paranoid", "lab"], self.settings.security_policy), false, false),
                (String::from("Ring0 UDMI/UDXI"), if self.settings.ring0_udmi_udxi { "on" } else { "off" }.to_string(), true, false),
                (String::from("HPVMX_AUTOLYTIC"), String::from("enabled"), false, false),
            ],
            _ => alloc::vec![
                (String::from("HPVMX_VERSION"), env!("CARGO_PKG_VERSION").to_string(), false, true),
                (String::from("HPVMX_BUILD"), String::from("dev"), false, true),
                (String::from("HPVMX_ENV_COUNT"), format!("{}", crate::env::global_vars_snapshot().len()), false, true),
                (String::from("UEFI_VERSION"), String::from("2.10"), false, true),
            ],
        }
    }

    pub fn package_names(&self) -> Vec<String> {
        self.package_manager.registry.keys().cloned().collect()
    }

    pub fn selected_package_name(&self) -> Option<String> {
        self.package_names().get(self.selected_package_idx).cloned()
    }

    pub fn execute_package_action(&mut self) {
        match self.package_action_idx {
            0 => {
                self.package_manager.load_registry();
                self.status_line = String::from("Package registry refreshed");
            }
            1 => {
                if let Some(name) = self.selected_package_name() {
                    self.package_manager.verify_dependencies(&name);
                    self.status_line = format!("Verified dependencies for {}", name);
                }
            }
            2 => {
                if let Some(name) = self.selected_package_name() {
                    self.package_manager.registry.remove(&name);
                    self.selected_package_idx = self.selected_package_idx.saturating_sub(1);
                    self.status_line = format!("Uninstalled {}", name);
                }
            }
            3 => {
                if let Some(name) = self.selected_package_name() {
                    self.status_line = format!("{} marked for update on next index refresh", name);
                }
            }
            4 => {
                if let Some(name) = self.selected_package_name() {
                    self.package_manager.download_package(&name);
                    self.status_line = format!("Downloaded package: {}", name);
                }
            }
            5 => {
                if let Some(name) = self.selected_package_name() {
                    self.package_manager.autocompile_package(&name);
                    self.status_line = format!("Autocompiled package: {}", name);
                }
            }
            _ => {}
        }
    }

    pub fn execute_network_action(&mut self) {
        match self.selected_network_action_idx {
            0 => {
                match crate::devices::net_hw::init() {
                    Ok(()) => self.status_line = String::from("NIC initialized"),
                    Err(e) => self.status_line = format!("NIC init failed: {}", e),
                }
            }
            1 => {
                crate::devices::net::status();
                self.status_line = String::from("Network status refreshed");
            }
            2 => {
                let _ = crate::devices::net::ping(&self.network_target, 4, 250);
                self.status_line = format!("Ping sent to {}", self.network_target);
            }
            3 => {
                crate::devices::net::lanscan("192.168.1.");
                self.status_line = String::from("LAN scan started for 192.168.1.*");
            }
            4 => {
                crate::devices::net::httpd_start(8080);
                self.status_line = String::from("HTTP management server started on 8080");
            }
            5 => {
                crate::devices::net::httpd_stop();
                self.status_line = String::from("HTTP management server stopped");
            }
            _ => {}
        }
    }

    pub fn toggle_selected_setting(&mut self) {
        let cycle = |idx: &mut usize, max: usize| {
            *idx = (*idx + 1) % max;
        };

        match (self.selected_settings_category_idx, self.selected_settings_idx) {
            (0, 0) => cycle(&mut self.settings.general_profile, 3),
            (0, 1) => self.settings.extra_debug_info = !self.settings.extra_debug_info,
            (0, 3) => self.settings.experimental_mem_comp = !self.settings.experimental_mem_comp,
            (1, 0) => cycle(&mut self.settings.boot_target, 3),
            (1, 1) => self.settings.state_save_restore = !self.settings.state_save_restore,
            (2, 0) => cycle(&mut self.settings.interface_density, 3),
            (2, 1) => cycle(&mut self.settings.ui_scaling, 4),
            (2, 2) => self.settings.extended_symbol_library = !self.settings.extended_symbol_library,
            (2, 3) => self.settings.pg_vshaders = !self.settings.pg_vshaders,
            (2, 4) => self.settings.pg_scanlines = !self.settings.pg_scanlines,
            (2, 5) => self.settings.pg_dither = !self.settings.pg_dither,
            (2, 6) => self.settings.pg_glitch = !self.settings.pg_glitch,
            (2, 7) => cycle(&mut self.settings.pg_aberration, 6),
            (3, 0) => cycle(&mut self.settings.vm_safety_policy, 3),
            (3, 1) => {
                self.new_vm_memory_mb = match self.new_vm_memory_mb {
                    256 => 512,
                    512 => 1024,
                    1024 => 2048,
                    _ => 256,
                };
            }
            (3, 2) => self.new_vm_vcpus = if self.new_vm_vcpus >= 4 { 1 } else { self.new_vm_vcpus + 1 },
            (4, 0) => cycle(&mut self.settings.network_profile, 3),
            (4, 1) => {
                self.network_target = if self.network_target == "127.0.0.1" {
                    String::from("192.168.1.1")
                } else if self.network_target == "192.168.1.1" {
                    String::from("10.0.0.1")
                } else {
                    String::from("127.0.0.1")
                };
            }
            (5, 0) => cycle(&mut self.settings.storage_policy, 3),
            (5, 1) => self.settings.folder_absolute_sizes = !self.settings.folder_absolute_sizes,
            (5, 2) => self.settings.auto_refresh_storage = !self.settings.auto_refresh_storage,
            (5, 3) => self.settings.show_hidden_files = !self.settings.show_hidden_files,
            (6, 0) => cycle(&mut self.settings.package_policy, 3),
            (7, 0) => cycle(&mut self.settings.developer_level, 3),
            (7, 1) => cycle(&mut self.settings.terminal_font, 3),
            (7, 2) => {
                if !self.settings.controllang_support {
                    self.status_line = String::from("ControlLang support is disabled in this build");
                    self.publish_selected_setting();
                    return;
                }
                self.settings.controllang_support = !self.settings.controllang_support;
            }
            (8, 0) => cycle(&mut self.settings.security_policy, 3),
            (8, 1) => {
                self.status_line = String::from("Ring0 UDMI/UDXI is blocked");
                self.publish_selected_setting();
                return;
            }
            _ => {}
        }
        self.publish_selected_setting();
    }

    pub fn publish_selected_setting(&mut self) {
        let rows = self.settings_rows();
        if let Some((key, value, blocked, disabled)) = rows.get(self.selected_settings_idx) {
            if *blocked || *disabled {
                self.status_line = format!("{} is not writable", key);
                return;
            }
            crate::env::set_global_var(key, value);
            self.status_line = format!("{}={}", key, value);
        }
    }

    pub fn get_selected_vm_id(&self) -> Option<u32> {
        self.vms.get(self.selected_vm_idx).map(|vm| vm.id)
    }

    pub fn get_selected_action(&self) -> usize {
        self.vm_action_idx
    }

    pub fn is_create_vm_requested(&self) -> bool {
        matches!(self.selected_tab, DashboardTab::CreateVM) && self.create_vm_focus_idx == 3
    }

    pub fn set_tab(&mut self, tab: DashboardTab) {
        self.selected_tab = tab;
    }

    pub fn get_tab(&self) -> DashboardTab {
        self.selected_tab
    }

    fn execute_palette_command(&mut self, cmd: &str) {
        let run_help = |topic: &'static str, package_manager: &mut PackageManager| {
            let command = vec!["help", topic];
            let parts = command.clone();
            let body = command.clone();
            terminal::cmd(command, &parts, body, package_manager);
        };

        let run_simple = |command: Vec<&'static str>, package_manager: &mut PackageManager| {
            let parts = command.clone();
            let body = command.clone();
            terminal::cmd(command, &parts, body, package_manager);
        };

        let selected_vm_id = self.get_selected_vm_id().map(|id| id.to_string());
        let select_setting = |ui: &mut Self, category: usize, row: usize| {
            ui.selected_settings_category_idx = category;
            ui.selected_settings_idx = row;
            ui.selected_tab = DashboardTab::Settings;
            ui.toggle_selected_setting();
        };

        match cmd {
            "Overview: Show System Summary" => self.selected_tab = DashboardTab::Overview,
            "Overview: Refresh Resources" => {
                self.selected_tab = DashboardTab::Overview;
                self.status_line = String::from("Resource snapshot is live");
            }
            "VM: List Virtual Machines" | "VM: Refresh List" => {
                self.selected_tab = DashboardTab::VirtualMachines;
                run_simple(vec!["vm", "list"], &mut self.package_manager);
            }
            "VM: Create New VM" => self.selected_tab = DashboardTab::CreateVM,
            "VM: Start Selected VM" => {
                if let Some(id) = selected_vm_id.as_deref() {
                    crate::handle_vm_command(&["vm", "start", id]);
                } else {
                    self.ui_error_with_detail(1, Some("No VM selected"));
                }
            }
            "VM: Stop Selected VM" => {
                if let Some(id) = selected_vm_id.as_deref() {
                    crate::handle_vm_command(&["vm", "stop", id]);
                } else {
                    self.ui_error_with_detail(1, Some("No VM selected"));
                }
            }
            "VM: Delete Selected VM" => {
                if let Some(id) = selected_vm_id.as_deref() {
                    crate::handle_vm_command(&["vm", "delete", id]);
                } else {
                    self.ui_error_with_detail(1, Some("No VM selected"));
                }
            }
            "VM: Boot with Media" => {
                if self.get_selected_vm_id().is_some() {
                    crate::handle_vm_command(&["vm", "boot"]);
                } else {
                    self.ui_error_with_detail(1, Some("No VM selected"));
                }
            }
            "VM: Simulate Violation" => crate::handle_vm_command(&["vm", "simulate-violation"]),
            "Resources: View CPU/Memory" => self.selected_tab = DashboardTab::Resources,
            "Storage: Browse Files" => self.selected_tab = DashboardTab::Storage,
            "Storage: Refresh Drives" | "System: Refresh Storage" => {
                self.refresh_storage();
                self.notifications.push(("Storage refreshed".to_string(), 60));
            }
            "Storage: New File" => {
                let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                let new_file = format!("{}{}new_file_{}.txt", self.current_path, sep, self.filesys_new_counter);
                match crate::FileSystem::touch(&new_file) {
                    Ok(_) => {
                        self.filesys_new_counter += 1;
                        self.status_line = format!("Created {}", new_file);
                        self.refresh_storage();
                    }
                    Err(e) => self.status_line = format!("Create failed: {}", e),
                }
            }
            "Storage: New Folder" => {
                let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                let new_dir = format!("{}{}new_folder_{}", self.current_path, sep, self.filesys_new_counter);
                match crate::FileSystem::mkdir(&new_dir) {
                    Ok(_) => {
                        self.filesys_new_counter += 1;
                        self.status_line = format!("Created {}", new_dir);
                        self.refresh_storage();
                    }
                    Err(e) => self.status_line = format!("Create folder failed: {}", e),
                }
            }
            "Storage: Rename Selected" => {
                self.filesys_pending_action = Some(FilePendingAction::Rename);
                self.selected_tab = DashboardTab::Storage;
                self.status_line = String::from("Confirm rename in Storage with End");
            }
            "Storage: Copy Selected" => {
                self.filesys_pending_action = Some(FilePendingAction::Copy);
                self.selected_tab = DashboardTab::Storage;
                self.status_line = String::from("Confirm copy in Storage with End");
            }
            "Storage: Move Selected" => {
                self.filesys_pending_action = Some(FilePendingAction::Move);
                self.selected_tab = DashboardTab::Storage;
                self.status_line = String::from("Confirm move in Storage with End");
            }
            "Storage: Delete Selected" => {
                self.filesys_pending_action = Some(FilePendingAction::Delete);
                self.selected_tab = DashboardTab::Storage;
                self.status_line = String::from("Confirm delete in Storage with End");
            }
            "Storage: Toggle Hidden Files" => {
                self.settings.show_hidden_files = !self.settings.show_hidden_files;
                self.refresh_storage();
                self.status_line = format!("Show hidden files: {}", if self.settings.show_hidden_files { "on" } else { "off" });
            }
            "Storage: Toggle Auto Refresh" => {
                self.settings.auto_refresh_storage = !self.settings.auto_refresh_storage;
                self.status_line = format!("Auto-refresh storage: {}", if self.settings.auto_refresh_storage { "on" } else { "off" });
            }
            "Network: View Connections" => self.selected_tab = DashboardTab::Network,
            "Network: Show NIC Status" => self.execute_network_action_with_index(1),
            "Network: Initialize SNP" => self.execute_network_action_with_index(0),
            "Network: Ping Target" => self.execute_network_action_with_index(2),
            "Network: LAN Scan" => self.execute_network_action_with_index(3),
            "Network: Start HTTP Server" => self.execute_network_action_with_index(4),
            "Network: Stop HTTP Server" => self.execute_network_action_with_index(5),
            "Console: Attach to VM" => self.selected_tab = DashboardTab::Console,
            "Devices: Manage Hardware" => self.selected_tab = DashboardTab::Devices,
            "Devices: List Drives" => run_simple(vec!["devs"], &mut self.package_manager),
            "Devices: Refresh Hardware" | "System: Refresh Devices" => {
                self.refresh_devices();
                self.notifications.push(("Devices refreshed".to_string(), 60));
            }
            "Apps: Run Applications" => self.selected_tab = DashboardTab::Apps,
            "Packages: Software Manager" => self.selected_tab = DashboardTab::Packages,
            "Packages: Reload Registry" => run_simple(vec!["pm", "reload"], &mut self.package_manager),
            "Packages: Verify Dependencies" => run_simple(vec!["pm", "verify"], &mut self.package_manager),
            "Packages: Show Version" => run_simple(vec!["pm", "version"], &mut self.package_manager),
            "Settings: UI Configuration" => self.selected_tab = DashboardTab::Settings,
            "Settings: Toggle Extra Debug Info" => select_setting(self, 0, 1),
            "Settings: Toggle State Save/Restore" => select_setting(self, 1, 1),
            "Settings: Toggle Scanlines" => select_setting(self, 2, 4),
            "Settings: Toggle Dither" => select_setting(self, 2, 5),
            "Settings: Toggle Glitch" => select_setting(self, 2, 6),
            "Settings: Cycle Aberration" => select_setting(self, 2, 7),
            "Settings: Cycle Interface Density" => select_setting(self, 2, 0),
            "Settings: Cycle UI Scaling" => select_setting(self, 2, 1),
            "Settings: Cycle General Profile" => select_setting(self, 0, 0),
            "Settings: Cycle Boot Target" => select_setting(self, 1, 0),
            "Settings: Cycle VM Safety" => select_setting(self, 3, 0),
            "Settings: Cycle Network Profile" => select_setting(self, 4, 0),
            "Settings: Cycle Storage Policy" => select_setting(self, 5, 0),
            "Settings: Cycle Package Policy" => select_setting(self, 6, 0),
            "Settings: Cycle Developer Level" => select_setting(self, 7, 0),
            "Settings: Cycle Security Policy" => select_setting(self, 8, 0),
            "Test: Run Diagnostics" => self.selected_tab = DashboardTab::Test,
            "Editor: Text Editor" => self.selected_tab = DashboardTab::Editor,
            "Editor: New Buffer" => {
                self.editor = Some(TextEditor::new(String::from("untitled.txt"), Vec::new()));
                self.selected_tab = DashboardTab::Editor;
            }
            "VMM: Show Statistics" => crate::handle_vmm_command(&["vmm", "info"]),
            "VMM: Advanced Stats" => crate::handle_vmm_command(&["vmm", "info-adv"]),
            "System: Refresh All" => {
                self.refresh_storage();
                self.refresh_devices();
                self.package_manager.load_registry();
                self.notifications.push(("System refreshed".to_string(), 60));
            }
            "System: Clear Logs" => self.notifications.push(("Logs cleared (UI only)".to_string(), 60)),
            "System: System Information" => run_simple(vec!["sysinfo"], &mut self.package_manager),
            "System: BIOS (Exit)" => run_simple(vec!["BIOS"], &mut self.package_manager),
            "System: Reboot Host" => runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, Some(&[255])),
            "System: Shutdown" => runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, Some(&[0])),
            "System: Mouse Debugging" => run_simple(vec!["mouse-debug"], &mut self.package_manager),
            "Help: Command Palette Shortcuts" => self.ui_error_with_detail(0, Some("UP/DOWN/TAB: Navigate, ENTER: Execute, ESC: Close, $: Terminal command mode")),
            "Help: FileSystem Help" => run_help("fs", &mut self.package_manager),
            "Help: VM Help" => run_help("vm", &mut self.package_manager),
            "Help: Hypervisor Help" => run_help("hv", &mut self.package_manager),
            "Help: Network Help" => run_help("net", &mut self.package_manager),
            "Help: Package Manager Help" => run_help("pm", &mut self.package_manager),
            "Help: Micro-C Help" => run_help("micro-c", &mut self.package_manager),
            "Help: Misc Help" => run_help("misc", &mut self.package_manager),
            "System: Scan PCI Bus" => {
                self.pci_devices = crate::hardware::pci::scan_bus();
                self.notifications.push((format!("PCI Scan: {} devices found", self.pci_devices.len()), 120));
            }
            _ => {}
        }
    }

    fn execute_network_action_with_index(&mut self, idx: usize) {
        self.selected_network_action_idx = idx;
        self.execute_network_action();
    }

    pub fn handle_input(&mut self, key: Key) {
        use uefi::proto::console::text::ScanCode;

        // Command Palette handling
        if self.command_palette_active {
            match key {
                Key::Printable(c) => {
                    let ch = char::from(c);

                    if ch == '\r' || ch == '\n' {
                        if self.command_palette_query.starts_with('$') {
                            let command_str = self.command_palette_query[1..].trim().to_string();
                            let filtered = self.filtered_command_palette_items();
                            let selected_command = filtered
                                .get(self.command_palette_selected)
                                .map(|cmd| Self::command_core(cmd).to_string());

                            if let Some(cmd) = selected_command {
                                self.run_terminal_command_string(&cmd);
                            } else if !command_str.is_empty() {
                                self.run_terminal_command_string(&command_str);
                            }

                            self.close_command_palette();
                        } else {
                            let filtered = self.filtered_command_palette_items();
                            if let Some(cmd) = filtered.get(self.command_palette_selected) {
                                let cmd_core = Self::command_core(cmd).to_string();
                                self.status_line = alloc::format!("Executing: {}", cmd_core);
                                self.notifications.push((alloc::format!("Command: {}", cmd_core), 120));
                                self.execute_palette_command(cmd);
                            }
                            self.close_command_palette();
                        }
                    } else if ch == '\u{08}' {
                        self.command_palette_query.pop();
                        self.command_palette_selected = 0;
                        self.command_palette_scroll_offset = 0;
                    } else if ch == '\t' {
                        let filtered_count = self.command_palette_filtered_count();
                        if filtered_count > 0 {
                            self.command_palette_selected = (self.command_palette_selected + 1) % filtered_count;
                            if self.command_palette_selected < self.command_palette_scroll_offset {
                                self.command_palette_scroll_offset = self.command_palette_selected;
                            } else if self.command_palette_selected >= self.command_palette_scroll_offset + COMMAND_PALETTE_VISIBLE_COUNT {
                                self.command_palette_scroll_offset = self.command_palette_selected - COMMAND_PALETTE_VISIBLE_COUNT + 1;
                            }
                        }
                    } else if !ch.is_control() {
                        self.command_palette_query.push(ch);
                        self.command_palette_selected = 0;
                        self.command_palette_scroll_offset = 0;
                    }
                }
                Key::Special(ScanCode::ESCAPE) => self.close_command_palette(),
                Key::Special(ScanCode::UP) => {
                    if self.command_palette_selected > 0 {
                        self.command_palette_selected -= 1;
                        if self.command_palette_selected < self.command_palette_scroll_offset {
                            self.command_palette_scroll_offset = self.command_palette_selected;
                        }
                    }
                }
                Key::Special(ScanCode::DOWN) => {
                    let filtered_count = self.command_palette_filtered_count();
                    if filtered_count > 0 && self.command_palette_selected < filtered_count - 1 {
                        self.command_palette_selected += 1;
                        if self.command_palette_selected >= self.command_palette_scroll_offset + COMMAND_PALETTE_VISIBLE_COUNT {
                            self.command_palette_scroll_offset = self.command_palette_selected - COMMAND_PALETTE_VISIBLE_COUNT + 1;
                        }
                    }
                }
                _ => {}
            }
            return;
        }


        match self.selected_tab {
            DashboardTab::CreateVM => {
                match key {
                    Key::Printable(c) => {
                        let ch = char::from(c).to_ascii_lowercase();
                        match ch {
                            ' ' => {
                                if self.create_vm_focus_idx == 0 {
                                    self.new_vm_name.push(' ');
                                }
                            }
                            '+' | '=' => {
                                if self.create_vm_focus_idx == 1 {
                                    self.new_vm_vcpus += 1;
                                } else if self.create_vm_focus_idx == 2 {
                                    self.new_vm_memory_mb += 128;
                                }
                            }
                            '-' | '_' => {
                                if self.create_vm_focus_idx == 1 {
                                    self.new_vm_vcpus = self.new_vm_vcpus.saturating_sub(1).max(1);
                                } else if self.create_vm_focus_idx == 2 {
                                    self.new_vm_memory_mb = self.new_vm_memory_mb.saturating_sub(128).max(128);
                                }
                            }
                            '\u{08}' => { // Backspace
                                if self.create_vm_focus_idx == 0 {
                                    self.new_vm_name.pop();
                                }
                            }
                            '\t' => {
                                self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5;
                            }
                            '\r' | '\n' => {
                                if self.create_vm_focus_idx == 3 {
                                    // Trigger Create VM (will be handled in main.rs)
                                    // We set a flag or just wait for main.rs to poll
                                } else if self.create_vm_focus_idx == 4 {
                                    self.selected_tab = DashboardTab::VirtualMachines;
                                } else {
                                    self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5;
                                }
                            }
                            'q' => {
                                self.selected_tab = DashboardTab::VirtualMachines;
                            }
                            _ => {
                                if self.create_vm_focus_idx == 0 && (ch.is_alphanumeric() || ch == '_') {
                                    self.new_vm_name.push(ch);
                                }
                            }
                        }
                    }
                    Key::Special(ScanCode::ESCAPE) => {
                        self.selected_tab = DashboardTab::VirtualMachines;
                    }
                    _ => {}
                }
                return;
            }
            DashboardTab::Storage => {
                match key {
                    Key::Special(ScanCode::LEFT) => {
                        if self.filesys_action_idx >= 1 {self.filesys_action_idx -= 1 } else { self.filesys_action_idx = 0 }
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        if self.filesys_action_idx < 7 {self.filesys_action_idx += 1 } else { self.filesys_action_idx = 7 }
                    }
                    Key::Special(ScanCode::ESCAPE) => {
                        self.filesys_pending_action = None;
                        self.status_line = String::from("File operation canceled");
                    }
                    Key::Special(ScanCode::END) => {
                        if self.files.is_empty() {
                            if self.filesys_action_idx == 2 {
                                let new_file = format!("{}{}new_file_{}.txt", self.current_path, if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" }, self.filesys_new_counter);
                                match crate::FileSystem::touch(&new_file) {
                                    Ok(_) => {
                                        self.filesys_new_counter += 1;
                                        self.status_line = format!("Created {}", new_file);
                                        self.refresh_storage();
                                    }
                                    Err(e) => self.status_line = format!("Create failed: {}", e),
                                }
                            } else if self.filesys_action_idx == 3 {
                                let new_dir = format!("{}{}new_folder_{}", self.current_path, if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" }, self.filesys_new_counter);
                                match crate::FileSystem::mkdir(&new_dir) {
                                    Ok(_) => {
                                        self.filesys_new_counter += 1;
                                        self.status_line = format!("Created {}", new_dir);
                                        self.refresh_storage();
                                    }
                                    Err(e) => self.status_line = format!("Create folder failed: {}", e),
                                }
                            }
                            return;
                        }
                        let entry = self.files[self.selected_file_idx].clone();
                        let sep = if self.current_path.ends_with('\\') || self.current_path.ends_with('/') { "" } else { "\\" };
                        let full_path = format!("{}{}{}", self.current_path, sep, entry.name);

                        if let Some(action) = self.filesys_pending_action {
                            let result = match action {
                                FilePendingAction::Rename => {
                                    let dst = format!("{}{}renamed_{}", self.current_path, sep, entry.name);
                                    crate::FileSystem::move_file(&full_path, &dst)
                                }
                                FilePendingAction::Copy => {
                                    let dst = format!("{}{}{}_copy", self.current_path, sep, entry.name);
                                    if entry.is_dir {
                                        crate::FileSystem::clone_dir(&full_path, &dst)
                                    } else {
                                        crate::FileSystem::copy(&full_path, &dst)
                                    }
                                }
                                FilePendingAction::Move => {
                                    let dst = format!("{}{}{}_moved", self.current_path, sep, entry.name);
                                    crate::FileSystem::move_file(&full_path, &dst)
                                }
                                FilePendingAction::Delete => crate::FileSystem::remove(&full_path),
                            };

                            match result {
                                Ok(_) => {
                                    self.status_line = format!("{:?} complete for {}", action, entry.name);
                                    self.filesys_pending_action = None;
                                    self.refresh_storage();
                                }
                                Err(e) => {
                                    self.status_line = format!("{:?} failed: {}", action, e);
                                    self.filesys_pending_action = None;
                                }
                            }
                            return;
                        }

                        match self.filesys_action_idx {
                            0 => {
                                if entry.is_dir {
                                    if entry.name == "." {
                                        return;
                                    } else if entry.name == ".." {
                                        if let Some(pos) = self.current_path.rfind('\\') {
                                            if pos == 0 {
                                                self.current_path = String::from("\\");
                                            } else {
                                                self.current_path.truncate(pos);
                                            }
                                        }
                                        self.refresh_storage();
                                        return;
                                    } else {
                                        if !self.current_path.ends_with('\\') {
                                            self.current_path.push('\\');
                                        }
                                        self.current_path.push_str(&entry.name);
                                        self.selected_file_idx = 0;
                                        self.refresh_storage();
                                        return;
                                    }
                                }
                                if (entry.name == "PAGEFILE") || (entry.name == "BOOTX64.EFI") {
                                    self.ui_error(25);
                                } else {
                                    match crate::FileSystem::read_file(&full_path) {
                                        Ok(data) => {
                                            let is_hex = core::str::from_utf8(&data).is_err();

                                            self.editor = Some(TextEditor {
                                                file_path: full_path,
                                                buffer: data,
                                                cursor_pos: (0, 0),
                                                scroll_offset: 0,
                                                mode: EditorMode::Normal,
                                                is_hex,
                                                command_buffer: "".to_string(),
                                            });
                                            self.selected_tab = DashboardTab::Editor;
                                        }
                                        Err(_) => self.ui_error(29),
                                    }
                                }
                            }
                            1 => {
                                self.status_line = format!("{}: {} bytes, {}", entry.name, entry.size, if entry.is_dir { "directory" } else { "file" });
                            }
                            2 => {
                                let new_file = format!("{}{}new_file_{}.txt", self.current_path, sep, self.filesys_new_counter);
                                match crate::FileSystem::touch(&new_file) {
                                    Ok(_) => {
                                        self.filesys_new_counter += 1;
                                        self.status_line = format!("Created {}", new_file);
                                        self.refresh_storage();
                                    }
                                    Err(e) => self.status_line = format!("Create failed: {}", e),
                                }
                            }
                            3 => {
                                let new_dir = format!("{}{}new_folder_{}", self.current_path, sep, self.filesys_new_counter);
                                match crate::FileSystem::mkdir(&new_dir) {
                                    Ok(_) => {
                                        self.filesys_new_counter += 1;
                                        self.status_line = format!("Created {}", new_dir);
                                        self.refresh_storage();
                                    }
                                    Err(e) => self.status_line = format!("Create folder failed: {}", e),
                                }
                            }
                            4 => {
                                self.filesys_pending_action = Some(FilePendingAction::Rename);
                                self.status_line = format!("Confirm rename of {}", entry.name);
                            }
                            5 => {
                                self.filesys_pending_action = Some(FilePendingAction::Copy);
                                self.status_line = format!("Confirm copy of {}", entry.name);
                            }
                            6 => {
                                self.filesys_pending_action = Some(FilePendingAction::Move);
                                self.status_line = format!("Confirm move of {}", entry.name);
                            }
                            7 => {
                                self.filesys_pending_action = Some(FilePendingAction::Delete);
                                self.status_line = format!("Confirm delete of {}", entry.name);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            DashboardTab::Network => {
                match key {
                    Key::Printable(c) => {
                        match char::from(c) {
                            '\r' | '\n' => self.execute_network_action(),
                            '+' | '=' => self.network_target = String::from("192.168.1.1"),
                            '-' | '_' => self.network_target = String::from("127.0.0.1"),
                            _ => {}
                        }
                    }
                    Key::Special(ScanCode::LEFT) => {
                        self.selected_network_action_idx = self.selected_network_action_idx.saturating_sub(1);
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        self.selected_network_action_idx = (self.selected_network_action_idx + 1).min(5);
                    }
                    _ => {}
                }
            }
            DashboardTab::Packages => {
                match key {
                    Key::Printable(c) => {
                        if matches!(char::from(c), '\r' | '\n') {
                            self.execute_package_action();
                        }
                    }
                    Key::Special(ScanCode::UP) => {
                        self.selected_package_idx = self.selected_package_idx.saturating_sub(1);
                    }
                    Key::Special(ScanCode::DOWN) => {
                        let len = self.package_manager.registry.len();
                        if len > 0 {
                            self.selected_package_idx = (self.selected_package_idx + 1).min(len - 1);
                        }
                    }
                    Key::Special(ScanCode::LEFT) => {
                        self.package_action_idx = self.package_action_idx.saturating_sub(1);
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        self.package_action_idx = (self.package_action_idx + 1).min(5);
                    }
                    _ => {}
                }
            }
            DashboardTab::Settings => {
                match key {
                    Key::Printable(c) => {
                        if matches!(char::from(c), '\r' | '\n') {
                            self.toggle_selected_setting();
                        }
                    }
                    Key::Special(ScanCode::LEFT) => {
                        self.selected_settings_category_idx = self.selected_settings_category_idx.saturating_sub(1);
                        self.selected_settings_idx = 0;
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        self.selected_settings_category_idx = (self.selected_settings_category_idx + 1).min(8);
                        self.selected_settings_idx = 0;
                    }
                    Key::Special(ScanCode::UP) => {
                        self.selected_settings_idx = self.selected_settings_idx.saturating_sub(1);
                    }
                    Key::Special(ScanCode::DOWN) => {
                        let rows_count = self.settings_rows().len();
                        self.selected_settings_idx = (self.selected_settings_idx + 1).min(rows_count.saturating_sub(1));
                    }
                    _ => {}
                }
            }
            DashboardTab::Editor => {
                let ed = match self.editor.as_mut() {
                    Some(ed) => ed,
                    None => {
                        self.ui_error(30);
                        self.selected_tab = DashboardTab::Storage;
                        return;
                    },
                };

                match ed.mode {
                    EditorMode::Normal => match key {
                        Key::Printable(c) => match char::from(c) {
                            'i' => ed.mode = EditorMode::Insert,
                            ':' => {
                                ed.mode = EditorMode::Command;
                                ed.command_buffer.clear();
                            }
                            'j' => ed.scroll_offset += 1,
                            'k' => ed.scroll_offset = ed.scroll_offset.saturating_sub(1),
                            _ => {}
                        },
                        _ => {}
                    },
                    EditorMode::Insert => match key {
                        Key::Special(ScanCode::ESCAPE) => ed.mode = EditorMode::Normal,
                        Key::Printable(char16) => {
                            match char::from(char16) {
                                '\u{8}'=> {
                                ed.buffer.pop(); // Simple end-of-file backspace for now
                                }
                                _ => {
                                    let c: char = char16.into();
                                    if c.is_ascii() {
                                        ed.buffer.push(c as u8);
                                    } else {
                                        // Optional: Handle non-ASCII (e.g., push UTF-8 bytes)
                                        let mut b = [0; 4];
                                        for &byte in c.encode_utf8(&mut b).as_bytes() {
                                            ed.buffer.push(byte);
                                        }
                                    }
                                }
                            }

                        }
                        _ => {}
                    },
                    EditorMode::Command => match key {
                        Key::Special(ScanCode::ESCAPE) => ed.mode = EditorMode::Normal,
                        Key::Printable(c) => {
                            let ch = char::from(c);
                            if ch == '\r' || ch == '\n' {
                                match ed.command_buffer.as_str() {
                                    "w" => {
                                        let failed = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w').is_err();
                                        ed.mode = EditorMode::Normal;
                                        if failed {
                                            self.ui_error(16);
                                        }
                                    }
                                    "q" => self.selected_tab = DashboardTab::Storage,
                                    "wq" => {
                                        let failed = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w').is_err();
                                        if failed {
                                            ed.mode = EditorMode::Normal;
                                            self.ui_error(16);
                                        } else {
                                            self.selected_tab = DashboardTab::Storage;
                                        }
                                    }
                                    _ => {
                                        ed.mode = EditorMode::Normal;
                                        self.ui_error(1);

                                    },
                                }
                            } else {
                                ed.command_buffer.push(ch);
                            }
                        }
                        _ => {}
                    }
                }
            }
            DashboardTab::Console => {
                match key {
                    Key::Printable(c) => {
                        if self.term_selected {
                            let ch = char::from(c);
                            match ch {
                                '\u{8}' => { self.term_buf.pop(); }
                                '\r' | '\n' => {
                                    let unclean = self.term_buf.trim();

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
                                    if !command.is_empty() {
                                        let command_parts = command.split(" ").collect::<Vec<&str>>();
                                        let parts = command_parts.clone();

                                        terminal::cmd(command_parts, &parts, body, &mut self.package_manager);
                                        self.command_history.push(command);
                                        self.history_idx = None;
                                    } else {
                                        hpvm_warn!("dashboard", "empty command");
                                    }
                                    self.term_buf.clear();
                                }
                                _ => { self.term_buf.push(ch) }
                            }
                        }
                    }

                    Key::Special(ScanCode::ESCAPE) => {
                        self.term_selected = false;
                        if matches!(self.selected_tab, DashboardTab::Apps) {
                            self.active_apps.clear();
                            self.focused_process_idx = None;
                        }
                    }
                    Key::Special(ScanCode::UP) => {
                        if self.term_selected && !self.command_history.is_empty() {
                            let new_idx = match self.history_idx {
                                Some(idx) => idx.saturating_sub(1),
                                None => self.command_history.len().saturating_sub(1),
                            };
                            self.history_idx = Some(new_idx);
                            self.term_buf = self.command_history[new_idx].clone();
                        }
                    }
                    Key::Special(ScanCode::DOWN) => {
                        if self.term_selected {
                            if let Some(idx) = self.history_idx {
                                if idx + 1 < self.command_history.len() {
                                    let new_idx = idx + 1;
                                    self.history_idx = Some(new_idx);
                                    self.term_buf = self.command_history[new_idx].clone();
                                } else {
                                    self.history_idx = None;
                                    self.term_buf.clear();
                                }
                            }
                        }
                    }
                    Key::Special(ScanCode::END) => {
                        self.term_selected = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                if !(matches!(self.selected_tab, DashboardTab::Editor)) && !self.term_selected {
                    match ch {
                        'q' => {
                            self.ui_error(22);
                        }
                        'o' => self.selected_tab = DashboardTab::Overview,
                        'v' => self.selected_tab = DashboardTab::VirtualMachines,
                        'r' => self.selected_tab = DashboardTab::Resources,
                        's' => self.selected_tab = DashboardTab::Storage,
                        'n' => self.selected_tab = DashboardTab::Network,
                        'd' => self.selected_tab = DashboardTab::Devices,
                        'c' => self.selected_tab = DashboardTab::Console,
                        't' => self.selected_tab = DashboardTab::Test,
                        'z' => self.selected_tab = DashboardTab::Settings,
                        'p' => self.selected_tab = DashboardTab::Packages,
                        'a' => self.selected_tab = DashboardTab::Apps,
                        ' ' => {
                            if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                                self.selected_tab = DashboardTab::CreateVM;
                            }
                        }
                        '\r' | '\n' => {
                            if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                                // Action execution handled in main.rs
                            } else if matches!(self.selected_tab, DashboardTab::Storage) {
                                if self.selected_file_idx < self.files.len() {
                                    let entry = &self.files[self.selected_file_idx];
                                    if entry.is_dir {
                                        if entry.name == "." {
                                            // Do nothing
                                        } else if entry.name == ".." {
                                            // Go up
                                            if let Some(pos) = self.current_path.rfind('\\') {
                                                if pos == 0 {
                                                    self.current_path = String::from("\\");
                                                } else {
                                                    self.current_path.truncate(pos);
                                                }
                                            }
                                        } else {
                                            // Go down
                                            if !self.current_path.ends_with('\\') {
                                                self.current_path.push('\\');
                                            }
                                            self.current_path.push_str(&entry.name);
                                        }
                                        self.selected_file_idx = 0;
                                        self.refresh_storage();
                                    }
                                }
                            } else if matches!(self.selected_tab, DashboardTab::Apps) {
                                let (name, _, _, _) = crate::apps::APP_REGISTRY[self.selected_app_idx];
                                if let Some(app_ctx) = SteppedApplicationContext::from_name(name) {
                                    self.active_apps.push(app_ctx);
                                    self.focused_process_idx = Some(self.active_apps.len() - 1);
                                }
                            } else if matches!(self.selected_tab, DashboardTab::Devices) {
                                // Toggle expansion
                                let mut current_idx = 0;
                                let mut found = false;
                                for i in 0..self.categories.len() {
                                    if current_idx == self.selected_device_idx {
                                        self.categories[i].expanded = !self.categories[i].expanded;
                                        break;
                                    }
                                    current_idx += 1;
                                    if self.categories[i].expanded {
                                        for _ in &self.categories[i].devices {
                                            if current_idx == self.selected_device_idx {
                                                // Can't toggle a device, only categories
                                                found = true;
                                                break;
                                            }
                                            current_idx += 1;
                                        }
                                    }
                                    if found { break; }
                                }
                            }
                        }
                        // 'i' => self.cursor.y -= if (self.cursor.y > 10) {10} else { 0 },
                        // 'k' => self.cursor.y += 10,
                        // 'j' => self.cursor.x -= if (self.cursor.x > 10) {10} else { 0 },
                        // 'l' => self.cursor.x += 10,
                        // 'u' => self.cursor.left_button = true,
                        '\t' => {
                            self.selected_tab = match self.selected_tab {
                                DashboardTab::Overview => DashboardTab::VirtualMachines,
                                DashboardTab::VirtualMachines => DashboardTab::Resources,
                                DashboardTab::Resources => DashboardTab::Storage,
                                DashboardTab::Storage => DashboardTab::Network,
                                DashboardTab::Network => DashboardTab::Devices,
                                DashboardTab::Devices => DashboardTab::Console,
                                DashboardTab::Console => DashboardTab::Test,
                                DashboardTab::Test => DashboardTab::Settings,
                                DashboardTab::CreateVM => DashboardTab::VirtualMachines,
                                DashboardTab::Editor => DashboardTab::Storage,
                                DashboardTab::Settings => DashboardTab::Packages,
                                DashboardTab::Packages => DashboardTab::Apps,
                                DashboardTab::Apps => DashboardTab::Overview
                            };
                        }
                        '/' => {
                            match self.selected_tab {
                                DashboardTab::Overview => unsafe {
                                    crate::state::SAVE(Some(self));
                                }
                                _ => {}
                            }
                        }
                        '1' => { self.cursor.x -= 4; }
                        '2' => { self.cursor.y += 4; }
                        '3' => { self.cursor.y -= 4; }
                        '4' => { self.cursor.x += 4; }
                        'x' => {
                            runtime::reset(ResetType::SHUTDOWN, Status::SUCCESS, Some(&[0]))
                        }
                        'q' => {
                            self.exit_requested = true
                        }
                        'k' => {
                            self.command_palette_active = true
                        }


                        _ => {}
                    }

                }
                // if (self.cursor.left_button) {
                //     self.cursor.left_button = false;
                // }
            }
            Key::Special(ScanCode::UP) => {
                if matches!(self.selected_tab, DashboardTab::Storage) {
                    if self.selected_file_idx > 0 {
                        self.selected_file_idx -= 1;
                    }
                } else if matches!(self.selected_tab, DashboardTab::Apps) {
                    let card_w = 200usize;
                    let gutter = 12usize;
                    let margin = 16usize;
                    let (width, _) = if let Some(pg) = PixelGraphics::new() { pg.resolution() } else { (1024, 768) };
                    let cols = (width - margin * 2) / (card_w + gutter);
                    let cols = if cols == 0 { 1 } else { cols };
                    if self.selected_app_idx >= cols {
                        self.selected_app_idx -= cols;
                    }
                } else if matches!(self.selected_tab, DashboardTab::Devices) {
                    if self.selected_device_idx > 0 {
                        self.selected_device_idx -= 1;
                    }
                } else if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                    if self.selected_vm_idx > 0 {
                        self.selected_vm_idx -= 1;
                    }
                } else {
                    if self.scroll_offset > 0 {
                        self.scroll_offset -= 1;
                    }
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if matches!(self.selected_tab, DashboardTab::Storage) {
                    if self.selected_file_idx < self.files.len().saturating_sub(1) {
                        self.selected_file_idx += 1;
                    }
                } else if matches!(self.selected_tab, DashboardTab::Apps) {
                    let card_w = 200usize;
                    let gutter = 12usize;
                    let margin = 16usize;
                    let (width, _) = if let Some(pg) = PixelGraphics::new() { pg.resolution() } else { (1024, 768) };
                    let cols = (width - margin * 2) / (card_w + gutter);
                    let cols = if cols == 0 { 1 } else { cols };
                    if self.selected_app_idx + cols < crate::apps::APP_REGISTRY.len() {
                        self.selected_app_idx += cols;
                    }
                } else if matches!(self.selected_tab, DashboardTab::Devices) {
                    let mut total_rows = 0;
                    for cat in &self.categories {
                        total_rows += 1;
                        if cat.expanded {
                            total_rows += cat.devices.len();
                        }
                    }
                    if self.selected_device_idx < total_rows.saturating_sub(1) {
                        self.selected_device_idx += 1;
                    }
                } else if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                    if self.selected_vm_idx < self.vms.len().saturating_sub(1) {
                        self.selected_vm_idx += 1;
                    }
                } else {
                    if self.scroll_offset < self.vms.len().saturating_sub(1) {
                        self.scroll_offset += 1;
                    }
                }
            }
            Key::Special(ScanCode::LEFT) => {
                if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                    self.vm_action_idx = self.vm_action_idx.saturating_sub(1);
                } else if matches!(self.selected_tab, DashboardTab::Apps) {
                    if self.selected_app_idx > 0 {
                        self.selected_app_idx -= 1;
                    }
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                    self.vm_action_idx = (self.vm_action_idx + 1).min(6);
                } else if matches!(self.selected_tab, DashboardTab::Apps) {
                    if self.selected_app_idx + 1 < crate::apps::APP_REGISTRY.len() {
                        self.selected_app_idx += 1;
                    }
                }
            }

            Key::Special(ScanCode::FUNCTION_2) => {
                self.ctrl_mode = !self.ctrl_mode;
            }
            Key::Special(ScanCode::FUNCTION_3) => {
                self.alt_mode = !self.alt_mode;
            }
            Key::Special(ScanCode::FUNCTION_4) => {
                self.fn_mode = !self.fn_mode;
            }
            _ => {}
        }
        
        // Handle mouse clicks for tab switching
        if self.cursor.left_button {
            if self.cursor.y >= 48 && self.cursor.y <= 80 {
                // Crude tab switching based on X position
                let x = self.cursor.x;
                if x < 100 { self.selected_tab = DashboardTab::Overview; }
                else if x < 180 { self.selected_tab = DashboardTab::VirtualMachines; }
                else if x < 280 { self.selected_tab = DashboardTab::Resources; }
                else if x < 380 { self.selected_tab = DashboardTab::Storage; }
                else if x < 480 { self.selected_tab = DashboardTab::Network; }
                else if x < 580 { self.selected_tab = DashboardTab::Devices; }
                else if x < 680 { self.selected_tab = DashboardTab::Console; }
                else if x < 780 { self.selected_tab = DashboardTab::Test; }
                else if x < 880 { self.selected_tab = DashboardTab::Settings; }
                else if x < 980 { self.selected_tab = DashboardTab::Packages; }
                else if x < 1080 { self.selected_tab = DashboardTab::Apps; }
            }
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub fn ui_error(&mut self, typ: usize) {
        self.ui_error_with_detail(typ, None);
    }

    pub fn ui_error_with_detail(&mut self, typ: usize, detail: Option<&str>) {
        let types: &[(&str, &str)] = &[
            ("Generic", "Generic Error"),
            ("Invalid", ""),
            ("AccessDenied", ""),
            ("NotFound", ""),
            ("OutOfMemBounds", ""),
            ("Overflow", ""),
            ("SegFault", ""),
            ("Lookup", ""),
            ("RuntimeProblem", ""),
            ("OutOfMemory", ""),
            ("PathNotFound", ""),
            ("BadEnvironment", ""),
            ("WriteProtect", ""),
            ("BadCommand", ""),
            ("CRC", ""),
            ("DiskReadFault", ""),
            ("DiskWriteFault", ""),
            ("NetTxFault", ""),
            ("NetRxFault", ""),
            ("DeviceBusy", ""),
            ("BadSector", ""),
            ("BadDevice", ""),
            ("WrongOperation", "The operation was incorrect"),
            ("CpuFault", ""),
            ("InvalidFormat", ""),
            ("SystemFile", "The file is of the system"),
            ("ActiveFile", "The file is in use"),
            ("StorageUnavailable", "The filesystem protocol is unavailable"),
            ("DirectoryOpen", "The directory could not be opened"),
            ("FileReadFault", "The file could not be read"),
            ("EditorUnavailable", "The editor has no active file"),
        ];

        let (name, default_detail) = types.get(typ).copied().unwrap_or(types[0]);
        let detail = detail.unwrap_or(default_detail);
        hpvm_error!("UI", "ERROR {}: {}", typ, name);

        let error = if detail.is_empty() {
            format!("Error {}: {}", typ, name)
        } else {
            format!("Error {}: {}\n {}", typ, name, detail)
        };
        let app = ErrorApp { error };
        let dims = crate::env::AppInfo::dimensions(&app);
        let mut app = Application::new(Box::new(app));
        app.name = format!("ERROR {}", typ);
        app.dimensions = dims;

        self.active_apps.push(SteppedApplicationContext::new(app, None));
        self.focused_process_idx = Some(self.active_apps.len() - 1);
    }
}
