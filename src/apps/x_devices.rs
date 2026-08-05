use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, DeviceCategory};
use crate::ui::pixel_graphics::PixelGraphics;

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
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Device Manager", 0x00FF00);
        
        let mut curr_y = y + 50;
        for (i, cat) in self.categories.iter().enumerate() {
            let is_selected = i == self.selected_device_idx;
            let color = if is_selected { 0xFFFF00 } else { 0xFFFFFF };
            pg.draw_text(x + 20, curr_y, &format!("{}: {} devices", cat.name, cat.devices.len()), color);
            curr_y += 20;
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::UP) => {
                if self.selected_device_idx > 0 { self.selected_device_idx -= 1; }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.selected_device_idx + 1 < self.categories.len() { self.selected_device_idx += 1; }
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
