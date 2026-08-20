use alloc::{format, vec, vec::Vec};
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use uefi::Identify;
use uefi::proto::console::text::{Key, ScanCode};
use crate::ui::{
    pixel_graphics::{self, PixelGraphics},
    DashboardTab, DashboardUI, DeviceCategory, DiskTabInfo, EditorMode, FileEntry, FilePendingAction,
    ResourceMonitorTab, SystemResources, TextEditor, UiSettings, VmDisplayInfo,
};
use crate::env::{AppInfo, Environment, Runnable};
use crate::pm::{Package, PackageManager};
use crate::{runtime, vdebug, TSC_PER_US, HYPERVISOR, GLOBALENV};
use crate::terminal;

/// Unified dispatcher to draw any dashboard tab content.
pub fn draw_tab(
    tab: DashboardTab,
    ui: &mut DashboardUI,
    pg: &mut PixelGraphics,
    origin_x: usize,
    origin_y: usize,
    width: usize,
    height: usize,
) {
    match tab {
        DashboardTab::Overview => overview::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::VirtualMachines => vms::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Resources => resources::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Storage => storage::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Network => network::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Console => console::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Devices => devices::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Test => test::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::CreateVM => createvm::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Editor => editor::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Settings => settings::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Packages => packages::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::Apps => apps::draw(ui, pg, origin_x, origin_y, width, height),
        DashboardTab::SystemInfo => sysinfo::draw(ui, pg, origin_x, origin_y, width, height),
    }
}

/// Unified dispatcher to handle user input for any dashboard tab.
/// Returns true if the key event was consumed by the tab.
pub fn handle_tab_input(tab: DashboardTab, ui: &mut DashboardUI, key: Key) -> bool {
    match tab {
        DashboardTab::Overview => overview::input(ui, key),
        DashboardTab::VirtualMachines => vms::input(ui, key),
        DashboardTab::Resources => resources::input(ui, key),
        DashboardTab::Storage => storage::input(ui, key),
        DashboardTab::Network => network::input(ui, key),
        DashboardTab::Console => console::input(ui, key),
        DashboardTab::Devices => devices::input(ui, key),
        DashboardTab::Test => test::input(ui, key),
        DashboardTab::CreateVM => createvm::input(ui, key),
        DashboardTab::Editor => editor::input(ui, key),
        DashboardTab::Settings => settings::input(ui, key),
        DashboardTab::Packages => packages::input(ui, key),
        DashboardTab::Apps => apps::input(ui, key),
        DashboardTab::SystemInfo => sysinfo::input(ui, key),
    }
}

/// Unified dispatcher to update periodic/tick logic for any dashboard tab.
pub fn update_tab_logic(tab: DashboardTab, ui: &mut DashboardUI) {
    match tab {
        DashboardTab::Overview => overview::logic(ui),
        DashboardTab::VirtualMachines => vms::logic(ui),
        DashboardTab::Resources => resources::logic(ui),
        DashboardTab::Storage => storage::logic(ui),
        DashboardTab::Network => network::logic(ui),
        DashboardTab::Console => console::logic(ui),
        DashboardTab::Devices => devices::logic(ui),
        DashboardTab::Test => test::logic(ui),
        DashboardTab::CreateVM => createvm::logic(ui),
        DashboardTab::Editor => editor::logic(ui),
        DashboardTab::Settings => settings::logic(ui),
        DashboardTab::Packages => packages::logic(ui),
        DashboardTab::Apps => apps::logic(ui),
        DashboardTab::SystemInfo => sysinfo::logic(ui),
    }
}

// =========================================================================
// 1. Overview Tab
// =========================================================================
pub mod overview {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, _width: usize, _height: usize) {
        pg.draw_text(20, 100, "System Overview", 0x00FF00);

        let mut y = 130;
        pg.draw_text(20, y, "System Health: OK", 0x00FF00);
        y += 30;
        pg.draw_text(20, y, &alloc::format!("CPU:   {} Cores, {}% Usage", ui.resources.cpu_count, ui.resources.cpu_usage), 0xFFFFFF);
        y += 20;
        pg.draw_text(20, y, &alloc::format!("Memory: {} / {} MB", ui.resources.used_memory_mb, ui.resources.total_memory_mb), 0xFFFFFF);
        y += 30;

        pg.draw_text(20, y, "I/O Performance:", 0xAAAAAA);
        y += 20;
        pg.draw_text(40, y, &alloc::format!("Disk:   Read {} KB/s, Write {} KB/s", ui.resources.disk_read_kbps, ui.resources.disk_write_kbps), 0xCCCCCC);
        y += 20;
        pg.draw_text(40, y, &alloc::format!("Network: RX {} KB/s, TX {} KB/s", ui.resources.net_rx_kbps, ui.resources.net_tx_kbps), 0xCCCCCC);
        y += 30;

        pg.draw_text(20, y, &alloc::format!("Virtualization: {} VMs Running", ui.vms.iter().filter(|v| v.state.contains("Running")).count()), 0xFFFFFF);
        y += 20;
        pg.draw_text(20, y, &alloc::format!("Total VMs: {}", ui.vms.len()), 0xCCCCCC);
        y += 30;

        pg.draw_text(20, y, "Hardware Categories:", 0xAAAAAA);
        y += 20;
        pg.draw_text(40, y, &alloc::format!("Storage: {} Files in current path", ui.files.len()), 0xCCCCCC);
        y += 20;
        pg.draw_text(40, y, &alloc::format!("Devices: {} Categories detected", ui.categories.len()), 0xCCCCCC);
        y += 60;
        pg.draw_text_bg(40, y, "STATE BACKUP", 0xFF7700, 0x444444);
        y += 20;
        pg.fill_rect(40, y, 70, 30, 0x553333);
        pg.draw_text(42, y + 2, "SAVE [/]", 0xBBBBAA);

        y = 100;
        if let Ok((time, caps)) = runtime::get_time_and_caps() {
            let time_data_0 = format!("{:?}", time);
            let time_data_1 = format!("{:?}", caps);
            pg.draw_text(420, y, &time_data_0, 0xFFFFFF);
            y += 10;
            pg.draw_text(420, y, &time_data_1, 0xFFFFFF);
        }
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        if let Key::Printable(c) = key {
            if char::from(c) == '/' {
                unsafe {
                    crate::state::SAVE(Some(ui));
                }
                return true;
            }
        }
        false
    }

    #[derive(Clone)]
    pub struct X_Overview {
        pub cpu_count: u32,
        pub cpu_usage: u32,
        pub used_memory_mb: u32,
        pub total_memory_mb: u32,
        pub disk_read_kbps: u32,
        pub disk_write_kbps: u32,
        pub net_rx_kbps: u32,
        pub net_tx_kbps: u32,
        pub running_vms: usize,
        pub total_vms: usize,
        pub files_count: usize,
        pub categories_count: usize,
    }

    impl X_Overview {
        pub fn new() -> Self {
            Self {
                cpu_count: crate::hardware::cpu::core_count().max(1),
                cpu_usage: 0,
                used_memory_mb: 0,
                total_memory_mb: 0,
                disk_read_kbps: 0,
                disk_write_kbps: 0,
                net_rx_kbps: 0,
                net_tx_kbps: 0,
                running_vms: 0,
                total_vms: 0,
                files_count: 0,
                categories_count: 0,
            }
        }
    }

    impl Runnable for X_Overview {
        fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
            let menu_w = 200;
            let content_x = x + menu_w;
            pg.fill_rect(x, y, menu_w, 600, 0x333333);
            
            let categories = ["Overview", "Hardware", "Virtualization", "Storage", "Network", "Security", "Logs", "Tools", "Help"];
            let mut menu_y = y + 20;
            for (idx, cat) in categories.iter().enumerate() {
                let color = if idx == 0 { 0x00FF00 } else { 0xCCCCCC };
                pg.draw_text(x + 20, menu_y, cat, color);
                menu_y += 30;
            }

            pg.draw_text(content_x + 20, y + 20, "System Overview", 0x00FF00);
            
            let mut curr_y = y + 50;
            pg.draw_text(content_x + 20, curr_y, "System Health: OK", 0x00FF00);
            curr_y += 30;
            pg.draw_text(content_x + 20, curr_y, &format!("CPU:   {} Cores, {}% Usage", self.cpu_count, self.cpu_usage), 0xFFFFFF);
            curr_y += 20;
            pg.draw_text(content_x + 20, curr_y, &format!("Memory: {} / {} MB", self.used_memory_mb, self.total_memory_mb), 0xFFFFFF);
            curr_y += 30;
            
            pg.draw_text(content_x + 20, curr_y, "I/O Performance:", 0xAAAAAA);
            curr_y += 20;
            pg.draw_text(content_x + 40, curr_y, &format!("Disk:   Read {} KB/s, Write {} KB/s", self.disk_read_kbps, self.disk_write_kbps), 0xCCCCCC);
            curr_y += 20;
            pg.draw_text(content_x + 40, curr_y, &format!("Network: RX {} KB/s, TX {} KB/s", self.net_rx_kbps, self.net_tx_kbps), 0xCCCCCC);
            curr_y += 30;
            
            pg.draw_text(content_x + 20, curr_y, &format!("Virtualization: {} VMs Running", self.running_vms), 0xFFFFFF);
            curr_y += 20;
            pg.draw_text(content_x + 20, curr_y, &format!("Total VMs: {}", self.total_vms), 0xCCCCCC);
            curr_y += 30;

            pg.draw_text(content_x + 20, curr_y, "Hardware Categories:", 0xAAAAAA);
            curr_y += 20;
            pg.draw_text(content_x + 40, curr_y, &format!("Storage: {} Files in current path", self.files_count), 0xCCCCCC);
            curr_y += 20;
            pg.draw_text(content_x + 40, curr_y, &format!("Devices: {} Categories detected", self.categories_count), 0xCCCCCC);
            curr_y += 60;
            pg.draw_text_bg(content_x + 40, curr_y, "STATE BACKUP", 0xFF7700, 0x444444);
            curr_y += 20;
            pg.fill_rect(content_x + 40, curr_y, 70, 30, 0x553333);
            pg.draw_text(content_x + 42, curr_y + 2, "SAVE [/]", 0xBBBBAA);

            let time_y = y + 20;
            let time_x = content_x + 320;
            if let Ok((time, caps)) = runtime::get_time_and_caps() {
                pg.draw_text(time_x, time_y, &format!("{:?}", time), 0xFFFFFF);
                pg.draw_text(time_x, time_y + 10, &format!("{:?}", caps), 0xFFFFFF);
            }
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.cpu_count = data.resources.cpu_count;
                self.cpu_usage = data.resources.cpu_usage;
                self.used_memory_mb = data.resources.used_memory_mb;
                self.total_memory_mb = data.resources.total_memory_mb;
                self.disk_read_kbps = data.resources.disk_read_kbps as u32;
                self.disk_write_kbps = data.resources.disk_write_kbps as u32;
                self.net_rx_kbps = data.resources.net_rx_kbps as u32;
                self.net_tx_kbps = data.resources.net_tx_kbps as u32;
                self.running_vms = data.vms.iter().filter(|v| v.state.contains("Running")).count();
                self.total_vms = data.vms.len();
                self.files_count = data.files.len();
                self.categories_count = data.categories.len();
            }
        }

        fn input(&mut self, _key: Key) {}

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Overview {
        fn name(&self) -> &str { "Overview" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::CUBE_WINDOW_RED_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 2. Apps Tab (Application Registry)
// =========================================================================
pub mod apps {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let content_top = 80usize;

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

            let is_selected = idx == ui.selected_app_idx;
            let border_color = if is_selected { 0x00FF00 } else { 0x666666 };
            let bg_color = if is_selected { 0x334433 } else { 0x333333 };

            pg.fill_rect(x, y, card_w, card_h, bg_color);
            pg.draw_rect_outline(x, y, card_w, card_h, border_color);

            pg.draw_icon(x + card_w / 2 - 20, y + 20, 32, 32, icon);
            pg.draw_text(x + 10, y + card_h - 20, name, 0xFFFFFF);

            let pos_info = alloc::format!("v{}", version);
            pg.draw_text(x + 10, y + 5, &pos_info, 0x888888);

            if is_selected {
                pg.draw_text(x + card_w - 30, y + 5, "[*]", 0xFFFF00);
            }
        }

        pg.draw_text(margin, height - 40, "Use ARROWS to navigate | ENTER to Launch | ESC to close Apps", 0x888888);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        let card_w = 100usize;
        let gutter = 12usize;
        let margin = 16usize;
        let (width, _) = if let Some(pg) = PixelGraphics::new() { pg.resolution() } else { (1024, 768) };
        let cols = (width.saturating_sub(margin * 2)) / (card_w + gutter);
        let cols = if cols == 0 { 1 } else { cols };

        match key {
            Key::Special(ScanCode::LEFT) => {
                if ui.selected_app_idx > 0 { ui.selected_app_idx -= 1; }
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                if ui.selected_app_idx + 1 < crate::apps::APP_REGISTRY.len() { ui.selected_app_idx += 1; }
                true
            }
            Key::Special(ScanCode::UP) => {
                if ui.selected_app_idx >= cols { ui.selected_app_idx -= cols; }
                true
            }
            Key::Special(ScanCode::DOWN) => {
                if ui.selected_app_idx + cols < crate::apps::APP_REGISTRY.len() { ui.selected_app_idx += cols; }
                true
            }
            Key::Printable(c) if matches!(char::from(c), '\r' | '\n') => {
                let (name, _, _, _) = crate::apps::APP_REGISTRY[ui.selected_app_idx];
                if let Some(app_ctx) = crate::env::SteppedApplicationContext::from_name(name) {
                    ui.add_app_window(app_ctx);
                }
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Apps {
        pub selected_app_idx: usize,
    }

    impl X_Apps {
        pub fn new() -> Self {
            Self {
                selected_app_idx: 0,
            }
        }
    }

    impl Runnable for X_Apps {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let margin = 16usize;
            let gutter = 12usize;
            let width = 800;

            pg.draw_text(x + margin, y + margin, "Application Registry", 0x00FF00);
            pg.draw_text(x + margin, y + margin + 20, "Select an app to launch it in a stepped context", 0xAAAAAA);

            let start_y = y + margin + 60;
            let card_w = 100usize;
            let card_h = 75usize;
            let cols = (width - margin * 2) / (card_w + gutter);
            let cols = if cols == 0 { 1 } else { cols };

            for (idx, (name, _, icon, version)) in crate::apps::APP_REGISTRY.iter().enumerate() {
                let col = idx % cols;
                let row = idx / cols;
                let card_x = x + margin + col * (card_w + gutter);
                let card_y = start_y + row * (card_h + gutter);

                let is_selected = idx == self.selected_app_idx;
                let border_color = if is_selected { 0x00FF00 } else { 0x444444 };
                let bg_color = if is_selected { 0x224422 } else { 0x111111 };

                pg.fill_rect(card_x, card_y, card_w, card_h, bg_color);
                pg.draw_rect_outline(card_x, card_y, card_w, card_h, border_color);

                pg.draw_icon(card_x + (card_w - 32) / 2, card_y + 10, 32, 32, icon);
                pg.draw_text(card_x + 5, card_y + 50, name, 0xFFFFFF);
                pg.draw_text(card_x + 5, card_y + 62, version, 0x888888);
            }
        }

        fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

        fn input(&mut self, key: Key) {
            let cols = (800 - 32) / (100 + 12);
            match key {
                Key::Special(ScanCode::LEFT) => {
                    if self.selected_app_idx > 0 { self.selected_app_idx -= 1; }
                }
                Key::Special(ScanCode::RIGHT) => {
                    if self.selected_app_idx + 1 < crate::apps::APP_REGISTRY.len() { self.selected_app_idx += 1; }
                }
                Key::Special(ScanCode::UP) => {
                    if self.selected_app_idx >= cols { self.selected_app_idx -= cols; }
                }
                Key::Special(ScanCode::DOWN) => {
                    if self.selected_app_idx + cols < crate::apps::APP_REGISTRY.len() { self.selected_app_idx += cols; }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Apps {
        fn name(&self) -> &str { "Apps" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 3. Virtual Machines Tab
// =========================================================================
pub mod vms {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 15usize;
        let content_top = 80usize;

        pg.draw_text(margin, content_top + margin + 4, "Virtual Machines", 0x00FF00);

        let create_btn_x = width - margin - 120;
        let create_btn_y = content_top + margin;
        pg.fill_rect(create_btn_x, create_btn_y, 120, 24, 0x008000);
        pg.draw_text(create_btn_x + 10, create_btn_y + 4, "[+] Create VM", 0xFFFFFF);

        let table_x = margin;
        let table_y = content_top + margin + 32;
        let table_w = core::cmp::min(width - margin * 2, 760);
        let table_h = core::cmp::min(height.saturating_sub(table_y + 120), 260);
        pg.draw_rect_outline(table_x, table_y, table_w, table_h, 0x888888);

        pg.fill_rect(table_x + 1, table_y + 1, table_w - 2, line_h, 0x333333);
        pg.draw_text(table_x + 8, table_y + 4, "ID  NAME             STATE       CPU  MEM    UPTIME", 0xCCCCCC);

        let mut y = table_y + line_h + gutter;
        for (idx, vm) in ui.vms.iter().enumerate() {
            if y + line_h > table_y + table_h - 2 { break; }
            let is_selected = idx == ui.selected_vm_idx;
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

        let props_x = table_x + table_w + gutter;
        let props_w = width.saturating_sub(props_x + margin);
        if props_w > 150 {
            let props_h = table_h;
            pg.draw_rect_outline(props_x, table_y, props_w, props_h, 0x888888);
            pg.draw_text_bg(props_x + 10, table_y - 4, "VM Properties", 0x00FF00, 0x222222);

            if let Some(vm) = ui.vms.get(ui.selected_vm_idx) {
                let mut py = table_y + 10;
                pg.draw_text(props_x + 10, py, &alloc::format!("Name: {}", vm.name), 0xFFFFFF);
                py += 20;
                pg.draw_text(props_x + 10, py, &alloc::format!("ID:   {}", vm.id), 0xCCCCCC);
                py += 20;
                pg.draw_text(props_x + 10, py, &alloc::format!("State: {}", vm.state), if vm.state.contains("Running") { 0x00FF00 } else { 0xFFFFFF });
                py += 20;
                pg.draw_text(props_x + 10, py, &alloc::format!("vCPUs: {}", vm.cpu_usage), 0xCCCCCC);
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

        if !ui.vms.is_empty() {
            let actions_y = table_y + table_h + gutter;
            pg.draw_text(margin, actions_y, "Actions for Selected VM:", 0xCCCCCC);
            let actions = ["Start", "Stop", "Reset", "Zero", "Delete", "Save", "Restore", "Console"];
            let mut action_x = margin;
            let action_y = actions_y + 20;
            for (idx, action) in actions.iter().enumerate() {
                let is_focused = idx == ui.vm_action_idx;
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

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Special(ScanCode::UP) => {
                if ui.selected_vm_idx > 0 { ui.selected_vm_idx -= 1; }
                true
            }
            Key::Special(ScanCode::DOWN) => {
                if ui.selected_vm_idx < ui.vms.len().saturating_sub(1) { ui.selected_vm_idx += 1; }
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.vm_action_idx = ui.vm_action_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.vm_action_idx = (ui.vm_action_idx + 1).min(7);
                true
            }
            Key::Printable(c) if char::from(c) == ' ' => {
                ui.set_tab(DashboardTab::CreateVM);
                true
            }
            Key::Printable(c) if matches!(char::from(c), '\r' | '\n') => {
                if let Some(vm) = ui.vms.get(ui.selected_vm_idx) {
                    let vm_id = vm.id;
                    unsafe {
                        if let Some(hv) = HYPERVISOR.as_mut() {
                            match ui.vm_action_idx {
                                0 => { let _ = hv.start_vm(vm_id); }
                                1 => { let _ = hv.stop_vm(vm_id); }
                                2 => { let _ = hv.reset_vm(vm_id); }
                                3 => { let _ = hv.zero_vm(vm_id); }
                                4 => { let _ = hv.delete_vm(vm_id); }
                                5 => { let _ = hv.save_vm_metadata("/VMSTATE"); }
                                6 => { let _ = hv.restore_vm_metadata("/VMSTATE"); }
                                _ => {}
                            }
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_VMs {
        pub selected_vm_idx: usize,
        pub vm_action_idx: usize,
        pub vms: Vec<VmDisplayInfo>,
    }

    impl X_VMs {
        pub fn new() -> Self {
            Self {
                selected_vm_idx: 0,
                vm_action_idx: 0,
                vms: Vec::new(),
            }
        }
    }

    impl Runnable for X_VMs {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let margin = 16usize;
            let gutter = 12usize;
            let line_h = 15usize;
            let width = 800;
            let height = 600usize;

            pg.draw_text(x + margin, y + margin, "Virtual Machines", 0x00FF00);
            
            // VM Table
            let table_x = x + margin;
            let table_y = y + margin + 30;
            let table_w = core::cmp::min(width - margin * 2, 600);
            let table_h = height.saturating_sub(margin + 30 + 120);
            pg.draw_rect_outline(table_x, table_y, table_w, table_h, 0xCCCCCC);
            
            // Header
            pg.fill_rect(table_x + 1, table_y + 1, table_w - 2, line_h, 0x333333);
            pg.draw_text(table_x + 8, table_y + 4, "ID  NAME             STATE       CPU%  MEM(MB)  UPTIME", 0xAAAAAA);
            
            let mut curr_y = table_y + line_h + 4;
            for (i, vm) in self.vms.iter().enumerate() {
                if curr_y + line_h > table_y + table_h { break; }
                let is_selected = i == self.selected_vm_idx;
                let text_color = if is_selected { 0xFFFF00 } else { 0xFFFFFF };
                if is_selected {
                    pg.fill_rect(table_x + 1, curr_y, table_w - 2, line_h, 0x444444);
                }

                let uptime = if vm.uptime_seconds < 60 {
                    format!("{}s", vm.uptime_seconds)
                } else if vm.uptime_seconds < 3600 {
                    format!("{}m {}s", vm.uptime_seconds / 60, vm.uptime_seconds % 60)
                } else {
                    format!("{}h {}m", vm.uptime_seconds / 3600, (vm.uptime_seconds % 3600) / 60)
                };
                let info = format!("{:<3} {:<16} {:<11} {:>3}% {:>5}MB  {:>10}",
                    vm.id, vm.name, vm.state, vm.cpu_usage, vm.memory_usage_mb, uptime);
                pg.draw_text(table_x + 8, curr_y, &info, text_color);
                curr_y += line_h;
            }

            // Properties Panel
            let props_x = table_x + table_w + gutter;
            let props_w = width.saturating_sub(props_x + margin);
            if props_w > 150 {
                pg.draw_rect_outline(props_x, table_y, props_w, table_h, 0x888888);
                pg.draw_text_bg(props_x + 10, table_y - 4, "VM Properties", 0x00FF00, 0x222222);
                
                if let Some(vm) = self.vms.get(self.selected_vm_idx) {
                    let mut py = table_y + 10;
                    pg.draw_text(props_x + 10, py, &format!("Name: {}", vm.name), 0xFFFFFF);
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("ID:   {}", vm.id), 0xCCCCCC);
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("State: {}", vm.state), if vm.state.contains("Running") { 0x00FF00 } else { 0xFFFFFF });
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("vCPUs: {}", vm.cpu_usage), 0xCCCCCC);
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("RAM:   {} MB", vm.memory_usage_mb), 0xCCCCCC);
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("Disk:  {} MB", vm.disk_usage_mb), 0xCCCCCC);
                    py += 20;
                    pg.draw_text(props_x + 10, py, &format!("Uptime: {}s", vm.uptime_seconds), 0x888888);
                } else {
                    pg.draw_text(props_x + 10, table_y + 10, "No VM selected", 0x888888);
                }
            }

            // Actions Bar
            if !self.vms.is_empty() {
                let actions_y = table_y + table_h + gutter;
                pg.draw_text(x + margin, actions_y, "Actions for Selected VM:", 0xCCCCCC);
                let actions = ["Start", "Stop", "Reset", "Zero", "Delete", "Save", "Restore", "Console"];
                let mut action_x = x + margin;
                let action_y = actions_y + 20;
                for (idx, action) in actions.iter().enumerate() {
                    let is_focused = idx == self.vm_action_idx;
                    let color = if is_focused { 0x00AA00 } else { 0x444444 };
                    pg.fill_rect(action_x, action_y, 78, 24, color);
                    pg.draw_text(action_x + 8, action_y + 4, action, 0xFFFFFF);
                    action_x += 88;
                }
                pg.draw_text(x + margin, action_y + 32, "Press ENTER to execute action | SPACE to Create VM", 0x888888);
            } else {
                pg.draw_text(x + margin, table_y + table_h + gutter, "No VMs. Press SPACE to Create VM", 0x888888);
            }
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.vms = data.vms.clone();
                self.selected_vm_idx = self.selected_vm_idx.min(self.vms.len().saturating_sub(1));
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Special(ScanCode::UP) => {
                    if self.selected_vm_idx > 0 { self.selected_vm_idx -= 1; }
                }
                Key::Special(ScanCode::DOWN) => {
                    if self.selected_vm_idx + 1 < self.vms.len() { self.selected_vm_idx += 1; }
                }
                Key::Special(ScanCode::LEFT) => {
                    if self.vm_action_idx > 0 { self.vm_action_idx -= 1; }
                }
                Key::Special(ScanCode::RIGHT) => {
                    if self.vm_action_idx < 7 { self.vm_action_idx += 1; }
                }
                Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                    if let Some(vm) = self.vms.get(self.selected_vm_idx) {
                        let vm_id = vm.id;
                        unsafe {
                            if let Some(hv) = crate::HYPERVISOR.as_mut() {
                                match self.vm_action_idx {
                                    0 => { let _ = hv.start_vm(vm_id); }
                                    1 => { let _ = hv.stop_vm(vm_id); }
                                    2 => { let _ = hv.reset_vm(vm_id); }
                                    3 => { let _ = hv.zero_vm(vm_id); }
                                    4 => { let _ = hv.delete_vm(vm_id); }
                                    5 => { let _ = hv.save_vm_metadata("/VMSTATE"); }
                                    6 => { let _ = hv.restore_vm_metadata("/VMSTATE"); }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_VMs {
        fn name(&self) -> &str { "Virtual Machines" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::CUBE_WINDOW_RED_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 4. Create VM Tab
// =========================================================================
pub mod createvm {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, _width: usize, _height: usize) {
        let margin = 16usize;
        let content_top = 80usize;

        pg.draw_text(margin, content_top + margin, "Create New Virtual Machine", 0x00FF00);

        let form_x = margin + 20;
        let mut form_y = content_top + margin + 40;

        pg.draw_text(form_x, form_y, "Name:", 0xFFFFFF);
        let name_focus = ui.create_vm_focus_idx == 0;
        pg.draw_rect_outline(form_x + 100, form_y - 4, 200, 24, if name_focus { 0xFFFF00 } else { 0x888888 });
        pg.draw_text(form_x + 105, form_y, &ui.new_vm_name, 0xFFFFFF);

        form_y += 40;
        pg.draw_text(form_x, form_y, "vCPUs:", 0xFFFFFF);
        let cpu_focus = ui.create_vm_focus_idx == 1;
        pg.draw_rect_outline(form_x + 100, form_y - 4, 100, 24, if cpu_focus { 0xFFFF00 } else { 0x888888 });
        pg.draw_text(form_x + 105, form_y, &alloc::format!("{}", ui.new_vm_vcpus), 0xFFFFFF);
        pg.draw_text(form_x + 210, form_y, "(Use + / - to change)", 0x888888);

        form_y += 40;
        pg.draw_text(form_x, form_y, "Memory (MB):", 0xFFFFFF);
        let mem_focus = ui.create_vm_focus_idx == 2;
        pg.draw_rect_outline(form_x + 100, form_y - 4, 100, 24, if mem_focus { 0xFFFF00 } else { 0x888888 });
        pg.draw_text(form_x + 105, form_y, &alloc::format!("{}", ui.new_vm_memory_mb), 0xFFFFFF);
        pg.draw_text(form_x + 210, form_y, "(Use + / - to change)", 0x888888);

        form_y += 60;
        let create_focused = ui.create_vm_focus_idx == 3;
        pg.fill_rect(form_x, form_y, 120, 32, if create_focused { 0x00AA00 } else { 0x006600 });
        pg.draw_text(form_x + 20, form_y + 8, "CREATE", 0xFFFFFF);

        let cancel_focused = ui.create_vm_focus_idx == 4;
        pg.fill_rect(form_x + 140, form_y, 120, 32, if cancel_focused { 0xAA0000 } else { 0x660000 });
        pg.draw_text(form_x + 20 + 140, form_y + 8, "CANCEL", 0xFFFFFF);

        pg.draw_text(margin, form_y + 50, "TAB to switch fields | ENTER to confirm | ESC to cancel", 0x888888);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                match ch {
                    ' ' => {
                        if ui.create_vm_focus_idx == 0 {
                            ui.new_vm_name.push(' ');
                        }
                    }
                    '+' | '=' => {
                        if ui.create_vm_focus_idx == 1 {
                            ui.new_vm_vcpus += 1;
                        } else if ui.create_vm_focus_idx == 2 {
                            ui.new_vm_memory_mb += 128;
                        }
                    }
                    '-' | '_' => {
                        if ui.create_vm_focus_idx == 1 {
                            ui.new_vm_vcpus = ui.new_vm_vcpus.saturating_sub(1).max(1);
                        } else if ui.create_vm_focus_idx == 2 {
                            ui.new_vm_memory_mb = ui.new_vm_memory_mb.saturating_sub(128).max(128);
                        }
                    }
                    '\u{08}' => {
                        if ui.create_vm_focus_idx == 0 {
                            ui.new_vm_name.pop();
                        }
                    }
                    '\t' => {
                        ui.create_vm_focus_idx = (ui.create_vm_focus_idx + 1) % 5;
                    }
                    '\r' | '\n' => {
                        if ui.create_vm_focus_idx == 3 {
                            unsafe {
                                if let Some(hv) = HYPERVISOR.as_mut() {
                                    let _ = hv.create_vm(&ui.new_vm_name, ui.new_vm_memory_mb, ui.new_vm_vcpus);
                                }
                            }
                            ui.selected_tab = DashboardTab::VirtualMachines;
                        } else if ui.create_vm_focus_idx == 4 {
                            ui.selected_tab = DashboardTab::VirtualMachines;
                        } else {
                            ui.create_vm_focus_idx = (ui.create_vm_focus_idx + 1) % 5;
                        }
                    }
                    'q' => {
                        ui.selected_tab = DashboardTab::VirtualMachines;
                    }
                    _ => {
                        if ui.create_vm_focus_idx == 0 && (ch.is_alphanumeric() || ch == '_') {
                            ui.new_vm_name.push(ch);
                        }
                    }
                }
                true
            }
            Key::Special(ScanCode::ESCAPE) => {
                ui.selected_tab = DashboardTab::VirtualMachines;
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_CreateVM {
        pub new_vm_name: String,
        pub new_vm_memory_mb: u32,
        pub new_vm_vcpus: u32,
        pub create_vm_focus_idx: usize,
    }

    impl X_CreateVM {
        pub fn new() -> Self {
            Self {
                new_vm_name: "new-vm".to_string(),
                new_vm_memory_mb: 512,
                new_vm_vcpus: 1,
                create_vm_focus_idx: 0,
            }
        }
    }

    impl Runnable for X_CreateVM {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            pg.draw_text(x + 20, y + 20, "Create New Virtual Machine", 0x00FF00);
            
            let mut curr_y = y + 60;
            pg.draw_text(x + 20, curr_y, &format!("Name: {}", self.new_vm_name), if self.create_vm_focus_idx == 0 { 0xFFFF00 } else { 0xFFFFFF });
            curr_y += 30;
            pg.draw_text(x + 20, curr_y, &format!("vCPUs: {}", self.new_vm_vcpus), if self.create_vm_focus_idx == 1 { 0xFFFF00 } else { 0xFFFFFF });
            curr_y += 30;
            pg.draw_text(x + 20, curr_y, &format!("Memory: {} MB", self.new_vm_memory_mb), if self.create_vm_focus_idx == 2 { 0xFFFF00 } else { 0xFFFFFF });
            curr_y += 50;

            let create_color = if self.create_vm_focus_idx == 3 { 0x00FF00 } else { 0x008000 };
            pg.fill_rect(x + 20, curr_y, 120, 30, create_color);
            pg.draw_text(x + 30, curr_y + 8, "CREATE", 0xFFFFFF);

            let cancel_color = if self.create_vm_focus_idx == 4 { 0xFF5555 } else { 0x880000 };
            pg.fill_rect(x + 160, curr_y, 120, 30, cancel_color);
            pg.draw_text(x + 170, curr_y + 8, "CANCEL", 0xFFFFFF);

            pg.draw_text(x + 20, curr_y + 50, "TAB to switch fields, ENTER to create, ESC to cancel", 0x888888);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

        fn input(&mut self, key: Key) {
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
                                unsafe {
                                    if let Some(hv) = crate::HYPERVISOR.as_mut() {
                                        let _ = hv.create_vm(&self.new_vm_name, self.new_vm_memory_mb, self.new_vm_vcpus);
                                    }
                                }
                            } else if self.create_vm_focus_idx == 4 {
                                self.create_vm_focus_idx = 0;
                            } else {
                                self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5;
                            }
                        }
                        _ => {
                            if self.create_vm_focus_idx == 0 && (ch.is_alphanumeric() || ch == '_') {
                                self.new_vm_name.push(ch);
                            }
                        }
                    }
                }
                Key::Special(ScanCode::ESCAPE) => {
                    // Cancel
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_CreateVM {
        fn name(&self) -> &str { "Create VM" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 5. Resources Tab
// =========================================================================
pub mod resources {
    use super::*;

    pub fn draw_resources_view(
        resources: &SystemResources,
        pg: &mut PixelGraphics,
        margin: usize,
        content_top: usize,
        width: usize,
        height: usize,
    ) {
        // Top sub-tab selector buttons
        pg.fill_rect(margin, content_top - 6, 110, 22, 0x007799);
        pg.draw_rect_outline(margin, content_top - 6, 110, 22, 0x00FFFF);
        pg.draw_text(margin + 12, content_top - 2, "Resources", 0xFFFFFF);

        pg.fill_rect(margin + 120, content_top - 6, 110, 22, 0x333333);
        pg.draw_rect_outline(margin + 120, content_top - 6, 110, 22, 0x666666);
        pg.draw_text(margin + 132, content_top - 2, "Processes", 0xAAAAAA);

        pg.fill_rect(margin + 240, content_top - 6, 120, 22, 0x333333);
        pg.draw_rect_outline(margin + 240, content_top - 6, 120, 22, 0x666666);
        pg.draw_text(margin + 250, content_top - 2, "System Info", 0xAAAAAA);

        let panel_x = margin;
        let panel_y = content_top + 22;
        let panel_w = width.saturating_sub(margin * 2).min(780);
        let panel_h = height.saturating_sub(panel_y + 36);

        // Summary banner box (matching processes tab styling)
        let banner_h = 42usize;
        pg.fill_rect(panel_x, panel_y, panel_w, banner_h, 0x181F2A);
        pg.draw_rect_outline(panel_x, panel_y, panel_w, banner_h, 0x0088AA);

        let mem_pct = if resources.total_memory_mb > 0 {
            (resources.used_memory_mb * 100 / resources.total_memory_mb)
        } else {
            0
        };
        let tsc_mhz = unsafe { crate::TSC_PER_US };

        let summary_line1 = format!(
            "CPU: {} Cores @ {}% Total Load | Memory: {} / {} MB ({}%) | Host Clock: {} MHz",
            resources.cpu_count,
            resources.cpu_usage,
            resources.used_memory_mb,
            resources.total_memory_mb,
            mem_pct,
            tsc_mhz,
        );
        let summary_line2 = format!(
            "Net RX: {} KB/s | Net TX: {} KB/s | Disk R: {} KB/s | Disk W: {} KB/s | GPU: {}% | FPS: {} ({}ms)",
            resources.net_rx_kbps,
            resources.net_tx_kbps,
            resources.disk_read_kbps,
            resources.disk_write_kbps,
            resources.gpu_usage,
            resources.fps,
            resources.frame_ms,
        );
        pg.draw_text(panel_x + 10, panel_y + 5, &summary_line1, 0x00FFFF);
        pg.draw_text(panel_x + 10, panel_y + 22, &summary_line2, 0xCCCCCC);

        // Content Area Columns
        let content_y = panel_y + banner_h + 8;
        let btn_h = 24usize;
        let content_h = panel_h.saturating_sub(banner_h + 8 + btn_h + 10);
        let col_gap = 8usize;
        let left_w = (panel_w.saturating_sub(col_gap)) / 2;
        let right_w = panel_w.saturating_sub(left_w + col_gap);
        let left_x = panel_x;
        let right_x = panel_x + left_w + col_gap;

        // ==========================================
        // LEFT COLUMN: CPU & Individual Core Graphs
        // ==========================================
        pg.fill_rect(left_x, content_y, left_w, content_h, 0x161B26);
        pg.draw_rect_outline(left_x, content_y, left_w, content_h, 0x0088AA);

        // Header
        pg.fill_rect(left_x, content_y, left_w, 22, 0x243042);
        pg.draw_rect_outline(left_x, content_y, left_w, 22, 0x4A607A);
        let cpu_hdr = format!("CPU Utilization ({} Cores - {}%)", resources.cpu_count, resources.cpu_usage);
        pg.draw_text(left_x + 8, content_y + 4, &cpu_hdr, 0x88CCFF);

        // Total CPU Graph
        let total_graph_y = content_y + 26;
        let total_graph_h = 36usize;
        let total_graph_w = left_w.saturating_sub(16);
        let total_graph_x = left_x + 8;
        pg.draw_text(total_graph_x, total_graph_y, "Total CPU Usage History:", 0x00FFFF);
        pg.fill_rect(total_graph_x, total_graph_y + 14, total_graph_w, total_graph_h, 0x0D1117);
        pg.draw_rect_outline(total_graph_x, total_graph_y + 14, total_graph_w, total_graph_h, 0x243447);
        pg.draw_line_graph(total_graph_x, total_graph_y + 14, total_graph_w, total_graph_h, &resources.cpu_history, 100, 0x00FF88, 60);

        // Individual Cores Grid Header
        let cores_label_y = total_graph_y + 14 + total_graph_h + 4;
        pg.draw_text(total_graph_x, cores_label_y, "Individual Core Graphs:", 0x88CCFF);

        // Grid of Individual Core Graphs
        let grid_x = left_x + 8;
        let grid_y = cores_label_y + 15;
        let grid_w = left_w.saturating_sub(16);
        let grid_h = (content_y + content_h).saturating_sub(grid_y + 6);
        let cores = (resources.cpu_count.max(1) as usize).min(16);
        let cols = if cores <= 1 { 1 } else if cores <= 4 { 2 } else if cores <= 8 { 2 } else { 4 };
        let rows = (cores + cols - 1) / cols;
        let cell_gap = 4usize;
        let cell_w = (grid_w.saturating_sub((cols - 1) * cell_gap)) / cols;
        let cell_h = (grid_h.saturating_sub((rows - 1) * cell_gap)) / rows;

        for i in 0..cores {
            let c = i % cols;
            let r = i / cols;
            let cx = grid_x + c * (cell_w + cell_gap);
            let cy = grid_y + r * (cell_h + cell_gap);
            let usage = if i < resources.cpu_core_usage.len() {
                resources.cpu_core_usage[i]
            } else {
                resources.cpu_usage
            };

            // Cell Container
            pg.fill_rect(cx, cy, cell_w, cell_h, 0x141A24);
            pg.draw_rect_outline(cx, cy, cell_w, cell_h, 0x0088AA);

            // Mini Header
            let hdr_h = 13usize.min(cell_h.saturating_sub(10));
            pg.fill_rect(cx + 1, cy + 1, cell_w.saturating_sub(2), hdr_h, 0x1E293B);
            pg.draw_text(cx + 3, cy + 1, &format!("C{}", i), 0x88CCFF);
            let ucol = if usage > 80 { 0xFF5555 } else if usage > 50 { 0xFFAA00 } else { 0x00FF88 };
            let ustr = format!("{:>3}%", usage);
            let u_off = cell_w.saturating_sub(ustr.len() * 8 + 4);
            if u_off > 20 {
                pg.draw_text(cx + u_off, cy + 1, &ustr, ucol);
            }

            // Core Line Graph
            if cell_h > hdr_h + 8 {
                let gx = cx + 2;
                let gy = cy + hdr_h + 2;
                let gw = cell_w.saturating_sub(4);
                let gh = cell_h.saturating_sub(hdr_h + 4);
                pg.fill_rect(gx, gy, gw, gh, 0x0C1017);
                pg.draw_rect_outline(gx, gy, gw, gh, 0x223040);
                if let Some(hist) = resources.cpu_core_history.get(i) {
                    pg.draw_line_graph(gx, gy, gw, gh, hist, 100, 0x00FF88, 60);
                }
            }
        }

        // ==========================================
        // RIGHT COLUMN: Memory, Network, Disk, GPU
        // ==========================================
        let card_gap = 6usize;
        let card_h = (content_h.saturating_sub(card_gap * 3)) / 4;

        // 1. Memory Card
        let ry0 = content_y;
        pg.fill_rect(right_x, ry0, right_w, card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry0, right_w, card_h, 0x0088AA);
        pg.fill_rect(right_x, ry0, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry0, right_w, 18, 0x4A607A);
        let mem_hdr = format!("Memory: {} / {} MB ({}%)", resources.used_memory_mb, resources.total_memory_mb, mem_pct);
        pg.draw_text(right_x + 6, ry0 + 3, &mem_hdr, 0x88CCFF);
        pg.draw_progress_bar(right_x + 6, ry0 + 20, right_w.saturating_sub(12), 5, mem_pct as usize, 100, 0x00CCFF);
        let mg_y = ry0 + 27;
        let mg_h = card_h.saturating_sub(31);
        let mg_w = right_w.saturating_sub(12);
        pg.fill_rect(right_x + 6, mg_y, mg_w, mg_h, 0x0D1117);
        pg.draw_rect_outline(right_x + 6, mg_y, mg_w, mg_h, 0x243447);
        pg.draw_line_graph(right_x + 6, mg_y, mg_w, mg_h, &resources.mem_history, 100, 0x00CCFF, 60);

        // 2. Network Card
        let ry1 = content_y + card_h + card_gap;
        pg.fill_rect(right_x, ry1, right_w, card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry1, right_w, card_h, 0x0088AA);
        pg.fill_rect(right_x, ry1, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry1, right_w, 18, 0x4A607A);
        let net_hdr = format!("Net Traffic (RX: {} KB/s | TX: {} KB/s)", resources.net_rx_kbps, resources.net_tx_kbps);
        pg.draw_text(right_x + 6, ry1 + 3, &net_hdr, 0x88CCFF);
        let sub_w = (right_w.saturating_sub(16)) / 2;
        let sub_h = card_h.saturating_sub(35);
        let sub_y1 = ry1 + 31;
        pg.draw_text(right_x + 6, ry1 + 19, "RX (Cyan)", 0x00FFFF);
        pg.fill_rect(right_x + 6, sub_y1, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(right_x + 6, sub_y1, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(right_x + 6, sub_y1, sub_w, sub_h, &resources.net_rx_history, 1024, 0x00FFFF, 60);

        let tx_x = right_x + 10 + sub_w;
        pg.draw_text(tx_x, ry1 + 19, "TX (Yellow)", 0xFFFF00);
        pg.fill_rect(tx_x, sub_y1, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(tx_x, sub_y1, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(tx_x, sub_y1, sub_w, sub_h, &resources.net_tx_history, 1024, 0xFFFF00, 60);

        // 3. Disk Card
        let ry2 = content_y + (card_h + card_gap) * 2;
        let sub_y2 = ry2 + 31;
        pg.fill_rect(right_x, ry2, right_w, card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry2, right_w, card_h, 0x0088AA);
        pg.fill_rect(right_x, ry2, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry2, right_w, 18, 0x4A607A);
        let disk_hdr = format!("Disk I/O (Read: {} KB/s | Write: {} KB/s)", resources.disk_read_kbps, resources.disk_write_kbps);
        pg.draw_text(right_x + 6, ry2 + 3, &disk_hdr, 0x88CCFF);
        pg.draw_text(right_x + 6, ry2 + 19, "Read (White)", 0xFFFFFF);
        pg.fill_rect(right_x + 6, sub_y2, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(right_x + 6, sub_y2, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(right_x + 6, sub_y2, sub_w, sub_h, &resources.disk_read_history, 1024, 0xFFFFFF, 60);

        let wr_x = right_x + 10 + sub_w;
        pg.draw_text(wr_x, ry2 + 19, "Write (Red)", 0xFF5555);
        pg.fill_rect(wr_x, sub_y2, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(wr_x, sub_y2, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(wr_x, sub_y2, sub_w, sub_h, &resources.disk_write_history, 1024, 0xFF4444, 60);

        // 4. GPU & Display Card
        let ry3 = content_y + (card_h + card_gap) * 3;
        let sub_y3 = ry3 + 31;
        pg.fill_rect(right_x, ry3, right_w, card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry3, right_w, card_h, 0x0088AA);
        pg.fill_rect(right_x, ry3, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry3, right_w, 18, 0x4A607A);
        let gpu_hdr = format!("GPU: {}% | FPS: {} | Frame: {}ms", resources.gpu_usage, resources.fps, resources.frame_ms);
        pg.draw_text(right_x + 6, ry3 + 3, &gpu_hdr, 0x88CCFF);
        pg.draw_text(right_x + 6, ry3 + 19, "GPU Load", 0xFFAA00);
        pg.fill_rect(right_x + 6, sub_y3, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(right_x + 6, sub_y3, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(right_x + 6, sub_y3, sub_w, sub_h, &resources.gpu_history, 100, 0xFF7700, 60);

        let fps_x = right_x + 10 + sub_w;
        pg.draw_text(fps_x, ry3 + 19, "FPS History", 0xFF66FF);
        pg.fill_rect(fps_x, sub_y3, sub_w, sub_h, 0x0D1117);
        pg.draw_rect_outline(fps_x, sub_y3, sub_w, sub_h, 0x243447);
        pg.draw_line_graph(fps_x, sub_y3, sub_w, sub_h, &resources.fps_history, 75, 0xFF44FF, 60);

        // Action Buttons Bar at bottom
        let btn_y = panel_y + panel_h.saturating_sub(34);
        pg.fill_rect(panel_x, btn_y, 110, btn_h, 0x007799);
        pg.draw_rect_outline(panel_x, btn_y, 110, btn_h, 0x00FFFF);
        pg.draw_text(panel_x + 8, btn_y + 4, "[>] Processes", 0xFFFFFF);

        pg.fill_rect(panel_x + 120, btn_y, 110, btn_h, 0x007799);
        pg.draw_rect_outline(panel_x + 120, btn_y, 110, btn_h, 0x00FFFF);
        pg.draw_text(panel_x + 128, btn_y + 4, "[>] SysInfo", 0xFFFFFF);

        pg.draw_text(panel_x + 245, btn_y + 4, "RIGHT / Tab: Cycle Sub-tabs | F1: Start Menu", 0x888888);
    }

    pub fn draw_sysinfo_view(
        resources: &SystemResources,
        pg: &mut PixelGraphics,
        margin: usize,
        content_top: usize,
        width: usize,
        height: usize,
    ) {
        let sysinfo = crate::hardware::sysinfo::SystemInformation::collect(resources);

        // Top sub-tab selector buttons
        pg.fill_rect(margin, content_top - 6, 110, 22, 0x333333);
        pg.draw_rect_outline(margin, content_top - 6, 110, 22, 0x666666);
        pg.draw_text(margin + 12, content_top - 2, "Resources", 0xAAAAAA);

        pg.fill_rect(margin + 120, content_top - 6, 110, 22, 0x333333);
        pg.draw_rect_outline(margin + 120, content_top - 6, 110, 22, 0x666666);
        pg.draw_text(margin + 132, content_top - 2, "Processes", 0xAAAAAA);

        pg.fill_rect(margin + 240, content_top - 6, 120, 22, 0x007799);
        pg.draw_rect_outline(margin + 240, content_top - 6, 120, 22, 0x00FFFF);
        pg.draw_text(margin + 250, content_top - 2, "System Info", 0xFFFFFF);

        let panel_x = margin;
        let panel_y = content_top + 22;
        let panel_w = width.saturating_sub(margin * 2).min(780);
        let panel_h = height.saturating_sub(panel_y + 36);

        // Summary banner box
        let banner_h = 58usize;
        pg.fill_rect(panel_x, panel_y, panel_w, banner_h, 0x181F2A);
        pg.draw_rect_outline(panel_x, panel_y, panel_w, banner_h, 0x0088AA);

        let banner_line1 = format!(
            "Host: {} | Firmware: {} v{} (UEFI {}) | Status: OPERATIONAL",
            sysinfo.boot_mode,
            sysinfo.fw_vendor,
            sysinfo.fw_revision,
            sysinfo.uefi_version,
        );
        let banner_line2 = format!(
            "CPU: {} | {} Cores @ {} MHz \n| Virt: {}",
            sysinfo.cpu_brand,
            sysinfo.cpu_cores,
            sysinfo.cpu_clock_mhz,
            sysinfo.virt_hardware_assist,
        );
        pg.draw_text(panel_x + 10, panel_y + 5, &banner_line1, 0x00FFFF);
        pg.draw_text(panel_x + 10, panel_y + 22, &banner_line2, 0xCCCCCC);

        // Content layout
        let content_y = panel_y + banner_h + 8;
        let btn_h = 24usize;
        let content_h = panel_h.saturating_sub(banner_h + 8 + btn_h + 10);
        let col_gap = 8usize;
        let left_w = (panel_w.saturating_sub(col_gap)) / 2;
        let right_w = panel_w.saturating_sub(left_w + col_gap);
        let left_x = panel_x;
        let right_x = panel_x + left_w + col_gap;

        let card_gap = 6usize;
        let left_card_h = (content_h.saturating_sub(card_gap * 2)) / 3;
        let right_card_h = (content_h.saturating_sub(card_gap * 3)) / 4;

        // LEFT 1: Processor & Microarchitecture
        let ly0 = content_y;
        pg.fill_rect(left_x, ly0, left_w, left_card_h, 0x161B26);
        pg.draw_rect_outline(left_x, ly0, left_w, left_card_h, 0x0088AA);
        pg.fill_rect(left_x, ly0, left_w, 20, 0x243042);
        pg.draw_rect_outline(left_x, ly0, left_w, 20, 0x4A607A);
        pg.draw_text(left_x + 8, ly0 + 4, "Processor & Microarchitecture", 0x88CCFF);
        
        pg.draw_text(left_x + 8, ly0 + 26, &format!("Model: {}", sysinfo.cpu_brand), 0xFFFFFF);
        pg.draw_text(left_x + 8, ly0 + 42, &format!("Vendor: {} | Clock: {} MHz", sysinfo.cpu_vendor, sysinfo.cpu_clock_mhz), 0xCCCCCC);
        pg.draw_text(left_x + 8, ly0 + 58, &format!("Cores: {} Physical \n| Threads: {} Logical \n| APs: {}", sysinfo.cpu_cores, sysinfo.cpu_threads, sysinfo.cpu_ap_count), 0x00FF88);
        let exts = format!("Exts: VMX:{} SVM:{} AVX2:{} SSE4.2:{} 64Bit:{}", 
            if sysinfo.cpu_vmx { "Y" } else { "N" },
            if sysinfo.cpu_svm { "Y" } else { "N" },
            if sysinfo.cpu_avx2 { "Y" } else { "N" },
            if sysinfo.cpu_sse42 { "Y" } else { "N" },
            if sysinfo.cpu_64bit { "Y" } else { "N" },
        );
        pg.draw_text(left_x + 8, ly0 + 110, &exts, 0x00FFFF);

        // LEFT 2: Firmware & Platform Architecture
        let ly1 = content_y + left_card_h + card_gap;
        pg.fill_rect(left_x, ly1, left_w, left_card_h, 0x161B26);
        pg.draw_rect_outline(left_x, ly1, left_w, left_card_h, 0x0088AA);
        pg.fill_rect(left_x, ly1, left_w, 20, 0x243042);
        pg.draw_rect_outline(left_x, ly1, left_w, 20, 0x4A607A);
        pg.draw_text(left_x + 8, ly1 + 4, "UEFI Firmware & Platform", 0x88CCFF);

        pg.draw_text(left_x + 8, ly1 + 26, &format!("Vendor: {}", sysinfo.fw_vendor), 0xFFFFFF);
        pg.draw_text(left_x + 8, ly1 + 42, &format!("Firmware Rev: {} | Spec Rev: {}", sysinfo.fw_revision, sysinfo.uefi_version), 0xCCCCCC);
        pg.draw_text(left_x + 8, ly1 + 58, &format!("Mode: {}", sysinfo.boot_mode), 0x00FF88);
        pg.draw_text(left_x + 8, ly1 + 74, &format!("Paging: {}", sysinfo.paging_mode), 0x88CCFF);

        // LEFT 3: Hypervisor Core & Virtualization
        let ly2 = content_y + (left_card_h + card_gap) * 2;
        let left_last_h = (content_y + content_h).saturating_sub(ly2);
        pg.fill_rect(left_x, ly2, left_w, left_last_h, 0x161B26);
        pg.draw_rect_outline(left_x, ly2, left_w, left_last_h, 0x0088AA);
        pg.fill_rect(left_x, ly2, left_w, 20, 0x243042);
        pg.draw_rect_outline(left_x, ly2, left_w, 20, 0x4A607A);
        pg.draw_text(left_x + 8, ly2 + 4, "Hypervisor & Virtualization", 0x88CCFF);

        pg.draw_text(left_x + 8, ly2 + 26, &format!("Engine: {}", sysinfo.hypervisor_engine), 0x00FFFF);
        pg.draw_text(left_x + 8, ly2 + 42, &format!("Virt Assist: {}", sysinfo.virt_hardware_assist), 0xFFFFFF);
        pg.draw_text(left_x + 8, ly2 + 58, &format!("Virtual Machines: {} Total ({} Running)", sysinfo.vm_count, sysinfo.vm_running), 0x00FF88);

        // RIGHT 1: Memory Subsystem
        let ry0 = content_y;
        pg.fill_rect(right_x, ry0, right_w, right_card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry0, right_w, right_card_h, 0x0088AA);
        pg.fill_rect(right_x, ry0, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry0, right_w, 18, 0x4A607A);
        pg.draw_text(right_x + 6, ry0 + 3, "Memory Subsystem", 0x88CCFF);
        let mem_pct = if sysinfo.total_memory_mb > 0 { (sysinfo.used_memory_mb * 100 / sysinfo.total_memory_mb) as usize } else { 0 };
        pg.draw_text(right_x + 6, ry0 + 22, &format!("Total: {} MB | Used: {} MB | Free: {} MB", sysinfo.total_memory_mb, sysinfo.used_memory_mb, sysinfo.free_memory_mb), 0xFFFFFF);
        pg.draw_progress_bar(right_x + 6, ry0 + 38, right_w.saturating_sub(12), 6, mem_pct, 100, 0x00CCFF);

        // RIGHT 2: GPU & Display Subsystem
        let ry1 = content_y + right_card_h + card_gap;
        pg.fill_rect(right_x, ry1, right_w, right_card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry1, right_w, right_card_h, 0x0088AA);
        pg.fill_rect(right_x, ry1, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry1, right_w, 18, 0x4A607A);
        pg.draw_text(right_x + 6, ry1 + 3, "GPU & Display Graphics (GOP)", 0x88CCFF);
        pg.draw_text(right_x + 6, ry1 + 22, &format!("Device: {}", sysinfo.gpu_device_name), 0xFFFFFF);
        pg.draw_text(right_x + 6, ry1 + 36, &format!("Resolution: {}x{} (Stride: {} px)", sysinfo.display_res.0, sysinfo.display_res.1, sysinfo.display_stride), 0xCCCCCC);
        pg.draw_text(right_x + 6, ry1 + 50, &format!("GPU Load: {}% | Rate: {} FPS ({} ms)", sysinfo.gpu_usage, sysinfo.fps, sysinfo.frame_ms), 0xFFAA00);

        // RIGHT 3: Storage Subsystem & Disks
        let ry2 = content_y + (right_card_h + card_gap) * 2;
        pg.fill_rect(right_x, ry2, right_w, right_card_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry2, right_w, right_card_h, 0x0088AA);
        pg.fill_rect(right_x, ry2, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry2, right_w, 18, 0x4A607A);
        pg.draw_text(right_x + 6, ry2 + 3, "Storage & Disk I/O", 0x88CCFF);
        pg.draw_text(right_x + 6, ry2 + 22, &format!("Volumes: {} Mounted | I/O: R:{} KB/s W:{} KB/s", sysinfo.volume_count, sysinfo.disk_read_kbps, sysinfo.disk_write_kbps), 0xFFFFFF);
        pg.draw_text(right_x + 6, ry2 + 36, &format!("Read: {} B ({} ops) | Write: {} B ({} ops)", sysinfo.disk_read_total_bytes, sysinfo.disk_read_ops, sysinfo.disk_write_total_bytes, sysinfo.disk_write_ops), 0xCCCCCC);

        // RIGHT 4: Network Stack & Interface
        let ry3 = content_y + (right_card_h + card_gap) * 3;
        let right_last_h = (content_y + content_h).saturating_sub(ry3);
        pg.fill_rect(right_x, ry3, right_w, right_last_h, 0x161B26);
        pg.draw_rect_outline(right_x, ry3, right_w, right_last_h, 0x0088AA);
        pg.fill_rect(right_x, ry3, right_w, 18, 0x243042);
        pg.draw_rect_outline(right_x, ry3, right_w, 18, 0x4A607A);
        pg.draw_text(right_x + 6, ry3 + 3, "Network Interface & Stack", 0x88CCFF);
        pg.draw_text(right_x + 6, ry3 + 22, &format!("Backend: {} | MAC: {}", sysinfo.net_backend, sysinfo.net_mac), 0xFFFFFF);
        pg.draw_text(right_x + 6, ry3 + 36, &format!("IPv4: {} / {}", sysinfo.net_ip, sysinfo.net_mask), 0x00FFFF);
        pg.draw_text(right_x + 6, ry3 + 50, &format!("Traffic: RX:{} KB/s ({} pkts) | TX:{} KB/s ({} pkts)", sysinfo.net_rx_kbps, sysinfo.net_rx_pkts, sysinfo.net_tx_kbps, sysinfo.net_tx_pkts), 0xCCCCCC);

        // Action Buttons Bar at bottom
        let btn_y = panel_y + panel_h.saturating_sub(34);
        pg.fill_rect(panel_x, btn_y, 110, btn_h, 0x007799);
        pg.draw_rect_outline(panel_x, btn_y, 110, btn_h, 0x00FFFF);
        pg.draw_text(panel_x + 8, btn_y + 4, "[<] Resources", 0xFFFFFF);

        pg.fill_rect(panel_x + 120, btn_y, 110, btn_h, 0x007799);
        pg.draw_rect_outline(panel_x + 120, btn_y, 110, btn_h, 0x00FFFF);
        pg.draw_text(panel_x + 128, btn_y + 4, "[>] Processes", 0xFFFFFF);

        pg.draw_text(panel_x + 245, btn_y + 4, "LEFT / RIGHT / Tab: Cycle Sub-tabs | F1: Start Menu", 0x888888);
    }

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let content_top = 80usize;

        match ui.resmon_tab {
            ResourceMonitorTab::Resources => {
                draw_resources_view(&ui.resources, pg, margin, content_top, width, height);
            }
            ResourceMonitorTab::Processes => {
                ui.draw_processes_tab(pg, margin, content_top, width, height);
            }
            ResourceMonitorTab::SystemInfo => {
                draw_sysinfo_view(&ui.resources, pg, margin, content_top, width, height);
            }
        }
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                let code = u16::from(c);
                if code == 9 || code == b'\t' as u16 {
                    ui.resmon_tab = match ui.resmon_tab {
                        ResourceMonitorTab::Resources => ResourceMonitorTab::Processes,
                        ResourceMonitorTab::Processes => ResourceMonitorTab::SystemInfo,
                        ResourceMonitorTab::SystemInfo => ResourceMonitorTab::Resources,
                    };
                    return true;
                }
                let ch = char::from(c).to_ascii_lowercase();
                if matches!(ui.resmon_tab, ResourceMonitorTab::Processes) {
                    match ch {
                        'k' => { ui.kill_selected_process(); true }
                        'f' => { ui.focus_selected_process(); true }
                        'm' => { ui.toggle_min_selected_process(); true }
                        'r' => { ui.resmon_tab = ResourceMonitorTab::Resources; true }
                        'i' => { ui.resmon_tab = ResourceMonitorTab::SystemInfo; true }
                        '\r' | '\n' => { ui.focus_selected_process(); true }
                        _ => false,
                    }
                } else if matches!(ui.resmon_tab, ResourceMonitorTab::Resources) {
                    match ch {
                        'p' => { ui.resmon_tab = ResourceMonitorTab::Processes; true }
                        'i' => { ui.resmon_tab = ResourceMonitorTab::SystemInfo; true }
                        _ => false,
                    }
                } else if matches!(ui.resmon_tab, ResourceMonitorTab::SystemInfo) {
                    match ch {
                        'r' => { ui.resmon_tab = ResourceMonitorTab::Resources; true }
                        'p' => { ui.resmon_tab = ResourceMonitorTab::Processes; true }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.resmon_tab = match ui.resmon_tab {
                    ResourceMonitorTab::Resources => ResourceMonitorTab::Processes,
                    ResourceMonitorTab::Processes => ResourceMonitorTab::SystemInfo,
                    ResourceMonitorTab::SystemInfo => ResourceMonitorTab::Resources,
                };
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.resmon_tab = match ui.resmon_tab {
                    ResourceMonitorTab::Resources => ResourceMonitorTab::SystemInfo,
                    ResourceMonitorTab::Processes => ResourceMonitorTab::Resources,
                    ResourceMonitorTab::SystemInfo => ResourceMonitorTab::Processes,
                };
                true
            }
            Key::Special(ScanCode::UP) => {
                if matches!(ui.resmon_tab, ResourceMonitorTab::Processes) {
                    ui.selected_process_idx = ui.selected_process_idx.saturating_sub(1);
                    true
                } else {
                    false
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if matches!(ui.resmon_tab, ResourceMonitorTab::Processes) {
                    let total = ui.total_process_count();
                    ui.selected_process_idx = (ui.selected_process_idx + 1).min(total.saturating_sub(1));
                    true
                } else {
                    false
                }
            }
            Key::Special(ScanCode::DELETE) => {
                if matches!(ui.resmon_tab, ResourceMonitorTab::Processes) {
                    ui.kill_selected_process();
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct ProcessItem {
        pub pid: usize,
        pub name: String,
        pub state: String,
        pub is_minimized: bool,
        pub is_focused: bool,
        pub cpu_time: usize,
        pub ui_time: usize,
        pub win_w: usize,
        pub win_h: usize,
        pub win_x: usize,
        pub win_y: usize,
        pub is_maximized: bool,
    }

    #[derive(Clone)]
    pub struct X_Resources {
        pub resources: SystemResources,
        pub resmon_tab: ResourceMonitorTab,
        pub procs: Vec<ProcessItem>,
        pub vms: Vec<VmDisplayInfo>,
        pub cycles: usize,
        pub selected_process_idx: usize,
    }

    impl X_Resources {
        pub fn new() -> Self {
            let cores = crate::hardware::cpu::core_count().max(1);
            Self {
                resources: SystemResources {
                    cpu_count: cores,
                    cpu_usage: 0,
                    total_memory_mb: 0,
                    used_memory_mb: 0,
                    disk_read_kbps: 0,
                    disk_write_kbps: 0,
                    net_rx_kbps: 0,
                    net_tx_kbps: 0,
                    gpu_usage: 0,
                    fps: 0,
                    frame_ms: 0,
                    mem_history: vec![],
                    disk_read_history: vec![],
                    disk_write_history: vec![],
                    net_rx_history: vec![],
                    net_tx_history: vec![],
                    gpu_history: vec![],
                    fps_history: vec![],
                    ft_ms_history: vec![],
                    cpu_core_usage: alloc::vec![0; cores as usize],
                    cpu_core_history: alloc::vec![Vec::with_capacity(100); cores as usize],
                    cpu_history: vec![],
                },
                resmon_tab: ResourceMonitorTab::Resources,
                procs: Vec::new(),
                vms: Vec::new(),
                cycles: 0,
                selected_process_idx: 0,
            }
        }

        pub fn total_process_count(&self) -> usize {
            2 + self.procs.len() + self.vms.len()
        }
    }

    impl Runnable for X_Resources {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let margin = 16usize;
            let gutter = 12usize;
            let line_h = 15usize;
            let content_top = y + 12;
            let width = 760;
            let height = 540;
            match self.resmon_tab {
                ResourceMonitorTab::Resources => {
                    draw_resources_view(&self.resources, pg, margin, content_top, width, height);
                }
                ResourceMonitorTab::Processes => {
                    // Top sub-tab selector buttons
                    pg.fill_rect(margin, content_top - 6, 110, 22, 0x333333);
                    pg.draw_rect_outline(margin, content_top - 6, 110, 22, 0x666666);
                    pg.draw_text(margin + 12, content_top - 2, "Resources", 0xAAAAAA);

                    pg.fill_rect(margin + 120, content_top - 6, 110, 22, 0x007799);
                    pg.draw_rect_outline(margin + 120, content_top - 6, 110, 22, 0x00FFFF);
                    pg.draw_text(margin + 132, content_top - 2, "Processes", 0xFFFFFF);

                    let panel_x = margin;
                    let panel_y = content_top + 22;
                    let panel_w = 720usize;
                    let panel_h = 460usize;

                    // Summary banner box
                    let banner_h = 42usize;
                    pg.fill_rect(panel_x, panel_y, panel_w, banner_h, 0x181F2A);
                    pg.draw_rect_outline(panel_x, panel_y, panel_w, banner_h, 0x0088AA);

                    let running_apps_count = self.procs.iter().filter(|a| !a.is_minimized).count();
                    let min_apps_count = self.procs.len().saturating_sub(running_apps_count);
                    let total_procs = self.total_process_count();

                    let summary_line1 = format!(
                        "Total Processes: {} | Active Windows: {} ({} Minimized) | VMs: {}",
                        total_procs,
                        self.procs.len(),
                        min_apps_count,
                        self.vms.len(),
                    );
                    let tsc_mhz = unsafe { TSC_PER_US };
                    let summary_line2 = format!(
                        "CPU Load: {}% | Memory: {} / {} MB | Host Clock: {} MHz",
                        self.resources.cpu_usage,
                        self.resources.used_memory_mb,
                        self.resources.total_memory_mb,
                        tsc_mhz,
                    );
                    pg.draw_text(panel_x + 10, panel_y + 5, &summary_line1, 0x00FFFF);
                    pg.draw_text(panel_x + 10, panel_y + 22, &summary_line2, 0xCCCCCC);

                    // Table Header
                    let table_y = panel_y + banner_h + 10;
                    let header_h = 24usize;
                    pg.fill_rect(panel_x, table_y, panel_w, header_h, 0x243042);
                    pg.draw_rect_outline(panel_x, table_y, panel_w, header_h, 0x4A607A);

                    // Column offsets
                    let col_pid = panel_x + 8;
                    let col_name = panel_x + 60;
                    let col_state = panel_x + 240;
                    let col_cpu = panel_x + 340;
                    let col_cyc = panel_x + 410;
                    let col_mem = panel_x + 500;

                    pg.draw_text(col_pid, table_y + 5, "PID", 0x88CCFF);
                    pg.draw_text(col_name, table_y + 5, "Process Name", 0x88CCFF);
                    pg.draw_text(col_state, table_y + 5, "State", 0x88CCFF);
                    pg.draw_text(col_cpu, table_y + 5, "CPU %", 0x88CCFF);
                    pg.draw_text(col_cyc, table_y + 5, "Cycles", 0x88CCFF);
                    pg.draw_text(col_mem, table_y + 5, "Window / Memory", 0x88CCFF);

                    // Process Rows
                    let row_h = 24usize;
                    let max_visible_rows = (panel_h.saturating_sub(banner_h + 10 + header_h + 50)) / row_h;
                    let rows_start_y = table_y + header_h + 2;

                    for i in 0..total_procs.min(max_visible_rows) {
                        let cur_y = rows_start_y + i * row_h;
                        let is_selected = i == self.selected_process_idx;

                        // Background
                        if is_selected {
                            pg.fill_rect(panel_x, cur_y, panel_w, row_h - 2, 0x004488);
                            pg.draw_rect_outline(panel_x, cur_y, panel_w, row_h - 2, 0x00FFFF);
                        } else if i % 2 == 0 {
                            pg.fill_rect(panel_x, cur_y, panel_w, row_h - 2, 0x16161E);
                        } else {
                            pg.fill_rect(panel_x, cur_y, panel_w, row_h - 2, 0x1F1F2A);
                        }

                        let (pid_str, name_str, state_str, state_col, cpu_str, cyc_str, mem_str) = if i == 0 {
                            (
                                "0".to_string(),
                                "HPVMx Hypervisor".to_string(),
                                "Running".to_string(),
                                0x55FF55,
                                format!("{}%", self.resources.cpu_usage),
                                format!("{:.1}M", self.cycles as f64 / 1_000_000.0),
                                "Kernel Ring 0".to_string(),
                            )
                        } else if i == 1 {
                            (
                                "1".to_string(),
                                "Hardware & Timers".to_string(),
                                "Active".to_string(),
                                0x55FF55,
                                "1%".to_string(),
                                format!("{}K", tsc_mhz),
                                "Hardware I/O".to_string(),
                            )
                        } else if i < 2 + self.procs.len() {
                            let app_idx = i - 2;
                            let app = &self.procs[app_idx];
                            let is_min = app.is_minimized;
                            let is_foc = app.is_focused;
                            let (st, st_col) = if is_min {
                                ("Minimized".to_string(), 0xFFAA00)
                            } else if is_foc {
                                ("Focused".to_string(), 0x00FFFF)
                            } else {
                                ("Running".to_string(), 0x55FF55)
                            };

                            let cpu_pct = ((app.cpu_time as u64 * self.resources.fps.max(1) as u64) / (tsc_mhz * 1_000_000).max(1)) * 100;
                            let total_cyc = app.cpu_time + app.ui_time;
                            let cyc_formatted = if total_cyc >= 1_000_000 {
                                format!("{:.1}M", total_cyc as f64 / 1_000_000.0)
                            } else if total_cyc >= 1_000 {
                                format!("{:.1}K", total_cyc as f64 / 1000.0)
                            } else {
                                format!("{}", total_cyc)
                            };

                            let win_info = if app.is_maximized {
                                "Maximized".to_string()
                            } else if is_min {
                                "Minimized (BG)".to_string()
                            } else {
                                format!("{}x{} @ {},{}", app.win_w, app.win_h, app.win_x, app.win_y)
                            };

                            (
                                format!("{}", app.pid),
                                app.name.clone(),
                                st,
                                st_col,
                                format!("{}%", cpu_pct.min(100)),
                                cyc_formatted,
                                win_info,
                            )
                        } else {
                            let vm_idx = i - 2 - self.procs.len();
                            let vm = &self.vms[vm_idx];
                            let is_run = vm.state.contains("running");
                            let st_col = if is_run { 0x55FF55 } else { 0x888888 };
                            (
                                format!("{}", 100 + vm.id),
                                format!("VM: {}", vm.name),
                                vm.state.clone(),
                                st_col,
                                format!("{}%", vm.cpu_usage),
                                format!("{}s", vm.uptime_seconds),
                                format!("{} MB RAM", vm.memory_usage_mb),
                            )
                        };

                        let txt_col = if is_selected { 0xFFFFFF } else { 0xDDDDDD };
                        pg.draw_text(col_pid, cur_y + 4, &pid_str, 0x888888);
                        pg.draw_text(col_name, cur_y + 4, &name_str, txt_col);
                        pg.draw_text(col_state, cur_y + 4, &state_str, state_col);
                        pg.draw_text(col_cpu, cur_y + 4, &cpu_str, if cpu_str != "0%" { 0xFF6666 } else { 0x888888 });
                        pg.draw_text(col_cyc, cur_y + 4, &cyc_str, 0xAAAAAA);
                        pg.draw_text(col_mem, cur_y + 4, &mem_str, 0xAAAAAA);
                    }

                    // Action buttons bar at bottom
                    let btn_y = panel_y + panel_h.saturating_sub(38);
                    let btn_h = 24usize;

                    // [ End Task (K) ]
                    pg.fill_rect(panel_x, btn_y, 110, btn_h, 0x880000);
                    pg.draw_rect_outline(panel_x, btn_y, 110, btn_h, 0xFF4444);
                    pg.draw_text(panel_x + 10, btn_y + 5, "End Task (K)", 0xFFFFFF);

                    // [ Focus Window (F / ENTER) ]
                    let btn_focus_x = panel_x + 120;
                    pg.fill_rect(btn_focus_x, btn_y, 150, btn_h, 0x006688);
                    pg.draw_rect_outline(btn_focus_x, btn_y, 150, btn_h, 0x00CCFF);
                    pg.draw_text(btn_focus_x + 10, btn_y + 5, "Focus Win (F/RET)", 0xFFFFFF);

                    // [ Toggle Min (M) ]
                    let btn_min_x = btn_focus_x + 160;
                    pg.fill_rect(btn_min_x, btn_y, 130, btn_h, 0x555500);
                    pg.draw_rect_outline(btn_min_x, btn_y, 130, btn_h, 0xFFFF00);
                    pg.draw_text(btn_min_x + 10, btn_y + 5, "Toggle Min (M)", 0xFFFFFF);

                    // Keybind hints footer
                    let hint_y = btn_y + btn_h + 4;
                    pg.draw_text(panel_x, hint_y, "UP/DOWN: Select | LEFT/RIGHT: Tab | DEL/K: Kill | F/ENTER: Focus | M: Minimize", 0x6688AA);
                }
                ResourceMonitorTab::SystemInfo => {
                    draw_sysinfo_view(&self.resources, pg, margin, content_top, width, height);
                }
            }
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.resources = data.resources.clone();
                self.vms = data.vms.clone();
                self.cycles = data.iter as usize;
                let mut new_procs = Vec::new();
                for (idx, app) in data.active_apps.iter().enumerate() {
                    new_procs.push(ProcessItem {
                        pid: app.pid,
                        name: app.application.name.clone(),
                        state: if app.window.is_minimized { "Minimized".to_string() } else { "Running".to_string() },
                        is_minimized: app.window.is_minimized,
                        is_focused: data.focused_process_idx == Some(idx),
                        cpu_time: app.cpu_time,
                        ui_time: app.ui_time,
                        win_w: app.window.width,
                        win_h: app.window.height,
                        win_x: app.window.x,
                        win_y: app.window.y,
                        is_maximized: app.window.is_maximized,
                    });
                }
                self.procs = new_procs;
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Special(ScanCode::RIGHT) => {
                    self.resmon_tab = ResourceMonitorTab::Processes;
                }
                Key::Special(ScanCode::LEFT) => {
                    self.resmon_tab = ResourceMonitorTab::Resources;
                }
                Key::Special(ScanCode::UP) => {
                    if matches!(self.resmon_tab, ResourceMonitorTab::Processes) {
                        self.selected_process_idx = self.selected_process_idx.saturating_sub(1);
                    }
                }
                Key::Special(ScanCode::DOWN) => {
                    if matches!(self.resmon_tab, ResourceMonitorTab::Processes) {
                        let total = self.total_process_count();
                        self.selected_process_idx = (self.selected_process_idx + 1).min(total.saturating_sub(1));
                    }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Resources {
        fn name(&self) -> &str { "Resources" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 6. Network Tab
// =========================================================================
pub mod network {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, _width: usize, _height: usize) {
        let x = 20usize;
        let mut y = 100usize;
        pg.draw_text(x, y, "Network Status", 0x00FF00);
        let net_stats = crate::devices::net_stack::stats();
        y += 30;
        pg.draw_text(x, y, &alloc::format!("Backend: {}", crate::devices::net_stack::backend_name()), 0xFFFFFF);
        y += 30;
        pg.draw_text(x, y, "Statistics:", 0xAAAAAA);

        let sub_x = x + 20;
        let mut sub_y = 180usize;
        pg.draw_text(sub_x, sub_y, &alloc::format!("RX Packets: {}", net_stats.rx_pkts), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("TX Packets: {}", net_stats.tx_pkts), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("RX Bytes:   {}", net_stats.rx_bytes), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("TX Bytes:   {}", net_stats.tx_bytes), 0xCCCCCC);

        sub_y += 100;
        let state = crate::devices::net_stack::get_state();
        pg.draw_text(sub_x, sub_y, &alloc::format!("IP: {}.{}.{}.{}", state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("GW: {}.{}.{}.{}", state.gateway[0], state.gateway[1], state.gateway[2], state.gateway[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("MASK: {}.{}.{}.{}", state.subnet_mask[0], state.subnet_mask[1], state.subnet_mask[2], state.subnet_mask[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", state.mac_addr[0], state.mac_addr[1], state.mac_addr[2], state.mac_addr[3], state.mac_addr[4], state.mac_addr[5]), 0xCCCCCC);
        sub_y += 40;
        let is_init = crate::devices::net_stack::is_initialized();
        pg.draw_text(sub_x, sub_y, &alloc::format!("Initialized: {is_init}"), 0xFFFFFF);
        sub_y += 35;

        pg.draw_text(sub_x, sub_y, &alloc::format!("Target: {}", ui.network_target), 0xCCCCCC);
        sub_y += 28;

        let actions = ["Net Up", "Status", "Ping", "LAN Scan", "HTTP On", "HTTP Off"];
        let mut action_x = sub_x;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == ui.selected_network_action_idx;
            pg.fill_rect(action_x, sub_y, 88, 24, if is_focused { 0x00AA00 } else { 0x444444 });
            pg.draw_text(action_x + 8, sub_y + 4, action, 0xFFFFFF);
            action_x += 96;
        }
        sub_y += 36;
        pg.draw_text(sub_x, sub_y, "LEFT/RIGHT chooses action, ENTER runs it, +/- cycles ping target", 0x888888);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &ui.status_line, 0xFFFF00);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                match char::from(c) {
                    '\r' | '\n' => {
                        ui.execute_network_action();
                        true
                    }
                    '+' | '=' => {
                        ui.network_target = String::from("192.168.1.1");
                        true
                    }
                    '-' | '_' => {
                        ui.network_target = String::from("127.0.0.1");
                        true
                    }
                    _ => false,
                }
            }
            Key::Special(ScanCode::LEFT) => {
                ui.selected_network_action_idx = ui.selected_network_action_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.selected_network_action_idx = (ui.selected_network_action_idx + 1).min(5);
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Network {
        pub selected_network_action_idx: usize,
        pub network_target: String,
    }

    impl X_Network {
        pub fn new() -> Self {
            Self {
                selected_network_action_idx: 0,
                network_target: String::new(),
            }
        }
    }

    impl Runnable for X_Network {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let x_off = x + 20;
            let mut y_off = y + 20;
            pg.draw_text(x_off, y_off, "Network Status", 0x00FF00);
            let net_stats = crate::devices::net_stack::stats();
            y_off += 30;
            pg.draw_text(x_off, y_off, &alloc::format!("Backend: {}", crate::devices::net_stack::backend_name()), 0xFFFFFF);
            y_off += 30;
            pg.draw_text(x_off, y_off, "Statistics:", 0xAAAAAA);

            let sub_x = x + 40;
            let mut sub_y = y + 100;
            pg.draw_text(sub_x, sub_y, &alloc::format!("RX Packets: {}", net_stats.rx_pkts), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("TX Packets: {}", net_stats.tx_pkts), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("RX Bytes:   {}", net_stats.rx_bytes), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("TX Bytes:   {}", net_stats.tx_bytes), 0xCCCCCC);

            sub_y += 40;
            let state = crate::devices::net_stack::get_state();
            pg.draw_text(sub_x, sub_y, &alloc::format!("IP: {}.{}.{}.{}", state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("GW: {}.{}.{}.{}", state.gateway[0], state.gateway[1], state.gateway[2], state.gateway[3]), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("MASK: {}.{}.{}.{}", state.subnet_mask[0], state.subnet_mask[1], state.subnet_mask[2], state.subnet_mask[3]), 0xCCCCCC);
            sub_y += 20;
            pg.draw_text(sub_x, sub_y, &alloc::format!("MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", state.mac_addr[0], state.mac_addr[1], state.mac_addr[2], state.mac_addr[3], state.mac_addr[4], state.mac_addr[5]), 0xCCCCCC);
            sub_y += 40;
            let is_init = crate::devices::net_stack::is_initialized();
            pg.draw_text(sub_x, sub_y, &alloc::format!("Initialized: {is_init}"), 0xFFFFFF);
            sub_y += 35;

            pg.draw_text(sub_x, sub_y, &alloc::format!("Target: {}", self.network_target), 0xCCCCCC);
            sub_y += 28;

            let actions = ["Net Up", "Status", "Ping", "LAN Scan", "HTTP On", "HTTP Off"];
            let mut action_x = sub_x;
            for (idx, action) in actions.iter().enumerate() {
                let is_focused = idx == self.selected_network_action_idx;
                pg.fill_rect(action_x, sub_y, 88, 24, if is_focused { 0x00AA00 } else { 0x444444 });
                pg.draw_text(action_x + 8, sub_y + 4, action, 0xFFFFFF);
                action_x += 96;
            }
            sub_y += 36;
            pg.draw_text(sub_x, sub_y, "LEFT/RIGHT chooses action, ENTER runs it, +/- cycles ping target", 0x888888);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                if self.network_target.is_empty() {
                    self.network_target = data.network_target.clone();
                }
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Special(ScanCode::LEFT) => {
                    if self.selected_network_action_idx > 0 { self.selected_network_action_idx -= 1; }
                }
                Key::Special(ScanCode::RIGHT) => {
                    if self.selected_network_action_idx < 5 { self.selected_network_action_idx += 1; }
                }
                Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                    match self.selected_network_action_idx {
                        0 => { let _ = crate::devices::net_hw::init(); }
                        1 => crate::devices::net::status(),
                        2 => { let _ = crate::devices::net::ping(&self.network_target, 4, 250); }
                        3 => crate::devices::net::lanscan("192.168.1."),
                        4 => crate::devices::net::httpd_start(8080),
                        5 => crate::devices::net::httpd_stop(),
                        _ => {}
                    }
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '+' || ch == '=' {
                        self.network_target = String::from("192.168.1.1");
                    } else if ch == '-' || ch == '_' {
                        self.network_target = String::from("127.0.0.1");
                    }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Network {
        fn name(&self) -> &str { "Network" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 7. Console Tab (Hypervisor Real-time Log & Command Line)
// =========================================================================
pub mod console {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        pg.draw_text(20, 100, "Hypervisor Real-time Log", 0x00FF00);
        let logs = crate::hpvmlog::get_logs();
        pg.draw_log_viewer(
            margin,
            130,
            width - margin * 2,
            height.saturating_sub(135 + margin * 8),
            &logs,
            ui.console_scroll_offset,
            ui.console_h_scroll_offset,
        );

        let y_msg = height.saturating_sub(margin * 6);
        pg.draw_text(margin, y_msg, "Use PgUp/PgDn to scroll logs, LEFT/RIGHT to scroll text, C to clear", 0x888888);

        pg.draw_rect_outline(margin, height.saturating_sub(95), width.saturating_sub(margin * 8), 35, 0x999999);
        if ui.term_selected {
            pg.draw_rect_outline_adv(margin - 1, height.saturating_sub(96), (width.saturating_sub(margin * 8)) + 2, 37, 0x888844, 3, 0x0F0F0F0F);
        }
        pg.draw_text(margin + 5, height.saturating_sub(60), "press enter to send, end to enter type mode, and esc to exit", 0x888888);
        pg.draw_text(margin + 5, height.saturating_sub(85), alloc::format!("HPVMx> {}", ui.term_buf).as_str(), 0xDDDDDD);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                if ui.term_selected {
                    let ch = char::from(c);
                    match ch {
                        '\u{8}' => { ui.term_buf.pop(); }
                        '\r' | '\n' => {
                            let unclean = ui.term_buf.trim();
                            let mut command = String::with_capacity(unclean.len());
                            let mut consecutive_backspaces = 0;
                            for ch_item in unclean.chars() {
                                if ch_item == '\u{8}' {
                                    consecutive_backspaces += 1;
                                    if consecutive_backspaces >= 2 {
                                        command.pop();
                                    }
                                    command.pop();
                                } else {
                                    consecutive_backspaces = 0;
                                    command.push(ch_item);
                                }
                            }

                            if !command.is_empty() {
                                let body = command.split(' ').collect::<Vec<&str>>();
                                let command_parts = command.split(' ').collect::<Vec<&str>>();
                                let parts = command_parts.clone();

                                terminal::cmd(command_parts, &parts, body, &mut ui.package_manager);
                                ui.command_history.push(command);
                                ui.history_idx = None;
                            }
                            ui.term_buf.clear();
                        }
                        _ => { ui.term_buf.push(ch); }
                    }
                    return true;
                }
                false
            }
            Key::Special(ScanCode::ESCAPE) => {
                if ui.term_selected {
                    ui.term_selected = false;
                    return true;
                }
                false
            }
            Key::Special(ScanCode::END) => {
                ui.term_selected = true;
                true
            }
            Key::Special(ScanCode::UP) => {
                if ui.term_selected && !ui.command_history.is_empty() {
                    let new_idx = match ui.history_idx {
                        Some(idx) => idx.saturating_sub(1),
                        None => ui.command_history.len().saturating_sub(1),
                    };
                    ui.history_idx = Some(new_idx);
                    ui.term_buf = ui.command_history[new_idx].clone();
                    return true;
                }
                ui.console_scroll_offset = ui.console_scroll_offset.saturating_add(1);
                true
            }
            Key::Special(ScanCode::DOWN) => {
                if ui.term_selected {
                    if let Some(idx) = ui.history_idx {
                        if idx + 1 < ui.command_history.len() {
                            let new_idx = idx + 1;
                            ui.history_idx = Some(new_idx);
                            ui.term_buf = ui.command_history[new_idx].clone();
                        } else {
                            ui.history_idx = None;
                            ui.term_buf.clear();
                        }
                    }
                    return true;
                }
                ui.console_scroll_offset = ui.console_scroll_offset.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.console_h_scroll_offset = ui.console_h_scroll_offset.saturating_add(1);
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.console_h_scroll_offset = ui.console_h_scroll_offset.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::PAGE_UP) => {
                ui.console_scroll_offset = ui.console_scroll_offset.saturating_add(5);
                true
            }
            Key::Special(ScanCode::PAGE_DOWN) => {
                ui.console_scroll_offset = ui.console_scroll_offset.saturating_sub(5);
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Console {
        pub term_buf: String,
        term_selected: bool,
        command_history: Vec<String>,
        history_idx: Option<usize>,
    }

    impl X_Console {
        pub fn new() -> Self {
            Self { term_buf: String::new(), term_selected: false, command_history: Vec::new(), history_idx: None }
        }
    }

    impl Runnable for X_Console {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            const WIDTH: usize = 800;
            const HEIGHT: usize = 600;
            const MARGIN: usize = 16;

            pg.draw_text(x + 20, y + 20, "Hypervisor Real-time Log", 0x00FF00);
            let logs = crate::hpvmlog::get_logs();
            pg.draw_log_viewer(x + MARGIN, y + 50, WIDTH - MARGIN * 2, HEIGHT - 135 - MARGIN * 8, &logs, 0, 0);

            let input_y = y + HEIGHT - 95;
            pg.draw_text(x + MARGIN, y + HEIGHT - MARGIN * 6, "Use PgUp/PgDn to scroll, C to clear", 0x888888);
            pg.draw_rect_outline(x + MARGIN, input_y, WIDTH - MARGIN * 8, 35, 0x999999);
            if self.term_selected {
                pg.draw_rect_outline_adv(x + MARGIN - 1, input_y - 1, WIDTH - MARGIN * 8 + 2, 37, 0x888844, 3, 0x0F0F0F0F);
            }
            pg.draw_text(x + MARGIN + 5, y + HEIGHT - 60, "TAB edits, ENTER sends, ESC leaves edit mode", 0x888888);
            pg.draw_text(x + MARGIN + 5, y + HEIGHT - 85, &alloc::format!("HPVMx> {}", self.term_buf), 0xDDDDDD);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

        fn input(&mut self, key: Key) {
            match key {
                Key::Printable(c) if char::from(c) == '\t' => self.term_selected = !self.term_selected,
                Key::Special(ScanCode::ESCAPE) => self.term_selected = false,
                Key::Special(ScanCode::UP) if self.term_selected && !self.command_history.is_empty() => {
                    let idx = self.history_idx.unwrap_or(self.command_history.len()).saturating_sub(1);
                    self.history_idx = Some(idx);
                    self.term_buf = self.command_history[idx].clone();
                }
                Key::Special(ScanCode::DOWN) if self.term_selected => {
                    if let Some(idx) = self.history_idx {
                        if idx + 1 < self.command_history.len() {
                            self.history_idx = Some(idx + 1);
                            self.term_buf = self.command_history[idx + 1].clone();
                        } else {
                            self.history_idx = None;
                            self.term_buf.clear();
                        }
                    }
                }
                Key::Printable(c) if self.term_selected => {
                    let ch = char::from(c);
                    match ch {
                        '\u{8}' => { self.term_buf.pop(); }
                        '\r' | '\n' => {
                            let command = self.term_buf.trim().to_string();
                            if !command.is_empty() {
                                let command_parts = command.split_whitespace().collect::<Vec<_>>();
                                let parts = command_parts.clone();
                                let body = command_parts.clone();
                                let mut package_manager = PackageManager::new();
                                crate::terminal::cmd(command_parts, &parts, body, &mut package_manager);
                                self.command_history.push(command);
                            }
                            self.history_idx = None;
                            self.term_buf.clear();
                        }
                        _ if !ch.is_control() => self.term_buf.push(ch),
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Console {
        fn name(&self) -> &str { "Console" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::COM_PORT_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 8. Devices Tab (Device Manager)
// =========================================================================
pub mod devices {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        pg.draw_text(20, 100, "Device Manager", 0x00FF00);

        let mut y = 130;
        let mut current_idx = 0;

        for cat in &ui.categories {
            let expanded_icon = if cat.expanded { "[-] " } else { "[+] " };
            let color = if current_idx == ui.selected_device_idx { 0xFFFF00 } else { 0xAAAAAA };
            pg.draw_text(20, y, &alloc::format!("{}{}{} ({})", expanded_icon, cat.icon, cat.name, cat.devices.len()), color);
            y += 20;
            current_idx += 1;

            if cat.expanded {
                for dev in &cat.devices {
                    let color = if current_idx == ui.selected_device_idx { 0xFFFF00 } else { 0xFFFFFF };
                    pg.draw_icon(35, y - 2, 16, 16, if cat.name == "Network Adapters" { &pixel_graphics::icons::PCI_GREEN_ICON_DATA } else { &pixel_graphics::icons::PCI_BLUE_ICON_DATA });

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

        let detail_x = width / 2;
        let detail_y = 130;
        let detail_w = (width / 2) - 20;
        let detail_h = height.saturating_sub(200);
        pg.draw_rect_outline(detail_x, detail_y, detail_w, detail_h, 0x888888);
        pg.draw_text_bg(detail_x + 10, detail_y - 4, "Device Properties", 0x00FF00, 0x222222);

        let mut current_search_idx = 0;
        let mut selected_device = None;
        for cat in &ui.categories {
            if current_search_idx == ui.selected_device_idx {
                break;
            }
            current_search_idx += 1;
            if cat.expanded {
                for dev in &cat.devices {
                    if current_search_idx == ui.selected_device_idx {
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

            let path = &dev.path;
            let mut parts = alloc::vec::Vec::new();
            let mut slash_count = 0;
            let mut last_split = 0;
            for (i, c) in path.char_indices() {
                if c == '/' {
                    slash_count += 1;
                    if slash_count == 3 {
                        parts.push(&path[..i + 1]);
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

            let chunk_size = (detail_w.saturating_sub(30)) / 8;
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
                if let Some(pci) = ui.pci_devices.iter().find(|p| {
                    alloc::format!("{:02X}:{:02X}.{}", p.bus, p.device, p.function) == dev.name
                }) {
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Vendor:   {}", pci.vendor_name()), 0xFFFFFF);
                    dy += 16;
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Device:   0x{:04X}", pci.device_id), 0xFFFFFF);
                    dy += 16;
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Class:    {}", pci.class_name()), 0xFFFFFF);
                    dy += 16;
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Revision: 0x{:02X}", pci.revision_id), 0xCCCCCC);
                    dy += 16;
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Interface: 0x{:02X}", pci.interface_id), 0xCCCCCC);
                    dy += 20;
                    pg.draw_text(detail_x + 10, dy, "Hardware Status: Online", 0x55FF55);
                } else {
                    pg.draw_text(detail_x + 10, dy, "Scanning for PCI Vendor/Device IDs...", 0x666666);
                }
            }
        } else {
            pg.draw_text(detail_x + 10, detail_y + 10, "Select a device to view properties", 0x888888);
        }

        let action_y = detail_y + detail_h + 20;
        let actions = ["Refresh List", "Scan PCI Bus", "Diagnostics", "Toggle Expanded"];
        let mut ax = 20;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == ui.device_action_idx && !ui.term_selected;
            pg.fill_rect(ax, action_y, 140, 26, if is_focused { 0x00AA00 } else { 0x444444 });
            pg.draw_text(ax + 8, action_y + 5, action, 0xFFFFFF);
            ax += 150;
        }
        if ui.device_action_idx == 1 {
            pg.draw_text(20, action_y + 35, "Scans the PCI bus using Port IO (0xCF8/0xCFC) to detect hardware", 0x00AAAA);
        }
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Special(ScanCode::UP) => {
                if ui.selected_device_idx > 0 { ui.selected_device_idx -= 1; }
                true
            }
            Key::Special(ScanCode::DOWN) => {
                let mut total_rows = 0;
                for cat in &ui.categories {
                    total_rows += 1;
                    if cat.expanded {
                        total_rows += cat.devices.len();
                    }
                }
                if ui.selected_device_idx < total_rows.saturating_sub(1) {
                    ui.selected_device_idx += 1;
                }
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.device_action_idx = ui.device_action_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.device_action_idx = (ui.device_action_idx + 1).min(3);
                true
            }
            Key::Printable(c) if matches!(char::from(c), '\r' | '\n') => {
                let mut current_idx = 0;
                let mut found = false;
                for i in 0..ui.categories.len() {
                    if current_idx == ui.selected_device_idx {
                        ui.categories[i].expanded = !ui.categories[i].expanded;
                        found = true;
                        break;
                    }
                    current_idx += 1;
                    if ui.categories[i].expanded {
                        for _ in &ui.categories[i].devices {
                            if current_idx == ui.selected_device_idx {
                                found = true;
                                break;
                            }
                            current_idx += 1;
                        }
                    }
                    if found { break; }
                }
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Devices {
        pub categories: Vec<DeviceCategory>,
        pub selected_device_idx: usize,
    }

    impl X_Devices {
        pub fn new() -> Self {
            Self {
                categories: Vec::new(),
                selected_device_idx: 0,
            }
        }
    }

    impl Runnable for X_Devices {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let x_off = x + 20;
            let mut y_off = y + 20;
            pg.draw_text(x_off, y_off, "Device Manager", 0x00FF00);

            let mut curr_y = y + 50;
            let mut current_idx = 0;

            for cat in &self.categories {
                let expanded_icon = if cat.expanded { "[-] " } else { "[+] " };
                let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xAAAAAA };
                pg.draw_text(x_off, curr_y, &alloc::format!("{}{}{} ({})", expanded_icon, cat.icon, cat.name, cat.devices.len()), color);
                curr_y += 20;
                current_idx += 1;

                if cat.expanded {
                    for dev in &cat.devices {
                        let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xFFFFFF };
                        pg.draw_icon(x_off + 15, curr_y - 2, 16, 16, if cat.name == "Network Adapters" { &pixel_graphics::icons::PCI_GREEN_ICON_DATA } else { &pixel_graphics::icons::PCI_BLUE_ICON_DATA });

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

                        let display_path = if let Some(idx) = split_idx { &path[idx..] } else { path };
                        pg.draw_text(x_off + 35, curr_y, &alloc::format!("{:<12} {}", dev.name, display_path), color);
                        curr_y += 18;
                        current_idx += 1;
                    }
                }
            }
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.categories = data.categories.clone();
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Special(ScanCode::UP) => {
                    if self.selected_device_idx > 0 { self.selected_device_idx -= 1; }
                }
                Key::Special(ScanCode::DOWN) => {
                    let mut total = 0;
                    for cat in &self.categories {
                        total += 1;
                        if cat.expanded { total += cat.devices.len(); }
                    }
                    if self.selected_device_idx + 1 < total { self.selected_device_idx += 1; }
                }
                Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                    // Dashboard handles expansion toggling after sync
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Devices {
        fn name(&self) -> &str { "Devices" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 9. Storage Tab (Disk & File Explorer)
// =========================================================================
pub mod storage {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 15usize;
        let content_top = 80usize;

        let base_y = content_top + margin;
        pg.draw_text(margin, base_y - 4, "Storage & Disk Explorer", 0x00FF00);

        let disk_tab_y = base_y + 12;
        let mut tab_x = margin;
        for (d_idx, disk) in ui.storage_disks.iter().enumerate() {
            let is_active = d_idx == ui.selected_disk_idx;
            let label_text = if !disk.volume_label.is_empty() {
                format!("{}: [{}]", disk.alias, disk.volume_label)
            } else if disk.total_bytes > 0 {
                let size_str = if disk.total_bytes >= 1024 * 1024 * 1024 {
                    format!("{}GB", disk.total_bytes / (1024 * 1024 * 1024))
                } else {
                    format!("{}MB", disk.total_bytes / (1024 * 1024))
                };
                format!("{}: ({})", disk.alias, size_str)
            } else {
                format!("{}: {}", disk.alias, disk.media_type)
            };

            let tab_w = (label_text.len() * 8 + 16).max(84);
            let bg = if is_active { 0x0055AA } else { 0x2A2A2A };
            let border = if is_active { 0x00AAFF } else { 0x555555 };
            let text_color = if is_active { 0xFFFFFF } else { 0xAAAAAA };

            pg.fill_rect(tab_x, disk_tab_y, tab_w, 20, bg);
            pg.draw_rect_outline(tab_x, disk_tab_y, tab_w, 20, border);
            pg.draw_text(tab_x + 8, disk_tab_y + 3, &label_text, text_color);

            tab_x += tab_w + 6;
        }

        let path_y = disk_tab_y + 24;
        pg.draw_text(margin, path_y, &alloc::format!("Path: {}", ui.current_path), 0x00EEEE);
        if let Some(disk) = ui.storage_disks.get(ui.selected_disk_idx) {
            let disk_summary = if disk.total_bytes > 0 {
                let total_mb = disk.total_bytes / (1024 * 1024);
                let free_mb = disk.free_bytes / (1024 * 1024);
                format!("Type: {} | Free: {} MB / {} MB", disk.media_type, free_mb, total_mb)
            } else {
                format!("Type: {}", disk.media_type)
            };
            pg.draw_text(margin + 360, path_y, &disk_summary, 0x88CC88);
        }

        let list_x = margin;
        let list_y = path_y + 18;
        let list_w = core::cmp::min(width - margin * 2, 720);
        let list_h = core::cmp::min(height.saturating_sub(list_y + 90), 440);
        pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);

        pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, line_h, 0x333333);
        pg.draw_text(list_x + 8, list_y + 4, "TYPE  NAME                                 SIZE (BYTES)  ATTR", 0xCCCCCC);
        pg.draw_line(list_x + 48, list_y + 1, list_x + 48, list_y + list_h - 1, 0x444444);
        pg.draw_line(list_x + 340, list_y + 1, list_x + 340, list_y + list_h - 1, 0x444444);

        let mut y = list_y + line_h + gutter;
        for (i, entry) in ui.files.iter().enumerate() {
            if y + line_h > list_y + list_h - 2 { break; }
            let color = if i == ui.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
            let icon = if entry.is_dir { pixel_graphics::icons::FOLDER_ICON_DATA } else {
                let dec_syn = ["json", "xml", "toml", "yaml", "yml"];
                let sys_syn = ["sys", "efi", "asm"];
                let prog_syn = ["micro", "ufe", "dmx", "bin", "rs"];

                let ext = entry.name.split('.').last().unwrap_or("");
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
            } else if entry.size / 1024 < 10000 {
                format!("{}K", (entry.size / 1024))
            } else {
                format!("{}M", (entry.size / 1024) / 1024)
            };

            let background = if i == ui.selected_file_idx { 0x333333 } else { 0x222222 };
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
            if let Some(entry) = ui.files.get(ui.selected_file_idx) {
                let sep = if ui.current_path.ends_with('\\') || ui.current_path.ends_with('/') { "" } else { "\\" };
                let full_path = format!("{}{}{}", ui.current_path, sep, entry.name);
                pg.draw_text(props_x + 10, list_y + 40, &format!("Name: {}", entry.name), 0xFFFFFF);
                pg.draw_text(props_x + 10, list_y + 60, &format!("Type: {}", if entry.is_dir { "Directory" } else { "File" }), 0xCCCCCC);
                pg.draw_text(props_x + 10, list_y + 80, &format!("Size: {} bytes", entry.size), 0xCCCCCC);
                pg.draw_text(props_x + 10, list_y + 100, &format!("Path: {}", full_path), 0x888888);
                pg.draw_text(props_x + 10, list_y + 130, &format!("Index: {} / {}", ui.selected_file_idx + 1, ui.files.len()), 0x888888);
            } else {
                pg.draw_text(props_x + 10, list_y + 40, "No item selected", 0x888888);
            }

            if let Some(action) = ui.filesys_pending_action {
                let confirm_y = list_y + list_h - 90;
                pg.fill_rect(props_x + 8, confirm_y, props_w - 16, 72, 0x332222);
                pg.draw_rect_outline(props_x + 8, confirm_y, props_w - 16, 72, 0xFFAA00);
                pg.draw_text(props_x + 16, confirm_y + 10, "Confirm Operation", 0xFFAA00);

                if action == FilePendingAction::Rename {
                    pg.draw_text(props_x + 16, confirm_y + 30, &format!("New Name: {}", ui.filesys_rename_buffer), 0xFFFFFF);
                    pg.draw_text(props_x + 16, confirm_y + 50, "ENTER to Rename, ESC to cancel", 0xCCCCCC);
                } else if action == FilePendingAction::Move {
                    pg.draw_text(props_x + 16, confirm_y + 30, &format!("Move to: {}_moved", ui.files[ui.selected_file_idx].name), 0xFFFFFF);
                    pg.draw_text(props_x + 16, confirm_y + 50, "END to Move, ESC to cancel", 0xCCCCCC);
                } else {
                    pg.draw_text(props_x + 16, confirm_y + 30, &format!("{:?}", action), 0xFFFFFF);
                    pg.draw_text(props_x + 16, confirm_y + 50, "END confirms, ESC cancels", 0xCCCCCC);
                }
            }
        }

        let actions_y = list_h + margin * 8;
        pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
        let actions = ["Open", "Props", "New File", "New Dir", "Rename", "Copy", "Move", "Delete"];
        let mut action_x = margin;
        let action_y = actions_y + 20;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == ui.filesys_action_idx;
            let color = if is_focused { 0x00AA00 } else { 0x444444 };
            pg.fill_rect(action_x, action_y, 92, 24, color);
            pg.draw_text(action_x + 6, action_y + 4, action, 0xFFFFFF);
            action_x += 100;
        }
        pg.draw_text(margin, action_y + 34, "[ / ] Switch Disk Tab | LEFT/RIGHT Select Action | END Run Action | ESC Cancel", 0x888888);
        pg.draw_text(margin, action_y + 52, &ui.status_line, 0xFFFF00);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        if let Some(action) = ui.filesys_pending_action {
            if action == FilePendingAction::Rename {
                match key {
                    Key::Printable(c) => {
                        let ch = char::from(c);
                        if ch == '\r' || ch == '\n' {
                            if !ui.files.is_empty() && !ui.filesys_rename_buffer.is_empty() {
                                let entry = ui.files[ui.selected_file_idx].clone();
                                let sep = if ui.current_path.ends_with('\\') || ui.current_path.ends_with('/') { "" } else { "\\" };
                                let full_path = format!("{}{}{}", ui.current_path, sep, entry.name);
                                let dst = format!("{}{}{}", ui.current_path, sep, ui.filesys_rename_buffer);

                                match crate::FileSystem::rename(&full_path, &dst) {
                                    Ok(_) => {
                                        ui.status_line = format!("Renamed {} to {}", entry.name, ui.filesys_rename_buffer);
                                        ui.filesys_pending_action = None;
                                        ui.refresh_storage();
                                    }
                                    Err(e) => {
                                        ui.status_line = format!("Rename failed: {}", e);
                                        ui.filesys_pending_action = None;
                                    }
                                }
                            }
                        } else if ch == '\u{08}' {
                            ui.filesys_rename_buffer.pop();
                        } else if !ch.is_control() {
                            ui.filesys_rename_buffer.push(ch);
                        }
                    }
                    Key::Special(ScanCode::ESCAPE) => {
                        ui.filesys_pending_action = None;
                        ui.status_line = String::from("Rename canceled");
                    }
                    _ => {}
                }
                return true;
            }
        }

        match key {
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch == '[' || ch == '{' {
                    if ui.selected_disk_idx > 0 {
                        ui.select_disk_tab(ui.selected_disk_idx - 1);
                    }
                    true
                } else if ch == ']' || ch == '}' {
                    if ui.selected_disk_idx + 1 < ui.storage_disks.len() {
                        ui.select_disk_tab(ui.selected_disk_idx + 1);
                    }
                    true
                } else if (b'1'..=b'9').contains(&(ch as u8)) {
                    let idx = (ch as u8 - b'1') as usize;
                    if idx < ui.storage_disks.len() {
                        ui.select_disk_tab(idx);
                    }
                    true
                } else if ch == '\t' {
                    if !ui.storage_disks.is_empty() {
                        let next = (ui.selected_disk_idx + 1) % ui.storage_disks.len();
                        ui.select_disk_tab(next);
                    }
                    true
                } else if ch == '\r' || ch == '\n' {
                    let saved_action = ui.filesys_action_idx;
                    ui.filesys_action_idx = 0;
                    let _ = input(ui, Key::Special(ScanCode::END));
                    ui.filesys_action_idx = saved_action;
                    true
                } else {
                    false
                }
            }
            Key::Special(ScanCode::PAGE_UP) => {
                if ui.selected_disk_idx > 0 {
                    ui.select_disk_tab(ui.selected_disk_idx - 1);
                }
                true
            }
            Key::Special(ScanCode::PAGE_DOWN) => {
                if ui.selected_disk_idx + 1 < ui.storage_disks.len() {
                    ui.select_disk_tab(ui.selected_disk_idx + 1);
                }
                true
            }
            Key::Special(ScanCode::UP) => {
                if ui.selected_file_idx > 0 { ui.selected_file_idx -= 1; }
                true
            }
            Key::Special(ScanCode::DOWN) => {
                if ui.selected_file_idx < ui.files.len().saturating_sub(1) { ui.selected_file_idx += 1; }
                true
            }
            Key::Special(ScanCode::LEFT) => {
                if ui.filesys_action_idx >= 1 { ui.filesys_action_idx -= 1; } else { ui.filesys_action_idx = 0; }
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                if ui.filesys_action_idx < 7 { ui.filesys_action_idx += 1; } else { ui.filesys_action_idx = 7; }
                true
            }
            Key::Special(ScanCode::ESCAPE) => {
                ui.filesys_pending_action = None;
                ui.status_line = String::from("File operation canceled");
                true
            }
            Key::Special(ScanCode::END) => {
                let sep = if ui.current_path.ends_with('\\') || ui.current_path.ends_with('/') { "" } else { "\\" };
                if ui.files.is_empty() {
                    if ui.filesys_action_idx == 2 {
                        let new_file = format!("{}{}new_file_{}.txt", ui.current_path, sep, ui.filesys_new_counter);
                        match crate::FileSystem::touch(&new_file) {
                            Ok(_) => {
                                ui.filesys_new_counter += 1;
                                ui.status_line = format!("Created {}", new_file);
                                ui.refresh_storage();
                            }
                            Err(e) => ui.status_line = format!("Create failed: {}", e),
                        }
                    } else if ui.filesys_action_idx == 3 {
                        let new_dir = format!("{}{}new_folder_{}", ui.current_path, sep, ui.filesys_new_counter);
                        match crate::FileSystem::mkdir(&new_dir) {
                            Ok(_) => {
                                ui.filesys_new_counter += 1;
                                ui.status_line = format!("Created {}", new_dir);
                                ui.refresh_storage();
                            }
                            Err(e) => ui.status_line = format!("Create folder failed: {}", e),
                        }
                    }
                    return true;
                }

                let entry = ui.files[ui.selected_file_idx].clone();
                let full_path = format!("{}{}{}", ui.current_path, sep, entry.name);

                if let Some(action) = ui.filesys_pending_action {
                    let result = match action {
                        FilePendingAction::Rename => {
                            if ui.filesys_rename_buffer.is_empty() {
                                ui.filesys_rename_buffer = entry.name.clone();
                            }
                            return true;
                        }
                        FilePendingAction::Copy => {
                            let dst = format!("{}{}{}_copy", ui.current_path, sep, entry.name);
                            if entry.is_dir {
                                crate::FileSystem::clone_dir(&full_path, &dst)
                            } else {
                                crate::FileSystem::copy(&full_path, &dst)
                            }
                        }
                        FilePendingAction::Move => {
                            let dst = format!("{}{}{}_moved", ui.current_path, sep, entry.name);
                            if entry.is_dir {
                                crate::FileSystem::move_directory(&full_path, &dst)
                            } else {
                                crate::FileSystem::move_file(&full_path, &dst)
                            }
                        }
                        FilePendingAction::Delete => {
                            if entry.is_dir {
                                crate::FileSystem::remove_dir(&full_path)
                            } else {
                                crate::FileSystem::remove(&full_path)
                            }
                        }
                    };

                    match result {
                        Ok(_) => {
                            ui.status_line = format!("{:?} complete for {}", action, entry.name);
                            ui.filesys_pending_action = None;
                            ui.refresh_storage();
                        }
                        Err(e) => {
                            ui.status_line = format!("{:?} failed: {}", action, e);
                            ui.filesys_pending_action = None;
                        }
                    }
                    return true;
                }

                match ui.filesys_action_idx {
                    0 => {
                        if entry.is_dir {
                            if entry.name == "." {
                                return true;
                            } else if entry.name == ".." {
                                if let Some(pos) = ui.current_path.rfind('\\') {
                                    let colon_pos = ui.current_path.find(':');
                                    let min_pos = if let Some(cp) = colon_pos { cp + 1 } else { 0 };
                                    if pos <= min_pos {
                                        if let Some(cp) = colon_pos {
                                            ui.current_path = format!("{}:\\", &ui.current_path[..cp]);
                                        } else {
                                            ui.current_path = String::from("\\");
                                        }
                                    } else {
                                        ui.current_path.truncate(pos);
                                    }
                                }
                                ui.refresh_storage();
                                return true;
                            } else {
                                if !ui.current_path.ends_with('\\') {
                                    ui.current_path.push('\\');
                                }
                                ui.current_path.push_str(&entry.name);
                                ui.selected_file_idx = 0;
                                ui.refresh_storage();
                                return true;
                            }
                        }
                        if (entry.name == "PAGEFILE") || (entry.name == "BOOTX64.EFI") {
                            ui.ui_error(25);
                        } else {
                            match crate::FileSystem::read_file(&full_path) {
                                Ok(data) => {
                                    let is_hex = core::str::from_utf8(&data).is_err();
                                    ui.editor = Some(TextEditor {
                                        file_path: full_path,
                                        buffer: data,
                                        cursor_pos: (0, 0),
                                        scroll_offset: 0,
                                        mode: EditorMode::Normal,
                                        is_hex,
                                        command_buffer: "".to_string(),
                                    });
                                    ui.selected_tab = DashboardTab::Editor;
                                }
                                Err(e) => {
                                    let errortext = format!("path {} -> {}", full_path, e);
                                    ui.ui_error_with_detail(29, Some(errortext.as_str()))
                                },
                            }
                        }
                    }
                    1 => {
                        ui.status_line = format!("{}: {} bytes, {}", entry.name, entry.size, if entry.is_dir { "directory" } else { "file" });
                    }
                    2 => {
                        let new_file = format!("{}{}new_file_{}.txt", ui.current_path, sep, ui.filesys_new_counter);
                        match crate::FileSystem::touch(&new_file) {
                            Ok(_) => {
                                ui.filesys_new_counter += 1;
                                ui.status_line = format!("Created {}", new_file);
                                ui.refresh_storage();
                            }
                            Err(e) => ui.status_line = format!("Create failed: {}", e),
                        }
                    }
                    3 => {
                        let new_dir = format!("{}{}new_folder_{}", ui.current_path, sep, ui.filesys_new_counter);
                        match crate::FileSystem::mkdir(&new_dir) {
                            Ok(_) => {
                                ui.filesys_new_counter += 1;
                                ui.status_line = format!("Created {}", new_dir);
                                ui.refresh_storage();
                            }
                            Err(e) => ui.status_line = format!("Create folder failed: {}", e),
                        }
                    }
                    4 => {
                        ui.filesys_pending_action = Some(FilePendingAction::Rename);
                        ui.filesys_rename_buffer = entry.name.clone();
                        ui.status_line = format!("Rename: {}", entry.name);
                    }
                    5 => {
                        ui.filesys_pending_action = Some(FilePendingAction::Copy);
                        ui.status_line = format!("Confirm copy of {}", entry.name);
                    }
                    6 => {
                        ui.filesys_pending_action = Some(FilePendingAction::Move);
                        ui.status_line = format!("Confirm move of {}", entry.name);
                    }
                    7 => {
                        ui.filesys_pending_action = Some(FilePendingAction::Delete);
                        ui.status_line = format!("Confirm delete of {}", entry.name);
                    }
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Storage {
        pub current_path: String,
        pub files: Vec<FileEntry>,
        pub selected_file_idx: usize,
        pub storage_disks: Vec<DiskTabInfo>,
        pub selected_disk_idx: usize,
        pub filesys_action_idx: usize,
        pub filesys_pending_action: Option<FilePendingAction>,
        pub status_line: String,
        pub filesys_new_counter: usize,
    }

    impl X_Storage {
        pub fn new() -> Self {
            X_Storage {
                current_path: "/".to_string(),
                files: vec![],
                selected_file_idx: 0,
                storage_disks: vec![],
                selected_disk_idx: 0,
                filesys_action_idx: 0,
                filesys_pending_action: None,
                status_line: "".to_string(),
                filesys_new_counter: 0,
            }
        }

        pub fn select_disk_tab(&mut self, idx: usize) {
            if idx < self.storage_disks.len() {
                self.selected_disk_idx = idx;
                let alias = self.storage_disks[idx].alias.clone();
                self.current_path = format!("{}:\\", alias);
                self.selected_file_idx = 0;
                self.refresh_storage();
            }
        }

        pub fn refresh_storage(&mut self) {
            use uefi::proto::media::file::{File, FileMode, FileAttribute};
            use uefi::proto::media::fs::SimpleFileSystem;

            self.files.clear();

            // Refresh and query all detected disk filesystems
            let fs_handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&SimpleFileSystem::GUID))
                .map(|hb| hb.to_vec())
                .unwrap_or_default();

            struct TempDisk {
                handle: uefi::Handle,
                volume_label: String,
                total_bytes: u64,
                free_bytes: u64,
                block_size: u32,
                media_type: String,
            }

            let mut valid_disks = Vec::new();

            for handle in &fs_handles {
                let mut volume_label = String::new();
                let mut total_bytes = 0u64;
                let mut free_bytes = 0u64;
                let mut block_size = 512u32;
                let mut media_type = String::from("Storage Volume");

                if let Ok(mut sfs) = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(*handle) {
                    if let Ok(mut root_dir) = sfs.open_volume() {
                        let mut info_buf = [0u8; 1024];
                        if let Ok(fs_info) = root_dir.get_info::<uefi::proto::media::file::FileSystemInfo>(&mut info_buf) {
                            volume_label = fs_info.volume_label().to_string();
                            total_bytes = fs_info.volume_size();
                            free_bytes = fs_info.free_space();
                            block_size = fs_info.block_size();
                        }
                    }
                }

                if volume_label.trim().is_empty() {
                    continue;
                }

                let dp_res = unsafe {
                    uefi::boot::open_protocol::<uefi::proto::device_path::DevicePath>(
                        uefi::boot::OpenProtocolParams {
                            handle: *handle,
                            agent: uefi::boot::image_handle(),
                            controller: None,
                        },
                        uefi::boot::OpenProtocolAttributes::GetProtocol,
                    )
                };

                if let Ok(dp) = dp_res {
                    for node in dp.node_iter() {
                        use uefi_raw::protocol::device_path::{DeviceType, DeviceSubType};
                        match (node.device_type(), node.sub_type()) {
                            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_NVME_NAMESPACE) => {
                                media_type = String::from("NVMe SSD");
                            }
                            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_SATA) => {
                                media_type = String::from("SATA AHCI");
                            }
                            (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB)
                            | (DeviceType::MESSAGING, DeviceSubType::MESSAGING_USB_CLASS) => {
                                media_type = String::from("USB Drive");
                            }
                            (DeviceType::MEDIA, DeviceSubType::MEDIA_CD_ROM) => {
                                media_type = String::from("CD/DVD-ROM");
                            }
                            (DeviceType::MEDIA, DeviceSubType::MEDIA_RAM_DISK) => {
                                media_type = String::from("RAM Disk");
                            }
                            (DeviceType::MEDIA, DeviceSubType::MEDIA_HARD_DRIVE) => {
                                if media_type == "Storage Volume" {
                                    media_type = String::from("Hard Disk");
                                }
                            }
                            _ => {}
                        }
                    }
                }

                valid_disks.push(TempDisk {
                    handle: *handle,
                    volume_label,
                    total_bytes,
                    free_bytes,
                    block_size,
                    media_type,
                });
            }

            valid_disks.sort_by(|a, b| {
                let a_is_boot = a.volume_label.eq_ignore_ascii_case("boot");
                let b_is_boot = b.volume_label.eq_ignore_ascii_case("boot");
                b_is_boot.cmp(&a_is_boot)
            });

            let mut detected_disks = Vec::new();
            for (idx, disk) in valid_disks.into_iter().enumerate() {
                detected_disks.push(DiskTabInfo {
                    alias: format!("dsk{}", idx),
                    volume_label: disk.volume_label,
                    total_bytes: disk.total_bytes,
                    free_bytes: disk.free_bytes,
                    block_size: disk.block_size,
                    media_type: disk.media_type,
                    handle: Some(disk.handle),
                });
            }

            if detected_disks.is_empty() {
                detected_disks.push(DiskTabInfo {
                    alias: String::from("dsk0"),
                    volume_label: String::from("System"),
                    total_bytes: 0,
                    free_bytes: 0,
                    block_size: 512,
                    media_type: String::from("Default Volume"),
                    handle: None,
                });
            }

            self.storage_disks = detected_disks;

            if self.selected_disk_idx >= self.storage_disks.len() {
                self.selected_disk_idx = self.storage_disks.len().saturating_sub(1);
            }

            let active_disk = self.storage_disks.get(self.selected_disk_idx);
            let disk_handle = active_disk.and_then(|d| d.handle);
            let sfs_handle = disk_handle.or_else(|| uefi::boot::get_handle_for_protocol::<SimpleFileSystem>().ok());

            let handle = match sfs_handle {
                Some(h) => h,
                None => return,
            };

            let mut sfs = match uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(handle) {
                Ok(s) => s,
                Err(_) => return,
            };

            let mut root_dir = match sfs.open_volume() {
                Ok(d) => d,
                Err(_) => return,
            };

            let sub_path = if let Some(colon_idx) = self.current_path.find(':') {
                &self.current_path[colon_idx + 1..]
            } else {
                &self.current_path
            };

            let mut target_dir = if sub_path == "\\" || sub_path == "/" || sub_path.is_empty() {
                root_dir
            } else {
                let clean_path = sub_path.trim_start_matches('\\').trim_start_matches('/');
                if clean_path.is_empty() {
                    root_dir
                } else {
                    let mut u16_path: Vec<u16> = clean_path.encode_utf16().collect();
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
                    Ok(None) | Err(_) => break,
                }
            }

            if !self.files.is_empty() {
                if self.selected_file_idx >= self.files.len() {
                    self.selected_file_idx = self.files.len() - 1;
                }
            } else {
                self.selected_file_idx = 0;
            }
        }
    }

    impl Runnable for X_Storage {
        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                if self.current_path.is_empty() {
                    self.current_path = data.current_path.clone();
                }
                self.files = data.files.clone();
                self.selected_file_idx = self.selected_file_idx.min(self.files.len().saturating_sub(1));
            }

            if self.current_path.is_empty() {
                self.current_path = String::from("/");
            }
            self.filesys_new_counter += 1;
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                    if self.files.get(self.selected_file_idx).map(|entry| entry.is_dir).unwrap_or(false) {
                        let selected_action = self.filesys_action_idx;
                        self.filesys_action_idx = 0;
                        self.input(Key::Special(ScanCode::END));
                        self.filesys_action_idx = selected_action;
                    }
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '[' || ch == '{' {
                        if self.selected_disk_idx > 0 {
                            self.select_disk_tab(self.selected_disk_idx - 1);
                        }
                    } else if ch == ']' || ch == '}' {
                        if self.selected_disk_idx + 1 < self.storage_disks.len() {
                            self.select_disk_tab(self.selected_disk_idx + 1);
                        }
                    } else if ch >= '1' && ch <= '9' {
                        let idx = (ch as u8 - b'1') as usize;
                        if idx < self.storage_disks.len() {
                            self.select_disk_tab(idx);
                        }
                    } else if ch == '\t' {
                        if !self.storage_disks.is_empty() {
                            let next = (self.selected_disk_idx + 1) % self.storage_disks.len();
                            self.select_disk_tab(next);
                        }
                    }
                }
                Key::Special(ScanCode::PAGE_UP) => {
                    if self.selected_disk_idx > 0 {
                        self.select_disk_tab(self.selected_disk_idx - 1);
                    }
                }
                Key::Special(ScanCode::PAGE_DOWN) => {
                    if self.selected_disk_idx + 1 < self.storage_disks.len() {
                        self.select_disk_tab(self.selected_disk_idx + 1);
                    }
                }
                Key::Special(ScanCode::UP) => {
                    self.selected_file_idx = self.selected_file_idx.saturating_sub(1);
                }
                Key::Special(ScanCode::DOWN) => {
                    if !self.files.is_empty() {
                        self.selected_file_idx = (self.selected_file_idx + 1).min(self.files.len() - 1);
                    }
                }
                Key::Special(ScanCode::LEFT) => {
                    if self.filesys_action_idx >= 1 { self.filesys_action_idx -= 1 } else { self.filesys_action_idx = 0 }
                }
                Key::Special(ScanCode::RIGHT) => {
                    if self.filesys_action_idx < 7 { self.filesys_action_idx += 1 } else { self.filesys_action_idx = 7 }
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
                            self.status_line = format!("Opening {}...", entry.name);
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

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }

        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let content_top = y;
            let margin = 16usize;
            let gutter = 12usize;
            let line_h = 15usize;
            let width = 600usize;
            let height = 500usize;
            let base_y = content_top + margin;
            let strx = format!("File Explorer ({:#?})", self.filesys_new_counter);
            pg.draw_text(margin + x, base_y - 4, strx.as_str(), 0x00FF00);
            pg.draw_text(margin + x, base_y + 8, &alloc::format!("Path: {}", self.current_path), 0xAAAAAA);

            let list_x = margin + x;
            let list_y = base_y + 28;
            let list_w = core::cmp::min(width - margin * 2, 720);
            let list_h = core::cmp::min(height.saturating_sub(margin + 28 + 90), 460);
            pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);

            pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, line_h, 0x333333);
            pg.draw_text(list_x + 8, list_y + 4, "TYPE  NAME                                 SIZE (BYTES)  ATTR", 0xCCCCCC);
            pg.draw_line(list_x + 48, list_y + 1, list_x + 48, list_y + list_h - 1, 0x444444);
            pg.draw_line(list_x + 340, list_y + 1, list_x + 340, list_y + list_h - 1, 0x444444);

            let mut cur_y = list_y + line_h + gutter;
            for (i, entry) in self.files.iter().enumerate() {
                if cur_y + line_h > list_y + list_h - 2 { break; }
                let color = if i == self.selected_file_idx { 0xFFFF00 } else { 0xFFFFFF };
                let icon = if entry.is_dir { pixel_graphics::icons::FOLDER_ICON_DATA } else {
                    let dec_syn = ["json", "xml", "toml", "yaml", "yml"];
                    let sys_syn = ["sys", "efi", "asm"];
                    let prog_syn = ["micro", "module", "dmx", "bin", "rs"];
                    let exec_syn = ["cxf", "cxp"];

                    let ext = entry.name.split('.').last().unwrap_or("");
                    if dec_syn.contains(&ext) {
                        pixel_graphics::icons::JSON_ICON_DATA
                    } else if sys_syn.contains(&ext) {
                        pixel_graphics::icons::EXECUTABLE_ICON_DATA
                    } else if prog_syn.contains(&ext) {
                        pixel_graphics::icons::CODE_ICON_DATA
                    } else if exec_syn.contains(&ext) {
                        pixel_graphics::icons::EXECUTABLE_ICON_DATA
                    } else {
                        pixel_graphics::icons::FILE_ICON_DATA
                    }
                };

                let size: String = if entry.size < 10000 {
                    format!("{}", entry.size)
                } else if entry.size / 1024 < 10000 {
                    format!("{}K", (entry.size / 1024))
                } else {
                    format!("{}M", (entry.size / 1024) / 1024)
                };

                let background = if i == self.selected_file_idx { 0x333333 } else { 0x222222 };
                pg.draw_icon(list_x + 16, cur_y, 16, 16, &icon);
                pg.draw_text_bg(list_x + 56, cur_y, &alloc::format!("{:<32}", entry.name), color, background);
                pg.draw_text_bg(list_x + 348, cur_y, &alloc::format!("{:>12}", size), 0xCCCCCC, background);
                pg.draw_text_bg(list_x + 470, cur_y, if entry.is_dir { "DIR" } else { "FILE" }, 0x6666FF, background);
                cur_y += line_h;
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

            let actions_y = list_h + margin * 8;
            pg.draw_text(margin, actions_y, "Actions for Selected Item", 0xCCCCCC);
            let actions = ["Open", "Props", "New File", "New Dir", "Rename", "Copy", "Move", "Delete"];
            let mut action_x = margin + x;
            let action_y = actions_y + 20;
            for (idx, action) in actions.iter().enumerate() {
                let is_focused = idx == self.filesys_action_idx;
                let color = if is_focused { 0x00AA00 } else { 0x444444 };
                pg.fill_rect(action_x, action_y, 92, 24, color);
                pg.draw_text(action_x + 6, action_y + 4, action, 0xFFFFFF);
                action_x += 100;
            }
            pg.draw_text(margin + x, action_y + 34, "[ / ] Switch Disk Tab | LEFT/RIGHT Select Action | END Run Action | ESC Cancel", 0x888888);
            pg.draw_text(margin + x, action_y + 52, &self.status_line, 0xFFFF00);
        }
    }

    impl AppInfo for X_Storage {
        fn name(&self) -> &str { "Storage" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::CUBE_WINDOW_RED_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 10. Test Tab (Qt6 Style Test Bed)
// =========================================================================
pub mod test {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _origin_x: usize, _origin_y: usize, width: usize, height: usize) {
        pg.draw_text(20, 100, &alloc::format!("UI Components Test Bed (Qt6 Style)  res: {}x{}", width, height), 0x00FF00);

        let mut y = 130;
        pg.draw_text(20, y, "Buttons & Inputs:", 0xAAAAAA);
        y += 25;
        pg.fill_rect(20, y, 100, 25, 0x444444);
        pg.draw_text(25, y + 5, "Push Button", 0xFFFFFF);
        pg.fill_rect(130, y, 30, 25, 0x444444);
        pg.draw_text(138, y + 5, "?", 0xFFFFFF);
        y += 35;

        pg.draw_checkbox(20, y, true, false, false, "CheckBox (Checked)");
        y += 25;
        pg.draw_checkbox(20, y, false, false, false, "CheckBox (Unchecked)");
        y += 25;
        pg.draw_checkbox(20, y, false, true, false, "CheckBox (Blocked/Denied)");
        y += 25;
        pg.draw_checkbox(20, y, true, false, true, "CheckBox (Disabled)");
        y += 25;

        pg.draw_radio_button(20, y, true);
        pg.draw_text(40, y, "RadioButton 1", 0xFFFFFF);
        y += 25;
        pg.draw_radio_button(20, y, false);
        pg.draw_text(40, y, "RadioButton 2", 0xFFFFFF);
        y += 35;

        pg.draw_text(20, y, "LineEdit:", 0xAAAAAA);
        y += 20;
        pg.draw_rect_outline(20, y, 150, 20, 0x888888);
        pg.fill_rect(21, y + 1, 148, 18, 0xFFFFFF);
        pg.draw_text(25, y + 2, "Editable text..ſ", 0x000000);
        y += 30;

        pg.draw_text(20, y, "SpinBox / DoubleSpinBox:", 0xAAAAAA);
        y += 15;
        pg.draw_spinbox(20, y, 60, 42, "int");
        pg.draw_double_spinbox(120, y, 60, 3.14, 2);
        y += 30;

        let mut y = 130;
        let x2 = 250;
        pg.draw_text(x2, y, "Sliders & Progress:", 0xAAAAAA);
        y += 25;
        pg.draw_slider(x2, y, 150, 40, 100, false);
        y += 25;
        pg.draw_slider(x2 + 160, 130, 100, 30, 100, true);

        pg.draw_text(x2, y, "Progress Bar:", 0xAAAAAA);
        y += 20;
        pg.draw_progress_bar(x2, y, 150, 20, 65, 100, 0x00FF00);
        y += 35;

        pg.draw_text(x2, y, "LCD Number:", 0xAAAAAA);
        y += 20;
        pg.draw_lcd_number(x2, y, "123.45");
        y += 40;

        pg.draw_text(x2, y, "ScrollBars:", 0xAAAAAA);
        y += 20;
        pg.draw_rect_outline(x2, y, 150, 15, 0x444444);
        pg.fill_rect(x2 + 40, y + 1, 30, 13, 0x888888);
        y += 25;

        pg.draw_text(x2, y, "Date/Time Edits:", 0xAAAAAA);
        y += 20;
        pg.draw_text(x2, y, "2026-02-23 10:25", 0x00FFFF);

        let mut y = 130;
        let x3 = 500;
        pg.draw_text(x3, y, "Complex Views (Mock):", 0xAAAAAA);
        y += 25;
        pg.draw_rect_outline(x3, y, 200, 60, 0x888888);
        pg.draw_text(x3 + 5, y + 5, "ListView Item A", 0xFFFFFF);
        pg.draw_text(x3 + 5, y + 25, "ListView Item B", 0xFFFF00);
        pg.draw_text(x3 + 5, y + 45, "ListView Item C", 0xFFFFFF);
        y += 70;

        pg.draw_rect_outline(x3, y, 200, 60, 0x888888);
        pg.draw_text(x3 + 5, y + 5, "[-] Root", 0xFFFFFF);
        pg.draw_text(x3 + 20, y + 25, " └─ Child 1", 0xAAAAAA);
        pg.draw_text(x3 + 20, y + 45, " └─ Child 2", 0xAAAAAA);
        y += 70;

        pg.draw_rect_outline(x3, y, 200, 60, 0x888888);
        pg.draw_line(x3, y + 20, x3 + 200, y + 20, 0x888888);
        pg.draw_line(x3 + 60, y, x3 + 60, y + 60, 0x888888);
        pg.draw_text(x3 + 5, y + 2, "H1", 0xAAAAAA);
        pg.draw_text(x3 + 65, y + 2, "Header 2", 0xAAAAAA);
        pg.draw_text(x3 + 5, y + 25, "Val 1", 0xFFFFFF);
        pg.draw_text(x3 + 65, y + 25, "Data 2", 0xFFFFFF);

        let y = 450;
        pg.draw_rect_outline(20, y, 200, 100, 0x888888);
        pg.fill_rect(30, y - 8, 80, 16, 0x222222);
        pg.draw_text(35, y - 8, "GroupBox", 0xAAAAAA);
        pg.draw_text(40, y + 20, "Internal content", 0x888888);

        pg.draw_rect_outline(240, y, 200, 100, 0x888888);
        pg.fill_rect(240, y, 200, 20, 0x444444);
        pg.draw_text(245, y + 2, "ToolBox Tab 1 [v]", 0xFFFFFF);
        pg.fill_rect(240, y + 80, 200, 20, 0x444444);
        pg.draw_text(245, y + 82, "ToolBox Tab 2 [>]", 0xFFFFFF);

        pg.draw_icon(20, y + 400, 16, 16, &pixel_graphics::icons::RAM_ICON_DATA);
        pg.draw_icon(44, y + 400, 16, 16, &pixel_graphics::icons::PCI_GREEN_ICON_DATA);
        pg.draw_icon(68, y + 400, 16, 16, &pixel_graphics::icons::PCI_BLUE_ICON_DATA);
        pg.draw_icon(92, y + 400, 16, 16, &pixel_graphics::icons::CPU_ICON_DATA);
        pg.draw_icon(116, y + 400, 16, 16, &pixel_graphics::icons::HOURGLASS_ICON_DATA);
        pg.draw_icon(140, y + 400, 16, 16, &pixel_graphics::icons::ETHERNET_ICON_DATA);
        pg.draw_icon(164, y + 400, 16, 16, &pixel_graphics::icons::HDD_INTERNAL_ICON_DATA);
        pg.draw_icon(188, y + 400, 16, 16, &pixel_graphics::icons::SETTINGS_ICON_DATA);
        pg.draw_icon(220, y + 400, 32, 32, &pixel_graphics::icons::GTK_CUBE_32_ICON_DATA);
        pg.draw_icon(20, y + 450, 32, 32, &pixel_graphics::icons::CD_DISK_32_ICON_DATA);
        pg.draw_icon(70, y + 450, 32, 32, &pixel_graphics::icons::SCRIPT_YELLOW_32_ICON_DATA);
        pg.draw_icon(120, y + 450, 32, 32, &pixel_graphics::icons::TAPE_WRITE_32_ICON_DATA);
        pg.draw_icon(170, y + 450, 32, 32, &pixel_graphics::icons::CUBE_TREE_32_ICON_DATA);
        pg.draw_icon(220, y + 450, 32, 32, &pixel_graphics::icons::GEAR_WINDOW_SETTINGS_32_ICON_DATA);
        pg.draw_icon(270, y + 450, 32, 32, &pixel_graphics::icons::GRAPHICS_2D_32_ICON_DATA);
        pg.draw_icon(20, y + 500, 32, 32, &pixel_graphics::icons::BLADE_NETWORK_32_ICON_DATA);
        pg.draw_icon(70, y + 500, 32, 32, &pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA);
        pg.draw_icon(120, y + 500, 32, 32, &pixel_graphics::icons::WINOBJ_SEMAPHORE_32_ICON_DATA);
        pg.draw_icon(170, y + 500, 32, 32, &pixel_graphics::icons::REGEDIT_CUBES_32_ICON_DATA);
        pg.draw_icon(220, y + 500, 32, 32, &pixel_graphics::icons::REGISTRY_HIVE_32_ICON_DATA);
        pg.draw_icon(270, y + 500, 32, 32, &pixel_graphics::icons::DATABASE_CLUSTER_32_ICON_DATA);
    }

    pub fn logic(ui: &mut DashboardUI) {
        ui.iter = ui.iter.wrapping_add(1);
    }

    pub fn input(_ui: &mut DashboardUI, _key: Key) -> bool {
        false
    }

    #[derive(Clone)]
    pub struct X_Test {
        pub iter: usize,
    }

    impl X_Test {
        pub fn new() -> Self {
            Self {
                iter: 0,
            }
        }
    }

    impl Runnable for X_Test {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            pg.draw_text(x + 20, y + 20, "System Diagnostics", 0x00FF00);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.iter = data.iter as usize;
            }
        }

        fn input(&mut self, _key: Key) {}

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Test {
        fn name(&self) -> &str { "Test" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 11. Editor Tab
// =========================================================================
pub mod editor {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let content_top = 80usize;

        if let Some(ref ed) = ui.editor {
            let mode_text = if ed.mode == EditorMode::Insert { "-- INSERT --" } else if ed.mode == EditorMode::Command { "-- COMMAND --" } else { "-- NORMAL --" };
            let view_type = if ed.is_hex { "[HEX VIEW]" } else { "[TEXT VIEW]" };
            pg.draw_text(margin, content_top + 5, &alloc::format!("Editing: {} {}", ed.file_path, view_type), 0x00FF00);
            pg.draw_text(width.saturating_sub(150), content_top + 5, mode_text, 0xFFFF00);

            let edit_y_start = content_top + 30;
            let visible_lines = core::cmp::min((height.saturating_sub(edit_y_start + 150)) / 20, 32);

            if ed.is_hex {
                let mut cur_y = content_top + 40;
                let line_height = 20;
                let hex_start_x = margin + 110;
                let ascii_start_x = margin + 600;

                for (i, chunk) in ed.buffer.chunks(16).enumerate().skip(ed.scroll_offset) {
                    if cur_y > height.saturating_sub(150) { break; }

                    let offset = i * 16;
                    let current_row = i;
                    pg.draw_text(margin + 10, cur_y, &alloc::format!("{:08X}", offset), 0x888888);

                    for (j, &byte) in chunk.iter().enumerate() {
                        let color = match byte {
                            0..=31 | 127 => 0x5555FF,
                            32..=126 => 0xFFFFFF,
                            _ => 0xFF00FF,
                        };

                        let byte_x = hex_start_x + (j * 30);
                        let ascii_x = ascii_start_x + (j * 12);

                        if ed.mode != EditorMode::Command && ed.cursor_pos.0 == current_row && ed.cursor_pos.1 == j {
                            let cursor_color = if ed.mode == EditorMode::Insert { 0x00FF00 } else { 0xAAAAAA };
                            pg.fill_rect(byte_x.saturating_sub(2), cur_y.saturating_sub(2), 22, 16, cursor_color);
                            pg.fill_rect(ascii_x.saturating_sub(1), cur_y.saturating_sub(2), 10, 16, cursor_color);

                            pg.draw_text(byte_x, cur_y, &alloc::format!("{:02X}", byte), 0x000000);
                            let ascii_char = if (32..=126).contains(&byte) { byte as char } else { '.' };
                            pg.draw_text(ascii_x, cur_y, &ascii_char.to_string(), 0x000000);
                        } else {
                            pg.draw_text(byte_x, cur_y, &alloc::format!("{:02X}", byte), color);
                            let ascii_char = if (32..=126).contains(&byte) { byte as char } else { '.' };
                            pg.draw_text(ascii_x, cur_y, &ascii_char.to_string(), color);
                        }
                    }
                    cur_y += line_height;
                }
            } else {
                let content = core::str::from_utf8(&ed.buffer).unwrap_or("");
                let mut line_y = edit_y_start;
                for (i, line) in content.lines().chain(core::iter::once("")).skip(ed.scroll_offset).enumerate() {
                    if i >= visible_lines { break; }
                    let current_line_idx = ed.scroll_offset + i;

                    pg.draw_text(margin, line_y, &alloc::format!("{:3}", current_line_idx + 1), 0x666666);
                    pg.draw_text(margin + 40, line_y, line, 0xFFFFFF);

                    if ed.mode != EditorMode::Command && ed.cursor_pos.0 == current_line_idx {
                        let char_x = margin + 40 + (ed.cursor_pos.1 * 8);
                        if char_x < width.saturating_sub(margin) {
                            let cursor_color = if ed.mode == EditorMode::Insert { 0x00FF00 } else { 0xAAAAAA };
                            pg.fill_rect(char_x, line_y.saturating_sub(1), 8, 16, cursor_color);

                            if let Some(c) = line.chars().nth(ed.cursor_pos.1) {
                                pg.draw_text(char_x, line_y, &c.to_string(), 0x000000);
                            }
                        }
                    }

                    line_y += 20;
                }
            }

            pg.draw_text(margin, height.saturating_sub(70), ":w - Save | :q - Quit | i - Insert | Esc - Normal", 0x888888);
            pg.draw_text(margin + 600, height.saturating_sub(70), alloc::format!(":{}", ed.command_buffer).as_str(), 0xFFFFFF);
        }
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        let ed = match ui.editor.as_mut() {
            Some(ed) => ed,
            None => {
                ui.ui_error(30);
                ui.selected_tab = DashboardTab::Storage;
                return true;
            }
        };

        let (width, height) = if let Some(pg) = PixelGraphics::new() {
            pg.resolution()
        } else {
            (1024, 768)
        };
        let content_top = 48;
        let edit_y_start = content_top + 30;
        let visible_lines = core::cmp::min((height.saturating_sub(edit_y_start + 150)) / 20, 32);

        match ed.mode {
            EditorMode::Normal => match key {
                Key::Printable(c) => match char::from(c) {
                    'i' => { ed.mode = EditorMode::Insert; true }
                    ':' => {
                        ed.mode = EditorMode::Command;
                        ed.command_buffer.clear();
                        true
                    }
                    'j' => {
                        ed.cursor_pos.0 += 1;
                        if ed.is_hex {
                            let row_count = (ed.buffer.len() + 15) / 16;
                            if ed.cursor_pos.0 >= row_count {
                                ed.cursor_pos.0 = row_count.saturating_sub(1);
                            }
                        } else {
                            let content = core::str::from_utf8(&ed.buffer).unwrap_or("");
                            let line_count = content.lines().count();
                            if ed.cursor_pos.0 >= line_count {
                                ed.cursor_pos.0 = line_count.saturating_sub(1);
                            }
                        }
                        if ed.cursor_pos.0 >= ed.scroll_offset + visible_lines {
                            ed.scroll_offset = ed.cursor_pos.0 - visible_lines + 1;
                        }
                        true
                    }
                    'k' => {
                        if ed.cursor_pos.0 > 0 {
                            ed.cursor_pos.0 -= 1;
                            if ed.cursor_pos.0 < ed.scroll_offset {
                                ed.scroll_offset = ed.cursor_pos.0;
                            }
                        }
                        true
                    }
                    'h' => {
                        if ed.cursor_pos.1 > 0 {
                            ed.cursor_pos.1 -= 1;
                        }
                        true
                    }
                    'l' => {
                        ed.cursor_pos.1 += 1;
                        if ed.is_hex {
                            if ed.cursor_pos.1 > 15 {
                                ed.cursor_pos.1 = 15;
                            }
                        } else {
                            let content = core::str::from_utf8(&ed.buffer).unwrap_or("");
                            if let Some(line) = content.lines().nth(ed.cursor_pos.0) {
                                if ed.cursor_pos.1 > line.len() {
                                    ed.cursor_pos.1 = line.len();
                                }
                            }
                        }
                        true
                    }
                    _ => false,
                },
                _ => false,
            },
            EditorMode::Insert => match key {
                Key::Special(ScanCode::ESCAPE) => {
                    ed.mode = EditorMode::Normal;
                    true
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ed.is_hex {
                        if let Some(hex_digit) = ch.to_digit(16) {
                            let byte_idx = ed.cursor_pos.0 * 16 + ed.cursor_pos.1;
                            if byte_idx < ed.buffer.len() {
                                ed.buffer[byte_idx] = (ed.buffer[byte_idx] << 4) | (hex_digit as u8);
                            }
                        }
                    } else {
                        let content = core::str::from_utf8(&ed.buffer).unwrap_or("");
                        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
                        if lines.is_empty() {
                            lines.push(String::new());
                        }

                        while ed.cursor_pos.0 >= lines.len() {
                            lines.push(String::new());
                        }

                        if ch == '\r' || ch == '\n' {
                            let current_line = &lines[ed.cursor_pos.0];
                            let split_pos = ed.cursor_pos.1.min(current_line.len());
                            let (left, right) = current_line.split_at(split_pos);
                            let new_left = left.to_string();
                            let new_right = right.to_string();

                            lines[ed.cursor_pos.0] = new_left;
                            lines.insert(ed.cursor_pos.0 + 1, new_right);

                            ed.cursor_pos.0 += 1;
                            ed.cursor_pos.1 = 0;
                        } else if ch == '\u{08}' {
                            if ed.cursor_pos.1 > 0 {
                                lines[ed.cursor_pos.0].remove(ed.cursor_pos.1 - 1);
                                ed.cursor_pos.1 -= 1;
                            } else if ed.cursor_pos.0 > 0 {
                                let current_line = lines.remove(ed.cursor_pos.0);
                                ed.cursor_pos.0 -= 1;
                                ed.cursor_pos.1 = lines[ed.cursor_pos.0].len();
                                lines[ed.cursor_pos.0].push_str(&current_line);
                            }
                        } else if !ch.is_control() {
                            let pos = ed.cursor_pos.1.min(lines[ed.cursor_pos.0].len());
                            lines[ed.cursor_pos.0].insert(pos, ch);
                            ed.cursor_pos.1 += 1;
                        }

                        let new_content = lines.join("\n");
                        ed.buffer = new_content.into_bytes();
                    }
                    true
                }
                _ => false,
            },
            EditorMode::Command => match key {
                Key::Special(ScanCode::ESCAPE) => {
                    ed.mode = EditorMode::Normal;
                    true
                }
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '\r' || ch == '\n' {
                        match ed.command_buffer.as_str() {
                            "w" => {
                                let failed = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w').is_err();
                                ed.mode = EditorMode::Normal;
                                if failed {
                                    ui.ui_error(16);
                                }
                            }
                            "q" => ui.selected_tab = DashboardTab::Storage,
                            "wq" => {
                                let failed = crate::FileSystem::write_to_file_bytes(&ed.file_path, &ed.buffer, 'w').is_err();
                                if failed {
                                    ed.mode = EditorMode::Normal;
                                    ui.ui_error(16);
                                } else {
                                    ui.selected_tab = DashboardTab::Storage;
                                }
                            }
                            _ => {
                                ed.mode = EditorMode::Normal;
                                ui.ui_error(1);
                            }
                        }
                    } else {
                        ed.command_buffer.push(ch);
                    }
                    true
                }
                _ => false,
            },
        }
    }

    #[derive(Clone)]
    pub struct X_Editor {}

    impl X_Editor {
        pub fn new() -> Self {
            Self {}
        }
    }

    impl Runnable for X_Editor {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            pg.draw_text(x + 20, y + 20, "Text Editor", 0x00FF00);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

        fn input(&mut self, _key: Key) {}

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Editor {
        fn name(&self) -> &str { "Editor" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::SCRIPT_YELLOW_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 12. Settings Tab
// =========================================================================
pub mod settings {
    use super::*;

    pub fn draw(ui: &mut DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let page_y = 80usize;
        pg.draw_text(10, page_y - 15, "SYSTEM SETTINGS", 0x00FF00);

        let left_x = 10;
        let left_y = page_y + 2;
        let left_w = (width / 3).saturating_sub(20);
        let right_x = left_x + left_w + 10;
        let right_w = width.saturating_sub(right_x + 10);
        let content_h = height.saturating_sub(page_y + 60);

        pg.fill_rect(left_x, left_y, left_w, content_h, 0x1A1A1A);
        pg.draw_rect_outline(left_x, left_y, left_w, content_h, 0x333333);
        pg.fill_rect(right_x, left_y, right_w, content_h, 0x151515);
        pg.draw_rect_outline(right_x, left_y, right_w, content_h, 0x333333);

        let categories = [
            "General", "Boot & Init", "Interface", "VM & Safety",
            "Network", "Storage", "Packages", "Developer", "Security", "Display (PG)",
        ];

        let mut cy = left_y + 8;
        for (i, cat) in categories.iter().enumerate() {
            let selected = i == ui.selected_settings_category_idx;
            if selected {
                pg.fill_rect(left_x + 4, cy - 2, left_w - 8, 20, 0x005577);
            }
            pg.draw_text(left_x + 12, cy, cat, if selected { 0x00FFFF } else { 0xCCCCCC });
            cy += 24;
        }

        let rows = ui.settings_rows();
        let mut ry = left_y + 12;
        for (i, (label, val, readonly, is_display)) in rows.iter().enumerate() {
            let selected = i == ui.selected_settings_idx;
            if selected {
                pg.fill_rect(right_x + 6, ry - 3, right_w - 12, 34, 0x223322);
                pg.draw_rect_outline(right_x + 6, ry - 3, right_w - 12, 34, 0x00AA00);
            }
            pg.draw_text(right_x + 12, ry, label, if selected { 0x00FF00 } else { 0xFFFFFF });
            pg.draw_text(right_x + right_w - 160, ry, val, 0xFFFF00);
            // pg.draw_text(right_x + 12, ry + 16, desc, 0x888888);
            ry += 38;
        }

        pg.draw_text(10, height - 35, "LEFT/RIGHT select category | UP/DOWN select option | ENTER toggle/cycle", 0x777777);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                if matches!(char::from(c), '\r' | '\n') {
                    ui.toggle_selected_setting();
                    return true;
                }
                false
            }
            Key::Special(ScanCode::LEFT) => {
                ui.selected_settings_category_idx = ui.selected_settings_category_idx.saturating_sub(1);
                ui.selected_settings_idx = 0;
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.selected_settings_category_idx = (ui.selected_settings_category_idx + 1).min(9);
                ui.selected_settings_idx = 0;
                true
            }
            Key::Special(ScanCode::UP) => {
                ui.selected_settings_idx = ui.selected_settings_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::DOWN) => {
                let rows_count = ui.settings_rows().len();
                ui.selected_settings_idx = (ui.selected_settings_idx + 1).min(rows_count.saturating_sub(1));
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Settings {
        pub settings: UiSettings,
        pub selected_settings_idx: usize,
        pub selected_settings_category_idx: usize,
    }

    impl X_Settings {
        pub fn new() -> Self {
            Self {
                settings: UiSettings {
                    extra_debug_info: false,
                    folder_absolute_sizes: false,
                    state_save_restore: false,
                    extended_symbol_library: false,
                    ring0_udmi_udxi: false,
                    controllang_support: false,
                    pg_vshaders: false,
                    experimental_mem_comp: false,
                    auto_refresh_storage: false,
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
                    ui_scaling: 0,
                    terminal_font: 0,
                    pg_scanlines: false,
                    pg_dither: false,
                    pg_glitch: false,
                    pg_aberration: 0,
                },
                selected_settings_idx: 0,
                selected_settings_category_idx: 0,
            }
        }

        fn option_value(&self, options: &[&'static str], idx: usize) -> String {
            options.get(idx).copied().unwrap_or(options[0]).to_string()
        }

        pub fn settings_rows(&self) -> Vec<(String, String, bool, bool)> {
            match self.selected_settings_category_idx {
                0 => vec![
                    (String::from("HPVMX_PROFILE"), self.option_value(&["balanced", "diagnostic", "performance"], self.settings.general_profile), false, false),
                    (String::from("Extra Debug Info"), if self.settings.extra_debug_info { "on" } else { "off" }.to_string(), false, false),
                    (String::from("HPVMX_USER"), String::from("operator"), false, false),
                    (String::from("Experimental Mem Comp"), if self.settings.experimental_mem_comp { "on" } else { "off" }.to_string(), false, false),
                ],
                1 => vec![
                    (String::from("HPVMX_BOOT_TARGET"), self.option_value(&["dashboard", "shell", "last-vm"], self.settings.boot_target), false, false),
                    (String::from("State Save/Restore"), if self.settings.state_save_restore { "on" } else { "off" }.to_string(), false, false),
                    (String::from("HPVMX_WATCHDOG"), String::from("disabled"), false, false),
                ],
                2 => vec![
                    (String::from("HPVMX_UI_DENSITY"), self.option_value(&["normal", "compact", "wide"], self.settings.interface_density), false, false),
                    (String::from("HPVMX_UI_SCALING"), self.option_value(&["50%", "100%", "150%", "200%"], self.settings.ui_scaling), false, false),
                    (String::from("Extended Symbol Library"), if self.settings.extended_symbol_library { "on" } else { "off" }.to_string(), false, false),
                    (String::from("PG VShaders"), if self.settings.pg_vshaders { "on" } else { "off" }.to_string(), false, false),
                    (String::from("PG Scanlines"), if self.settings.pg_scanlines { "on" } else { "off" }.to_string(), false, false),
                    (String::from("PG Dither"), if self.settings.pg_dither { "on" } else { "off" }.to_string(), false, false),
                    (String::from("PG Glitch"), if self.settings.pg_glitch { "on" } else { "off" }.to_string(), false, false),
                    (String::from("PG Aberration"), self.option_value(&["off", "low", "mid", "high", "super", "extreme"], self.settings.pg_aberration), false, false),
                ],
                3 => vec![
                    (String::from("HPVMX_VM_SAFETY"), self.option_value(&["prompt", "auto-save", "strict"], self.settings.vm_safety_policy), false, false),
                    (String::from("HPVMX_VM_DEFAULT_MEM"), format!("1024MB"), false, false),
                    (String::from("HPVMX_VM_DEFAULT_CPUS"), format!("1"), false, false),
                ],
                4 => vec![
                    (String::from("HPVMX_NET_PROFILE"), self.option_value(&["dhcp", "static", "loopback"], self.settings.network_profile), false, false),
                    (String::from("HPVMX_NET_TARGET"), String::from("none"), false, false),
                    (String::from("HPVMX_HTTPD_PORT"), String::from("8080"), false, false),
                ],
                5 => vec![
                    (String::from("HPVMX_STORAGE_POLICY"), self.option_value(&["preserve", "confirm-delete", "developer"], self.settings.storage_policy), false, false),
                    (String::from("Folder Absolute Sizes"), if self.settings.folder_absolute_sizes { "on" } else { "off" }.to_string(), false, false),
                    (String::from("Auto-refresh Storage"), if self.settings.auto_refresh_storage { "on" } else { "off" }.to_string(), false, false),
                    (String::from("Show Hidden Files"), if self.settings.show_hidden_files { "on" } else { "off" }.to_string(), false, false),
                ],
                6 => vec![
                    (String::from("HPVMX_PM_VERIFY"), self.option_value(&["standard", "quick", "full"], self.settings.package_policy), false, false),
                    (String::from("HPVMX_PM_AUTOHEAL"), String::from("off"), false, false),
                    (String::from("HPVMX_PM_INDEX"), String::from("/PACKAGES"), false, false),
                ],
                7 => vec![
                    (String::from("HPVMX_DEV_LEVEL"), self.option_value(&["normal", "verbose", "toolchain"], self.settings.developer_level), false, false),
                    (String::from("Terminal Font"), self.option_value(&["8x16", "dualscale (experimental)"], self.settings.terminal_font), false, false),
                    (String::from("ControlLang Support"), if self.settings.controllang_support { "on" } else { "off" }.to_string(), false, true),
                    (String::from("HPVMX_MICRO_C_TARGET"), String::from("x86_64"), false, false),
                ],
                8 => vec![
                    (String::from("HPVMX_SECURITY_POLICY"), self.option_value(&["standard", "paranoid", "lab"], self.settings.security_policy), false, false),
                    (String::from("Ring0 UDMI/UDXI"), if self.settings.ring0_udmi_udxi { "on" } else { "off" }.to_string(), true, false),
                    (String::from("HPVMX_AUTOLYTIC"), String::from("enabled"), false, false),
                ],
                _ => vec![
                    (String::from("HPVMX_VERSION"), String::from("1.0.0"), false, true),
                    (String::from("Hypervisor Build"), String::from("Debug-Rust-0.9.4"), false, true),
                    (String::from("Target Arch"), String::from("x86_64-unknown-uefi"), false, true),
                    (String::from("Firmware Vendor"), String::from("HPVMx Virtual Firmware"), false, true),
                    (String::from("License"), String::from("Custom / Proprietary"), false, true),
                ]
            }
        }
    }

    impl Runnable for X_Settings {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            let left_x = x + 20;
            let left_y = y + 20;
            let left_w = 200;
            let right_x = left_x + left_w + 20;
            let right_w = 500;
            let content_h = 500;

            pg.fill_rect(left_x, left_y, left_w, content_h, 0x1A1A1A);
            pg.draw_rect_outline(left_x, left_y, left_w, content_h, 0x333333);
            pg.fill_rect(right_x, left_y, right_w, content_h, 0x151515);
            pg.draw_rect_outline(right_x, left_y, right_w, content_h, 0x333333);

            let categories = [
                "General", "Boot & Init", "Interface", "VM & Safety",
                "Network", "Storage", "Packages", "Developer", "Security", "Display (PG)",
            ];

            let mut cy = left_y + 8;
            for (i, cat) in categories.iter().enumerate() {
                let selected = i == self.selected_settings_category_idx;
                if selected {
                    pg.fill_rect(left_x + 4, cy - 2, left_w - 8, 20, 0x005577);
                }
                pg.draw_text(left_x + 12, cy, cat, if selected { 0x00FFFF } else { 0xCCCCCC });
                cy += 24;
            }

            let rows = self.settings_rows();
            let mut ry = left_y + 12;
            for (i, (label, val, _readonly, _is_display)) in rows.iter().enumerate() {
                let selected = i == self.selected_settings_idx;
                if selected {
                    pg.fill_rect(right_x + 6, ry - 3, right_w - 12, 34, 0x223322);
                    pg.draw_rect_outline(right_x + 6, ry - 3, right_w - 12, 34, 0x00AA00);
                }
                pg.draw_text(right_x + 12, ry, label, if selected { 0x00FF00 } else { 0xFFFFFF });
                pg.draw_text(right_x + right_w - 160, ry, val, 0xFFFF00);
                ry += 38;
            }

            pg.draw_text(x + 20, y + 540, "[ / ] Category | UP/DOWN Option | ENTER/SPACE Toggle | +/- Cycle", 0x777777);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.settings = data.settings.clone();
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Printable(c) => {
                    let ch = char::from(c);
                    if ch == '[' || ch == '{' {
                        if self.selected_settings_category_idx > 0 {
                            self.selected_settings_category_idx -= 1;
                            self.selected_settings_idx = 0;
                        }
                    } else if ch == ']' || ch == '}' {
                        if self.selected_settings_category_idx < 9 {
                            self.selected_settings_category_idx += 1;
                            self.selected_settings_idx = 0;
                        }
                    } else if ch == ' ' || ch == '\r' || ch == '\n' {
                        match self.selected_settings_category_idx {
                            0 => match self.selected_settings_idx {
                                0 => self.settings.general_profile = (self.settings.general_profile + 1) % 3,
                                1 => self.settings.extra_debug_info = !self.settings.extra_debug_info,
                                3 => self.settings.experimental_mem_comp = !self.settings.experimental_mem_comp,
                                _ => {}
                            },
                            1 => match self.selected_settings_idx {
                                0 => self.settings.boot_target = (self.settings.boot_target + 1) % 3,
                                1 => self.settings.state_save_restore = !self.settings.state_save_restore,
                                _ => {}
                            },
                            2 => match self.selected_settings_idx {
                                0 => self.settings.interface_density = (self.settings.interface_density + 1) % 3,
                                1 => self.settings.ui_scaling = (self.settings.ui_scaling + 1) % 4,
                                2 => self.settings.extended_symbol_library = !self.settings.extended_symbol_library,
                                3 => self.settings.pg_vshaders = !self.settings.pg_vshaders,
                                4 => self.settings.pg_scanlines = !self.settings.pg_scanlines,
                                5 => self.settings.pg_dither = !self.settings.pg_dither,
                                6 => self.settings.pg_glitch = !self.settings.pg_glitch,
                                7 => self.settings.pg_aberration = (self.settings.pg_aberration + 1) % 6,
                                _ => {}
                            },
                            3 => match self.selected_settings_idx {
                                0 => self.settings.vm_safety_policy = (self.settings.vm_safety_policy + 1) % 3,
                                _ => {}
                            },
                            4 => match self.selected_settings_idx {
                                0 => self.settings.network_profile = (self.settings.network_profile + 1) % 3,
                                _ => {}
                            },
                            5 => match self.selected_settings_idx {
                                0 => self.settings.storage_policy = (self.settings.storage_policy + 1) % 3,
                                1 => self.settings.folder_absolute_sizes = !self.settings.folder_absolute_sizes,
                                2 => self.settings.auto_refresh_storage = !self.settings.auto_refresh_storage,
                                3 => self.settings.show_hidden_files = !self.settings.show_hidden_files,
                                _ => {}
                            },
                            6 => match self.selected_settings_idx {
                                0 => self.settings.package_policy = (self.settings.package_policy + 1) % 3,
                                _ => {}
                            },
                            7 => match self.selected_settings_idx {
                                0 => self.settings.developer_level = (self.settings.developer_level + 1) % 3,
                                1 => self.settings.terminal_font = (self.settings.terminal_font + 1) % 2,
                                2 => self.settings.controllang_support = !self.settings.controllang_support,
                                _ => {}
                            },
                            8 => match self.selected_settings_idx {
                                0 => self.settings.security_policy = (self.settings.security_policy + 1) % 3,
                                1 => self.settings.ring0_udmi_udxi = !self.settings.ring0_udmi_udxi,
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                Key::Special(ScanCode::UP) => {
                    if self.selected_settings_idx > 0 { self.selected_settings_idx -= 1; }
                }
                Key::Special(ScanCode::DOWN) => {
                    let rows = self.settings_rows();
                    if self.selected_settings_idx + 1 < rows.len() { self.selected_settings_idx += 1; }
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Settings {
        fn name(&self) -> &str { "Settings" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::GEAR_WINDOW_SETTINGS_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 13. Packages Tab
// =========================================================================
pub mod packages {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, _width: usize, _height: usize) {
        pg.draw_text(20, 100, "Packages", 0x00FF00);

        let package_names = ui.package_names();
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
            let Some(pkg) = ui.package_manager.registry.get(name) else { continue; };
            if idx == ui.selected_package_idx {
                pg.fill_rect(list_x + 2, y - 2, list_w - 4, 16, 0x444400);
            }
            pg.draw_text(list_x + 8, y, &alloc::format!("{:<28} {:?}", pkg.name, pkg.package_type), if idx == ui.selected_package_idx { 0xFFFF00 } else { 0xFFFFFF });
            pg.draw_package_icon(list_x + list_w - 24, y - 1, true);
            y += 18;
        }

        let detail_x = list_x + list_w + 30;
        let detail_w = 520;
        pg.draw_rect_outline(detail_x, list_y, detail_w, 420, 0x888888);
        pg.fill_rect(detail_x + 1, list_y + 1, detail_w - 2, 18, 0x333333);
        pg.draw_text(detail_x + 8, list_y + 4, "PACKAGE DETAILS", 0x00FF00);

        if let Some(name) = ui.selected_package_name() {
            if let Some(pkg) = ui.package_manager.registry.get(&name) {
                let mut dy = list_y + 30;
                pg.draw_text(detail_x + 10, dy, &alloc::format!("Name:      {}", pkg.name), 0xFFFFFF);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &alloc::format!("Version:   {}", pkg.version), 0x00FFFF);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &alloc::format!("Type:      {:?}", pkg.package_type), 0xAAAAAA);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &alloc::format!("Author:    {}", pkg.author), 0xFFFFFF);
                dy += 20;

                if let Some(ref url) = pkg.repo_url {
                    pg.draw_text(detail_x + 10, dy, &alloc::format!("Repo:      {}", url), 0x5555FF);
                    dy += 20;
                }

                let status_color = if pkg.has_compilation_issues { 0xFF5555 } else { 0x55FF55 };
                let status_text = if pkg.has_compilation_issues { "FAILED / ISSUES" } else { "READY / OK" };
                pg.draw_text(detail_x + 10, dy, &alloc::format!("Status:    {}", status_text), status_color);
                dy += 30;

                pg.draw_text(detail_x + 10, dy, "Dependencies:", 0x00FF00);
                dy += 20;
                if pkg.deps.is_empty() {
                    pg.draw_text(detail_x + 20, dy, "none", 0x888888);
                    dy += 20;
                } else {
                    for dep in &pkg.deps {
                        pg.draw_text(detail_x + 20, dy, &alloc::format!("- {}", dep), 0xCCCCCC);
                        dy += 16;
                    }
                }
                dy += 10;

                pg.draw_text(detail_x + 10, dy, "Description:", 0x00FF00);
                dy += 20;
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
            let is_focused = idx == ui.package_action_idx;
            pg.fill_rect(action_x, action_y, 110, 26, if is_focused { 0x00AA00 } else { 0x444444 });
            pg.draw_text(action_x + 8, action_y + 5, action, 0xFFFFFF);
            action_x += 120;
        }
        if ui.package_action_idx == 1 {
            pg.draw_text(40, action_y + 30, "Verifies package dependencies and integrity", 0x00AAAA);
        }
        pg.draw_text(40, action_y + 40, "UP/DOWN selects package, LEFT/RIGHT chooses action, ENTER runs it", 0x888888);
        pg.draw_text(40, action_y + 60, &ui.status_line, 0xFFFF00);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                if matches!(char::from(c), '\r' | '\n') {
                    ui.execute_package_action();
                    return true;
                }
                false
            }
            Key::Special(ScanCode::UP) => {
                ui.selected_package_idx = ui.selected_package_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::DOWN) => {
                let len = ui.package_manager.registry.len();
                if len > 0 {
                    ui.selected_package_idx = (ui.selected_package_idx + 1).min(len - 1);
                }
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.package_action_idx = ui.package_action_idx.saturating_sub(1);
                true
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.package_action_idx = (ui.package_action_idx + 1).min(5);
                true
            }
            _ => false,
        }
    }

    #[derive(Clone)]
    pub struct X_Packages {
        pub selected_package_idx: usize,
        pub package_action_idx: usize,
        pub registry: BTreeMap<String, Package>,
        pub status_line: String,
    }

    impl X_Packages {
        pub fn new() -> Self {
            Self {
                selected_package_idx: 0,
                package_action_idx: 0,
                registry: BTreeMap::new(),
                status_line: String::from("Ready"),
            }
        }

        pub fn package_names(&self) -> Vec<String> {
            self.registry.keys().cloned().collect()
        }

        pub fn selected_package_name(&self) -> Option<String> {
            let names = self.package_names();
            names.get(self.selected_package_idx).cloned()
        }
    }

    impl Runnable for X_Packages {
        fn draw(&self, pg: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
            pg.draw_text(x + 20, y + 20, "Package Manager", 0x00FF00);

            let list_x = x + 20;
            let list_y = y + 50;
            let list_w = 280;
            let list_h = 420;

            pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);
            pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, 18, 0x333333);
            pg.draw_text(list_x + 8, list_y + 4, "NAME                         TYPE", 0xCCCCCC);

            let package_names = self.package_names();
            let mut curr_y = list_y + 28;
            for (idx, name) in package_names.iter().enumerate() {
                if curr_y > list_y + list_h - 20 { break; }
                let Some(pkg) = self.registry.get(name) else { continue; };
                if idx == self.selected_package_idx {
                    pg.fill_rect(list_x + 2, curr_y - 2, list_w - 4, 16, 0x444400);
                }
                pg.draw_text(list_x + 8, curr_y, &format!("{:<28} {:?}", pkg.name, pkg.package_type), if idx == self.selected_package_idx { 0xFFFF00 } else { 0xFFFFFF });
                pg.draw_package_icon(list_x + list_w - 24, curr_y - 1, true);
                curr_y += 18;
            }

            let detail_x = list_x + list_w + 30;
            let detail_w = 400;
            pg.draw_rect_outline(detail_x, list_y, detail_w, 420, 0x888888);
            pg.fill_rect(detail_x + 1, list_y + 1, detail_w - 2, 18, 0x333333);
            pg.draw_text(detail_x + 8, list_y + 4, "PACKAGE DETAILS", 0x00FF00);

            if let Some(name) = self.selected_package_name() {
                if let Some(pkg) = self.registry.get(&name) {
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
                    let desc = &pkg.description;
                    let words: Vec<&str> = desc.split_whitespace().collect();
                    let mut line = String::new();
                    for word in words {
                        if line.len() + word.len() > 45 {
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
            let mut action_x = x + 20;
            let action_y = list_y + list_h + 24;
            for (idx, action) in actions.iter().enumerate() {
                let is_focused = idx == self.package_action_idx;
                pg.fill_rect(action_x, action_y, 110, 26, if is_focused { 0x00AA00 } else { 0x444444 });
                pg.draw_text(action_x + 8, action_y + 5, action, 0xFFFFFF);
                action_x += 120;
            }
            pg.draw_text(x + 20, action_y + 40, "UP/DOWN selects package, LEFT/RIGHT chooses action, ENTER runs it", 0x888888);
            pg.draw_text(x + 20, action_y + 60, &self.status_line, 0xFFFF00);
        }

        fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
            if let Some(data) = env.global_data.as_ref() {
                self.registry = data.package_manager.registry.clone();
                self.selected_package_idx = self.selected_package_idx.min(self.registry.len().saturating_sub(1));
            }
        }

        fn input(&mut self, key: Key) {
            match key {
                Key::Special(ScanCode::UP) => {
                    if self.selected_package_idx > 0 {
                        self.selected_package_idx -= 1;
                    }
                }
                Key::Special(ScanCode::DOWN) => {
                    if self.selected_package_idx + 1 < self.registry.len() {
                        self.selected_package_idx += 1;
                    }
                }
                Key::Special(ScanCode::LEFT) => {
                    if self.package_action_idx > 0 {
                        self.package_action_idx -= 1;
                    }
                }
                Key::Special(ScanCode::RIGHT) => {
                    if self.package_action_idx < 5 {
                        self.package_action_idx += 1;
                    }
                }
                Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                    // Dashboard will execute action after sync
                }
                _ => {}
            }
        }

        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    impl AppInfo for X_Packages {
        fn name(&self) -> &str { "Packages" }
        fn version(&self) -> &str { "1.0.0" }
        fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
        fn dimensions(&self) -> (usize, usize) { (800, 600) }
    }
}

// =========================================================================
// 14. System Information Tab
// =========================================================================
pub mod sysinfo {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let content_top = 80usize;
        resources::draw_sysinfo_view(&ui.resources, pg, margin, content_top, width, height);
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                let code = u16::from(c);
                if code == 9 || code == b'\t' as u16 {
                    ui.resmon_tab = ResourceMonitorTab::Resources;
                    true
                } else {
                    let ch = char::from(c).to_ascii_lowercase();
                    match ch {
                        'r' => { ui.resmon_tab = ResourceMonitorTab::Resources; true }
                        'p' => { ui.resmon_tab = ResourceMonitorTab::Processes; true }
                        _ => false,
                    }
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.resmon_tab = ResourceMonitorTab::Resources;
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.resmon_tab = ResourceMonitorTab::Processes;
                true
            }
            _ => false,
        }
    }
}
