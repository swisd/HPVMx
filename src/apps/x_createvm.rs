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
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {
        match key {
            Key::Printable(k) => {
                match k.to_string().as_str() {
                    "\t" => self.create_vm_focus_idx = (self.create_vm_focus_idx + 1) % 5,
                    _ => {}
                }

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
