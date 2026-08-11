use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_Editor {}

impl X_Editor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runnable for X_Editor {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Text Editor", 0x00FF00);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            if let Some(editor) = data.editor.as_ref() {
                // If we don't have an editor or it's different, we might want to sync.
                // However, X_Editor is usually its own state.
                // For "copy logic" we might just want to be aware of the global editor.
            }
        }
    }
    fn input(&mut self, key: Key) {}
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Editor {
    fn name(&self) -> &str { "Editor" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::SCRIPT_YELLOW_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
