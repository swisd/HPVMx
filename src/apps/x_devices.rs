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
        let x_off = x + 20;
        let mut y_off = y + 20;
        pg.draw_text(x_off, y_off, "Device Manager", 0x00FF00);

        let mut curr_y = y + 50;
        let mut current_idx = 0;

        for cat in &self.categories {
            let expanded_icon = if cat.expanded { "[-] " } else { "[+] " };
            let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xAAAAAA };
            pg.draw_text(x_off, curr_y, &alloc::format!("{}{}{} ({})", expanded_icon, cat.icon, cat.name, cat.devices.len()), color);
            curr_y += 20;
            current_idx += 1;

            if cat.expanded {
                for dev in &cat.devices {
                    let color = if current_idx == self.selected_device_idx { 0xFFFF00 } else { 0xFFFFFF };
                    pg.draw_icon(x_off + 15, curr_y - 2, 16, 16, if cat.name == "Network Adapters" { &pixel_graphics::icons::PCI_GREEN_ICON_DATA } else { &pixel_graphics::icons::PCI_BLUE_ICON_DATA });

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

                    let display_path = if let Some(idx) = split_idx { &path[idx..] } else { path };
                    pg.draw_text(x_off + 35, curr_y, &alloc::format!("{:<12} {}", dev.name, display_path), color);
                    curr_y += 18;
                    current_idx += 1;
                }
            }
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            self.categories = data.categories.clone();
        }
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::UP) => {
                if self.selected_device_idx > 0 { self.selected_device_idx -= 1; }
            }
            Key::Special(ScanCode::DOWN) => {
                let mut total = 0;
                for cat in &self.categories {
                    total += 1;
                    if cat.expanded { total += cat.devices.len(); }
                }
                if self.selected_device_idx + 1 < total { self.selected_device_idx += 1; }
            }
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Dashboard handles expansion toggling after sync
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
