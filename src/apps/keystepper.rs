use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct CH64App {

}
impl AppInfo for CH64App {
    fn name(&self) -> &str {
        "CH64App"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::CUBE_WINDOW_RED_32_ICON_DATA    }

    fn dimensions(&self) -> (usize, usize) {
        (120, 100)
    }
}

impl Runnable for CH64App {
    fn draw(&self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {

        graphics_entity.draw_u64_le_sym(x+ 5, y+ 5, 0x331cdbc25e55d768 , 0xFFFFFF);
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {
        // Update app state
    }

    fn input(&mut self, key: Key) {

    }
}