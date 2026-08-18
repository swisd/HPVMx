use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, ResourceMonitorTab, SystemResources, VmDisplayInfo};
use crate::ui::pixel_graphics::PixelGraphics;
use crate::{vdebug, TSC_PER_US};

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

                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, self.resources.cpu_usage as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, self.resources.used_memory_mb as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 8, hm_y + 20, self.resources.frame_ms as u64, 0xFFFFFF);
                pg.draw_u64_le_sym(panel_x + 20, hm_y + 20, self.resources.gpu_usage as u64, 0xFFFFFF);
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
                pg.draw_text(panel_x + 10, btn_y + 4, "[K] End Task", 0xFFFFFF);

                // [ Focus Window (F) ]
                pg.fill_rect(panel_x + 120, btn_y, 125, btn_h, 0x006666);
                pg.draw_rect_outline(panel_x + 120, btn_y, 125, btn_h, 0x00FFFF);
                pg.draw_text(panel_x + 130, btn_y + 4, "[F] Focus Window", 0xFFFFFF);

                // [ Min/Restore (M) ]
                pg.fill_rect(panel_x + 255, btn_y, 130, btn_h, 0x334466);
                pg.draw_rect_outline(panel_x + 255, btn_y, 130, btn_h, 0x6699FF);
                pg.draw_text(panel_x + 265, btn_y + 4, "[M] Min/Restore", 0xFFFFFF);

                // [ < Resources ]
                pg.fill_rect(panel_x + 395, btn_y, 115, btn_h, 0x333333);
                pg.draw_rect_outline(panel_x + 395, btn_y, 115, btn_h, 0x888888);
                pg.draw_text(panel_x + 405, btn_y + 4, "[<] Resources", 0xFFFFFF);

                // Key hints
                pg.draw_text(panel_x + 520, btn_y + 4, "UP/DOWN: Select | K: Kill | F: Focus", 0x888888);
            }
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            self.resources = data.resources.clone();
            self.resmon_tab = data.resmon_tab.clone();
            self.vms = data.vms.clone();
            self.cycles = data.cycles;
            self.selected_process_idx = data.selected_process_idx;

            self.procs.clear();
            for (idx, app) in data.active_apps.iter().enumerate() {
                self.procs.push(ProcessItem {
                    pid: app.pid,
                    name: app.application.name.clone(),
                    state: if app.window.is_minimized { "Minimized".to_string() } else if data.focused_process_idx == Some(idx) { "Focused".to_string() } else { "Running".to_string() },
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
        }
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::LEFT) => self.resmon_tab = ResourceMonitorTab::Resources,
            Key::Special(ScanCode::RIGHT) => self.resmon_tab = ResourceMonitorTab::Processes,
            Key::Special(ScanCode::UP) => {
                self.selected_process_idx = self.selected_process_idx.saturating_sub(1);
            }
            Key::Special(ScanCode::DOWN) => {
                let total = self.total_process_count();
                self.selected_process_idx = (self.selected_process_idx + 1).min(total.saturating_sub(1));
            }
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
