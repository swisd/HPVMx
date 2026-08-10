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
        
        let margin = 16usize;
        let gutter = 12usize;
        let line_h = 18usize;

        let list_x = x + margin;
        let list_y = y + 50;
        let list_w = 240;
        let list_h = 420;

        // Categories
        pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);
        let categories = ["General", "Boot", "Interface", "VM Policy", "Network", "Storage", "Packages", "Security", "Developer", "Experimental"];
        
        let mut curr_y = list_y + 10;
        for (idx, cat) in categories.iter().enumerate() {
            let color = if idx == 0 { 0xFFFF00 } else { 0xFFFFFF };
            pg.draw_text(list_x + 10, curr_y, cat, color);
            curr_y += line_h + 5;
        }

        let detail_x = list_x + list_w + gutter;
        let detail_w = 500;
        pg.draw_rect_outline(detail_x, list_y, detail_w, list_h, 0x888888);
        
        pg.draw_text(detail_x + 10, list_y + 10, "Settings for General", 0x00FFFF);
        
        let rows = [
            ("HPVMX_PROFILE", "balanced"),
            ("Extra Debug Info", if self.settings.extra_debug_info { "on" } else { "off" }),
            ("HPVMX_USER", "operator"),
            ("Experimental Mem Comp", if self.settings.experimental_mem_comp { "on" } else { "off" }),
        ];

        curr_y = list_y + 40;
        for (idx, (name, val)) in rows.iter().enumerate() {
            let color = if idx == self.selected_settings_idx { 0xFFFF00 } else { 0xFFFFFF };
            pg.draw_text(detail_x + 10, curr_y, &format!("{:<25} : {}", name, val), color);
            curr_y += line_h;
        }

        pg.draw_text(x + 20, list_y + list_h + 20, "UP/DOWN selects setting, ENTER toggles, S saves all to config.cfg", 0x888888);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::UP) => {
                if self.selected_settings_idx > 0 {
                    self.selected_settings_idx -= 1;
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.selected_settings_idx < 3 {
                    self.selected_settings_idx += 1;
                }
            }
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Toggle setting
            }
            _ => {}
        }
    }
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Settings {
    fn name(&self) -> &str { "Settings" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::GEAR_YB_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
