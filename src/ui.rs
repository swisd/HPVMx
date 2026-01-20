#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::proto::console::text::{Color, Key, ScanCode};
use uefi::system;

mod graphics;
pub use graphics::{Graphics, Rect, WinNTColors};

pub struct Window {
    pub title: String,
    pub rect: Rect,
    pub active: bool,
}

impl Window {
    pub fn new(title: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Window {
            title: String::from(title),
            rect: Rect::new(x, y, width, height),
            active: true,
        }
    }

    pub fn draw(&self) {
        Graphics::draw_box(&self.rect, &self.title, self.active);
    }
}

pub struct Button {
    pub label: String,
    pub rect: Rect,
    pub focused: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(label: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Button {
            label: String::from(label),
            rect: Rect::new(x, y, width, height),
            focused: false,
            clicked: false,
        }
    }

    pub fn draw(&self) {
        Graphics::draw_button(&self.rect, &self.label, self.focused);
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.rect.contains(x, y)
    }
}

pub struct TextBox {
    pub text: String,
    pub rect: Rect,
    pub focused: bool,
    pub max_length: usize,
}

impl TextBox {
    pub fn new(x: usize, y: usize, width: usize, max_length: usize) -> Self {
        TextBox {
            text: String::new(),
            rect: Rect::new(x, y, width, 3),
            focused: false,
            max_length,
        }
    }

    pub fn draw(&self) {
        Graphics::draw_textbox(&self.rect, &self.text, self.focused);
    }

    pub fn add_char(&mut self, ch: char) {
        if self.text.len() < self.max_length {
            self.text.push(ch);
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();
    }
}

pub struct ListBox {
    pub items: Vec<String>,
    pub rect: Rect,
    pub selected: usize,
}

impl ListBox {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        ListBox {
            items: Vec::new(),
            rect: Rect::new(x, y, width, height),
            selected: 0,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn draw(&self) {
        let items_str: Vec<&str> = self.items.iter().map(|s| s.as_str()).collect();
        Graphics::draw_list(&self.rect, &items_str, self.selected);
    }

    pub fn select_next(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }
}

pub struct WinNTShell {
    pub windows: Vec<Window>,
    pub buttons: Vec<Button>,
    pub textboxes: Vec<TextBox>,
    pub listbox: Option<ListBox>,
    pub focused_button: usize,
    pub focused_textbox: usize,
}

impl WinNTShell {
    pub fn new() -> Self {
        WinNTShell {
            windows: Vec::new(),
            buttons: Vec::new(),
            textboxes: Vec::new(),
            listbox: None,
            focused_button: 0,
            focused_textbox: 0,
        }
    }

    pub fn init_desktop(&mut self) {
        // Clear screen with gray background
        Graphics::clear_screen(Color::LightGray);

        // Draw menu bar
        Graphics::draw_menu_bar(&["File", "Edit", "View", "Help"]);

        // Draw taskbar
        Graphics::draw_taskbar("12:00");

        // Create main shell window
        let shell_window = Window::new("HPVMx Shell", 5, 3, 70, 18);
        self.windows.push(shell_window);

        // Create OK and Cancel buttons
        let ok_button = Button::new("OK", 35, 20, 8, 2);
        let cancel_button = Button::new("Cancel", 45, 20, 10, 2);

        self.buttons.push(ok_button);
        self.buttons.push(cancel_button);

        // Create a command input textbox
        let input_box = TextBox::new(7, 5, 66, 50);
        self.textboxes.push(input_box);

        // Create output listbox
        let mut output_list = ListBox::new(7, 8, 66, 10);
        output_list.add_item("HPVMx v0.1.0 - Shell Interface");
        output_list.add_item("Type 'help' for available commands");
        self.listbox = Some(output_list);
    }

    pub fn draw(&self) {
        for window in &self.windows {
            window.draw();
        }

        for button in &self.buttons {
            button.draw();
        }

        for textbox in &self.textboxes {
            textbox.draw();
        }

        if let Some(ref listbox) = self.listbox {
            listbox.draw();
        }
    }

    pub fn focus_button(&mut self, idx: usize) {
        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.focused = i == idx;
        }
        self.focused_button = idx;
    }

    pub fn focus_textbox(&mut self, idx: usize) {
        for (i, textbox) in self.textboxes.iter_mut().enumerate() {
            textbox.focused = i == idx;
        }
        self.focused_textbox = idx;
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                if self.focused_textbox < self.textboxes.len() {
                    let ch = char::from(c);
                    if ch != '\r' && ch != '\n' {
                        self.textboxes[self.focused_textbox].add_char(ch);
                    }
                }
            }
            Key::Special(ScanCode::DELETE) => {
                if self.focused_textbox < self.textboxes.len() {
                    self.textboxes[self.focused_textbox].backspace();
                }
            }
            Key::Special(ScanCode::UP) => {
                if let Some(ref mut listbox) = self.listbox {
                    listbox.select_prev();
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if let Some(ref mut listbox) = self.listbox {
                    listbox.select_next();
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                let next_button = (self.focused_button + 1) % self.buttons.len();
                self.focus_button(next_button);
            }
            _ => {}
        }
    }

    pub fn get_input(&self) -> Option<String> {
        if self.textboxes.len() > 0 {
            Some(self.textboxes[0].text.clone())
        } else {
            None
        }
    }

    pub fn clear_input(&mut self) {
        if self.textboxes.len() > 0 {
            self.textboxes[0].text.clear();
        }
    }

    pub fn add_output(&mut self, text: &str) {
        if let Some(ref mut listbox) = self.listbox {
            listbox.add_item(text);
        }
    }
    
    pub  fn process_mouse(){
        Graphics::
    }
}