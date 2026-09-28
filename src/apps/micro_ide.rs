use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::any::Any;
use uefi::proto::console::text::{Key, ScanCode};

use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
use crate::filesystem::FileSystem;
use crate::micro_c::compiler;
use crate::ui::pixel_graphics::{icons, PixelGraphics};

#[derive(Clone, Copy, PartialEq, Eq)]
enum SourceLanguage { MicroC, C }

pub struct MicroIdeApp {
    source: String,
    output: String,
    source_path: String,
    cursor: usize,
    scroll: usize,
    target_idx: usize,
    language: SourceLanguage,
    editing_path: bool,
    status: String,
}

impl MicroIdeApp {
    pub fn new() -> Self {
        Self {
            source: String::from("export fn hpx_step(host, state) {\n    let frames = 42;\n    return frames;\n}\n"),
            output: String::from("F5 compile  F2 save  F3 load  F8 language"),
            source_path: String::from("\\untitled.micro"),
            cursor: 0,
            scroll: 0,
            target_idx: 0,
            language: SourceLanguage::MicroC,
            editing_path: false,
            status: String::from("Ready"),
        }
    }

    fn language_name(&self) -> &'static str {
        match self.language { SourceLanguage::MicroC => "Micro-C", SourceLanguage::C => "C" }
    }

    fn target(&self) -> &'static str {
        match self.target_idx {
            0 => "x86_64",
            1 => "win64",
            2 => "arm64",
            3 => "bytecode64",
            _ => "x86_64",
        }
    }

    fn output_path(&self) -> String {
        let separator = self.source_path.rfind(|c| c == '/' || c == '\\').map(|i| i + 1).unwrap_or(0);
        let stem_end = self.source_path.rfind('.').filter(|i| *i >= separator).unwrap_or(self.source_path.len());
        let extension = match self.language { SourceLanguage::MicroC => "asm", SourceLanguage::C => "o" };
        format!("{}.{}", &self.source_path[..stem_end], extension)
    }

    fn save(&mut self) {
        let _ = FileSystem::remove(&self.source_path);
        match FileSystem::write_to_file(&self.source_path, &self.source, 'w') {
            Ok(()) => self.status = format!("Saved {}", self.source_path),
            Err(error) => self.status = format!("Save failed: {}", error),
        }
    }

    fn load(&mut self) {
        match FileSystem::read_file_to_string(&self.source_path) {
            Ok(source) => {
                self.source = source;
                self.cursor = 0;
                self.scroll = 0;
                self.status = format!("Loaded {}", self.source_path);
            }
            Err(error) => self.status = format!("Load failed: {}", error),
        }
    }

    fn compile(&mut self) {
        let output_path = self.output_path();
        match self.language {
            SourceLanguage::MicroC => {
                let asm = compiler::compile(&self.source, self.target());
                if asm.is_empty() {
                    self.output = String::from("Micro-C compiler returned no assembly. Check the source and target.");
                    self.status = String::from("Compile failed");
                    return;
                }
                let _ = FileSystem::remove(&output_path);
                match FileSystem::write_to_file(&output_path, &asm, 'w') {
                    Ok(()) => {
                        self.output = asm;
                        self.status = format!("Compiled {} -> {}", self.target(), output_path);
                    }
                    Err(error) => {
                        self.output = format!("Compilation succeeded, but saving {} failed: {}", output_path, error);
                        self.status = String::from("Output write failed");
                    }
                }
            }
            SourceLanguage::C => {
                let result = (|| -> Result<(String, Vec<u8>), String> {
                    let asm = crate::tools::c_compiler::compile_to_assembly(&self.source)?;
                    let object = crate::tools::c_object::encode(&asm)?;
                    Ok((asm, object))
                })();
                match result {
                    Ok((asm, object)) => {
                        let _ = FileSystem::remove(&output_path);
                        match FileSystem::write_to_file_bytes(&output_path, &object, 'w') {
                            Ok(()) => {
                                self.output = format!("HPVMx HXO object: {} bytes\nOutput: {}\n\n{}", object.len(), output_path, asm);
                                self.status = format!("C compiled -> {}", output_path);
                            }
                            Err(error) => {
                                self.output = format!("C compilation succeeded, but writing {} failed: {}", output_path, error);
                                self.status = String::from("Object write failed");
                            }
                        }
                    }
                    Err(error) => {
                        self.output = format!("C compile failed: {}", error);
                        self.status = String::from("Compile failed");
                    }
                }
            }
        }
    }

    fn insert_char(&mut self, ch: char) {
        if self.editing_path {
            self.source_path.push(ch);
            return;
        }
        self.cursor = self.cursor.min(self.source.len());
        self.source.insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
    }

    fn backspace(&mut self) {
        if self.editing_path {
            self.source_path.pop();
            return;
        }
        if self.cursor == 0 { return; }
        let mut prev = self.cursor - 1;
        while !self.source.is_char_boundary(prev) { prev -= 1; }
        self.source.remove(prev);
        self.cursor = prev;
    }

    fn move_horizontal(&mut self, right: bool) {
        if self.editing_path { return; }
        if right {
            if self.cursor < self.source.len() {
                self.cursor += self.source[self.cursor..].chars().next().map(char::len_utf8).unwrap_or(0);
            }
        } else if self.cursor > 0 {
            let mut prev = self.cursor - 1;
            while !self.source.is_char_boundary(prev) { prev -= 1; }
            self.cursor = prev;
        }
    }

    fn move_vertical(&mut self, down: bool) {
        if self.editing_path { return; }
        let cursor = self.cursor.min(self.source.len());
        let line_start = self.source[..cursor].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = self.source[line_start..cursor].chars().count();
        if down {
            if let Some(relative_end) = self.source[cursor..].find('\n') {
                let next_start = cursor + relative_end + 1;
                let next_end = self.source[next_start..].find('\n').map(|n| next_start + n).unwrap_or(self.source.len());
                self.cursor = self.source[next_start..next_end].char_indices().nth(column).map(|(i,_)|next_start+i).unwrap_or(next_end);
            }
        } else if line_start > 0 {
            let prev_end = line_start - 1;
            let prev_start = self.source[..prev_end].rfind('\n').map(|i| i + 1).unwrap_or(0);
            self.cursor = self.source[prev_start..prev_end].char_indices().nth(column).map(|(i,_)|prev_start+i).unwrap_or(prev_end);
        }
    }

    fn switch_language(&mut self) {
        self.language = match self.language { SourceLanguage::MicroC => SourceLanguage::C, SourceLanguage::C => SourceLanguage::MicroC };
        let extension = match self.language { SourceLanguage::MicroC => ".micro", SourceLanguage::C => ".c" };
        let separator = self.source_path.rfind(|c| c == '/' || c == '\\').map(|i| i + 1).unwrap_or(0);
        let stem_end = self.source_path.rfind('.').filter(|i| *i >= separator).unwrap_or(self.source_path.len());
        self.source_path.truncate(stem_end);
        self.source_path.push_str(extension);
        self.source = match self.language {
            SourceLanguage::MicroC => String::from("export fn hpx_step(host, state) {\n    return 1;\n}\n"),
            SourceLanguage::C => String::from("#include <HPVMx>\nuint32_t hpx_step(const void *host, void *state) {\n    return 1;\n}\n"),
        };
        self.cursor = 0;
        self.scroll = 0;
        self.status = format!("Language: {}", self.language_name());
    }
}

impl AppInfo for MicroIdeApp {
    fn name(&self) -> &str { "Micro-C / C IDE" }
    fn version(&self) -> &str { "0.2.0" }
    fn icon(&self) -> [u32; 1024] { icons::SCRIPT_YELLOW_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (900, 560) }
}

impl RunnableClone for MicroIdeApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        Box::new(MicroIdeApp::new())
    }
}

impl Runnable for MicroIdeApp {
    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        let (w, h) = self.dimensions();
        graphics.fill_rect(x, y, w, h, 0x151515);
        graphics.draw_text(x + 12, y + 10, "HPVMx IDE - Micro-C and C", 0x00FF00);
        graphics.draw_text(x + 12, y + 28, &format!("{} | {} | F2 Save F3 Load F4 Path F5 Compile F6 Target F8 Language", self.language_name(), self.target()), 0xAAAAAA);
        graphics.draw_text(x + w - 180, y + 28, &self.status, 0xFFFF00);
        graphics.draw_text(x + 12, y + 46, &format!("File: {}{}", self.source_path, if self.editing_path { "  [editing]" } else { "" }), 0x55AAFF);

        let source_x = x + 12;
        let source_y = y + 70;
        let source_w = 420;
        let pane_h = h - 106;
        graphics.draw_rect_outline(source_x, source_y, source_w, pane_h, 0x666666);
        graphics.draw_text(source_x + 8, source_y + 8, "source", 0x55AAFF);

        let cursor_line = self.source[..self.cursor.min(self.source.len())].bytes().filter(|b| *b == b'\n').count();
        let line_count = (pane_h.saturating_sub(40)) / 16;
        let first_line = cursor_line.saturating_sub(line_count.saturating_sub(1));
        let mut row_y = source_y + 30;
        for (idx, line) in self.source.lines().enumerate().skip(first_line) {
            if row_y + 16 > source_y + pane_h - 8 { break; }
            let marker = if idx == cursor_line && !self.editing_path { ">" } else { " " };
            graphics.draw_text(source_x + 8, row_y, &format!("{}{:>3}", marker, idx + 1), 0x666666);
            graphics.draw_text(source_x + 42, row_y, line, 0xFFFFFF);
            row_y += 16;
        }

        let output_x = source_x + source_w + 16;
        let output_w = w - source_w - 40;
        graphics.draw_rect_outline(output_x, source_y, output_w, pane_h, 0x666666);
        graphics.draw_text(output_x + 8, source_y + 8, "compiler output / diagnostics", 0x55AAFF);
        let mut out_y = source_y + 30;
        for line in self.output.lines().take((pane_h - 40) / 16) {
            graphics.draw_text(output_x + 8, out_y, line, 0xCCCCCC);
            out_y += 16;
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {}

    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::FUNCTION_2) => self.save(),
            Key::Special(ScanCode::FUNCTION_3) => self.load(),
            Key::Special(ScanCode::FUNCTION_4) => {
                self.editing_path = !self.editing_path;
                self.status = if self.editing_path { String::from("Type a path, then press F4") } else { String::from("Path edit finished") };
            }
            Key::Special(ScanCode::FUNCTION_5) => self.compile(),
            Key::Special(ScanCode::FUNCTION_6) => {
                self.target_idx = (self.target_idx + 1) % 4;
                self.status = format!("Micro-C target: {}", self.target());
            }
            Key::Special(ScanCode::FUNCTION_7) => {
                self.output.clear();
                self.status = String::from("Output cleared");
            }
            Key::Special(ScanCode::FUNCTION_8) => self.switch_language(),
            Key::Special(ScanCode::LEFT) => self.move_horizontal(false),
            Key::Special(ScanCode::RIGHT) => self.move_horizontal(true),
            Key::Special(ScanCode::UP) => self.move_vertical(false),
            Key::Special(ScanCode::DOWN) => self.move_vertical(true),
            Key::Special(ScanCode::DELETE) => self.backspace(),
            Key::Printable(c) => match char::from(c) {
                '\r' | '\n' if !self.editing_path => self.insert_char('\n'),
                '\u{8}' => self.backspace(),
                '\r' | '\n' => {},
                ch => self.insert_char(ch),
            },
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

