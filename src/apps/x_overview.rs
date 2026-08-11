use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;
use crate::{runtime, TSC_PER_US};

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
            cpu_count: 0,
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
        // Draw side menu background
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

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
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

    fn input(&mut self, key: Key) {
        // Handle input if any
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Overview {
    fn name(&self) -> &str { "Overview" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::CUBE_WINDOW_RED_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
