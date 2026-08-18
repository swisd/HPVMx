use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct ErrorApp {
    pub(crate) error: String,
}

impl ErrorApp {
    pub fn new(error: &str) -> (Box<dyn Runnable>, (usize, usize)) {
        let app = ErrorApp { error: error.to_string() };
        let dims = AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }
    fn wrap_text(text: &str, max_chars_per_line: usize) -> Vec<&str> {
        let mut lines = Vec::new();
        let mut current_line_start = 0;
        let mut last_space_idx = None;
        let mut current_len = 0;

        for (i, c) in text.char_indices() {
            if c == ' ' {
                last_space_idx = Some(i);
            }

            current_len += 1;

            if current_len > max_chars_per_line {
                if let Some(space_idx) = last_space_idx {
                    lines.push(&text[current_line_start..space_idx]);
                    current_line_start = space_idx + 1;
                    last_space_idx = None;
                    current_len = i - space_idx;
                } else {
                    lines.push(&text[current_line_start..i]);
                    current_line_start = i;
                    current_len = 1;
                }
            }
        }

        if current_line_start < text.len() {
            lines.push(&text[current_line_start..]);
        }

        lines
    }
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

impl RunnableClone for ErrorApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        Box::new(ErrorApp {
            error: self.error.clone(),
        })
    }
}

impl Runnable for ErrorApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.draw_icon(x+10, y+10, 16, 16, &icons::ERROR_16_ICON_DATA);
        let error: Vec<&str> = self.error.split('\n').collect();
        graphics_entity.draw_text(x+30, y+12, error[0].split(':').next().unwrap_or(&error[0]), 0xFFFFFF);
        graphics_entity.draw_text(x+50, y+24, error[0].split(':').last().unwrap_or(""), 0xFFFFFF);
        if error.len() > 1 {
            let mut xmod = 48;
            for line in Self::wrap_text(error[1], 32usize) {
                graphics_entity.draw_text(x + 5, y + xmod, line, 0xFFFFFF);
                xmod += 18;
            }
        }
        graphics_entity.draw_rect_outline(x+30, y+160, 90, 20, 0xFFFFFF);
        graphics_entity.draw_text(x+32, y+162, "[ESC] Close", 0xFFFFFF);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        //
    }

    fn input(&mut self, key: Key) {
        //
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}