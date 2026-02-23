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
    }

    pub fn focus_textbox(&mut self, idx: usize) {
        for (i, textbox) in self.textboxes.iter_mut().enumerate() {
            textbox.focused = i == idx;
        }
        self.focused_textbox = idx;
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                if self.focused_textbox < self.textboxes.len() {
                    let ch = char::from(c);
                    if ch != '\r' && ch != '\n' {
                        self.textboxes[self.focused_textbox].add_char(ch);
                    }
                }
            }
            Key::Special(ScanCode::DELETE) => {
                if self.focused_textbox < self.textboxes.len() {
                    self.textboxes[self.focused_textbox].backspace();
                }
            }
            Key::Special(ScanCode::UP) => {
                if let Some(ref mut listbox) = self.listbox {
                    listbox.select_prev();
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if let Some(ref mut listbox) = self.listbox {
                    listbox.select_next();
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                let next_button = (self.focused_button + 1) % self.buttons.len();
                self.focus_button(next_button);
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
            },
            scroll_offset: 0,
            cursor: crate::graphics::Cursor::new(),
            current_path: String::from("\\"),
            files: Vec::new(),
            selected_file_idx: 0,
            categories: Vec::new(),
            selected_device_idx: 0,
        }
    }

    pub fn add_vm(&mut self, vm: VmDisplayInfo) {
        self.vms.push(vm);
    }

    pub fn set_resources(&mut self, resources: SystemResources) {
        self.resources = resources;
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
            let nav_text = "O Overview | V VMs | R Resources | S Storage | N Network | D Devices | C Console";
            pg.draw_text(10, 56, nav_text, 0xFFFFFF);

            // Content area based on selected tab
            match self.selected_tab {
                DashboardTab::Overview => {
                    pg.draw_text(20, 100, "System Overview", 0x00FF00);
                    pg.draw_text(20, 130, &alloc::format!("CPU Count: {}", self.resources.cpu_count), 0xFFFFFF);
                    pg.draw_text(20, 150, &alloc::format!("CPU Usage: {}%", self.resources.cpu_usage), 0xFFFFFF);
                    pg.draw_text(20, 170, &alloc::format!("Memory: {} / {} MB", self.resources.used_memory_mb, self.resources.total_memory_mb), 0xFFFFFF);
                    pg.draw_text(20, 190, &alloc::format!("Total VMs: {}", self.vms.len()), 0xFFFFFF);
                }
                DashboardTab::VirtualMachines => {
                    pg.draw_text(20, 100, "Virtual Machines", 0x00FF00);
                    let mut y = 130;
                    pg.draw_text(20, y, "ID   NAME             STATE        CPU  MEM", 0xAAAAAA);
                    y += 20;
                    for vm in &self.vms {
                        let info = alloc::format!("{:<4} {:<16} {:<12} {:>3}% {:>5}MB", 
                            vm.id, vm.name, vm.state, vm.cpu_usage, vm.memory_usage_mb);
                        pg.draw_text(20, y, &info, 0xFFFFFF);
                        y += 20;
                        if y > height - 60 { break; }
                    }
                }
                DashboardTab::Resources => {
                    pg.draw_text(20, 100, "Resource Monitor", 0x00FF00);
                    pg.draw_text(20, 130, &alloc::format!("CPU Cores: {}", self.resources.cpu_count), 0xFFFFFF);
                    pg.draw_text(20, 150, &alloc::format!("Total Memory: {} MB", self.resources.total_memory_mb), 0xFFFFFF);
                    pg.draw_text(20, 170, &alloc::format!("Used Memory: {} MB", self.resources.used_memory_mb), 0xFFFFFF);
                    
                    // Draw memory usage bar
                    pg.draw_text(20, 200, "Memory Usage:", 0xCCCCCC);
                    pg.fill_rect(150, 200, 200, 20, 0x444444);
                    let usage_width = (200 * self.resources.used_memory_mb) / self.resources.total_memory_mb.max(1);
                    pg.fill_rect(150, 200, usage_width as usize, 20, 0x00FF00);

                    pg.draw_text(20, 240, "CPU Usage per Core (Mocked):", 0xFFFFFF);
                    for i in 0..self.resources.cpu_count {
                        pg.draw_text(40, 260 + (i as usize * 20), &alloc::format!("Core {}: 25%", i), 0xCCCCCC);
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
                    pg.draw_text(20, 130, "[Note: Historical log viewing implementation in progress]", 0x888888);
                    pg.draw_text(20, 150, "Latest Messages:", 0xAAAAAA);
                    
                    // Mock some recent logs since we don't have a global log buffer yet
                    pg.draw_text(20, 180, "[INFO] Hypervisor initialized successfully", 0x00FF00);
                    pg.draw_text(20, 200, "[INFO] Filesystem mounted", 0x00FF00);
                    pg.draw_text(20, 220, "[WARN] No external network detected", 0xFFFF00);
                    pg.draw_text(20, 240, "[INFO] Dashboard UI started", 0x00FF00);
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
                                pg.draw_text(40, y, &alloc::format!("└─ {}: {}", dev.name, dev.path), color);
                                y += 20;
                                current_idx += 1;
                                if y > height - 60 { break; }
                            }
                        }
                        if y > height - 60 { break; }
                    }
                }
                DashboardTab::Storage => {
                    pg.draw_text(20, 100, "File Explorer", 0x00FF00);
                    pg.draw_text(20, 130, &alloc::format!("Path: {}", self.current_path), 0xAAAAAA);
                    
                    let mut y = 160;
                    pg.draw_text(20, y, "NAME                           SIZE (BYTES)  ATTR", 0x888888);
                    y += 20;
                    
                    for (i, entry) in self.files.iter().enumerate() {
                        let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
                        let icon = if entry.is_dir { "[D]" } else { "[F]" };
                        let info = alloc::format!("{:<3} {:<30} {:>12}  {}", 
                            icon, entry.name, entry.size, if entry.is_dir { "DIR" } else { "FILE" });
                        
                        pg.draw_text(20, y, &info, color);
                        y += 20;
                        if y > height - 100 { break; }
                    }
                }
            }

            // Draw footer
            pg.fill_rect(0, height - 48, width, 48, 0x000080); // Blue
            pg.draw_text(10, height - 32, "Press 'Q' to exit dashboard | Use keys O, V, R, S, N, D, C to switch tabs", 0xFFFFFF);

            // Update and draw cursor
            self.cursor.update_from_mouse(width, height);
            pg.draw_cursor(self.cursor.x as usize, self.cursor.y as usize);

            pg.flip();

        } else {
            self.draw_header();
            self.draw_navigation_bar();

            match self.selected_tab {
                DashboardTab::Overview => self.draw_overview(),
                DashboardTab::VirtualMachines => self.draw_vms_list(),
                DashboardTab::Resources => self.draw_resources(),
                DashboardTab::Storage => self.draw_storage(),
                DashboardTab::Network => self.draw_network(),
                DashboardTab::Console => self.draw_console(),
                DashboardTab::Devices => {},
            }

            self.draw_footer();
        }
    }

    fn draw_header(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.set_color(Color::Cyan, Color::Black);
            let _ = core::fmt::Write::write_str(
                stdout,
                "╔════════════════════════════════════════════════════════════╗\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "║           HPVMx - Hypervisor Management Console            ║\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "╚════════════════════════════════════════════════════════════╝\n"
            );
            let _ = stdout.set_color(Color::White, Color::Black);
        });
    }

    fn draw_navigation_bar(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.set_color(Color::LightGray, Color::Black);
            let _ = core::fmt::Write::write_str(
                stdout,
                " [O]verview │ [V]Ms │ [R]esources │ [S]torage │ [N]etwork │ [C]onsole\n"
            );
            let _ = stdout.set_color(Color::White, Color::Black);
        });
    }

    fn draw_overview(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ System Overview ────────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_fmt(
                stdout,
                format_args!("│ CPU Usage:    {:3}%                                    │\n",
                             self.resources.cpu_usage)
            );
            let _ = core::fmt::Write::write_fmt(
                stdout,
                format_args!("│ Memory:       {}/{} MB                          │\n",
                             self.resources.used_memory_mb,
                             self.resources.total_memory_mb)
            );
            let _ = core::fmt::Write::write_fmt(
                stdout,
                format_args!("│ VMs Running:  {}                                      │\n",
                             self.count_running_vms())
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "└──────────────────────────────────────────────────────────┘\n"
            );
        });
    }

    fn draw_vms_list(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ Virtual Machines ───────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ ID │ Name       │ State    │ CPU │ Memory │ Uptime    │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "├────┼────────────┼──────────┼─────┼────────┼───────────┤\n"
            );

            let end = core::cmp::min(
                self.scroll_offset + 10,
                self.vms.len()
            );

            for vm in &self.vms[self.scroll_offset..end] {
                let name_len = core::cmp::min(10, vm.name.len());
                let _ = core::fmt::Write::write_fmt(
                    stdout,
                    format_args!(
                        "│ {:2} │ {:<10} │ {:<8} │ {:3}% │ {:5} MB │ {:>4}h {:>2}m │\n",
                        vm.id,
                        &vm.name[..name_len],
                        &vm.state,
                        vm.cpu_usage,
                        vm.memory_usage_mb,
                        vm.uptime_seconds / 3600,
                        (vm.uptime_seconds % 3600) / 60
                    )
                );
            }

            let _ = core::fmt::Write::write_str(
                stdout,
                "└────┴────────────┴──────────┴─────┴────────┴───────────┘\n"
            );
        });
    }

    fn draw_resources(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ System Resources ───────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ Processor: 8 cores @ 3.2 GHz                             │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ Memory: 16 GB total, 12 GB allocated to VMs              │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ Thermal: CPU 45°C, Host 38°C                             │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "└──────────────────────────────────────────────────────────┘\n"
            );
        });
    }

    fn draw_storage(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ Storage ────────────────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ /dev/sda   500 GB   [████████░░] 80% used               │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ /dev/sdb   1.0 TB   [██████░░░░] 60% used               │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ /dev/sdc   2.0 TB   [███░░░░░░░] 30% used               │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "└──────────────────────────────────────────────────────────┘\n"
            );
        });
    }

    fn draw_network(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ Network ────────────────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ eth0: 192.168.1.100  RX: 125 MB/s   TX: 87 MB/s          │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ eth1: 10.0.0.50      RX: 12 MB/s    TX: 34 MB/s          │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "└──────────────────────────────────────────────────────────┘\n"
            );
        });
    }

    fn draw_console(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = core::fmt::Write::write_str(
                stdout,
                "\n┌─ Console Output ─────────────────────────────────────────┐\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│ [Connected to VM console]                                │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "│                                                          │\n"
            );
            let _ = core::fmt::Write::write_str(
                stdout,
                "└──────────────────────────────────────────────────────────┘\n"
            );
        });
    }

    fn draw_footer(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.set_color(Color::LightGray, Color::Black);
            let _ = core::fmt::Write::write_str(
                stdout,
                " [↑↓] Navigate  [Enter] Select  [Q] Quit  [?] Help\n"
            );
            let _ = stdout.set_color(Color::White, Color::Black);
        });
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

    pub fn handle_input(&mut self, key: Key) {
        use uefi::proto::console::text::ScanCode;

        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                match ch {
                    'o' => self.selected_tab = DashboardTab::Overview,
                    'v' => self.selected_tab = DashboardTab::VirtualMachines,
                    'r' => self.selected_tab = DashboardTab::Resources,
                    's' => self.selected_tab = DashboardTab::Storage,
                    'n' => self.selected_tab = DashboardTab::Network,
                    'd' => self.selected_tab = DashboardTab::Devices,
                    'c' => self.selected_tab = DashboardTab::Console,
                    '\r' | '\n' => {
                        if matches!(self.selected_tab, DashboardTab::Storage) {
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
                    _ => {}
                }
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
                } else {
                    if self.scroll_offset < self.vms.len().saturating_sub(1) {
                        self.scroll_offset += 1;
                    }
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
            }
        }
    }
}
