#![allow(dead_code)]

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Color, Key, ScanCode};

mod graphics;
pub mod pixel_graphics;
mod dashboard;
mod vm_manager;
mod terminal;

pub use graphics::{Graphics, Rect};
use crate::message;

pub struct Window {
    pub title: String,
    pub rect: Rect,
    pub active: bool,
}

impl Window {
    pub fn new(title: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Window {
            title: String::from(title),
            rect: Rect::new(x, y, width, height),
            active: true,
        }
    }

    pub fn draw(&self) {
        if let Some(pg) = pixel_graphics::PixelGraphics::new() {
            let mut pg = pg;
            pg.fill_rect(self.rect.x * 8, self.rect.y * 16, self.rect.width * 8, self.rect.height * 16, 0xCCCCCC);
            pg.draw_text(self.rect.x * 8 + 4, self.rect.y * 16 + 4, &self.title, 0x000000);
        } else {
            Graphics::draw_box(&self.rect, &self.title, self.active);
        }
    }
}

pub struct Button {
    pub label: String,
    pub rect: Rect,
    pub focused: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(label: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Button {
            label: String::from(label),
            rect: Rect::new(x, y, width, height),
            focused: false,
            clicked: false,
        }
    }

    pub fn draw(&self) {
        if let Some(pg) = pixel_graphics::PixelGraphics::new() {
            let mut pg = pg;
            let bg = if self.focused { 0xFFFFFF } else { 0xBBBBBB };
            pg.fill_rect(self.rect.x * 8, self.rect.y * 16, self.rect.width * 8, self.rect.height * 16, bg);
            pg.draw_text(self.rect.x * 8 + 10, self.rect.y * 16 + 8, &self.label, 0x000000);
        } else {
            Graphics::draw_button(&self.rect, &self.label, self.focused);
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.rect.contains(x, y)
    }
}

pub struct TextBox {
    pub text: String,
    pub rect: Rect,
    pub focused: bool,
    pub max_length: usize,
}

impl TextBox {
    pub fn new(x: usize, y: usize, width: usize, max_length: usize) -> Self {
        TextBox {
            text: String::new(),
            rect: Rect::new(x, y, width, 3),
            focused: false,
            max_length,
        }
    }

    pub fn draw(&self) {
        Graphics::draw_textbox(&self.rect, &self.text, self.focused);
    }

    pub fn add_char(&mut self, ch: char) {
        if self.text.len() < self.max_length {
            self.text.push(ch);
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();
    }
}

pub struct ListBox {
    pub items: Vec<String>,
    pub rect: Rect,
    pub selected: usize,
}

impl ListBox {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        ListBox {
            items: Vec::new(),
            rect: Rect::new(x, y, width, height),
            selected: 0,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn draw(&self) {
        let items_str: Vec<&str> = self.items.iter().map(|s| s.as_str()).collect();
        Graphics::draw_list(&self.rect, &items_str, self.selected);
    }

    pub fn select_next(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }
}

pub struct WinNTShell {
    pub windows: Vec<Window>,
    pub buttons: Vec<Button>,
    pub textboxes: Vec<TextBox>,
    pub listbox: Option<ListBox>,
    pub focused_button: usize,
    pub focused_textbox: usize,
    focus_target: FocusTarget,
    exit_requested: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FocusTarget {
    TextBox,
    Button,
    ListBox,
}

impl WinNTShell {
    pub fn new() -> Self {
        WinNTShell {
            windows: Vec::new(),
            buttons: Vec::new(),
            textboxes: Vec::new(),
            listbox: None,
            focused_button: 0,
            focused_textbox: 0,
            focus_target: FocusTarget::TextBox,
            exit_requested: false,
        }
    }

    pub fn init_desktop(&mut self) {
        // Clear screen with gray background
        Graphics::clear_screen(Color::LightGray);

        // Draw menu bar
        Graphics::draw_menu_bar(&["File", "Edit", "View", "Help"]);

        // Draw taskbar
        Graphics::draw_taskbar("12:00");

        // Create main shell window
        let shell_window = Window::new("HPVMx Shell", 5, 3, 70, 18);
        self.windows.push(shell_window);

        // Create OK and Cancel buttons
        let ok_button = Button::new("OK", 35, 20, 8, 2);
        let cancel_button = Button::new("Cancel", 45, 20, 10, 2);

        self.buttons.push(ok_button);
        self.buttons.push(cancel_button);

        // Create a command input textbox
        let input_box = TextBox::new(7, 5, 66, 50);
        self.textboxes.push(input_box);

        // Create output listbox
        let mut output_list = ListBox::new(7, 8, 66, 10);
        output_list.add_item("HPVMx v0.1.0 - Shell Interface");
        output_list.add_item("Type 'help' for available commands");
        self.listbox = Some(output_list);

        if !self.textboxes.is_empty() {
            self.focus_textbox(0);
        }
        self.exit_requested = false;
    }

    pub fn draw(&self) {
        for window in &self.windows {
            window.draw();
        }

        for button in &self.buttons {
            button.draw();
        }

        for textbox in &self.textboxes {
            textbox.draw();
        }

        if let Some(ref listbox) = self.listbox {
            listbox.draw();
        }
    }

    pub fn focus_button(&mut self, idx: usize) {
        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.focused = i == idx;
        }
        self.focused_button = idx;
        self.focus_target = FocusTarget::Button;
    }

    pub fn focus_textbox(&mut self, idx: usize) {
        for (i, textbox) in self.textboxes.iter_mut().enumerate() {
            textbox.focused = i == idx;
        }
        self.focused_textbox = idx;
        self.focus_target = FocusTarget::TextBox;
    }

    fn focus_listbox(&mut self) {
        self.focus_target = FocusTarget::ListBox;
    }

    fn cycle_focus(&mut self) {
        let has_textbox = !self.textboxes.is_empty();
        let has_button = !self.buttons.is_empty();
        let has_listbox = self.listbox.is_some();

        let mut next = self.focus_target;
        let mut attempts = 0;
        while attempts < 3 {
            next = match next {
                FocusTarget::TextBox => FocusTarget::Button,
                FocusTarget::Button => FocusTarget::ListBox,
                FocusTarget::ListBox => FocusTarget::TextBox,
            };
            if (next == FocusTarget::TextBox && has_textbox)
                || (next == FocusTarget::Button && has_button)
                || (next == FocusTarget::ListBox && has_listbox)
            {
                break;
            }
            attempts += 1;
        }

        match next {
            FocusTarget::TextBox => {
                let idx = self.focused_textbox.min(self.textboxes.len().saturating_sub(1));
                self.focus_textbox(idx);
            }
            FocusTarget::Button => {
                let idx = self.focused_button.min(self.buttons.len().saturating_sub(1));
                self.focus_button(idx);
            }
            FocusTarget::ListBox => self.focus_listbox(),
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch.to_ascii_lowercase() == 'q' {
                    self.exit_requested = true;
                    return;
                }
                if ch == '\t' {
                    self.cycle_focus();
                    return;
                }
                if self.focus_target == FocusTarget::TextBox && self.focused_textbox < self.textboxes.len() {
                    if ch != '\r' && ch != '\n' {
                        self.textboxes[self.focused_textbox].add_char(ch);
                    }
                }
            }
            Key::Special(ScanCode::DELETE) => {
                if self.focus_target == FocusTarget::TextBox && self.focused_textbox < self.textboxes.len() {
                    self.textboxes[self.focused_textbox].backspace();
                }
            }
            Key::Special(ScanCode::UP) => {
                if self.focus_target == FocusTarget::ListBox {
                    if let Some(ref mut listbox) = self.listbox {
                        listbox.select_prev();
                    }
                } else if self.focus_target == FocusTarget::Button && !self.textboxes.is_empty() {
                    self.focus_textbox(self.focused_textbox);
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.focus_target == FocusTarget::ListBox {
                    if let Some(ref mut listbox) = self.listbox {
                        listbox.select_next();
                    }
                } else if self.focus_target == FocusTarget::TextBox && self.listbox.is_some() {
                    self.focus_listbox();
                }
            }
            Key::Special(ScanCode::LEFT) => {
                if self.focus_target == FocusTarget::Button && !self.buttons.is_empty() {
                    let next_button = if self.focused_button == 0 {
                        self.buttons.len() - 1
                    } else {
                        self.focused_button - 1
                    };
                    self.focus_button(next_button);
                } else if self.focus_target == FocusTarget::ListBox && !self.buttons.is_empty() {
                    self.focus_button(self.focused_button);
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.focus_target == FocusTarget::Button && !self.buttons.is_empty() {
                    let next_button = (self.focused_button + 1) % self.buttons.len();
                    self.focus_button(next_button);
                } else if self.focus_target == FocusTarget::TextBox && !self.buttons.is_empty() {
                    self.focus_button(self.focused_button);
                }
            }
            Key::Special(ScanCode::END) => {
                self.cycle_focus();
            }
            Key::Special(ScanCode::HOME) => {
                if self.focus_target == FocusTarget::Button && self.focused_button < self.buttons.len() {
                    let label = self.buttons[self.focused_button].label.clone();
                    self.buttons[self.focused_button].clicked = true;
                    if label.eq_ignore_ascii_case("ok") {
                        if let Some(input) = self.get_input() {
                            if !input.is_empty() {
                                self.add_output(&alloc::format!("> {}", input));
                                self.clear_input();
                            }
                        }
                    } else if label.eq_ignore_ascii_case("cancel") {
                        self.clear_input();
                    } else {
                        self.add_output(&alloc::format!("[{}] pressed", label));
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_input(&self) -> Option<String> {
        if self.textboxes.len() > 0 {
            Some(self.textboxes[0].text.clone())
        } else {
            None
        }
    }

    pub fn clear_input(&mut self) {
        if self.textboxes.len() > 0 {
            self.textboxes[0].text.clear();
        }
    }

    pub fn add_output(&mut self, text: &str) {
        if let Some(ref mut listbox) = self.listbox {
            listbox.add_item(text);
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }
    
    pub  fn process_mouse(){
        //Graphics::get_cursor()
        return;
    }
}

// Add new UI module structure
pub mod resource_monitor;
pub mod console;


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
    pub fn new() -> Self {
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
            new_vm_memory_mb: 1024,
            new_vm_vcpus: 1,
            create_vm_focus_idx: 0,
            vm_action_idx: 0,
            selected_vm_idx: 0,
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

            // Draw navigation
            pg.fill_rect(0, 48, width, 32, 0x444444); // Dark Gray
            let nav_text = "O Overview | V VMs | R Resources | S Storage | N Network | D Devices | C Console | T Test";
            pg.draw_text(10, 56, nav_text, 0xFFFFFF);

            // Layout constants for consistent spacing across tabs
            let header_h = 48usize;
            let nav_h = 32usize;
            let content_top = header_h + nav_h; // 80px from top
            let margin = 16usize; // outer margin
            let gutter = 12usize; // space between widgets/rows
            let line_h = 20usize; // standard text line height

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
                    let panel_h = 240usize;
                    pg.draw_text(panel_x, panel_y - 4, "Resource Monitor", 0x00FF00);
                    pg.draw_rect_outline(panel_x, panel_y, panel_w, panel_h, 0x888888);
                    pg.draw_text(panel_x + 10, panel_y + 16, &alloc::format!("CPU Cores: {}", self.resources.cpu_count), 0xFFFFFF);
                    pg.draw_text(panel_x + 10, panel_y + 16 + line_h, &alloc::format!("Total Memory: {} MB", self.resources.total_memory_mb), 0xFFFFFF);
                    pg.draw_text(panel_x + 10, panel_y + 16 + line_h * 2, &alloc::format!("Used Memory: {} MB", self.resources.used_memory_mb), 0xFFFFFF);

                    // Memory usage bar and graph
                    let bar_y = panel_y + 16 + line_h * 3 + gutter;
                    pg.draw_text(panel_x + 10, bar_y, "Memory History (100s):", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &self.resources.mem_history, 100, 0x00FF00);

                    // I/O Stats and Graphs
                    let io_y = bar_y + 80 + gutter * 2;
                    pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, io_y + 20, 165, 50, &self.resources.net_rx_history, 1024, 0x00FFFF);
                    pg.draw_line_graph(panel_x + 185, io_y + 20, 165, 50, &self.resources.net_tx_history, 1024, 0xFFFF00);
                    
                    let disk_y = io_y + 80;
                    pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Red)", 0xCCCCCC);
                    pg.draw_line_graph(panel_x + 10, disk_y + 20, 165, 50, &self.resources.disk_read_history, 1024, 0xFFFFFF);
                    pg.draw_line_graph(panel_x + 185, disk_y + 20, 165, 50, &self.resources.disk_write_history, 1024, 0xFF0000);

                    // Right CPU core list panel or Total CPU Graph
                    let right_x = panel_x + panel_w + gutter * 2;
                    let right_y = panel_y;
                    let right_w = core::cmp::min(width - right_x - margin, 360);
                    let right_h = core::cmp::min(height - right_y - 100, 260);
                    pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x888888);
                    pg.draw_text(right_x + 10, right_y - 4, "Total CPU Usage History:", 0xFFFFFF);
                    pg.draw_line_graph(right_x + 10, right_y + 10, right_w - 20, 80, &self.resources.cpu_history, 100, 0x00FF00);
                    
                    pg.draw_text(right_x + 10, right_y + 100, "CPU Usage per Core:", 0xFFFFFF);
                    for i in 0..self.resources.cpu_count {
                        let row_y = right_y + 120 + (i as usize * (line_h + 4));
                        if row_y + line_h > right_y + right_h - 8 { break; }
                        let usage = if i < self.resources.cpu_core_usage.len() as u32 { self.resources.cpu_core_usage[i as usize] } else { 0 };
                        pg.draw_text(right_x + 10, row_y, &alloc::format!("C{}:{:>2}%", i, usage), 0xCCCCCC);
                        pg.draw_progress_bar(right_x + 70, row_y, right_w - 80, 12, usage as usize, 100, 0x00FF00);
                    }
                }
                DashboardTab::Network => {
                    pg.draw_text(20, 100, "Network Status", 0x00FF00);
                    let net_stats = crate::devices::net_stack::stats();
                    pg.draw_text(20, 130, &alloc::format!("Backend: {}", crate::devices::net_stack::backend_name()), 0xFFFFFF);
                    pg.draw_text(20, 160, "Statistics:", 0xAAAAAA);
                    pg.draw_text(40, 180, &alloc::format!("RX Packets: {}", net_stats.rx_pkts), 0xCCCCCC);
                    pg.draw_text(40, 200, &alloc::format!("TX Packets: {}", net_stats.tx_pkts), 0xCCCCCC);
                    pg.draw_text(40, 220, &alloc::format!("RX Bytes:   {}", net_stats.rx_bytes), 0xCCCCCC);
                    pg.draw_text(40, 240, &alloc::format!("TX Bytes:   {}", net_stats.tx_bytes), 0xCCCCCC);
                }
                DashboardTab::Console => {
                    pg.draw_text(20, 100, "System Log", 0x00FF00);
                    pg.draw_rect_outline(margin, 130, width - margin * 2, height - 130 - margin * 2, 0x888888);
                    
                    let mut y = 140;
                    let logs = crate::hpvmlog::get_logs();
                    
                    // Show last N logs that fit on screen
                    let max_visible = (height - 130 - margin * 2 - 20) / line_h;
                    let start_idx = logs.len().saturating_sub(max_visible);
                    
                    for i in start_idx..logs.len() {
                        let (color, tag, msg) = &logs[i];
                        let color_hex = match color {
                            uefi::proto::console::text::Color::Red => 0xFF0000,
                            uefi::proto::console::text::Color::Yellow => 0xFFFF00,
                            uefi::proto::console::text::Color::LightCyan => 0x00FFFF,
                            _ => 0xFFFFFF,
                        };
                        let log_line = if tag.is_empty() { msg.clone() } else { alloc::format!("[{}] {}", tag, msg) };
                        pg.draw_text(margin + 10, y, &log_line, color_hex);
                        y += line_h;
                        if y + line_h > height - margin * 2 { break; }
                    }
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
                                pg.draw_text(40, y, &alloc::format!("Ð {}: {}", dev.name, dev.path), color);
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
                    let list_h = core::cmp::min(height - list_y - 90, 360);
                    pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);

                    // Header row with better spacing and column guides
                    pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, line_h, 0x333333);
                    pg.draw_text(list_x + 8, list_y + 4, "TYPE  NAME                                      SIZE (BYTES)  ATTR", 0xCCCCCC);
                    // Optional column separators
                    pg.draw_line(list_x + 48, list_y + 1, list_x + 48, list_y + list_h - 1, 0x444444);
                    pg.draw_line(list_x + 340, list_y + 1, list_x + 340, list_y + list_h - 1, 0x444444);

                    // Rows
                    let mut y = list_y + line_h + gutter;
                    for (i, entry) in self.files.iter().enumerate() {
                        if y + line_h > list_y + list_h - 2 { break; }
                        let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
                        let icon = if entry.is_dir { "Ñ D" } else {
                            let dec_syn = ["json", "xml"];
                            let sys_syn = ["sys", "efi"];
                            let prog_syn = ["logical", "py", "dmx", "rts"];


                            #[allow(irrefutable_let_patterns)]
                            if let ext = entry.name.split(".").last().unwrap() {
                                if dec_syn.contains(&ext) {
                                    "Ò F"
                                } else if sys_syn.contains(&ext) {
                                    "Ó F"
                                } else if prog_syn.contains(&ext) {
                                    "Ô F"
                                } else {
                                    "Ç F"
                                }
                            } else {
                                "Ç F"
                            }
                        };
                        pg.draw_text(list_x + 8, y, icon, 0xCCCCCC);
                        pg.draw_text(list_x + 56, y, &alloc::format!("{:<28}", entry.name), color);
                        pg.draw_text(list_x + 348, y, &alloc::format!("{:>12}", entry.size), 0xCCCCCC);
                        pg.draw_text(list_x + 480, y, if entry.is_dir { "DIR" } else { "FILE" }, 0x6666FF);
                        y += line_h;
                    }
                }
                DashboardTab::Test => {
                    pg.draw_text(20, 100, "UI Components Test Bed (Qt6 Style)", 0x00FF00);
                    
                    // Column 1
                    let mut y = 130;
                    pg.draw_text(20, y, "Buttons & Inputs:", 0xAAAAAA); y += 25;
                    pg.fill_rect(20, y, 100, 25, 0x444444); pg.draw_text(25, y+5, "Push Button", 0xFFFFFF);
                    pg.fill_rect(130, y, 30, 25, 0x444444); pg.draw_text(138, y+5, "?", 0xFFFFFF); // ToolButton
                    y += 35;
                    
                    pg.draw_checkbox(20, y, true); pg.draw_text(40, y, "CheckBox (Checked)", 0xFFFFFF); y += 25;
                    pg.draw_checkbox(20, y, false); pg.draw_text(40, y, "CheckBox (Unchecked)", 0xFFFFFF); y += 25;
                    
                    pg.draw_radio_button(20, y, true); pg.draw_text(40, y, "RadioButton 1", 0xFFFFFF); y += 25;
                    pg.draw_radio_button(20, y, false); pg.draw_text(40, y, "RadioButton 2", 0xFFFFFF); y += 35;
                    
                    pg.draw_text(20, y, "LineEdit:", 0xAAAAAA); y += 20;
                    pg.draw_rect_outline(20, y, 150, 20, 0x888888); pg.fill_rect(21, y+1, 148, 18, 0xFFFFFF);
                    pg.draw_text(25, y+2, "Editable text..ſ", 0x000000); y += 30;
                    
                    pg.draw_text(20, y, "SpinBox / DoubleSpinBox:", 0xAAAAAA); y += 20;
                    pg.draw_rect_outline(20, y, 60, 20, 0x888888); pg.draw_text(25, y+2, "42", 0xFFFFFF); pg.draw_text(65, y+2, "[^v]", 0xAAAAAA);
                    pg.draw_rect_outline(100, y, 60, 20, 0x888888); pg.draw_text(105, y+2, "3.14", 0xFFFFFF); //y += 30;

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
                    let y = 400;
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
                    pg.draw_line(20, 500, 780, 500, 0x555555); // Horizontal Line
                    pg.draw_line(400, 505, 400, 550, 0x555555); // Vertical Line
                    
                    pg.draw_text(20, 510, "Labels & Browser:", 0xAAAAAA);
                    pg.draw_text(20, 530, "Standard Label", 0xFFFFFF);
                    pg.draw_rect_outline(150, 510, 230, 40, 0x444444);
                    pg.draw_text(155, 515, "Text Browser with <b>rich</b> content", 0xAAAAAA);
                    
                    pg.draw_text(420, 510, "Dial & Key Sequence:", 0xAAAAAA);
                    // Mock Dial
                    pg.draw_radio_button(420, 530, false); pg.draw_line(426, 536, 432, 530, 0xFF0000);
                    pg.draw_rect_outline(550, 530, 100, 20, 0x888888); pg.draw_text(555, 532, "Ctrl+Alt+Del", 0xFFFF00);
                }
            }

            // Draw footer
            pg.fill_rect(0, height - 48, width, 48, 0x000080); // Blue
            pg.draw_text(10, height - 32, "Press 'Q' to exit dashboard | Use keys O, V, R, S, N, D, C, T to switch tabs", 0xFFFFFF);

            // Update and draw cursor
            self.cursor.update_from_mouse(width, height);
            pg.draw_cursor(self.cursor.x as usize, self.cursor.y as usize);

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

    // fn draw_header(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = stdout.set_color(Color::Cyan, Color::Black);
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "╔════════════════════════════════════════════════════════════╗\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "║           HPVMx - Hypervisor Management Console            ║\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "╚════════════════════════════════════════════════════════════╝\n"
    //         );
    //         let _ = stdout.set_color(Color::White, Color::Black);
    //     });
    // }
    //
    // fn draw_navigation_bar(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = stdout.set_color(Color::LightGray, Color::Black);
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             " [O]verview │ [V]Ms │ [R]esources │ [S]torage │ [N]etwork │ [C]onsole\n"
    //         );
    //         let _ = stdout.set_color(Color::White, Color::Black);
    //     });
    // }

    // fn draw_overview(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ System Overview ────────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_fmt(
    //             stdout,
    //             format_args!("│ CPU Usage:    {:3}%                                    │\n",
    //                          self.resources.cpu_usage)
    //         );
    //         let _ = core::fmt::Write::write_fmt(
    //             stdout,
    //             format_args!("│ Memory:       {}/{} MB                          │\n",
    //                          self.resources.used_memory_mb,
    //                          self.resources.total_memory_mb)
    //         );
    //         let _ = core::fmt::Write::write_fmt(
    //             stdout,
    //             format_args!("│ VMs Running:  {}                                      │\n",
    //                          self.count_running_vms())
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└──────────────────────────────────────────────────────────┘\n"
    //         );
    //     });
    // }

    // fn draw_vms_list(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ Virtual Machines ───────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ ID │ Name       │ State    │ CPU │ Memory │ Uptime    │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "├────┼────────────┼──────────┼─────┼────────┼───────────┤\n"
    //         );
    //
    //         let end = core::cmp::min(
    //             self.scroll_offset + 10,
    //             self.vms.len()
    //         );
    //
    //         for vm in &self.vms[self.scroll_offset..end] {
    //             let name_len = core::cmp::min(10, vm.name.len());
    //             let _ = core::fmt::Write::write_fmt(
    //                 stdout,
    //                 format_args!(
    //                     "│ {:2} │ {:<10} │ {:<8} │ {:3}% │ {:5} MB │ {:>4}h {:>2}m │\n",
    //                     vm.id,
    //                     &vm.name[..name_len],
    //                     &vm.state,
    //                     vm.cpu_usage,
    //                     vm.memory_usage_mb,
    //                     vm.uptime_seconds / 3600,
    //                     (vm.uptime_seconds % 3600) / 60
    //                 )
    //             );
    //         }
    //
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└────┴────────────┴──────────┴─────┴────────┴───────────┘\n"
    //         );
    //     });
    // }

    // fn draw_resources(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ System Resources ───────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ Processor: 8 cores @ 3.2 GHz                             │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ Memory: 16 GB total, 12 GB allocated to VMs              │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ Thermal: CPU 45°C, Host 38°C                             │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└──────────────────────────────────────────────────────────┘\n"
    //         );
    //     });
    // }

    // fn draw_storage(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ Storage ────────────────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ /dev/sda   500 GB   [████████░░] 80% used               │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ /dev/sdb   1.0 TB   [██████░░░░] 60% used               │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ /dev/sdc   2.0 TB   [███░░░░░░░] 30% used               │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└──────────────────────────────────────────────────────────┘\n"
    //         );
    //     });
    // }

    // fn draw_network(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ Network ────────────────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ eth0: 192.168.1.100  RX: 125 MB/s   TX: 87 MB/s          │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ eth1: 10.0.0.50      RX: 12 MB/s    TX: 34 MB/s          │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└──────────────────────────────────────────────────────────┘\n"
    //         );
    //     });
    // }

    // fn draw_console(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "\n┌─ Console Output ─────────────────────────────────────────┐\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│ [Connected to VM console]                                │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "│                                                          │\n"
    //         );
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             "└──────────────────────────────────────────────────────────┘\n"
    //         );
    //     });
    // }

    // fn draw_footer(&self) {
    //     uefi::system::with_stdout(|stdout| {
    //         let _ = stdout.set_color(Color::LightGray, Color::Black);
    //         let _ = core::fmt::Write::write_str(
    //             stdout,
    //             " [↑↓] Navigate  [Enter] Select  [Q] Quit  [?] Help\n"
    //         );
    //         let _ = stdout.set_color(Color::White, Color::Black);
    //     });
    // }

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
            _ => {}
        }

        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                match ch {
                    'q' => {
                        self.exit_requested = true;
                        return;
                    }
                    'o' => self.selected_tab = DashboardTab::Overview,
                    'v' => self.selected_tab = DashboardTab::VirtualMachines,
                    'r' => self.selected_tab = DashboardTab::Resources,
                    's' => self.selected_tab = DashboardTab::Storage,
                    'n' => self.selected_tab = DashboardTab::Network,
                    'd' => self.selected_tab = DashboardTab::Devices,
                    'c' => self.selected_tab = DashboardTab::Console,
                    't' => self.selected_tab = DashboardTab::Test,
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
                            DashboardTab::Test => DashboardTab::Overview,
                            DashboardTab::CreateVM => DashboardTab::VirtualMachines,
                        };
                    }
                    _ => {}
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
