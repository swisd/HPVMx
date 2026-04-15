use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::Runnable;
use crate::ui::pixel_graphics::PixelGraphics;

pub struct SimpleApp {
    pub color: [u32;3]
}

impl Runnable for SimpleApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>) {
        graphics_entity.fill_rect(100, 100, 200, 100, self.color[0]);
        graphics_entity.draw_text(125, 125, "application", self.color[1]);
        graphics_entity.draw_text(125, 175, "[C] to chnage color", self.color[2])
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