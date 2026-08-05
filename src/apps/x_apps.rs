use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

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
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        let margin = 16usize;
        let gutter = 12usize;
        let width = 800;
        let height = 600;

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

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}

    fn input(&mut self, key: Key) {
        let cols = (800 - 32) / (100 + 12); // Approximate
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
