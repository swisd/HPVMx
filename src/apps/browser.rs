use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
use crate::ui::pixel_graphics::{icons, PixelGraphics};
use crate::devices::net_stack;
use crate::devices::net;
use alloc::format;

pub struct BrowserApp {
    url: String,
    content: Vec<String>,
    loading: bool,
    scroll_offset: usize,
    status: String,
}

impl BrowserApp {
    pub fn new() -> Self {
        Self {
            url: String::from("file:///README.md"),
            content: Vec::new(),
            loading: false,
            scroll_offset: 0,
            status: String::from("Ready"),
        }
    }

    fn fetch(&mut self) {
        self.loading = true;
        self.content.clear();
        self.scroll_offset = 0;

        if self.url.starts_with("file://") {
            self.fetch_file();
        } else {
            self.fetch_http();
        }

        self.loading = false;
    }

    fn fetch_file(&mut self) {
        let path = if self.url.starts_with("file:///") {
            self.url[8..].to_string()
        } else if self.url.starts_with("file://") {
            self.url[7..].to_string()
        } else {
            self.url.clone()
        };

        match crate::FileSystem::read_file_to_string(&path) {
            Ok(data) => {
                self.parse_and_display(&data);
                self.status = format!("Loaded file: {}", path);
            }
            Err(e) => {
                self.content.push(format!("Error reading file: {}", e));
                self.status = String::from("File error");
            }
        }
    }

    fn fetch_http(&mut self) {
        crate::devices::net::ensure_net();
        let clean_url = if self.url.starts_with("http://") {
            self.url[7..].to_string()
        } else {
            self.url.clone()
        };

        let (ip, port) = match net::parse_endpoint(&clean_url) {
            Some(res) => res,
            None => {
                self.content.push(String::from("Invalid address format. Use IP or IP:PORT"));
                return;
            }
        };

        self.status = format!("Connecting to {}...", clean_url);

        if net_stack::tcp_connect(ip, port) {
            let request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\nAccept: text/html\r\n\r\n",
                clean_url
            );
            net_stack::tcp_send(ip, port, request.as_bytes());

            self.status = String::from("Waiting for response...");
            
            let mut full_response = Vec::new();
            let mut received_anything = false;

            // Wait and poll for some data
            for _ in 0..500 {
                net_stack::poll_tick();
                let state = net_stack::get_state();
                if !state.tcp_rx_data.is_empty() {
                    full_response.extend_from_slice(&state.tcp_rx_data);
                    // Clear the buffer in NetState so we don't read it twice
                    unsafe {
                        let state_mut = crate::devices::net_stack::get_state_mut();
                        state_mut.tcp_rx_data.clear();
                    }
                    received_anything = true;
                }
                if state.tcp_fin_received || !state.tcp_connected {
                    if received_anything { break; }
                }
                uefi::boot::stall(core::time::Duration::from_micros(10_000));
            }

            if !full_response.is_empty() {
                let rx_str = String::from_utf8_lossy(&full_response);
                // Simple HTTP header stripping
                if let Some(body_start) = rx_str.find("\r\n\r\n") {
                    self.parse_and_display(&rx_str[body_start + 4..]);
                } else {
                    self.parse_and_display(&rx_str);
                }
                self.status = format!("Loaded from {}", clean_url);
            } else {
                self.content.push(String::from("No data received or connection closed."));
                self.status = String::from("Connection closed");
            }
        } else {
            self.content.push(String::from("Failed to connect."));
            self.status = String::from("Connection failed");
        }
    }

    fn parse_and_display(&mut self, html: &str) {
        self.content.clear();
        let mut stripped = String::new();
        let mut in_tag = false;
        
        // Very basic HTML tag stripper
        for c in html.chars() {
            if c == '<' {
                in_tag = true;
            } else if c == '>' {
                in_tag = false;
            } else if !in_tag {
                stripped.push(c);
            }
        }

        // Split into lines and wrap text simply
        for line in stripped.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                self.content.push(trimmed.to_string());
            }
        }
        
        if self.content.is_empty() && !html.is_empty() {
             self.content.push(String::from("Empty content after parsing."));
        }
    }
}

impl AppInfo for BrowserApp {
    fn name(&self) -> &str {
        "WebBrowser"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (600, 400)
    }
}

impl RunnableClone for BrowserApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        Box::new(BrowserApp::new())
    }
}

impl Runnable for BrowserApp {
    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics.fill_rect(x, y, 600, 400, 0x1A1A1A);
        graphics.draw_rect_outline(x, y, 600, 400, 0x444444);

        // Address bar
        graphics.fill_rect(x + 5, y + 5, 590, 30, 0x333333);
        graphics.draw_text(x + 10, y + 12, "URL: ", 0xAAAAAA);
        graphics.draw_text(x + 50, y + 12, &self.url, 0xFFFFFF);

        if !self.loading {
             // Simple cursor in address bar
             let cursor_x = x + 50 + (self.url.len() * 8);
             if cursor_x < x + 590 {
                graphics.fill_rect(cursor_x, y + 12, 2, 16, 0x00FF00);
             }
        }

        // Content area
        graphics.fill_rect(x + 5, y + 40, 590, 340, 0x000000);
        
        let mut curr_y = y + 45;
        let visible_lines = 320 / 16;
        for i in self.scroll_offset..(self.scroll_offset + visible_lines).min(self.content.len()) {
            let line = &self.content[i];
            graphics.draw_text(x + 10, curr_y, line, 0x00FF00);
            curr_y += 16;
        }

        // Status bar
        graphics.fill_rect(x + 5, y + 385, 590, 10, 0x222222);
        graphics.draw_text(x + 10, y + 385, &self.status, 0x888888);

        if self.loading {
            graphics.draw_text(x + 250, y + 200, "LOADING...", 0xFFFF00);
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>, env: &mut Environment) {
    }

    fn input(&mut self, key: Key) {
        if self.loading { return; }

        match key {
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch == '\r' || ch == '\n' {
                    self.fetch();
                } else if ch == '\u{8}' { // Backspace
                    self.url.pop();
                } else {
                    self.url.push(ch);
                }
            }
            Key::Special(s) => match s {
                uefi::proto::console::text::ScanCode::UP => {
                    self.scroll_offset = self.scroll_offset.saturating_sub(1);
                }
                uefi::proto::console::text::ScanCode::DOWN => {
                    if !self.content.is_empty() {
                        let visible_lines = 320 / 16;
                        if self.scroll_offset + visible_lines < self.content.len() {
                            self.scroll_offset += 1;
                        }
                    }
                }
                _ => {}
            }
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}
