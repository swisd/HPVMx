use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct SimpleApp {
    pub color: [u32;3]
}
impl AppInfo for SimpleApp {
    fn name(&self) -> &str {
        "SimpleAPp"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::TRAFFIC_LIGHT_32_ICON_DATA
    }
}

impl Runnable for SimpleApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.fill_rect(x+50, y+50, 200, 100, self.color[0]);
        graphics_entity.draw_text(x+75, y+75, "application", self.color[1]);
        graphics_entity.draw_text(x+75, y+125, "[C] to chnage color", self.color[2])
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {
        // Update app state
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                match ch {
                    'c' => {self.color.rotate_right(1)}
                    _ => {}
                }
            }
            _ => {}
        }
    }
}