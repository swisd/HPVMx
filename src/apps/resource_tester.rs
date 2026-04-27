use alloc::vec::Vec;
use alloc::string::String;
use libm::sqrt;
use uefi::proto::console::text::Key;
use uefi::prelude::*;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct SysTestApp {
    test_phase: u8,      // 0: Idle, 1: CPU (Int), 2: FPU, 3: RAM
    cpu_int_score: u64,
    fpu_score: u64,
    mem_verified_mb: u64,
    errors: u64,
    heat_map: [f32; 100],
    rng_state: u64,
    // Buffer for a real memory test (allocated on startup)
    test_buffer: Vec<u8>,
    mem_raw_buf: Vec<u64>,
}

impl SysTestApp {
    pub fn new() -> Self {
        Self {
            test_phase: 0,
            cpu_int_score: 0,
            fpu_score: 0,
            mem_verified_mb: 0,
            errors: 0,
            heat_map: [0.0; 100],
            rng_state: 0x5453_5445_524E_414C, // "STERNAL"
            test_buffer: Vec::with_capacity(1024 * 1024 * 64), // 64MB Test Buffer
            mem_raw_buf: Vec::new()
        }
    }

    fn next_rng(&mut self) -> u64 {
        self.rng_state ^= self.rng_state << 13;
        self.rng_state ^= self.rng_state >> 7;
        self.rng_state ^= self.rng_state << 17;
        self.rng_state
    }

    fn update_heat(&mut self, idx: usize, intensity: f32) {
        self.heat_map[idx % 100] = (self.heat_map[idx % 100] + intensity).min(1.0);
        for h in self.heat_map.iter_mut() { *h *= 0.97; } // Faster cooling
    }
}

impl AppInfo for SysTestApp {
    fn name(&self) -> &str { "SysBurner" }
    fn version(&self) -> &str { "2.0.0" }

    fn icon(&self) -> [u32; 1024] {
        icons::INTEGRATED_CIRCUIT_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) { (200, 260) }
}

impl Runnable for SysTestApp {
    fn logic(&mut self, _vars: &mut Vec<String>) {
        match self.test_phase {
            1 => { // PHASE 1: Integer Stress (Sieve)
                let mut local_primes = 0;
                for n in 2..1500000 {
                    let mut is_p = true;
                    for i in 2..(sqrt(n as f64) as usize + 1) {
                        if n % i == 0 { is_p = false; break; }
                    }
                    if is_p { local_primes += 1; }
                }
                self.cpu_int_score += local_primes;
                let idx = (self.next_rng() % 100) as usize;
                self.update_heat(idx, 0.5);
            }
            2 => { // PHASE 2: FPU Stress (Sine Taylor Series)
                let x: f64 = (self.next_rng() % 100) as f64 / 100.0;
                let mut sin_x = x;
                let mut term = x;
                for i in 1..2000000 { // 20 iterations of Taylor series
                    term *= -x * x / ((2 * i) * (2 * i + 1)) as f64;
                    sin_x += term;
                }
                if sin_x > 1.0 || sin_x < -1.0 { self.errors += 1; }
                self.fpu_score += 1;
                let idx = (self.next_rng() % 100) as usize;
                self.update_heat(idx, 0.6);
            }
            3 => { // PHASE 3: Memory Bandwidth (March C- Algorithm)
                let pattern = self.next_rng() as u8;
                let mut err_found = false;

                // Write pattern
                for i in 0..self.test_buffer.capacity() {
                    unsafe { *self.test_buffer.as_mut_ptr().add(i) = pattern; }
                }
                // Verify pattern
                for i in 0..self.test_buffer.capacity() {
                    unsafe { if *self.test_buffer.as_ptr().add(i) != pattern { err_found = true; } }
                }

                if err_found { self.errors += 1; }
                self.mem_verified_mb += 4; // 4MB chunked
                let idx = (self.next_rng() % 100) as usize;
                self.update_heat(idx, 0.8);
            }
            4 => {
                for i in 0..9999 {
                    let pattern = self.next_rng();
                    self.mem_raw_buf.push(pattern);
                    let idx = (self.next_rng() % 100) as usize;
                    self.update_heat(idx, 0.8);
                }

            }
            _ => self.update_heat(0, 0.0),
        }
    }

    fn input(&mut self, key: Key) {
        if let Key::Printable(c) = key {
            match u16::from(c) as u8 as char {
                '1' => self.test_phase = 1,
                '2' => self.test_phase = 2,
                '3' => self.test_phase = 3,
                '4' => self.test_phase = 4,
                '0' | 's' => self.test_phase = 0,
                _ => {}
            }
        }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics.fill_rect(x, y, 200, 200, 0x050505);

        // Render Heat Map
        for i in 0..100 {
            let heat = self.heat_map[i];
            let color = if self.errors > 0 {
                (( (heat * 255.0) as u32 ) << 16) | 0x000033 // Purple/Red tint on error
            } else {
                let r = (heat * 255.0) as u32;
                let g = ((heat * 180.0) as u32).saturating_sub(100);
                (r << 16) | (g << 8) | 20
            };
            graphics.fill_rect(x + (i % 10) * 20 + 1, y + (i / 10) * 20 + 1, 18, 18, color);
        }

        // Overlay Text
        let label = match self.test_phase {
            1 => "RUNNING: ALU_STRESS",
            2 => "RUNNING: FPU_STRESS",
            3 => "RUNNING: MEM_MARCH",
            4 => "RUNNING: MEM_FILL",
            _ => "SYSTEM TEST IDLE",
        };
        graphics.draw_text(x + 5, y + 5, label, 0x00FFFF);
        graphics.draw_text(x + 5, y + 175, &alloc::format!("INT: {}", self.cpu_int_score), 0xAAAAAA);
        graphics.draw_text(x + 5, y + 190, &alloc::format!("FPU: {}", self.fpu_score), 0xAAAAAA);
        graphics.draw_text(x + 5, y + 215, &alloc::format!("MEM: {}", self.mem_verified_mb), 0xAAAAAA);
        let smblen = self.mem_raw_buf.len() * 8;
        let text = if smblen/1000 < 1000 {alloc::format!("VEC: {}KB", (smblen)/1000)} else { alloc::format!("VEC: {}MB", (smblen)/1000/1000) };
        graphics.draw_text(x + 5, y + 230, &text, 0xAAAAAA);
        graphics.draw_text(x + 5, y + 245, &alloc::format!("ERR: {}", self.errors), 0xAAAAAA);
    }
}