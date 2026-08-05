use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

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
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Network Manager", 0x00FF00);
        pg.draw_text(x + 20, y + 50, &format!("Target: {}", self.network_target), 0xFFFFFF);
        
        let actions = ["SNP Init", "Ping", "Scan", "HTTP Start", "HTTP Stop"];
        let mut curr_x = x + 20;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == self.selected_network_action_idx;
            let color = if is_focused { 0x00AA00 } else { 0x444444 };
            pg.fill_rect(curr_x, y + 80, 100, 24, color);
            pg.draw_text(curr_x + 5, y + 84, action, 0xFFFFFF);
            curr_x += 110;
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::LEFT) => {
                if self.selected_network_action_idx > 0 { self.selected_network_action_idx -= 1; }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.selected_network_action_idx < 4 { self.selected_network_action_idx += 1; }
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
