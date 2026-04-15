use alloc::string::String;
use alloc::vec::Vec;
use libm::{sin, cos};
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct CubeApp {
    pub(crate) angle: f64,
}

impl AppInfo for CubeApp {
    fn name(&self) -> &str { "Cube3D" }
    fn version(&self) -> &str { "1.0.0" }

    fn icon(&self) -> [u32; 1024] {
        icons::CUBE_WINDOW_RED_32_ICON_DATA
    }
}

impl Runnable for CubeApp {
    fn logic(&mut self, _vars: &mut Vec<String>) {
        // Increment rotation angle every frame
        self.angle += 0.05;
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        // 1. Setup Cube Data (8 vertices for a cube centered at 0,0,0)
        let points = [
            [-1.0, -1.0,  1.0], [ 1.0, -1.0,  1.0], [ 1.0,  1.0,  1.0], [-1.0,  1.0,  1.0],
            [-1.0, -1.0, -1.0], [ 1.0, -1.0, -1.0], [ 1.0,  1.0, -1.0], [-1.0,  1.0, -1.0],
        ];

        // 2. Define the edges connecting the vertices (pairs of indices)
        let edges = [
            (0, 1), (1, 2), (2, 3), (3, 0), // Front face
            (4, 5), (5, 6), (6, 7), (7, 4), // Back face
            (0, 4), (1, 5), (2, 6), (3, 7), // Connecting lines
        ];

        // 3. Clear the window background
        graphics.fill_rect(x, y, 200, 200, 0x000000);

        // 4. Project and Draw
        let mut projected: [(isize, isize); 8] = [(0, 0); 8];
        let scale = 60.0; // Size of the cube
        let offset_x = (x + 100) as f64; // Center within the 200px window
        let offset_y = (y + 100) as f64;

        for (i, p) in points.iter().enumerate() {
            // Rotate on Y and X axes
            let mut px = p[0];
            let mut py = p[1];
            let mut pz = p[2];

            // Rotate Y
            let nx = px * cos(self.angle) - pz * sin(self.angle);
            let nz = px * sin(self.angle) + pz * cos(self.angle);
            px = nx; pz = nz;

            // Rotate X
            let ny = py * cos(self.angle) - pz * sin(self.angle);
            let nz_final = py * sin(self.angle) + pz * cos(self.angle);
            py = ny; pz = nz_final;

            // Simple Perspective Projection
            // z + 3.0 moves the cube away from the "camera"
            let perspective = 200.0 / (pz + 3.0);
            projected[i] = (
                (px * perspective * scale + offset_x) as isize,
                (py * perspective * scale + offset_y) as isize,
            );
        }

        // 5. Draw the lines
        for edge in edges.iter() {
            let p1 = projected[edge.0];
            let p2 = projected[edge.1];
            graphics.draw_line(p1.0 as usize, p1.1 as usize, p2.0 as usize, p2.1 as usize, 0x00FF00);
        }
    }

    fn input(&mut self, _key: uefi::proto::console::text::Key) {}
}