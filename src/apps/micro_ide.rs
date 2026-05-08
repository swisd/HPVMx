use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};

use crate::env::{AppInfo, Runnable};
use crate::micro_c::compiler;
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct MicroIdeApp {
    source: String,
    output: String,
    cursor: usize,
    scroll: usize,
    target_idx: usize,
    status: String,
}

impl MicroIdeApp {
    pub fn new() -> Self {
        Self {
            source: String::from("fn main() {\n    let x = 42;\n    return x;\n}\n"),
            output: String::from("Press F5 to compile. F6 changes target."),
            cursor: 0,
            scroll: 0,
            target_idx: 0,
            status: String::from("Ready"),
        }
    }

    fn target(&self) -> &'static str {
        match self.target_idx {
            0 => "x86_64",
            1 => "win64",
            2 => "arm64",
            _ => "x86_64",
        }
    }

    fn compile(&mut self) {
        let asm = compiler::compile(&self.source, self.target());
        if asm.is_empty() {
            self.output = String::from("Compiler returned no output. Check Micro-C syntax and target.");
            self.status = String::from("Compile failed");
        } else {
            self.output = asm;
            self.status = format!("Compiled for {}", self.target());
        }
    }

    fn insert_char(&mut self, ch: char) {
        if self.cursor > self.source.len() {
            self.cursor = self.source.len();
        }
        self.source.insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
    }

    fn backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        let mut prev = self.cursor - 1;
        while !self.source.is_char_boundary(prev) {
            prev -= 1;
        }
        self.source.remove(prev);
        self.cursor = prev;
    }
}

impl AppInfo for MicroIdeApp {
    fn name(&self) -> &str { "Micro-C IDE" }
    fn version(&self) -> &str { "0.1.0" }
    fn icon(&self) -> [u32; 1024] { icons::SCRIPT_YELLOW_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (820, 520) }
}

impl Runnable for MicroIdeApp {
    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        let (w, h) = self.dimensions();
        graphics.fill_rect(x, y, w, h, 0x151515);
        graphics.draw_text(x + 12, y + 10, "Micro-C Developer Toolchain", 0x00FF00);
        graphics.draw_text(x + 12, y + 28, &format!("Target: {}   F5 Compile   F6 Target   F7 Clear Output", self.target()), 0xAAAAAA);
        graphics.draw_text(x + w - 220, y + 28, &self.status, 0xFFFF00);

        let source_x = x + 12;
        let source_y = y + 56;
        let source_w = 390;
        let pane_h = h - 92;
        graphics.draw_rect_outline(source_x, source_y, source_w, pane_h, 0x666666);
        graphics.draw_text(source_x + 8, source_y + 8, "source", 0x55AAFF);

        let mut row_y = source_y + 30;
        for (idx, line) in self.source.lines().enumerate().skip(self.scroll) {
            if row_y + 16 > source_y + pane_h - 8 { break; }
            graphics.draw_text(source_x + 8, row_y, &format!("{:>3}", idx + 1), 0x666666);
            graphics.draw_text(source_x + 42, row_y, line, 0xFFFFFF);
            row_y += 16;
        }

        let output_x = source_x + source_w + 16;
        let output_w = w - source_w - 40;
        graphics.draw_rect_outline(output_x, source_y, output_w, pane_h, 0x666666);
        graphics.draw_text(output_x + 8, source_y + 8, "assembly / diagnostics", 0x55AAFF);

        let mut out_y = source_y + 30;
        for line in self.output.lines().take((pane_h - 40) / 16) {
            graphics.draw_text(output_x + 8, out_y, line, 0xCCCCCC);
            out_y += 16;
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {}

    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::FUNCTION_5) => self.compile(),
            Key::Special(ScanCode::FUNCTION_6) => {
                self.target_idx = (self.target_idx + 1) % 3;
                self.status = format!("Target changed to {}", self.target());
            }
            Key::Special(ScanCode::FUNCTION_7) => {
                self.output.clear();
                self.status = String::from("Output cleared");
            }
            Key::Special(ScanCode::UP) => self.scroll = self.scroll.saturating_sub(1),
            Key::Special(ScanCode::DOWN) => self.scroll += 1,
            Key::Special(ScanCode::DELETE) => self.backspace(),
            Key::Printable(c) => {
                match char::from(c) {
                    '\r' | '\n' => self.insert_char('\n'),
                    '\u{8}' => self.backspace(),
                    ch => self.insert_char(ch),
                }
            }
            _ => {}
        }
    }
}
