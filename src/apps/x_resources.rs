use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, SystemResources};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_Resources {
    pub resources: SystemResources,
}

impl X_Resources {
    pub fn new() -> Self {
        Self {
            resources: SystemResources {
                cpu_count: 0,
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
                cpu_core_usage: vec![],
                cpu_history: vec![],
            },
        }
    }
}

impl Runnable for X_Resources {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 15usize;

        let panel_x = x + margin;
        let panel_y = y + margin;
        let panel_w = 360usize;
        let panel_h = 480usize;
        pg.draw_rect_outline(panel_x, panel_y, panel_w, panel_h, 0x888888);
        pg.draw_text_bg(panel_x, panel_y - 4, "Resource Monitor", 0x20FF20, 0x222222);

        pg.draw_text(panel_x + 10, panel_y + 16, &format!("CPU Cores: {}", self.resources.cpu_count), 0xFFFFFF);
        pg.draw_text(panel_x + 10, panel_y + 16 + line_h, &format!("Total Memory: {} MB", self.resources.total_memory_mb), 0xFFFFFF);
        pg.draw_text(panel_x + 10, panel_y + 16 + line_h * 2, &format!("Used Memory: {} MB", self.resources.used_memory_mb), 0xFFFFFF);

        let bar_y = panel_y + 16 + line_h * 3 + gutter;
        pg.draw_text(panel_x + 10, bar_y, "Memory History (10s):", 0xCCCCCC);
        pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &self.resources.mem_history, 100, 0x00FF00, 60);

        let io_y = bar_y + 80 + gutter * 2;
        pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
        pg.draw_line_graph(panel_x + 10, io_y + 20, 340, 60, &self.resources.net_rx_history, 1000, 0x00FFFF, 60);
        pg.draw_line_graph(panel_x + 10, io_y + 20, 340, 60, &self.resources.net_tx_history, 1000, 0xFFFF00, 60);

        let disk_y = io_y + 80 + gutter * 2;
        pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Gray)", 0xCCCCCC);
        pg.draw_line_graph(panel_x + 10, disk_y + 20, 340, 60, &self.resources.disk_read_history, 10000, 0xFFFFFF, 60);
        pg.draw_line_graph(panel_x + 10, disk_y + 20, 340, 60, &self.resources.disk_write_history, 10000, 0x888888, 60);
        
        // Right side
        let info_x = panel_x + panel_w + gutter;
        pg.draw_text(info_x, panel_y, "GPU & Performance", 0x00FF00);
        pg.draw_text(info_x, panel_y + 20, &format!("FPS: {}", self.resources.fps), 0xFFFFFF);
        pg.draw_text(info_x, panel_y + 40, &format!("Frame Time: {} ms", self.resources.frame_ms), 0xFFFFFF);
        pg.draw_line_graph(info_x, panel_y + 60, 200, 60, &self.resources.fps_history, 144, 0xFF00FF, 60);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {}

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Resources {
    fn name(&self) -> &str { "Resource Monitor" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
