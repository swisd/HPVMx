#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::{Color, Key, ScanCode};

mod graphics;
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
        Graphics::draw_box(&self.rect, &self.title, self.active);
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
        Graphics::draw_button(&self.rect, &self.label, self.focused);
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


pub struct DashboardUI {
    selected_tab: DashboardTab,
    vms: Vec<VmDisplayInfo>,
    resources: SystemResources,
    scroll_offset: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum DashboardTab {
    Overview,
    VirtualMachines,
    Resources,
    Storage,
    Network,
    Console,
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
        }
    }

    pub fn add_vm(&mut self, vm: VmDisplayInfo) {
        self.vms.push(vm);
    }

    pub fn set_resources(&mut self, resources: SystemResources) {
        self.resources = resources;
    }

    pub fn draw(&self) {
        self.draw_header();
        self.draw_navigation_bar();

        match self.selected_tab {
            DashboardTab::Overview => self.draw_overview(),
            DashboardTab::VirtualMachines => self.draw_vms_list(),
            DashboardTab::Resources => self.draw_resources(),
            DashboardTab::Storage => self.draw_storage(),
            DashboardTab::Network => self.draw_network(),
            DashboardTab::Console => self.draw_console(),
        }

        self.draw_footer();
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
                    'c' => self.selected_tab = DashboardTab::Console,
                    _ => {}
                }
            }
            Key::Special(ScanCode::UP) => {
                if self.scroll_offset > 0 {
                    self.scroll_offset -= 1;
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.scroll_offset < self.vms.len().saturating_sub(1) {
                    self.scroll_offset += 1;
                }
            }
            _ => {}
        }
    }
}

