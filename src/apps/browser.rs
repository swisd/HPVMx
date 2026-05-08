use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};
use crate::devices::net_stack;
use crate::devices::net;
use alloc::format;

pub struct BrowserApp {
    url: String,
    content: Vec<String>,
    loading: bool,
    cursor_pos: usize,
}

impl BrowserApp {
    pub fn new() -> Self {
        Self {
            url: String::from("1.1.1.1"),
            content: Vec::new(),
            loading: false,
            cursor_pos: 7,
        }
    }

    fn fetch(&mut self) {
        self.loading = true;
        self.content.clear();
        self.content.push(format!("Connecting to {}...", self.url));

        let (ip, port) = match net::parse_endpoint(&self.url) {
            Some(res) => res,
            None => {
                self.content.push(String::from("Invalid address format. Use IP or IP:PORT"));
                self.loading = false;
                return;
            }
        };

        if net_stack::tcp_connect(ip, port) {
            self.content.push(String::from("Connected. Sending GET request..."));
            let request = format!(
                "GET / HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                self.url
            );
            net_stack::tcp_send(ip, port, request.as_bytes());

            self.content.push(String::from("Waiting for response..."));
            // Wait and poll for some data
            for _ in 0..2000 {
                net_stack::poll_tick();
                let state = net_stack::get_state();
                if !state.tcp_rx_data.is_empty() {
                    let rx_str = String::from_utf8_lossy(&state.tcp_rx_data);
                    self.content.clear();
                    for line in rx_str.lines() {
                        self.content.push(String::from(line));
                    }
                }
                if state.tcp_fin_received || !state.tcp_connected {
                    break;
                }
                uefi::boot::stall(core::time::Duration::from_micros(10_000));
            }
            if self.content.len() <= 3 {
                 self.content.push(String::from("No data received or connection closed."));
            }
        } else {
            self.content.push(String::from("Failed to connect."));
        }

        self.loading = false;
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
             graphics.fill_rect(cursor_x, y + 12, 2, 16, 0x00FF00);
        }

        // Content area
        graphics.fill_rect(x + 5, y + 40, 590, 355, 0x000000);
        
        let mut curr_y = y + 45;
        for (i, line) in self.content.iter().enumerate() {
            if curr_y + 16 > y + 395 { break; }
            graphics.draw_text(x + 10, curr_y, line, 0x00FF00);
            curr_y += 16;
        }

        if self.loading {
            graphics.draw_text(x + 250, y + 200, "LOADING...", 0xFFFF00);
        }
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {
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
            Key::Special(s) => {
                // Handle special keys if needed
            }
            _ => {}
        }
    }
}
