use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::{pixel_graphics, UiSettings};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_Settings {
    pub settings: UiSettings,
    pub selected_settings_idx: usize,
}

impl X_Settings {
    pub fn new() -> Self {
        Self {
            settings: UiSettings {
                extra_debug_info: false,
                folder_absolute_sizes: false,
                state_save_restore: false,
                extended_symbol_library: false,
                ring0_udmi_udxi: false,
                controllang_support: false,
                pg_vshaders: false,
                experimental_mem_comp: false,
                auto_refresh_storage: false,
                show_hidden_files: false,
                general_profile: 0,
                boot_target: 0,
                interface_density: 0,
                vm_safety_policy: 0,
                network_profile: 0,
                storage_policy: 0,
                package_policy: 0,
                developer_level: 0,
                security_policy: 0,
                ui_scaling: 0,
                terminal_font: 0,
                pg_scanlines: false,
                pg_dither: false,
                pg_glitch: false,
                pg_aberration: 0,
            },
            selected_settings_idx: 0,
        }
    }
}

impl Runnable for X_Settings {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "System Settings", 0x00FF00);
        pg.draw_text(x + 20, y + 50, &format!("Selected: {}", self.selected_settings_idx), 0xFFFFFF);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {}
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Settings {
    fn name(&self) -> &str { "Settings" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::GEAR_YB_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
