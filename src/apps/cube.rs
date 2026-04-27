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

        let points = [
            [-1.0, -1.0,  1.0], [ 1.0, -1.0,  1.0], [ 1.0,  1.0,  1.0], [-1.0,  1.0,  1.0], // Front
            [-1.0, -1.0, -1.0], [ 1.0, -1.0, -1.0], [ 1.0,  1.0, -1.0], [-1.0,  1.0, -1.0], // Back
        ];

        let faces = [
            ([0, 1, 2, 3], 0xFF0000), // Front  - Red
            ([1, 5, 6, 2], 0x00FF00), // Right  - Green
            ([5, 4, 7, 6], 0x0000FF), // Back   - Blue
            ([4, 0, 3, 7], 0xFFFF00), // Left   - Yellow
            ([3, 2, 6, 7], 0xFF00FF), // Top    - Magenta
            ([4, 5, 1, 0], 0x00FFFF), // Bottom - Cyan
        ];

        let s_x = sin(self.angle_x);
        let c_x = cos(self.angle_x);
        let s_y = sin(self.angle_y);
        let c_y = cos(self.angle_y);

        graphics.fill_rect(x, y, 200, 200, 0x000000);

        let mut projected: [(usize, usize); 8] = [(0, 0); 8];
        let center_x = (x + 100) as f64;
        let center_y = (y + 100) as f64;

        // Transform and Project vertices
        for (i, p) in points.iter().enumerate() {
            let px = p[0];
            let py = p[1];
            let pz = p[2];

            let xy = py * c_x - pz * s_x;
            let xz = py * s_x + pz * c_x;
            let yx = px * c_y + xz * s_y;
            let yz = -px * s_y + xz * c_y;

            let factor = 150.0 / (yz + 4.0);
            projected[i] = (
                (yx * factor + center_x) as usize,
                (xy * factor + center_y) as usize,
            );
        }

        for (indices, color) in faces.iter() {
            let p1 = projected[indices[0]];
            let p2 = projected[indices[1]];
            let p3 = projected[indices[2]];
            let p4 = projected[indices[3]];


            let v1x = p2.0 as isize - p1.0 as isize;
            let v1y = p2.1 as isize - p1.1 as isize;
            let v2x = p3.0 as isize - p1.0 as isize;
            let v2y = p3.1 as isize - p1.1 as isize;

            if (v1x * v2y - v1y * v2x) < 0 {
                let face_points = [p1, p2, p3, p4];
                graphics.polygon_fill(&face_points, *color);


                for i in 0..4 {
                    let start = face_points[i];
                    let end = face_points[(i + 1) % 4];
                    graphics.draw_line(start.0, start.1, end.0, end.1, 0x000000);
                }
            }
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
