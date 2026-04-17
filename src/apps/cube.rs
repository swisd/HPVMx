use alloc::string::String;
use alloc::vec::Vec;
use libm::{sin, cos};
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct CubeApp {
    angle_x: f64,
    angle_y: f64,
    speed_x: f64,
    speed_y: f64,
}

impl CubeApp {
    pub fn new() -> Self {
        Self {
            angle_x: 0.0,
            angle_y: 0.0,
            speed_x: 0.02, // Default slow rotation
            speed_y: 0.02,
        }
    }
}

impl Runnable for CubeApp {
    fn logic(&mut self, _vars: &mut Vec<String>) {
        // Apply rotation
        self.angle_x += self.speed_x;
        self.angle_y += self.speed_y;

        // Friction: gradually return to slow rotation if keys aren't held
        // (Optional: remove this if you want toggle-style speed)
        if self.speed_x.abs() > 0.02 { self.speed_x *= 0.95; }
        if self.speed_y.abs() > 0.02 { self.speed_y *= 0.95; }
    }

    fn input(&mut self, key: Key) {
        if let Key::Printable(c) = key {
            match u16::from(c) as u8 as char {
                'w' | 'W' => self.speed_x -= 0.05, // Rotate Up
                's' | 'S' => self.speed_x += 0.05, // Rotate Down
                'a' | 'A' => self.speed_y -= 0.05, // Rotate Left
                'd' | 'D' => self.speed_y += 0.05, // Rotate Right
                _ => {}
            }
        }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        // 1. Static vertex data
        let points = [
            [-1.0, -1.0,  1.0], [ 1.0, -1.0,  1.0], [ 1.0,  1.0,  1.0], [-1.0,  1.0,  1.0],
            [-1.0, -1.0, -1.0], [ 1.0, -1.0, -1.0], [ 1.0,  1.0, -1.0], [-1.0,  1.0, -1.0],
        ];

        let edges = [
            (0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6),
            (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7),
        ];

        // 2. Pre-calculate Trig once per frame (Huge Optimization)
        let s_x = sin(self.angle_x);
        let c_x = cos(self.angle_x);
        let s_y = sin(self.angle_y);
        let c_y = cos(self.angle_y);

        graphics.fill_rect(x, y, 200, 200, 0x000000);

        let mut projected: [(isize, isize); 8] = [(0, 0); 8];
        let center_x = (x + 100) as f64;
        let center_y = (y + 100) as f64;

        for (i, p) in points.iter().enumerate() {
            let px = p[0];
            let py = p[1];
            let pz = p[2];

            // Rotation X
            let xy = py * c_x - pz * s_x;
            let xz = py * s_x + pz * c_x;

            // Rotation Y
            let yx = px * c_y + xz * s_y;
            let yz = -px * s_y + xz * c_y;

            // Projection (Fixed Z-depth for speed)
            let factor = 150.0 / (yz + 4.0);
            projected[i] = (
                (yx * factor + center_x) as isize,
                (xy * factor + center_y) as isize,
            );
        }

        // 3. Draw lines
        for &(i1, i2) in &edges {
            let (p1x, p1y) = projected[i1];
            let (p2x, p2y) = projected[i2];
            graphics.draw_line(p1x as usize, p1y as usize, p2x as usize, p2y as usize, 0x00FF00);
        }
    }
}
impl AppInfo for CubeApp {
    fn name(&self) -> &str { "Cube3D" }
    fn version(&self) -> &str { "1.0.0" }

    fn icon(&self) -> [u32; 1024] {
        icons::CUBE_WINDOW_RED_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (200, 200)
    }
}
