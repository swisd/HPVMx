use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, ResourceMonitorTab, SystemResources};
use crate::ui::pixel_graphics::PixelGraphics;
use crate::{vdebug, TSC_PER_US};

#[derive(Clone)]
pub struct X_Resources {
    pub resources: SystemResources,
    resmon_tab: ResourceMonitorTab
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
            resmon_tab: ResourceMonitorTab::Resources,
        }
    }
}

impl Runnable for X_Resources {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 15usize;
        let content_top = y + 12;
        let width = 600;
        let height = 500;
        match self.resmon_tab {
            ResourceMonitorTab::Resources => {
                pg.draw_text(margin, content_top - 6, "[ Resources ]  | Processes |", 0xFFFFFF);

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
                pg.draw_line_graph(panel_x + 10, bar_y + 20, 340, 60, &self.resources.mem_history, 100, 0x00FF00, 60);

                // I/O Stats and Graphs
                let io_y = bar_y + 80 + gutter * 2;
                pg.draw_text(panel_x + 10, io_y, "Net Traffic (RX:Cyan TX:Yellow)", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, io_y + 20, 165, 50, &self.resources.net_rx_history, 1024, 0x00FFFF, 60);
                pg.draw_line_graph(panel_x + 185, io_y + 20, 165, 50, &self.resources.net_tx_history, 1024, 0xFFFF00, 60);

                let disk_y = io_y + 80;
                pg.draw_text(panel_x + 10, disk_y, "Disk I/O (Read:White Write:Red)", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, disk_y + 20, 165, 50, &self.resources.disk_read_history, 1024, 0xFFFFFF, 60);
                pg.draw_line_graph(panel_x + 185, disk_y + 20, 165, 50, &self.resources.disk_write_history, 1024, 0xFF0000, 60);

                let gpu_y = disk_y + 80;
                pg.draw_text(panel_x + 10, gpu_y, "GPU Usage:", 0xCCCCCC);
                pg.draw_line_graph(panel_x + 10, gpu_y + 20, 165, 50, &self.resources.gpu_history, 100, 0xFF7700, 60);


                // Right CPU core list panel or Total CPU Graph
                let right_x = panel_x + panel_w + gutter * 2;
                let right_y = panel_y;
                let right_w = core::cmp::min(width - right_x - margin, 360);
                let right_h = core::cmp::min(height - right_y - 100, 260);
                pg.draw_rect_outline(right_x, right_y, right_w, right_h, 0x888888);
                pg.draw_text_bg(right_x + 10, right_y - 4, "Total CPU Usage History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 10, right_w - 20, 80, &self.resources.cpu_history, 100, 0x00FF00, 60);

                pg.draw_text(right_x + 10, right_y + 100, "CPU Usage per Core:", 0xFFFFFF);
                for i in 0..self.resources.cpu_count {
                    let row_y = right_y + 120 + (i as usize * (line_h + 4));
                    if row_y + line_h > right_y + right_h - 8 { break; }
                    let usage = if i < self.resources.cpu_core_usage.len() as u32 { self.resources.cpu_core_usage[i as usize] } else { 0 };
                    pg.draw_text(right_x + 10, row_y, &alloc::format!("C{}:{:>2}%", i, usage), 0xCCCCCC);
                    pg.draw_progress_bar(right_x + 70, row_y, right_w - 80, 12, usage as usize, 100, 0x00FF00);
                }

                pg.draw_text_bg(right_x + 10, right_y + 300, "FPS History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 300, right_w - 20, 80, &self.resources.fps_history, 75, 0xFF44FF, 60);
                pg.draw_text_bg(right_x + 10, right_y + 400, "Frame MS History:", 0xFFFFFF, 0x222222);
                pg.draw_line_graph(right_x + 10, right_y + 400, right_w - 20, 80, &self.resources.ft_ms_history, 750, 0xFFAAFF, 60);

                // Heatmap for CPU Core usage
                let hm_y = right_y + 500;
                pg.draw_text(right_x + 10, hm_y, "CPU Heatmap (Real-time Core Stress):", 0xFFFFFF);
                let mut hm_data = [0.0f32; 16];
                for i in 0..core::cmp::min(self.resources.cpu_core_usage.len(), 1) {
                    hm_data[i] = self.resources.cpu_core_usage[i] as f32 / 100.0;
                }
                pg.draw_heatmap(right_x + 10, hm_y + 20, right_w - 20, 80, 4, 4, &hm_data);

                // draw u64 le text for all stats

                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, self.resources.cpu_usage as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, self.resources.used_memory_mb as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, self.resources.frame_ms as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, self.resources.gpu_usage as u64, 0xFFFFFF);
            }
            ResourceMonitorTab::Processes => {
                pg.draw_text(margin, content_top - 6, "| Resources |  [ Processes ]", 0xFFFFFF);

                let panel_x = margin;
                let panel_y = content_top + margin;
                let panel_w = 600usize;
                let panel_h = 480usize;

                pg.draw_text_bg(panel_x, panel_y - 4, "Process Monitor", 0x20FF20, 0x222222);
                let headers: &[&str] = &["name", "pid", "cycles", "cpu time"];
                // let cycles_string = format!("{:#?}", self.cycles);
                // let cycles_str = cycles_string.as_str();
                let mut rows: Vec<&[&str]> = vec![
                    &["system", "0", "x", "x"],
                    &["hardware", "9", "x", "x"],
                ];

                // // 1. Store actual fixed-size String arrays in memory
                // let mut row_storage: Vec<[String; 3]> = Vec::with_capacity(self.active_apps.len());
                //
                // for app in &self.active_apps {
                //     let name = app.application.name.to_string();
                //     let pid = format!("{:#?}", app.pid);
                //     let cycles = "x".to_string();
                //
                //     row_storage.push([name, pid, cycles]);
                // }
                //
                // // 2. Build slice references into the stored arrays
                // // Constructing &[&str] views referencing the backing String data
                // let row_refs: Vec<[&str; 3]> = row_storage
                //     .iter()
                //     .map(|row| [row[0].as_str(), row[1].as_str(), row[2].as_str()])
                //     .collect();
                //
                // // 3. Push slice views into rows
                // for row in &row_refs {
                //     rows.push(row);
                // }

                // 2. Hoist the backing storage variables OUTSIDE the fallback scope
                // These must stay alive as long as `rows` is being used
                let mut row_storage: Vec<[String; 4]> = Vec::new();
                let mut row_refs: Vec<[&str; 4]> = Vec::new();

                // Wrap the logic in a closure or function that returns an Option/Result
                // 3. Fallible logic scope (your "try" block)
                // let mut allocate_rows = || -> Option<()> {
                //     // Safely allocate room for the strings
                //     row_storage.try_reserve(self.active_apps.len()).ok()?;
                //
                //     for app in &self.active_apps {
                //         let name = app.application.name.to_string();
                //         let pid = format!("{:#?}", app.pid);
                //         let cycles = "x".to_string();
                //
                //         row_storage.push([name, pid, cycles]);
                //     }
                //
                //     // Safely reserve room for the slice references
                //     row_refs.try_reserve(self.active_apps.len()).ok()?;
                //
                //     // Build the string views into the outer row_refs
                //     row_refs.extend(
                //         row_storage
                //             .iter()
                //             .map(|row| [row[0].as_str(), row[1].as_str(), row[2].as_str()])
                //     );
                //
                //     // Push slice views into rows
                //     for row in &row_refs {
                //         rows.push(row);
                //     }
                //
                //     Some(())
                // };
                //
                // // "Try/Except" wrapper: If it returns None, execution safely skips the rest
                // if allocate_rows().is_none() {
                //     vdebug!("ui", "OOM alloc error DashboardTab::Resources.ResourceMonitorTab::Processes.var:rows")
                // }

                // 2. Use a labeled loop as a "try" block that we can break out of early
                vdebug!("ui", "[TRY] Entering 'try_block loop...");

                // 'try_block: loop {
                //     vdebug!("ui", "[TRY] Checking capacity for row_storage...");
                //     // Fallibly allocate capacity for strings
                //     if row_storage.try_reserve(self.active_apps.len()).is_err() {
                //         vdebug!("ui", "[FAIL] Allocation failed inside row_storage.try_reserve!");
                //         break 'try_block;
                //     }
                //     vdebug!("ui", "[SUCCESS] row_storage memory reserved.");
                //
                //     vdebug!("ui", "[TRY] Beginning active_apps iteration loop...");
                //
                //     // row_storage.push([String::from("dashboard"), String::from("14"), format!("{:#?}", self.cycles), format!("{:#?}", ((self.resources.fps as u64 * self.cycles as u64) / (TSC_PER_US * 1000000)) * 100)]);
                //
                //     for (index, app) in self.active_apps.iter().enumerate() {
                //         // NOTE: If your UEFI app freezes right here, one of these three allocations is failing.
                //         // Rust's default allocator will panic on OOM here unless custom catch mechanics are present.
                //         let name = app.application.name.to_string();
                //         let pid = format!("{:#?}", app.pid);
                //         let total_cyc = app.ui_time + app.cpu_time;
                //         let cycles = format!("{:#?}", total_cyc);
                //
                //         row_storage.push([name, pid, cycles, format!("{:#?}", ((self.resources.fps as u64 * app.cpu_time as u64) / TSC_PER_US * 1000000) * 100)]);
                //     }
                //     vdebug!("ui", "[SUCCESS] Finished active_apps loop. row_storage populated.");
                //
                //     vdebug!("ui", "[TRY] Checking capacity for row_refs...");
                //     // Fallibly allocate capacity for the slice references
                //     if row_refs.try_reserve(self.active_apps.len()).is_err() {
                //         vdebug!("ui", "[FAIL] Allocation failed inside row_refs.try_reserve!");
                //         break 'try_block;
                //     }
                //     vdebug!("ui", "[SUCCESS] row_refs memory reserved.");
                //
                //     vdebug!("ui", "[TRY] Extending row_refs mapping...");
                //     // Build views using references pointing directly to row_storage strings
                //     row_refs.extend(
                //         row_storage
                //             .iter()
                //             .map(|row| [row[0].as_str(), row[1].as_str(), row[2].as_str(), row[3].as_str()])
                //     );
                //     vdebug!("ui", "[SUCCESS] row_refs extended successfully.");
                //
                //     vdebug!("ui", "[TRY] Pushing slice views into rows vector...");
                //     // Safely push slice views into rows
                //     for row in &row_refs {
                //         rows.push(&row[..]);
                //     }
                //     vdebug!("ui", "[SUCCESS] All elements safely integrated into rows.");
                //
                //     break 'try_block;
                // }

                vdebug!("ui", "[EXIT] Left the 'try_block loop.");


                pg.draw_table_view(panel_x, panel_y + 4, panel_w, panel_h, headers, rows);
            }
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            self.resources = data.resources.clone();
            self.resmon_tab = data.resmon_tab.clone();
        }
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::LEFT) => self.resmon_tab = ResourceMonitorTab::Resources,
            Key::Special(ScanCode::RIGHT) => self.resmon_tab = ResourceMonitorTab::Processes,
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Resources {
    fn name(&self) -> &str { "Resource Monitor" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::INTEGRATED_CIRCUIT_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
