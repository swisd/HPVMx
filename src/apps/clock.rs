use alloc::string::String;
use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::runtime::Time;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};
use libm::{sin, cos};

pub struct ClockApp {}

impl AppInfo for ClockApp {
    fn name(&self) -> &str { "Clock" }
    fn version(&self) -> &str { "1.0.0" }
    fn icon(&self) -> [u32; 1024] { icons::CLOCK_RED_32_ICON_DATA }
}

impl Runnable for ClockApp {
    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        // 1. Get UEFI Runtime Time
        let time = uefi::runtime::get_time().unwrap();

        // 2. Clear background
        graphics.fill_rect(x, y, 200, 200, 0x1A1A1A);

        let center_x = (x + 100) as f64;
        let center_y = (y + 100) as f64;
        let PI = 3.1415926535;

        // 3. Draw 12 Hour Markings
        for i in 0..12 {
            let angle = (i as f64 / 12.0) * 2.0 * PI;

            // Start of tick (outer)
            let tick_start_len = 90.0;
            // End of tick (inner)
            let tick_end_len = 80.0;

            let start_x = center_x + tick_start_len * libm::sin(angle);
            let start_y = center_y - tick_start_len * libm::cos(angle);
            let end_x = center_x + tick_end_len * libm::sin(angle);
            let end_y = center_y - tick_end_len * libm::cos(angle);

            graphics.draw_line(start_x as usize, start_y as usize, end_x as usize, end_y as usize, 0x555555);
        }

        // 4. Helper to draw hands
        let mut draw_hand = |val: f64, max: f64, len: f64, color: u32| {
            let angle = (val / max) * 2.0 * PI;
            let end_x = center_x + len * libm::sin(angle);
            let end_y = center_y - len * libm::cos(angle);
            graphics.draw_line(center_x as usize, center_y as usize, end_x as usize, end_y as usize, color);
        };

        // 5. Draw Hands
        let hour = (time.hour() % 12) as f64 + (time.minute() as f64 / 60.0);
        draw_hand(hour, 12.0, 45.0, 0xFFFFFF);      // Hour
        draw_hand(time.minute() as f64, 60.0, 75.0, 0xAAAAAA); // Minute
        draw_hand(time.second() as f64, 60.0, 85.0, 0xFF0000); // Second

        // 6. Center "pin"
        graphics.fill_rect((center_x - 2.0) as usize, (center_y - 2.0) as usize, 4, 4, 0xFFFFFF);
    }

    fn logic(&mut self, _vars: &mut Vec<String>) {}
    fn input(&mut self, _key: uefi::proto::console::text::Key) {}
}