#![allow(dead_code, deprecated)]

use alloc::collections::BTreeMap;
use crate::hpvm_log;
use alloc::fmt::format;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::char;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::runtime;
use uefi::runtime::VariableKey;
use uefi_raw::Status;
use uefi_raw::table::runtime::ResetType;

mod graphics;
pub mod pixel_graphics;


use crate::{handle_vm_command, hpvm_warn, message, terminal};
use crate::pm::{Package, PackageManager, PackageType};
use pixel_graphics::{PixelGraphics, TreeViewNode};




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
    exit_requested: bool,

    // Fields for Create VM UI
    pub new_vm_name: String,
    pub new_vm_memory_mb: u32,
    pub new_vm_vcpus: u32,
    pub create_vm_focus_idx: usize, 
    pub vm_action_idx: usize, // For VM actions (0: Start, 1: Stop, 2: Reset, 3: Zero, 4: Delete)
    pub selected_vm_idx: usize,
    pub filesys_action_idx: usize,
    pub term_selected: bool,
    pub term_buf: String,
    pub editor: Option<TextEditor>,
    pub package_manager: PackageManager,
    pub iter: u64,

}

#[derive(Clone, Copy, Debug)]
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
}

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
}

impl DashboardUI {
    pub fn new(package_manager: PackageManager) -> Self {
        Self {
            selected_tab: DashboardTab::Overview,
            vms: alloc::vec::Vec::new(),
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
            },
            scroll_offset: 0,
            cursor: crate::graphics::Cursor::new(),
            current_path: String::from("\\"),
            files: Vec::new(),
            selected_file_idx: 0,
            categories: Vec::new(),
            selected_device_idx: 0,
            exit_requested: false,
            new_vm_name: String::from("NewVM"),
            new_vm_memory_mb: 256,
            new_vm_vcpus: 1,
            create_vm_focus_idx: 0,
            vm_action_idx: 0,
            selected_vm_idx: 0,
            filesys_action_idx: 0,
            term_selected: false,
            term_buf: "".to_string(),
            editor: None,
            package_manager: package_manager,
            iter: 0,
        }
    }

    pub fn add_vm(&mut self, vm: VmDisplayInfo) {
        self.vms.push(vm);
    }

    pub fn set_resources(&mut self, resources: SystemResources) {
        let old_cpu_hist = self.resources.cpu_history.clone();
        let old_mem_hist = self.resources.mem_history.clone();
        let old_disk_read_hist = self.resources.disk_read_history.clone();
        let old_disk_write_hist = self.resources.disk_write_history.clone();
        let old_net_rx_hist = self.resources.net_rx_history.clone();
        let old_net_tx_hist = self.resources.net_tx_history.clone();
        let old_gpu_hist = self.resources.gpu_history.clone();

        self.resources = resources;

        // Restore and update histories
        self.resources.cpu_history = old_cpu_hist;
        self.resources.mem_history = old_mem_hist;
        self.resources.disk_read_history = old_disk_read_hist;
        self.resources.disk_write_history = old_disk_write_hist;
        self.resources.net_rx_history = old_net_rx_hist;
        self.resources.net_tx_history = old_net_tx_hist;
        self.resources.gpu_history = old_gpu_hist;

        fn push_limit<T>(vec: &mut Vec<T>, val: T, limit: usize) {
            if vec.len() >= limit {
                vec.remove(0);
            }
            vec.push(val);
        }

        push_limit(&mut self.resources.cpu_history, self.resources.cpu_usage, 100);
        let mem_percent = if self.resources.total_memory_mb > 0 {
            (self.resources.used_memory_mb * 100 / self.resources.total_memory_mb) as u32
        } else { 0 };
        push_limit(&mut self.resources.mem_history, mem_percent, 100);
        push_limit(&mut self.resources.disk_read_history, self.resources.disk_read_kbps, 100);
        push_limit(&mut self.resources.disk_write_history, self.resources.disk_write_kbps, 100);
        push_limit(&mut self.resources.net_rx_history, self.resources.net_rx_kbps, 100);
        push_limit(&mut self.resources.net_tx_history, self.resources.net_tx_kbps, 100);
        push_limit(&mut self.resources.gpu_history, self.resources.gpu_usage, 100);
    }

    pub fn draw(&mut self) {
        if let Some(pg) = pixel_graphics::PixelGraphics::new() {
            self.iter += 1;
            let mut pg = pg.with_backbuffer();
            let (width, height) = pg.resolution();
            
            // Draw background
            pg.clear(0x222222);

            // Draw header
            pg.fill_rect(0, 0, width, 48, 0x008080); // Cyan-ish
            pg.draw_text(width / 2 - 160, 16, "HPVMx - Hypervisor Management Console", 0xFFFFFF);

            // Draw clock in top right
            if let Ok(time) = uefi::runtime::get_time() {
                let time_str = alloc::format!("{:02}:{:02}:{:02}", time.hour(), time.minute(), time.second());
                pg.draw_text(width - 100, 16, &time_str, 0xFFFF00); // Yellow clock
            }

            pg.draw_text(40, 1, "   __ _____ _   ____  ___", 0xFFFFFF);
            pg.draw_text(40, 11, "  / // / _ \\ | / /  |/  /_ __", 0xFFFFFF);
            pg.draw_text(40, 21, " / _  / ___/ |/ / /|_/ /\\ \\ /", 0xFFFFFF);
            pg.draw_text(40, 31, "/_//_/_/   |___/_/  /_//_\\_\\", 0xFFFFFF);

            // Draw navigation
            pg.fill_rect(0, 48, width, 32, 0x444444); // Dark Gray
            let nav_text = "O Overview | V VMs | R Resources | S Storage | N Network | D Devices | C Console | T Test | Z Settings | P Packages";
            pg.draw_text(10, 56, nav_text, 0xFFFFFF);
            let page_y = 100;

            // Layout constants for consistent spacing across tabs
            let header_h = 48usize;
            let nav_h = 32usize;
            let content_top = header_h + nav_h; // 80px fro5m top
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
                    pg.draw_text(table_x + 8, table_y + 4, "ID   NAME             STATE        CPU  MEM", 0xCCCCCC);

                    // Rows
                    let mut y = table_y + line_h + gutter;
                    for (idx, vm) in self.vms.iter().enumerate() {
                        if y + line_h > table_y + table_h - 2 { break; }
                        let is_selected = idx == self.selected_vm_idx;
                        let text_color = if is_selected { 0xFFFF00 } else { 0xFFFFFF };
                        if is_selected {
                            pg.fill_rect(table_x + 2, y - 2, table_w - 4, line_h, 0x444400);
                        }
                        let info = alloc::format!("{:<4} {:<16} {:<12} {:>3}% {:>5}MB",
                            vm.id, vm.name, vm.state, vm.cpu_usage, vm.memory_usage_mb);
                        pg.draw_text(table_x + 8, y, &info, text_color);
                        y += line_h;
                    }

                    // VM Actions Bar
                    if !self.vms.is_empty() {
                        let actions_y = table_y + table_h + gutter;
                        pg.draw_text(margin, actions_y, "Actions for Selected VM:", 0xCCCCCC);
                        let actions = ["Start", "Stop", "Reset", "Zero", "Delete"];
                        let mut action_x = margin;
                        let action_y = actions_y + 20;
                        for (idx, action) in actions.iter().enumerate() {
                            let is_focused = idx == self.vm_action_idx;
                            let color = if is_focused { 0x00AA00 } else { 0x444444 };
                            pg.fill_rect(action_x, action_y, 70, 24, color);
                            pg.draw_text(action_x + 8, action_y + 4, action, 0xFFFFFF);
                            action_x += 80;
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
                    pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &self.resources.mem_history, 100, 0x00FF00, 30);

                    // I/O Stats and Graphs
                    let io_y = bar_y + 80 + gutter * 2;
                    pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, io_y + 20, 165, 50, &self.resources.net_rx_history, 1024, 0x00FFFF, 30);
                    pg.draw_line_graph(panel_x + 185, io_y + 20, 165, 50, &self.resources.net_tx_history, 1024, 0xFFFF00, 30);
                    
                    let disk_y = io_y + 80;
                    pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Red)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, disk_y + 20, 165, 50, &self.resources.disk_read_history, 1024, 0xFFFFFF, 30);
                    pg.draw_line_graph(panel_x + 185, disk_y + 20, 165, 50, &self.resources.disk_write_history, 1024, 0xFF0000, 30);

                    let gpu_y = disk_y + 80;
                    pg.draw_text(panel_x+ 10, gpu_y, "GPU Usage:", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, gpu_y+20, 165, 50, &self.resources.gpu_history, 100, 0xFF7700, 30);




                    // Right CPU core list panel or Total CPU Graph
                    let right_x = panel_x + panel_w + gutter * 2;
                    let right_y = panel_y;
                    let right_w = core::cmp::min(width - right_x - margin, 360);
                    let right_h = core::cmp::min(height - right_y - 100, 260);
                    pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x888888);
                    pg.draw_text_bg(right_x + 10, right_y - 4, "Total CPU Usage History:", 0xFFFFFF, 0x222222);
                    pg.draw_line_graph(right_x + 10, right_y + 10, right_w - 20, 80, &self.resources.cpu_history, 100, 0x00FF00, 30);
                    
                    pg.draw_text(right_x + 10, right_y + 100, "CPU Usage per Core:", 0xFFFFFF);
                    for i in 0..self.resources.cpu_count {
                        let row_y = right_y + 120 + (i as usize * (line_h + 4));
                        if row_y + line_h > right_y + right_h - 8 { break; }
                        let usage = if i < self.resources.cpu_core_usage.len() as u32 { self.resources.cpu_core_usage[i as usize] } else { 0 };
                        pg.draw_text(right_x + 10, row_y, &alloc::format!("C{}:{:>2}%", i, usage), 0xCCCCCC);
                        pg.draw_progress_bar(right_x + 70, row_y, right_w - 80, 12, usage as usize, 100, 0x00FF00);
                    }

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
                    pg.draw_text(x, y, &alloc::format!("Initialized: {is_init}", ), 0xFFFFFF)




                }
                DashboardTab::Console => {
                    pg.draw_text(20, 100, "System Log", 0x00FF00);
                    pg.draw_rect_outline(margin, 130, width - margin * 2, height - 135 - margin * 6, 0x888888);
                    
                    let mut y = 140;
                    let logs = crate::hpvmlog::get_logs();


                    let max_y = height - (margin * 6) - 20; // Bottom boundary
                    let available_height = max_y - 140;     // Space between start y (140) and boundary
                    let mut total_lines_needed = 0;
                    let mut start_idx = logs.len();

                    // Iterate backwards to find how many logs (and their newlines) actually fit
                    for i in (0..logs.len()).rev() {
                        let (_, tag, msg) = &logs[i];
                        // Count 1 line + any additional newlines in the message
                        let lines_in_this_log = if tag.is_empty() {
                            msg.split('\n').count()
                        } else {
                            // Tagged logs usually follow "[tag] msg" format
                            alloc::format!("[{}] {}", tag, msg).split('\n').count()
                        };

                        if (total_lines_needed + lines_in_this_log) * (line_h-4) > available_height {
                            break; // This log won't fit entirely
                        }

                        total_lines_needed += lines_in_this_log;
                        start_idx = i;
                    }


                    // // Show last N logs that fit on screen
                    // let max_visible = (height - 130 - (margin * 6) - 20) / line_h;
                    // let start_idx = logs.len().saturating_sub(max_visible);
                    
                    for i in start_idx..logs.len() {
                        let (color, tag, msg) = &logs[i];
                        let color_hex = match color {
                            uefi::proto::console::text::Color::Red => 0xFF0000,
                            uefi::proto::console::text::Color::Yellow => 0xFFFF00,
                            uefi::proto::console::text::Color::LightCyan => 0x00FFFF,
                            _ => 0xFFFFFF,
                        };
                        let log_line = if tag.is_empty() { msg.clone() } else { alloc::format!("[{}] {}", tag, msg) };
                        for section in log_line.split("\n") {
                            pg.draw_text(margin + 10, y, section, color_hex);
                            y += (line_h-4);
                        }
                        if y + (line_h-4) > height - margin * 6 { break; }
                    }

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
                                pg.draw_text(45, y, &alloc::format!(" {}: {}", dev.name, dev.path), color);
                                y += 20;
                                current_idx += 1;
                                if y > height - 60 { break; }
                            }
                        }
                        if y > height - 60 { break; }
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
                    let list_w = core::cmp::min(width - margin * 2, 760);
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
                    let mut idx_types: Vec<&str> = Vec::new();
                    for (i, entry) in self.files.iter().enumerate() {
                        if y + line_h > list_y + list_h - 2 { break; }
                        let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
                        idx_types.push(if entry.is_dir { "D" } else { "F" }  );
                        let icon = if entry.is_dir { pixel_graphics::icons::FOLDER_ICON_DATA } else {
                            let dec_syn = ["json", "xml", "toml", "yaml", "yml"];
                            let sys_syn = ["sys", "efi"];
                            let prog_syn = ["micro", "ufe", "dmx", "bin"];


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
                        } else if entry.size/1000 < 10000 {
                            format!("{}K", (entry.size/1000))
                        } else {
                            format!("{}M", (entry.size/1000)/1000)
                        };


                        let background = if i == self.selected_file_idx { 0x333333 } else { 0x222222 };
                        pg.draw_icon(list_x + 16, y, 16, 16, &icon);
                        pg.draw_text_bg(list_x + 56, y, &alloc::format!("{:<32}", entry.name), color, background);
                        pg.draw_text_bg(list_x + 348, y, &alloc::format!("{:>12}", size), 0xCCCCCC, background);
                        pg.draw_text_bg(list_x + 470, y, if entry.is_dir { "DIR" } else { "FILE" }, 0x6666FF, background);
                        y += line_h;
                    }

                    if idx_types[self.selected_file_idx] == "F" {
                        let actions_y = list_h + margin*8;
                        pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
                        let actions = ["Open", "Edit", "Props", "Delete", "Clone", "Copy", "Cut", "Copy Path"];
                        let mut action_x = margin;
                        let action_y = actions_y + 20;
                        for (idx, action) in actions.iter().enumerate() {
                            let is_focused = idx == self.filesys_action_idx;
                            let color = if is_focused { 0x00AA00 } else { 0x444444 };
                            pg.fill_rect(action_x, action_y, 90, 24, color);
                            pg.draw_text(action_x + 8, action_y + 4, action, 0xFFFFFF);
                            action_x += 100;
                        }
                    } else {
                        let actions_y = list_h + margin*8;
                        pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
                        let actions = ["Props", "Delete", "Clone", "Copy", "Cut", "Compress", "Copy Path", "New"];
                        let mut action_x = margin;
                        let action_y = actions_y + 20;
                        for (idx, action) in actions.iter().enumerate() {
                            let is_focused = idx == self.filesys_action_idx;
                            let color = if is_focused { 0x00AA00 } else { 0x444444 };
                            pg.fill_rect(action_x, action_y, 90, 24, color);
                            pg.draw_text(action_x + 8, action_y + 4, action, 0xFFFFFF);
                            action_x += 100;
                        }
                    }
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

                    pg.draw_text(5, page_y - 15, "this page is in development.... coming soon!", 0xFFFFFF);


                    // settings ideas
                    // .. general boot settings (for now)

                    pg.draw_rect_outline(10, page_y + 2, (width/3)-10, (height*2)/3, 0xFFFFFF);



                    // features section
                    //
                    // [] = checkbox   () = radiobutton
                    //
                    // [] extra debug info
                    // [] folder absolute sizes

                    pg.draw_rect_outline_adv((width/3) + 10, page_y + 2, (width/3) - 10, (height*2)/3, 0x777777, 1, 0x3FFFFF);
                    let mut x = (width/3) + 20;
                    let mut y = page_y + 10;


                    pg.draw_text(x, y, "Optional Features", 0xFFFFFF);
                    y += 25;

                    pg.draw_checkbox(x, y, false, false, false, "Extra Debug Info");
                    y += 20;
                    pg.draw_checkbox(x, y, false, false, true, "Folder Absolute Sizes");
                    y += 20;
                    pg.draw_checkbox(x, y, true, false, false, "State Save/Restore");
                    y += 20;
                    pg.draw_checkbox(x, y, true, false, false, "Extended Symbol Library");
                    y += 20;
                    pg.draw_checkbox(x, y, false, true, false, "Ring0 UDMI/UDXI");
                    y += 20;
                    pg.draw_checkbox(x, y, false, false, true, "ControlLang Support");
                    y += 20;
                    pg.draw_checkbox(x, y, true, false, false, "PG VShaders");





                }
                DashboardTab::Packages => {
                    pg.draw_text(20, 100, "Packages", 0x00FF00);

                    let grouped_packages: BTreeMap<PackageType, Vec<Package>> = self.package_manager.get_packages();

                    let mut category_children: BTreeMap<PackageType, Vec<TreeViewNode>> = BTreeMap::new();

                    for (p_type, pkgs) in &grouped_packages {
                        let nodes: Vec<TreeViewNode> = pkgs
                            .iter()
                            .map(|p| {
                            TreeViewNode {
                                label: &p.name, // Note: p.name must live as long as the tree
                                children: &[],
                                expanded: true,
                            }
                        })
                            .collect();

                        category_children
                            .insert(p_type.clone(), nodes);
                    }

                    // Now create the category-level nodes
                    let categories: Vec<TreeViewNode> = category_children
                        .iter()
                        .map(|(p_type, children)| {
                        TreeViewNode {
                            label: match p_type { // Map enum to display string
                                PackageType::Library => "Libraries",
                                PackageType::Executable => "Executables",
                                PackageType::Driver => "Drivers",
                                PackageType::Extension => "Extensions",
                                PackageType::PShader => "Shaders",
                                PackageType::ResourcePack => "ResourcePacks",
                                _ => "Other",
                            },
                            children: children, // Reference the Vec stored in category_children
                            expanded: true,
                        }
                    })
                        .collect();

                    let root = TreeViewNode {
                        label: "Packages",
                        children: &categories,
                        expanded: true,
                    };
                    pg.draw_tree_view_icon(60, 160, 285, 450, &root, &pixel_graphics::icons::PACKAGE_ICON_DATA);
                    pg.draw_table_view(360, 160, 200, 450,  &["property", "value"], &[&["none", "none"]]);
                    pg.draw_table_view(660, 160,  150, 400, &["property", "value"], &[&["none", "none"]]);
                    pg.draw_table_view(820, 160,  400, 500, &["col"], &[&["row"]]);
                    pg.draw_button(660, 580, 120,30,  "Install", false);
                    pg.draw_button(60, 640, 120, 30,  "Uninstall", false);
                    pg.draw_button(220, 640, 120,30,  "Update", false);
                    pg.draw_button(380, 640, 120, 30,  "Disable", false);
                    pg.draw_button(660, 640,  120, 30,  "____", false);
                    pg.draw_rect_outline(820, 680, 300, 25, 0xCCCCC0);
                    pg.draw_button(1140, 680, 100, 25, "Search", false);
                }
                _ => {
                    pg.draw_text(5, page_y - 15, "this page is unavailable", 0xFFFFFF)
                }
            }

            // Draw footer
            pg.fill_rect(0, height - 48, width, 48, 0x000080); // Blue
            pg.draw_text(10, height - 32, " Use keys O, V, R, S, N, D, C, T, Z to switch tabs | X to shutdown", 0xFFFFFF);

            // Update and draw cursor
            if self.iter % 20 == 0 {
                unsafe {
                    self.cursor.update_from_mouse(width, height);
                }
            }
            pg.draw_cursor(self.cursor.x as usize, self.cursor.y as usize);

            //pg.apply_scanlines();
            //pg.apply_dither();
            //pg.apply_glitch();
            //pg.apply_edge_aberration(0.5);


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
        let mut expanded_map = alloc::collections::BTreeMap::new();
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
        if !pcis.is_empty() {
            self.categories.push(DeviceCategory {
                name: String::from("PCI Devices"),
                devices: pcis,
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
            Err(_) => return,
        };
        let mut sfs = match uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle) {
            Ok(s) => s,
            Err(_) => return,
        };
        let mut root_dir = match sfs.open_volume() {
            Ok(d) => d,
            Err(_) => return,
        };

        let mut target_dir = if self.current_path == "\\" || self.current_path == "/" {
            root_dir
        } else {
            let mut u16_path: Vec<u16> = self.current_path.encode_utf16().collect();
            u16_path.push(0);
            let path_cstr = match uefi::data_types::CStr16::from_u16_with_nul(&u16_path) {
                Ok(c) => c,
                Err(_) => return,
            };

            let handle = match root_dir.open(path_cstr, FileMode::Read, FileAttribute::DIRECTORY) {
                Ok(h) => h,
                Err(_) => return,
            };

            match handle.into_directory() {
                Some(d) => d,
                None => return,
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

    pub fn handle_input(&mut self, key: Key) {
        use uefi::proto::console::text::ScanCode;

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
                    Key::Special(ScanCode::END) => {
                        if self.filesys_action_idx == 0 || self.filesys_action_idx == 1 { // Open/Edit
                            let entry = &self.files[self.selected_file_idx];
                            if !entry.is_dir {
                                let full_path = format!("{}/{}", self.current_path, entry.name);
                                if let Ok(data) = crate::FileSystem::read_file(&full_path) {
                                    // Detect if binary: check for null bytes or invalid UTF-8
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
                            }
                        }
                    }
                    _ => {}
                }
            }
            DashboardTab::Editor => {
                let ed = self.editor.as_mut().unwrap();

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
                                        let _ = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w');
                                        ed.mode = EditorMode::Normal;
                                    }
                                    "q" => self.selected_tab = DashboardTab::Storage,
                                    "wq" => {
                                        let _ = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w');
                                        self.selected_tab = DashboardTab::Storage;
                                    }
                                    _ => ed.mode = EditorMode::Normal,
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
                                    let body = command.split(" ").collect::<Vec<&str>>();
                                    if !command.is_empty() {
                                        let command = command.split(" ").collect::<Vec<&str>>();
                                        let parts = command.clone();

                                        terminal::cmd(command, &parts, body, &mut self.package_manager);
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
                        // 'q' => {
                        //     self.exit_requested = true;
                        //     return;
                        // }
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
                                DashboardTab::Packages => DashboardTab::Overview
                            };
                        }
                        '/' => {
                            match self.selected_tab {
                                DashboardTab::Overview => unsafe {
                                    let stat = crate::state::KernelState::new(0, 0x0);
                                    crate::state::SAVE();
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
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                if matches!(self.selected_tab, DashboardTab::VirtualMachines) {
                    self.vm_action_idx = (self.vm_action_idx + 1).min(4);
                }
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
            }
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }
}
