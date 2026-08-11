use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_CreateVM {
    pub new_vm_name: String,
    pub new_vm_memory_mb: u32,
    pub new_vm_vcpus: u32,
    pub create_vm_focus_idx: usize,
}

impl X_CreateVM {
    pub fn new() -> Self {
        Self {
            new_vm_name: "new-vm".to_string(),
            new_vm_memory_mb: 512,
            new_vm_vcpus: 1,
            create_vm_focus_idx: 0,
        }
    }
}

impl Runnable for X_CreateVM {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Create New Virtual Machine", 0x00FF00);
        
        let mut curr_y = y + 60;
        pg.draw_text(x + 20, curr_y, &format!("Name: {}", self.new_vm_name), if self.create_vm_focus_idx == 0 { 0xFFFF00 } else { 0xFFFFFF });
        curr_y += 30;
        pg.draw_text(x + 20, curr_y, &format!("vCPUs: {}", self.new_vm_vcpus), if self.create_vm_focus_idx == 1 { 0xFFFF00 } else { 0xFFFFFF });
        curr_y += 30;
        pg.draw_text(x + 20, curr_y, &format!("Memory: {} MB", self.new_vm_memory_mb), if self.create_vm_focus_idx == 2 { 0xFFFF00 } else { 0xFFFFFF });
        curr_y += 50;

        let create_color = if self.create_vm_focus_idx == 3 { 0x00FF00 } else { 0x008000 };
        pg.fill_rect(x + 20, curr_y, 120, 30, create_color);
        pg.draw_text(x + 30, curr_y + 8, "CREATE", 0xFFFFFF);

        let cancel_color = if self.create_vm_focus_idx == 4 { 0xFF5555 } else { 0x880000 };
        pg.fill_rect(x + 160, curr_y, 120, 30, cancel_color);
        pg.draw_text(x + 170, curr_y + 8, "CANCEL", 0xFFFFFF);

        pg.draw_text(x + 20, curr_y + 50, "TAB to switch fields, ENTER to create, ESC to cancel", 0x888888);
    }

    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {
        // Form values and focus belong to this instance until submission.
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c).to_ascii_lowercase();
                match ch {
                    ' ' => {
                        if self.create_vm_focus_idx == 0 {
                            self.new_vm_name.push(' ');
                        }
                    }
                    '+' | '=' => {
                        if self.create_vm_focus_idx == 1 {
                            self.new_vm_vcpus += 1;
                        } else if self.create_vm_focus_idx == 2 {
                            self.new_vm_memory_mb += 128;
                        }
                    }
                    '-' | '_' => {
                        if self.create_vm_focus_idx == 1 {
                            self.new_vm_vcpus = self.new_vm_vcpus.saturating_sub(1).max(1);
                        } else if self.create_vm_focus_idx == 2 {
                            self.new_vm_memory_mb = self.new_vm_memory_mb.saturating_sub(128).max(128);
                        }
                    }
                    '\u{08}' => { // Backspace
                        if self.create_vm_focus_idx == 0 {
                            self.new_vm_name.pop();
                        }
                    }
                    '\t' => {
                        self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5;
                    }
                    '\r' | '\n' => {
                        if self.create_vm_focus_idx == 3 {
                            unsafe {
                                if let Some(hv) = crate::HYPERVISOR.as_mut() {
                                    let _ = hv.create_vm(&self.new_vm_name, self.new_vm_memory_mb, self.new_vm_vcpus);
                                }
                            }
                        } else if self.create_vm_focus_idx == 4 {
                            self.create_vm_focus_idx = 0;
                        } else {
                            self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5;
                        }
                    }
                    _ => {
                        if self.create_vm_focus_idx == 0 && (ch.is_alphanumeric() || ch == '_') {
                            self.new_vm_name.push(ch);
                        }
                    }
                }
            }
            Key::Special(ScanCode::ESCAPE) => {
                // Cancel
            }
            _ => {}
        }
    }
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_CreateVM {
    fn name(&self) -> &str { "Create VM" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
