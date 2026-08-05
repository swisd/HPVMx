use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_Packages {
    pub selected_package_idx: usize,
}

impl X_Packages {
    pub fn new() -> Self {
        Self {
            selected_package_idx: 0,
        }
    }
}

impl Runnable for X_Packages {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Package Manager", 0x00FF00);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {}
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Packages {
    fn name(&self) -> &str { "Packages" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
