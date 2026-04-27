use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct InstructionManualApp {
    pages: Vec<Vec<String>>,
    current_page: usize,
    scroll_y: usize,
    width: usize,
}

impl InstructionManualApp {
    pub fn new(content: &str, window_width: usize) -> Self {
        let mut pages = Vec::new();
        let mut current_page_lines = Vec::new();
        let max_chars_per_line = (window_width - 60) / 16; // Base scale 1 wrap

        for raw_line in content.lines() {
            // 1. Handle Paging (H1 Split)
            if raw_line.starts_with("# ") && !current_page_lines.is_empty() {
                pages.push(current_page_lines);
                current_page_lines = Vec::new();
            } else if raw_line.starts_with("## ") && !current_page_lines.is_empty() {
                pages.push(current_page_lines);
                current_page_lines = Vec::new();
            }
            
            // Scale affects wrap: H1 (Scale 3) wraps much sooner than Body (Scale 1)
            let scale = if raw_line.starts_with("# ") { 3 }
            else if raw_line.starts_with("## ") { 2 }
            else { 1 };

            let wrap_limit = (window_width - 60) / (16 * scale);

            if raw_line.len() > wrap_limit && !raw_line.starts_with("```") {
                let mut remaining = raw_line;
                while remaining.len() > wrap_limit {
                    let (part, rest) = remaining.split_at(wrap_limit);
                    current_page_lines.push(part.to_string());
                    remaining = rest;
                }
                current_page_lines.push(remaining.to_string());
            } else {
                current_page_lines.push(raw_line.to_string());
            }
        }

        if !current_page_lines.is_empty() { pages.push(current_page_lines); }

        Self {
            pages,
            current_page: 0,
            scroll_y: 0,
            width: window_width,
        }
    }
}

impl AppInfo for InstructionManualApp {
    fn name(&self) -> &str { "Instruction Manual" }
    fn version(&self) -> &str { "0.3.0" }
    fn icon(&self) -> [u32; 1024] { icons::MANUAL_BOOK_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (600, 500) }
}

impl Runnable for InstructionManualApp {
    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        let (win_w, win_h) = self.dimensions();
        graphics.fill_rect(x, y, win_w, win_h, 0x0F0F0F); // Deep black background

        if self.pages.is_empty() { return; }

        let page = &self.pages[self.current_page];
        // Y coordinate where we start drawing, relative to scroll
        let mut current_draw_y: isize = (y + 20) as isize - (self.scroll_y as isize);
        let mut in_code_block = false;

        for line in page {
            // Code Block Handling
            if line.contains("```") {
                in_code_block = !in_code_block;
                // Only draw a separator if the line is within view
                if current_draw_y > (y as isize) && current_draw_y < (y + win_h) as isize {
                    graphics.fill_rect(x + 20, current_draw_y as usize, win_w - 40, 1, 0x444444);
                }
                current_draw_y += 6;
                continue;
            }

            let mut line_height = 22;
            let draw_x = x + 30;

            // Vertical Culling (Above and Below detection)
            let is_visible = current_draw_y + 20 > (y as isize) && current_draw_y < (y + (win_h - 10)) as isize;

            if is_visible {
                if in_code_block {
                    graphics.fill_rect(x + 25, current_draw_y as usize, win_w - 50, 20, 0x000000);
                    graphics.draw_text_adv(draw_x, current_draw_y as usize + 2, line, 0x00FF00, 1);
                } else if line.starts_with("> ") {
                    graphics.fill_rect(x + 25, current_draw_y as usize, 4, 20, 0x55AAFF);
                    graphics.draw_text_adv(draw_x + 10, current_draw_y as usize + 2, &line[2..], 0xCCCCCC, 1);
                } else if line.starts_with("# ") {
                    line_height = 45;
                    graphics.draw_text_adv(draw_x, current_draw_y as usize, &line[2..], 0xFF9955, 3);
                } else if line.starts_with("## ") {
                    line_height = 35;
                    graphics.draw_text_adv(draw_x, current_draw_y as usize, &line[3..], 0x55AAFF, 2);
                } else {
                    if line.contains('`') {
                        if !in_code_block {
                            graphics.draw_text_bg(draw_x, current_draw_y as usize, line, 0xFFFFFF, 0x252525);
                        }
                    } else {
                        graphics.draw_text_adv(draw_x, current_draw_y as usize, line, 0xFFFFFF, 1);
                    }
                }
            } else {
                // If not visible, we still need to calculate line height for headers
                if line.starts_with("# ") { line_height = 45; }
                else if line.starts_with("## ") { line_height = 35; }
            }

            current_draw_y += line_height as isize;

            // Stop the loop entirely if we've passed the bottom of the window
            if current_draw_y > (y + win_h) as isize { break; }
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {}

    fn input(&mut self, key: Key) {
        match key {
            Key::Special(uefi::proto::console::text::ScanCode::UP) => self.scroll_y = self.scroll_y.saturating_sub(22),
            Key::Special(uefi::proto::console::text::ScanCode::DOWN) => self.scroll_y += 22,
            Key::Special(uefi::proto::console::text::ScanCode::LEFT) => {
                if self.current_page > 0 { self.current_page -= 1; self.scroll_y = 0; }
            }
            Key::Special(uefi::proto::console::text::ScanCode::RIGHT) => {
                if self.current_page < self.pages.len() - 1 { self.current_page += 1; self.scroll_y = 0; }
            }
            _ => {}
        }
    }
}