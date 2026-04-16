use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct NetworkManagerApp {

}

impl AppInfo for NetworkManagerApp {
    fn name(&self) -> &str {
        "Network Manager"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::BLADE_NETWORK_32_LIGHT_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (600, 600)
    }
}

impl Runnable for NetworkManagerApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.draw_text(x+10, y+10, "Network Manager", 0xFF9955)
    }

    fn logic(&mut self, vars: &mut Vec<String>) {
        //
    }

    fn input(&mut self, key: Key) {
        //
    }
}