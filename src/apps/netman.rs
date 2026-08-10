use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
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

impl RunnableClone for NetworkManagerApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        todo!()
    }
}

impl Runnable for NetworkManagerApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.draw_text(x+10, y+10, "Network Manager", 0xFF9955)
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        //
    }

    fn input(&mut self, key: Key) {
        //
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}