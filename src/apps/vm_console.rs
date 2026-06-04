use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use uefi::proto::console::text::Key;

use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct VmConsoleApp {
    pub vm_id: u32,
    pub media_path: String,
    pub media_kind: String,
    frame: u32,
}

impl VmConsoleApp {
    pub fn new(vm_id: u32, media_path: &str, media_kind: &str) -> Self {
        Self {
            vm_id,
            media_path: media_path.to_string(),
            media_kind: media_kind.to_string(),
            frame: 0,
        }
    }
}

impl AppInfo for VmConsoleApp {
    fn name(&self) -> &str {
        "VM Console"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (520, 320)
    }
}

impl Runnable for VmConsoleApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics_entity.fill_rect(x, y, 516, 296, 0x050505);
        graphics_entity.draw_rect_outline(x, y, 516, 296, 0x00AAAA);
        graphics_entity.draw_text(x + 12, y + 12, &format!("VM {} running {}", self.vm_id, self.media_kind), 0xFFFFFF);
        graphics_entity.draw_text(x + 12, y + 34, &self.media_path, 0xAAAAAA);
        graphics_entity.draw_text(x + 12, y + 64, "HPVMx virtual display", 0x00FFAA);
        graphics_entity.draw_text(x + 12, y + 88, "Boot channel: VMBUS -> VM Unit -> HWBUS", 0xAAAAAA);
        graphics_entity.draw_text(x + 12, y + 112, "DLS: instruction, call, IO, storage, and hardware events recorded", 0xAAAAAA);

        let pulse = if (self.frame / 20) % 2 == 0 { 0x00FF00 } else { 0x007700 };
        graphics_entity.fill_rect(x + 12, y + 148, 10, 10, pulse);
        graphics_entity.draw_text(x + 30, y + 144, "guest execution heartbeat", 0xCCCCCC);

        graphics_entity.draw_text(x + 12, y + 220, "ESC closes this console window", 0x888888);
    }

    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {
        self.frame = self.frame.wrapping_add(1);
    }

    fn input(&mut self, _key: Key) {}
}
