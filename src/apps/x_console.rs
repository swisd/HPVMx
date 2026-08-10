use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::pm::PackageManager;
use crate::ui::pixel_graphics::PixelGraphics;

/// Windowed counterpart of the legacy Console dashboard tab.
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
        pg.draw_text(x + MARGIN + 5, y + HEIGHT - 60, "ENTER sends, END edits, ESC leaves edit mode", 0x888888);
        pg.draw_text(x + MARGIN + 5, y + HEIGHT - 85, &alloc::format!("HPVMx> {}", self.term_buf), 0xDDDDDD);
    }

    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::END) => self.term_selected = true,
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
