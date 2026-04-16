use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct InstructionManualApp {

}

impl AppInfo for InstructionManualApp {
    fn name(&self) -> &str {
        "Instruction Manual"
    }

    fn version(&self) -> &str {
        "0.1.1"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::MANUAL_BOOK_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (600, 500)
    }
}

impl Runnable for InstructionManualApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.draw_text(x+10, y+10, "Instruction Manual", 0xFF9955)
    }

    fn logic(&mut self, vars: &mut Vec<String>) {
        //
    }

    fn input(&mut self, key: Key) {
        //
    }
}