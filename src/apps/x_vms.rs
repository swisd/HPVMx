use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, VmDisplayInfo};
use crate::ui::pixel_graphics::PixelGraphics;

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
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
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
        // `table_y` includes the window's screen-space origin.  Calculate
        // the available height in local layout coordinates so moving a window
        // cannot underflow this value and trigger a massive render loop.
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

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
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
