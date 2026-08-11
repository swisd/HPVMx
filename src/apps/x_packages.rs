use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

use alloc::collections::BTreeMap;
use crate::pm::Package;

#[derive(Clone)]
pub struct X_Packages {
    pub selected_package_idx: usize,
    pub package_action_idx: usize,
    pub registry: BTreeMap<String, Package>,
    pub status_line: String,
}

impl X_Packages {
    pub fn new() -> Self {
        Self {
            selected_package_idx: 0,
            package_action_idx: 0,
            registry: BTreeMap::new(),
            status_line: String::from("Ready"),
        }
    }

    pub fn package_names(&self) -> Vec<String> {
        self.registry.keys().cloned().collect()
    }

    pub fn selected_package_name(&self) -> Option<String> {
        let names = self.package_names();
        names.get(self.selected_package_idx).cloned()
    }
}

impl Runnable for X_Packages {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        pg.draw_text(x + 20, y + 20, "Package Manager", 0x00FF00);

        let list_x = x + 20;
        let list_y = y + 50;
        let list_w = 280;
        let list_h = 420;

        pg.draw_rect_outline(list_x, list_y, list_w, list_h, 0x888888);
        pg.fill_rect(list_x + 1, list_y + 1, list_w - 2, 18, 0x333333);
        pg.draw_text(list_x + 8, list_y + 4, "NAME                         TYPE", 0xCCCCCC);

        let package_names = self.package_names();
        let mut curr_y = list_y + 28;
        for (idx, name) in package_names.iter().enumerate() {
            if curr_y > list_y + list_h - 20 { break; }
            let Some(pkg) = self.registry.get(name) else { continue; };
            if idx == self.selected_package_idx {
                pg.fill_rect(list_x + 2, curr_y - 2, list_w - 4, 16, 0x444400);
            }
            pg.draw_text(list_x + 8, curr_y, &format!("{:<28} {:?}", pkg.name, pkg.package_type), if idx == self.selected_package_idx { 0xFFFF00 } else { 0xFFFFFF });
            pg.draw_package_icon(list_x + list_w - 24, curr_y - 1, true);
            curr_y += 18;
        }

        let detail_x = list_x + list_w + 30;
        let detail_w = 400; // Adjusted for 800 width
        pg.draw_rect_outline(detail_x, list_y, detail_w, 420, 0x888888);
        pg.fill_rect(detail_x + 1, list_y + 1, detail_w - 2, 18, 0x333333);
        pg.draw_text(detail_x + 8, list_y + 4, "PACKAGE DETAILS", 0x00FF00);

        if let Some(name) = self.selected_package_name() {
            if let Some(pkg) = self.registry.get(&name) {
                let mut dy = list_y + 30;
                pg.draw_text(detail_x + 10, dy, &format!("Name:      {}", pkg.name), 0xFFFFFF);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &format!("Version:   {}", pkg.version), 0x00FFFF);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &format!("Type:      {:?}", pkg.package_type), 0xAAAAAA);
                dy += 20;
                pg.draw_text(detail_x + 10, dy, &format!("Author:    {}", pkg.author), 0xFFFFFF);
                dy += 20;

                if let Some(ref url) = pkg.repo_url {
                    pg.draw_text(detail_x + 10, dy, &format!("Repo:      {}", url), 0x5555FF);
                    dy += 20;
                }

                let status_color = if pkg.has_compilation_issues { 0xFF5555 } else { 0x55FF55 };
                let status_text = if pkg.has_compilation_issues { "FAILED / ISSUES" } else { "READY / OK" };
                pg.draw_text(detail_x + 10, dy, &format!("Status:    {}", status_text), status_color);
                dy += 30;

                pg.draw_text(detail_x + 10, dy, "Dependencies:", 0x00FF00);
                dy += 20;
                if pkg.deps.is_empty() {
                    pg.draw_text(detail_x + 20, dy, "none", 0x888888);
                    dy += 20;
                } else {
                    for dep in &pkg.deps {
                        pg.draw_text(detail_x + 20, dy, &format!("- {}", dep), 0xCCCCCC);
                        dy += 16;
                    }
                }
                dy += 10;

                pg.draw_text(detail_x + 10, dy, "Description:", 0x00FF00);
                dy += 20;
                let desc = &pkg.description;
                let words: Vec<&str> = desc.split_whitespace().collect();
                let mut line = String::new();
                for word in words {
                    if line.len() + word.len() > 45 { // Adjusted width
                        pg.draw_text(detail_x + 20, dy, &line, 0xAAAAAA);
                        dy += 16;
                        line.clear();
                    }
                    if !line.is_empty() { line.push(' '); }
                    line.push_str(word);
                }
                if !line.is_empty() {
                    pg.draw_text(detail_x + 20, dy, &line, 0xAAAAAA);
                }
            }
        } else {
            pg.draw_text(detail_x + 10, list_y + 40, "No packages loaded", 0xAAAAAA);
        }

        let actions = ["Refresh", "Verify", "Uninstall", "Update", "Download", "Autocompile"];
        let mut action_x = x + 20;
        let action_y = list_y + list_h + 24;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == self.package_action_idx;
            pg.fill_rect(action_x, action_y, 110, 26, if is_focused { 0x00AA00 } else { 0x444444 });
            pg.draw_text(action_x + 8, action_y + 5, action, 0xFFFFFF);
            action_x += 120;
        }
        pg.draw_text(x + 20, action_y + 40, "UP/DOWN selects package, LEFT/RIGHT chooses action, ENTER runs it", 0x888888);
        pg.draw_text(x + 20, action_y + 60, &self.status_line, 0xFFFF00);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        if let Some(data) = env.global_data.as_ref() {
            self.registry = data.package_manager.registry.clone();
            self.selected_package_idx = self.selected_package_idx.min(self.registry.len().saturating_sub(1));
        }
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::UP) => {
                if self.selected_package_idx > 0 {
                    self.selected_package_idx -= 1;
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.selected_package_idx + 1 < self.registry.len() {
                    self.selected_package_idx += 1;
                }
            }
            Key::Special(ScanCode::LEFT) => {
                if self.package_action_idx > 0 {
                    self.package_action_idx -= 1;
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.package_action_idx < 5 {
                    self.package_action_idx += 1;
                }
            }
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Dashboard will execute action after sync
            }
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Packages {
    fn name(&self) -> &str { "Packages" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::ADD_PLUS_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
