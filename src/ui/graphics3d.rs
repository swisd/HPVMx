use crate::ui::pixel_graphics::PixelGraphics;
use libm::{cos, sin};
use alloc::vec::Vec;

pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct Matrix4 {
    pub m: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        for i in 0..4 {
            m[i][i] = 1.0;
        }
        Self { m }
    }

    pub fn rotation_x(angle: f32) -> Self {
        let mut res = Self::identity();
        let c = cos(angle as f64) as f32;
        let s = sin(angle as f64) as f32;
        res.m[1][1] = c;
        res.m[1][2] = -s;
        res.m[2][1] = s;
        res.m[2][2] = c;
        res
    }

    pub fn rotation_y(angle: f32) -> Self {
        let mut res = Self::identity();
        let c = cos(angle as f64) as f32;
        let s = sin(angle as f64) as f32;
        res.m[0][0] = c;
        res.m[0][2] = s;
        res.m[2][0] = -s;
        res.m[2][2] = c;
        res
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut res = Self::identity();
        let c = cos(angle as f64) as f32;
        let s = sin(angle as f64) as f32;
        res.m[0][0] = c;
        res.m[0][1] = -s;
        res.m[1][0] = s;
        res.m[1][1] = c;
        res
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut res = Self::identity();
        res.m[0][3] = x;
        res.m[1][3] = y;
        res.m[2][3] = z;
        res
    }

    pub fn multiply_vec(&self, v: &Vector3) -> Vector3 {
        let x = v.x * self.m[0][0] + v.y * self.m[0][1] + v.z * self.m[0][2] + self.m[0][3];
        let y = v.x * self.m[1][0] + v.y * self.m[1][1] + v.z * self.m[1][2] + self.m[1][3];
        let z = v.x * self.m[2][0] + v.y * self.m[2][1] + v.z * self.m[2][2] + self.m[2][3];
        let w = v.x * self.m[3][0] + v.y * self.m[3][1] + v.z * self.m[3][2] + self.m[3][3];
        
        if w != 0.0 && w != 1.0 {
            Vector3::new(x / w, y / w, z / w)
        } else {
            Vector3::new(x, y, z)
        }
    }
}

pub struct Graphics3D<'a> {
    pub pg: &'a mut PixelGraphics,
    pub width: f32,
    pub height: f32,
    pub fov: f32,
    pub z_near: f32,
    pub z_far: f32,
}

impl<'a> Graphics3D<'a> {
    pub fn new(pg: &'a mut PixelGraphics) -> Self {
        let (w, h) = pg.resolution();
        Self {
            pg,
            width: w as f32,
            height: h as f32,
            fov: 90.0,
            z_near: 0.1,
            z_far: 1000.0,
        }
    }

    pub fn project(&self, v: &Vector3) -> (usize, usize) {
        let aspect_ratio = self.height / self.width;
        let fov_rad = 1.0 / (self.fov * 0.5 / 180.0 * 3.14159).tan();
        
        let mut x = v.x * aspect_ratio * fov_rad;
        let mut y = v.y * fov_rad;
        let z = v.z; // Simplified projection

        // Perspective divide
        if z != 0.0 {
            x /= z;
            y /= z;
        }

        let sx = ((x + 1.0) * 0.5 * self.width) as usize;
        let sy = ((1.0 - y) * 0.5 * self.height) as usize;

        (sx, sy)
    }

    pub fn draw_wireframe_poly(&mut self, points: &[Vector3], color: u32) {
        let mut projected = Vec::new();
        for p in points {
            projected.push(self.project(p));
        }
        self.pg.polygon_outline(&projected, color);
    }

    pub fn draw_filled_poly(&mut self, points: &[Vector3], color: u32) {
        let mut projected = Vec::new();
        for p in points {
            projected.push(self.project(p));
        }
        self.pg.polygon_fill(&projected, color);
    }
}
