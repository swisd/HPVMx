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
    pub selected_settings_category_idx: usize,
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
            selected_settings_category_idx: 0,
        }
    }

    fn option_value(&self, options: &[&'static str], idx: usize) -> String {
        options.get(idx).copied().unwrap_or(options[0]).to_string()
    }

    pub fn settings_rows(&self) -> Vec<(String, String, bool, bool)> {
        match self.selected_settings_category_idx {
            0 => vec![
                (String::from("HPVMX_PROFILE"), self.option_value(&["balanced", "diagnostic", "performance"], self.settings.general_profile), false, false),
                (String::from("Extra Debug Info"), if self.settings.extra_debug_info { "on" } else { "off" }.to_string(), false, false),
                (String::from("HPVMX_USER"), String::from("operator"), false, false),
                (String::from("Experimental Mem Comp"), if self.settings.experimental_mem_comp { "on" } else { "off" }.to_string(), false, false),
            ],
            1 => vec![
                (String::from("HPVMX_BOOT_TARGET"), self.option_value(&["dashboard", "shell", "last-vm"], self.settings.boot_target), false, false),
                (String::from("State Save/Restore"), if self.settings.state_save_restore { "on" } else { "off" }.to_string(), false, false),
                (String::from("HPVMX_WATCHDOG"), String::from("disabled"), false, false),
            ],
            2 => vec![
                (String::from("HPVMX_UI_DENSITY"), self.option_value(&["normal", "compact", "wide"], self.settings.interface_density), false, false),
                (String::from("HPVMX_UI_SCALING"), self.option_value(&["50%", "100%", "150%", "200%"], self.settings.ui_scaling), false, false),
                (String::from("Extended Symbol Library"), if self.settings.extended_symbol_library { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG VShaders"), if self.settings.pg_vshaders { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Scanlines"), if self.settings.pg_scanlines { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Dither"), if self.settings.pg_dither { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Glitch"), if self.settings.pg_glitch { "on" } else { "off" }.to_string(), false, false),
                (String::from("PG Aberration"), self.option_value(&["off", "low", "mid", "high", "super", "extreme"], self.settings.pg_aberration), false, false),
            ],
            3 => vec![
                (String::from("HPVMX_VM_SAFETY"), self.option_value(&["prompt", "auto-save", "strict"], self.settings.vm_safety_policy), false, false),
                (String::from("HPVMX_VM_DEFAULT_MEM"), format!("1024MB"), false, false),
                (String::from("HPVMX_VM_DEFAULT_CPUS"), format!("1"), false, false),
            ],
            4 => vec![
                (String::from("HPVMX_NET_PROFILE"), self.option_value(&["dhcp", "static", "loopback"], self.settings.network_profile), false, false),
                (String::from("HPVMX_NET_TARGET"), String::from("none"), false, false),
                (String::from("HPVMX_HTTPD_PORT"), String::from("8080"), false, false),
            ],
            5 => vec![
                (String::from("HPVMX_STORAGE_POLICY"), self.option_value(&["preserve", "confirm-delete", "developer"], self.settings.storage_policy), false, false),
                (String::from("Folder Absolute Sizes"), if self.settings.folder_absolute_sizes { "on" } else { "off" }.to_string(), false, false),
                (String::from("Auto-refresh Storage"), if self.settings.auto_refresh_storage { "on" } else { "off" }.to_string(), false, false),
                (String::from("Show Hidden Files"), if self.settings.show_hidden_files { "on" } else { "off" }.to_string(), false, false),
            ],
            6 => vec![
                (String::from("HPVMX_PM_VERIFY"), self.option_value(&["standard", "quick", "full"], self.settings.package_policy), false, false),
                (String::from("HPVMX_PM_AUTOHEAL"), String::from("off"), false, false),
                (String::from("HPVMX_PM_INDEX"), String::from("/PACKAGES"), false, false),
            ],
            7 => vec![
                (String::from("HPVMX_DEV_LEVEL"), self.option_value(&["normal", "verbose", "toolchain"], self.settings.developer_level), false, false),
                (String::from("Terminal Font"), self.option_value(&["8x16", "dualscale (experimental)"], self.settings.terminal_font), false, false),
                (String::from("ControlLang Support"), if self.settings.controllang_support { "on" } else { "off" }.to_string(), false, true),
                (String::from("HPVMX_MICRO_C_TARGET"), String::from("x86_64"), false, false),
            ],
            8 => vec![
                (String::from("HPVMX_SECURITY_POLICY"), self.option_value(&["standard", "paranoid", "lab"], self.settings.security_policy), false, false),
                (String::from("Ring0 UDMI/UDXI"), if self.settings.ring0_udmi_udxi { "on" } else { "off" }.to_string(), true, false),
                (String::from("HPVMX_AUTOLYTIC"), String::from("enabled"), false, false),
            ],
            _ => vec![
                (String::from("HPVMX_VERSION"), String::from("1.0.0"), false, true),
                (String::from("HPVMX_BUILD"), String::from("dev"), false, true),
                (String::from("HPVMX_ENV_COUNT"), format!("0"), false, true),
                (String::from("UEFI_VERSION"), String::from("2.10"), false, true),
            ],
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
            let color = if idx == self.selected_settings_category_idx { 0xFFFF00 } else { 0xFFFFFF };
            pg.draw_text(list_x + 10, curr_y, cat, color);
            curr_y += line_h + 5;
        }

        let detail_x = list_x + list_w + gutter;
        let detail_w = 500;
        pg.draw_rect_outline(detail_x, list_y, detail_w, list_h, 0x888888);
        
        let category_name = categories.get(self.selected_settings_category_idx).unwrap_or(&"General");
        pg.draw_text(detail_x + 10, list_y + 10, &format!("Settings for {}", category_name), 0x00FFFF);
        
        let rows = self.settings_rows();

        curr_y = list_y + 40;
        for (idx, (name, val, is_danger, is_readonly)) in rows.iter().enumerate() {
            let color = if idx == self.selected_settings_idx { 
                0xFFFF00 
            } else if *is_readonly {
                0x888888
            } else if *is_danger {
                0xFF5555
            } else { 
                0xFFFFFF 
            };
            pg.draw_text(detail_x + 10, curr_y, &format!("{:<25} : {}", name, val), color);
            curr_y += line_h;
        }

        pg.draw_text(x + 20, list_y + list_h + 20, "UP/DOWN selects setting, LEFT/RIGHT changes category, ENTER toggles, S saves", 0x888888);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            self.settings = data.settings.clone();
        }
    }
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::UP) => {
                self.selected_settings_idx = self.selected_settings_idx.saturating_sub(1);
            }
            Key::Special(ScanCode::DOWN) => {
                let max = self.settings_rows().len().saturating_sub(1);
                self.selected_settings_idx = (self.selected_settings_idx + 1).min(max);
            }
            Key::Special(ScanCode::LEFT) => {
                self.selected_settings_category_idx = self.selected_settings_category_idx.saturating_sub(1);
                self.selected_settings_idx = 0;
            }
            Key::Special(ScanCode::RIGHT) => {
                self.selected_settings_category_idx = (self.selected_settings_category_idx + 1).min(9);
                self.selected_settings_idx = 0;
            }
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Toggle will be handled by dashboard after sync
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
