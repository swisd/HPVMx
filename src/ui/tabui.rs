use alloc::{format, vec, vec::Vec};
use alloc::string::{String, ToString};
use uefi::proto::console::text::{Key, ScanCode};
use crate::ui::{
    pixel_graphics::{self, PixelGraphics},
    DashboardTab, DashboardUI, DeviceCategory, DiskTabInfo, EditorMode, FileEntry, FilePendingAction,
    ResourceMonitorTab, SystemResources, TextEditor, UiSettings, VmDisplayInfo,
};
use crate::{runtime, vdebug, TSC_PER_US, HYPERVISOR};
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
}

// =========================================================================
// 5. Resources Tab
// =========================================================================
pub mod resources {
    use super::*;

    pub fn draw(ui: &DashboardUI, pg: &mut PixelGraphics, _x: usize, _y: usize, width: usize, height: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 15usize;
        let content_top = 80usize;

        match ui.resmon_tab {
            ResourceMonitorTab::Resources => {
                pg.draw_text(margin, content_top - 6, "[ Resources ]  | Processes |", 0xFFFFFF);

                let panel_x = margin;
                let panel_y = content_top + margin;
                let panel_w = 360usize;
                let panel_h = 480usize;
                pg.draw_rect_outline(panel_x, panel_y, panel_w, panel_h, 0x888888);
                pg.draw_text_bg(panel_x, panel_y - 4, "Resource Monitor", 0x20FF20, 0x222222);

                pg.draw_text(panel_x + 10, panel_y + 16, &alloc::format!("CPU Cores: {}", ui.resources.cpu_count), 0xFFFFFF);
                pg.draw_text(panel_x + 10, panel_y + 16 + line_h, &alloc::format!("Total Memory: {} MB", ui.resources.total_memory_mb), 0xFFFFFF);
                pg.draw_text(panel_x + 10, panel_y + 16 + line_h * 2, &alloc::format!("Used Memory: {} MB", ui.resources.used_memory_mb), 0xFFFFFF);

                let bar_y = panel_y + 16 + line_h * 3 + gutter;
                pg.draw_text(panel_x + 10, bar_y, "Memory History (10s):", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &ui.resources.mem_history, 100, 0x00FF00, 60);

                let io_y = bar_y + 80 + gutter * 2;
                pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, io_y + 20, 165, 50, &ui.resources.net_rx_history, 1024, 0x00FFFF, 60);
                pg.draw_line_graph(panel_x + 185, io_y + 20, 165, 50, &ui.resources.net_tx_history, 1024, 0xFFFF00, 60);

                let disk_y = io_y + 80;
                pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Red)", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, disk_y + 20, 165, 50, &ui.resources.disk_read_history, 1024, 0xFFFFFF, 60);
                pg.draw_line_graph(panel_x + 185, disk_y + 20, 165, 50, &ui.resources.disk_write_history, 1024, 0xFF0000, 60);

                let gpu_y = disk_y + 80;
                pg.draw_text(panel_x + 10, gpu_y, "GPU Usage:", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, gpu_y + 20, 165, 50, &ui.resources.gpu_history, 100, 0xFF7700, 60);

                let right_x = panel_x + panel_w + gutter * 2;
                let right_y = panel_y;
                let right_w = core::cmp::min(width.saturating_sub(right_x + margin), 360);
                let right_h = core::cmp::min(height.saturating_sub(right_y + 100), 260);
                pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x888888);
                pg.draw_text_bg(right_x + 10, right_y - 4, "Total CPU Usage History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 10, right_w - 20, 80, &ui.resources.cpu_history, 100, 0x00FF00, 60);

                pg.draw_text(right_x + 10, right_y + 100, "CPU Usage per Core:", 0xFFFFFF);
                for i in 0..ui.resources.cpu_count {
                    let row_y = right_y + 120 + (i as usize * (line_h + 4));
                    if row_y + line_h > right_y + right_h - 8 { break; }
                    let usage = if (i as usize) < ui.resources.cpu_core_usage.len() { ui.resources.cpu_core_usage[i as usize] } else { 0 };
                    pg.draw_text(right_x + 10, row_y, &alloc::format!("C{}:{:>2}%", i, usage), 0xCCCCCC);
                    pg.draw_progress_bar(right_x + 70, row_y, right_w.saturating_sub(80), 12, usage as usize, 100, 0x00FF00);
                }

                pg.draw_text_bg(right_x + 10, right_y + 300, "FPS History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 300, right_w.saturating_sub(20), 80, &ui.resources.fps_history, 75, 0xFF44FF, 60);
                pg.draw_text_bg(right_x + 10, right_y + 400, "Frame MS History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 400, right_w.saturating_sub(20), 80, &ui.resources.ft_ms_history, 750, 0xFFAAFF, 60);

                let hm_y = right_y + 500;
                pg.draw_text(right_x + 10, hm_y, "CPU Heatmap (Real-time Core Stress):", 0xFFFFFF);
                let mut hm_data = [0.0f32; 16];
                for i in 0..core::cmp::min(ui.resources.cpu_core_usage.len(), 1) {
                    hm_data[i] = ui.resources.cpu_core_usage[i] as f32 / 100.0;
                }
                pg.draw_heatmap(right_x + 10, hm_y + 20, right_w.saturating_sub(20), 80, 4, 4, &hm_data);

                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, ui.resources.cpu_usage as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, ui.resources.used_memory_mb as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, ui.resources.frame_ms as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, ui.resources.gpu_usage as u64, 0xFFFFFF);
            }
            ResourceMonitorTab::Processes => {
                ui.draw_processes_tab(pg, margin, content_top, width, height);
            }
        }
    }

    pub fn logic(_ui: &mut DashboardUI) {}

    pub fn input(ui: &mut DashboardUI, key: Key) -> bool {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                if matches!(ui.resmon_tab, ResourceMonitorTab::Processes) {
                    match ch {
                        'k' => { ui.kill_selected_process(); true }
                        'f' => { ui.focus_selected_process(); true }
                        'm' => { ui.toggle_min_selected_process(); true }
                        '\r' | '\n' => { ui.focus_selected_process(); true }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                ui.resmon_tab = ResourceMonitorTab::Processes;
                true
            }
            Key::Special(ScanCode::LEFT) => {
                ui.resmon_tab = ResourceMonitorTab::Resources;
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
                                Err(_) => ui.ui_error(29),
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
}
