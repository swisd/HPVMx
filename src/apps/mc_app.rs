use alloc::string::String;
use alloc::vec::Vec;
use libm::{sin, cos, sqrt};
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

struct Block {
    x: f64,
    y: f64,
    z: f64,
    color: u32,
}

pub struct MinecraftApp {
    // Camera Position
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
    // Camera Rotation (Yaw/Pitch)
    yaw: f64,
    pitch: f64,
    world: Vec<Block>,
}

impl MinecraftApp {
    pub fn new() -> Self {
        let mut world = Vec::new();
        // Create a 5x5 floor to make movement more obvious
        for bx in -2..=2 {
            for bz in -2..=2 {
                world.push(Block {
                    x: bx as f64 * 2.0,
                    y: 2.0, // Ground level
                    z: bz as f64 * 2.0,
                    color: if (bx + bz) % 2 == 0 { 0x228B22 } else { 0x32CD32 },
                });
            }
        }
        Self {
            cam_x: 0.0, cam_y: 0.0, cam_z: -10.0,
            yaw: 0.0, pitch: 0.0,
            world
        }
    }
    fn draw_block(&self, graphics: &mut PixelGraphics, block: &Block, ox: usize, oy: usize, sy: f64, cy: f64, sp: f64, cp: f64) {

        // Relative vertex positions for a 1x1x1 cube
        let vertices = [
            [-1.0, -1.0, 1.0], [1.0, -1.0, 1.0], [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0],
            [-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0],
        ];

        let mut projected = [(0, 0); 8];

        for (i, v) in vertices.iter().enumerate() {
            // Translate world coords relative to camera
            let dx = (v[0] + block.x) - self.cam_x;
            let dy = (v[1] + block.y) - self.cam_y;
            let dz = (v[2] + block.z) - self.cam_z;

            // Rotate Yaw (Around Y axis)
            let rx = dx * cy + dz * sy;
            let rz = -dx * sy + dz * cy;

            // Rotate Pitch (Around X axis)
            let ry = dy * cp - rz * sp;
            let final_z = dy * sp + rz * cp;

            // Project
            let factor = 400.0 / (final_z + 0.0001); // Avoid div by zero
            projected[i] = (
                (rx * factor + 200.0 + ox as f64) as usize,
                (ry * factor + 200.0 + oy as f64) as usize,
            );
        }

        let faces = [
            ([3, 2, 1, 0], block.color),      // Front
            ([1, 5, 6, 2], block.color),      // Right
            ([5, 4, 7, 6], block.color),      // Back
            ([4, 0, 3, 7], block.color),      // Left
            ([4, 5, 1, 0], block.color),         //
            ([3, 2, 6, 7], 0x8B4513),      // Top
        ];

        for (indices, color) in faces {
            let p1 = projected[indices[0]];
            let p2 = projected[indices[1]];
            let p3 = projected[indices[2]];
            let p4 = projected[indices[3]];

            // Re-use your working negative area check
            let area = (p2.0 as isize - p1.0 as isize) * (p3.1 as isize - p1.1 as isize) -
                (p2.1 as isize - p1.1 as isize) * (p3.0 as isize - p1.0 as isize);

            if area < 0 {
                graphics.polygon_fill(&[p1, p2, p3, p4], color);
            }
        }
    }
}

impl Runnable for MinecraftApp {
    fn input(&mut self, key: Key) {
        let move_speed = 0.5;
        let rot_speed = 0.1;

        if let Key::Printable(c) = key {
            match u16::from(c) as u8 as char {
                // Movement (Relative to Yaw)
                'w' | 'W' => {
                    self.cam_x += sin(self.yaw) * move_speed;
                    self.cam_z += cos(self.yaw) * move_speed;
                }
                's' | 'S' => {
                    self.cam_x -= sin(self.yaw) * move_speed;
                    self.cam_z -= cos(self.yaw) * move_speed;
                }
                'a' | 'A' => {
                    self.cam_x -= cos(self.yaw) * move_speed;
                    self.cam_z += sin(self.yaw) * move_speed;
                }
                'd' | 'D' => {
                    self.cam_x += cos(self.yaw) * move_speed;
                    self.cam_z -= sin(self.yaw) * move_speed;
                }
                // Rotation
                'j' | 'J' => self.yaw -= rot_speed,
                'l' | 'L' => self.yaw += rot_speed,
                'i' | 'I' => self.pitch -= rot_speed,
                'k' | 'K' => self.pitch += rot_speed,
                ' ' => self.cam_y -= 0.5,
                'x' | 'X' => self.cam_y += 0.5,
                _ => {}
            }
        }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics.fill_rect(x, y, 400, 400, 0x87CEEB);

        let s_y = sin(-self.yaw);
        let c_y = cos(-self.yaw);
        let s_p = sin(-self.pitch);
        let c_p = cos(-self.pitch);


        let mut sorted_blocks: Vec<(&Block, f64)> = self.world.iter().filter_map(|b| {
            let dx = b.x - self.cam_x;
            let dy = b.y - self.cam_y;
            let dz = b.z - self.cam_z;


            let depth = -dx * s_y + dz * c_y;


            if depth < 2.0 {
                None
            } else {
                Some((b, depth))
            }
        }).collect();

        // Sort Furthest -> Closest
        sorted_blocks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(core::cmp::Ordering::Equal));

        // Draw
        for (block, depth) in sorted_blocks {
            if depth > 0.5 { // Simple Near-Plane Clipping
                self.draw_block(graphics, block, x, y, s_y, c_y, s_p, c_p);
            }
        }
    }

    fn logic(&mut self, vars: &mut Vec<String>) {
        // todo
    }
}
impl AppInfo for MinecraftApp {
    fn name(&self) -> &str {
        "todo!()"
    }

    fn version(&self) -> &str {
        "todo!()"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::FLOPPY_SAVE_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (400, 400)
    }
}