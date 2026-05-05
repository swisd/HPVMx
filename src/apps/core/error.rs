use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct ErrorApp {
    pub(crate) error: String,
}

impl AppInfo for ErrorApp {
    fn name(&self) -> &str {
        "ERROR"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::ERROR_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (250, 200)
    }
}

impl Runnable for ErrorApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.draw_icon(x+10, y+10, 16, 16, &icons::ERROR_16_ICON_DATA);
        let error: Vec<&str> = self.error.split('\n').collect();
        graphics_entity.draw_text(x+30, y+12, error[0].split(':').next().unwrap_or(&error[0]), 0xFFFFFF);
        graphics_entity.draw_text(x+50, y+24, error[0].split(':').last().unwrap_or(""), 0xFFFFFF);
        if error.len() > 1 {
            graphics_entity.draw_text(x + 5, y + 48, error[1], 0xFFFFFF)
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>) {
        //
    }

    fn input(&mut self, key: Key) {
        //
    }
}