use alloc::{format, vec};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use uefi::proto::console::text::{Key, ScanCode};
use crate::env::{AppInfo, Environment, Runnable};
use crate::ui::pixel_graphics::PixelGraphics;

#[derive(Clone)]
pub struct X_Network {
    pub selected_network_action_idx: usize,
    pub network_target: String,
}

impl X_Network {
    pub fn new() -> Self {
        Self {
            selected_network_action_idx: 0,
            network_target: String::new(),
        }
    }
}

impl Runnable for X_Network {
    fn draw(&self, pg: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        let x_off = x + 20;
        let mut y_off = y + 20;
        pg.draw_text(x_off, y_off, "Network Status", 0x00FF00);
        let net_stats = crate::devices::net_stack::stats();
        y_off += 30;
        pg.draw_text(x_off, y_off, &alloc::format!("Backend: {}", crate::devices::net_stack::backend_name()), 0xFFFFFF);
        y_off += 30;
        pg.draw_text(x_off, y_off, "Statistics:", 0xAAAAAA);

        let sub_x = x + 40;
        let mut sub_y = y + 100;
        pg.draw_text(sub_x, sub_y, &alloc::format!("RX Packets: {}", net_stats.rx_pkts), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("TX Packets: {}", net_stats.tx_pkts), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("RX Bytes:   {}", net_stats.rx_bytes), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("TX Bytes:   {}", net_stats.tx_bytes), 0xCCCCCC);

        sub_y += 40;
        let state = crate::devices::net_stack::get_state();
        pg.draw_text(sub_x, sub_y, &alloc::format!("IP: {}.{}.{}.{}", state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("GW: {}.{}.{}.{}", state.gateway[0], state.gateway[1], state.gateway[2], state.gateway[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("MASK: {}.{}.{}.{}", state.subnet_mask[0], state.subnet_mask[1], state.subnet_mask[2], state.subnet_mask[3]), 0xCCCCCC);
        sub_y += 20;
        pg.draw_text(sub_x, sub_y, &alloc::format!("MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", state.mac_addr[0], state.mac_addr[1], state.mac_addr[2], state.mac_addr[3], state.mac_addr[4], state.mac_addr[5]), 0xCCCCCC);
        sub_y += 40;
        let is_init = crate::devices::net_stack::is_initialized();
        pg.draw_text(sub_x, sub_y, &alloc::format!("Initialized: {is_init}", ), 0xFFFFFF);
        sub_y += 35;

        pg.draw_text(sub_x, sub_y, &alloc::format!("Target: {}", self.network_target), 0xCCCCCC);
        sub_y += 28;

        let actions = ["Net Up", "Status", "Ping", "LAN Scan", "HTTP On", "HTTP Off"];
        let mut action_x = sub_x;
        for (idx, action) in actions.iter().enumerate() {
            let is_focused = idx == self.selected_network_action_idx;
            pg.fill_rect(action_x, sub_y, 88, 24, if is_focused { 0x00AA00 } else { 0x444444 });
            pg.draw_text(action_x + 8, sub_y + 4, action, 0xFFFFFF);
            action_x += 96;
        }
        sub_y += 36;
        pg.draw_text(sub_x, sub_y, "LEFT/RIGHT chooses action, ENTER runs it, +/- cycles ping target", 0x888888);
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {}
    fn input(&mut self, key: Key) {
        match key {
            Key::Special(ScanCode::LEFT) => {
                if self.selected_network_action_idx > 0 { self.selected_network_action_idx -= 1; }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.selected_network_action_idx < 5 { self.selected_network_action_idx += 1; }
            }
            Key::Printable(c) if u16::from(c) == 0x0D || u16::from(c) == 0x0A => {
                // Execute action
            }
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch == '+' || ch == '=' {
                    // cycle target
                } else if ch == '-' || ch == '_' {
                    // cycle target
                }
            }
            _ => {}
        }
    }
    fn as_any(&self) -> &dyn core::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
}

impl AppInfo for X_Network {
    fn name(&self) -> &str { "Network" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { crate::ui::pixel_graphics::icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA }
    fn dimensions(&self) -> (usize, usize) { (800, 600) }
}
