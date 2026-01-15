use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::console::text::Color;

pub struct UI {
    selected_menu_index: usize,
    selected_row_index: usize,
    menu_items: Vec<MenuItem>,
    vms: Vec<VMInfo>,
    cursor_x: usize,
    cursor_y: usize,
}

struct MenuItem {
    name: &'static str,
    icon: &'static str,
}

struct VMInfo {
    id: u32,
    name: &'static str,
    status: &'static str,
    cpu_usage: f32,
    mem_usage: f32,
    uptime: &'static str,
}

impl UI {
    pub fn new() -> Self {
        let mut menu_items = Vec::new();
        menu_items.push(MenuItem { name: "Datacenter", icon: "📊" });
        menu_items.push(MenuItem { name: "Summary", icon: "📋" });
        menu_items.push(MenuItem { name: "Search", icon: "🔍" });
        menu_items.push(MenuItem { name: "Cluster", icon: "🔗" });
        menu_items.push(MenuItem { name: "Storage", icon: "💾" });
        menu_items.push(MenuItem { name: "Backup", icon: "📦" });
        menu_items.push(MenuItem { name: "Permissions", icon: "🔐" });
        menu_items.push(MenuItem { name: "Options", icon: "⚙️" });

        let mut vms = Vec::new();
        vms.push(VMInfo {
            id: 102,
            name: "web-server-01",
            status: "running",
            cpu_usage: 2.5,
            mem_usage: 45.2,
            uptime: "45d 12h",
        });
        vms.push(VMInfo {
            id: 103,
            name: "db-server-01",
            status: "running",
            cpu_usage: 15.3,
            mem_usage: 72.8,
            uptime: "30d 5h",
        });
        vms.push(VMInfo {
            id: 104,
            name: "app-server-01",
            status: "stopped",
            cpu_usage: 0.0,
            mem_usage: 0.0,
            uptime: "-",
        });
        vms.push(VMInfo {
            id: 105,
            name: "test-machine",
            status: "running",
            cpu_usage: 8.5,
            mem_usage: 32.1,
            uptime: "7d 3h",
        });

        UI {
            selected_menu_index: 0,
            selected_row_index: 0,
            menu_items,
            vms,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    pub fn run(&mut self) {
        uefi::system::with_stdout(|s| {
            let _ = s.clear();
        });

        loop {
            self.render();

            if !self.handle_input() {
                break;
            }
        }

        uefi::system::with_stdout(|s| {
            let _ = s.clear();
        });
    }

    fn render(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = stdout.set_color(Color::White, Color::Black);

            // Header
            self.render_header(stdout);

            // Main layout: sidebar + content
            self.render_sidebar(stdout);
            self.render_content(stdout);

            // Status bar at bottom
            self.render_status_bar(stdout);

            // Render cursor
            self.render_cursor(stdout);
        });
    }

    fn render_header(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = write!(stdout, "┌────────────────────────────────────────────────────────────────────────────────────────┐\n");
        let _ = write!(stdout, "│  🔷 HPVMx Virtual Environment 1.0.0                          [Datacenter] [Summary]  │\n");
        let _ = write!(stdout, "└────────────────────────────────────────────────────────────────────────────────────────┘\n");
    }

    fn render_sidebar(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _ = write!(stdout, "┌──────────────────────┐ ┌──────────────────────────────────────────────────────────────────┐\n");

        for (idx, item) in self.menu_items.iter().enumerate() {
            if idx == self.selected_menu_index {
                let _ = stdout.set_color(Color::Black, Color::LightGray);
                let _ = write!(stdout, "│ ► {:<1} {:<17} │ │", item.icon, item.name);
            } else {
                let _ = stdout.set_color(Color::White, Color::Black);
                let _ = write!(stdout, "│   {:<1} {:<17} │ │", item.icon, item.name);
            }

            match idx {
                0 => { let _ = write!(stdout, " Datacenter View                                       │                              \n"); }
                1 => { let _ = write!(stdout, " Summary Information                                   │                              \n"); }
                _ => { let _ = write!(stdout, "                                                          │                              \n"); }
            }
        }

        // Remaining sidebar space
        for _ in self.menu_items.len()..20 {
            let _ = stdout.set_color(Color::White, Color::Black);
            let _ = write!(stdout, "│                      │ │                                                          │                              \n");
        }

        let _ = write!(stdout, "└──────────────────────┘ └──────────────────────────────────────────────────────────────────┘                              \n");
    }

    fn render_content(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _ = stdout.set_color(Color::White, Color::Black);

        // Table header
        let _ = write!(stdout, "┌──────────────────────┬──────────┬─────────────┬──────────────┬──────────┬──────────────┐\n");
        let _ = write!(stdout, "│ Name                 │ Status   │ CPU Usage   │ Memory (%)   │ CPU (%)  │ Uptime       │\n");
        let _ = write!(stdout, "├──────────────────────┼──────────┼─────────────┼──────────────┼──────────┼──────────────┤\n");

        // VM rows
        for (idx, vm) in self.vms.iter().enumerate() {
            if idx == self.selected_row_index {
                let _ = stdout.set_color(Color::Black, Color::LightGray);
            } else {
                let _ = stdout.set_color(Color::White, Color::Black);
            }

            let status_icon = if vm.status == "running" { "🟢" } else { "🔴" };

            let _ = write!(
                stdout,
                "│ {:<20} │ {} {:<6} │ {:<11.1} │ {:<12.1} │ {:<8.1} │ {:<12} │\n",
                vm.name, status_icon, vm.status, vm.cpu_usage, vm.mem_usage, vm.cpu_usage, vm.uptime
            );
        }

        let _ = stdout.set_color(Color::White, Color::Black);
        let _ = write!(stdout, "└──────────────────────┴──────────┴─────────────┴──────────────┴──────────┴──────────────┘\n");
    }

    fn render_status_bar(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _ = stdout.set_color(Color::Black, Color::LightGray);
        let _ = write!(
            stdout,
            " HPVMx v{} │ Selected: VM #{} │ Cursor: ({},{}) │ ↑↓: Navigate │ Q: Quit ",
            env!("CARGO_PKG_VERSION"),
            if self.selected_row_index < self.vms.len() {
                self.vms[self.selected_row_index].id
            } else {
                0
            },
            self.cursor_x,
            self.cursor_y
        );
        let _ = stdout.set_color(Color::White, Color::Black);
    }

    fn render_cursor(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _ = write!(stdout, "\x1b[{};{}H▶", self.cursor_y + 1, self.cursor_x + 1);
    }

    fn handle_input(&mut self) -> bool {
        loop {
            let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
            uefi::boot::wait_for_event(&mut events).unwrap();

            if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
                match key {
                    Key::Printable(c) => {
                        let ch = char::from(c);
                        match ch {
                            'q' | 'Q' => return false,
                            _ => {}
                        }
                    }
                    Key::Special(ScanCode::UP) => {
                        if self.selected_row_index > 0 {
                            self.selected_row_index -= 1;
                        }
                        return true;
                    }
                    Key::Special(ScanCode::DOWN) => {
                        if self.selected_row_index < self.vms.len() - 1 {
                            self.selected_row_index += 1;
                        }
                        return true;
                    }
                    Key::Special(ScanCode::LEFT) => {
                        if self.selected_menu_index > 0 {
                            self.selected_menu_index -= 1;
                        }
                        return true;
                    }
                    Key::Special(ScanCode::RIGHT) => {
                        if self.selected_menu_index < self.menu_items.len() - 1 {
                            self.selected_menu_index += 1;
                        }
                        return true;
                    }
                    Key::Printable(c) if char::from(c) == '\r' || char::from(c) == '\n' => {
                        self.execute_selection();
                        return true;
                    }
                    _ => {}
                }
            }
        }
    }

    fn execute_selection(&mut self) {
        match self.selected_menu_index {
            0 => {
                uefi::system::with_stdout(|stdout| {
                    let _ = stdout.clear();
                    let _ = write!(stdout, "\nDatacenter View - Selected VM: {}\n", self.vms[self.selected_row_index].name);
                    let _ = write!(stdout, "Press any key to return...\n");
                });
                self.wait_for_key();
            }
            1 => {
                uefi::system::with_stdout(|stdout| {
                    let _ = stdout.clear();
                    let _ = write!(stdout, "\nSummary Information\n");
                    let _ = write!(stdout, "Total VMs: {}\n", self.vms.len());
                    let _ = write!(stdout, "Running: {}\n", self.vms.iter().filter(|v| v.status == "running").count());
                    let _ = write!(stdout, "Press any key to return...\n");
                });
                self.wait_for_key();
            }
            _ => {}
        }
    }

    fn wait_for_key(&self) {
        loop {
            let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
            uefi::boot::wait_for_event(&mut events).unwrap();

            if uefi::system::with_stdin(|i| i.read_key().unwrap()).is_some() {
                break;
            }
        }
    }
}