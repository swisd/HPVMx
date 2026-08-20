use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use core::any::Any;
use libm::sqrt;
use uefi::proto::console::text::Key;
use uefi::prelude::*;
use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
use crate::page::Pagefile;
use crate::ui::pixel_graphics::{icons, PixelGraphics};

pub struct SysTestApp {
    test_phase: u8,      // 0: Idle, 1: CPU (Int), 2: FPU, 3: RAM March, 4: RAM Fill, 5: Pagefile RW, 6: Pagefile Write, 7: Pagefile Read
    cpu_int_score: u64,
    fpu_score: u64,
    mem_verified_mb: u64,
    errors: u64,
    heat_map: [f32; 100],
    rng_state: u64,
    // Buffer for a real memory test (allocated on startup)
    test_buffer: Vec<u8>,
    mem_raw_buf: Vec<u64>,
    // Pagefile test state
    pagefile: Pagefile,
    page_block_cursor: u32,
    page_written_kb: u64,
    page_read_kb: u64,
    page_verified_kb: u64,
    page_err_count: u64,
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
            mem_raw_buf: Vec::new(),
            pagefile: Pagefile::new(),
            page_block_cursor: 0,
            page_written_kb: 0,
            page_read_kb: 0,
            page_verified_kb: 0,
            page_err_count: 0,
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
    fn version(&self) -> &str { "2.1.0" }

    fn icon(&self) -> [u32; 1024] {
        icons::INTEGRATED_CIRCUIT_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) { (200, 290) }
}

impl RunnableClone for SysTestApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        Box::new(SysTestApp::new())
    }
}

impl Runnable for SysTestApp {
    fn logic(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) {
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
            4 => { // PHASE 4: Memory Fill
                for _ in 0..9999 {
                    let pattern = self.next_rng();
                    self.mem_raw_buf.push(pattern);
                    let idx = (self.next_rng() % 100) as usize;
                    self.update_heat(idx, 0.8);
                }
            }
            5 => { // PHASE 5: Pagefile Read/Write Verification Test
                let max_test_blocks = self.pagefile.max_blocks().min(4096);
                let blocks_to_test = 2;

                for _ in 0..blocks_to_test {
                    let block_id = self.page_block_cursor;
                    self.page_block_cursor = if max_test_blocks > 0 {
                        (self.page_block_cursor + 1) % max_test_blocks
                    } else {
                        0
                    };

                    // Generate deterministic test pattern
                    let mut write_buf = [0u8; 4096];
                    let seed = self.next_rng();
                    for j in (0..4096).step_by(8) {
                        let val = seed ^ ((block_id as u64) << 32) ^ (j as u64);
                        write_buf[j..j+8].copy_from_slice(&val.to_le_bytes());
                    }

                    // Write block
                    match self.pagefile.write_block(block_id, &write_buf) {
                        Ok(()) => {
                            self.page_written_kb += 4;
                        }
                        Err(_) => {
                            self.errors += 1;
                            self.page_err_count += 1;
                        }
                    }

                    // Read block back
                    match self.pagefile.read_block(block_id) {
                        Ok(read_buf) => {
                            self.page_read_kb += 4;
                            if read_buf == write_buf {
                                self.page_verified_kb += 4;
                            } else {
                                self.errors += 1;
                                self.page_err_count += 1;
                            }
                        }
                        Err(_) => {
                            self.errors += 1;
                            self.page_err_count += 1;
                        }
                    }

                    let idx = (block_id as usize) % 100;
                    self.update_heat(idx, 0.85);
                }
            }
            6 => { // PHASE 6: Pagefile Sequential Write Stress
                let max_test_blocks = self.pagefile.max_blocks().min(4096);
                let blocks_to_write = 4;

                for _ in 0..blocks_to_write {
                    let block_id = self.page_block_cursor;
                    self.page_block_cursor = if max_test_blocks > 0 {
                        (self.page_block_cursor + 1) % max_test_blocks
                    } else {
                        0
                    };

                    let mut write_buf = [0u8; 4096];
                    let seed = self.next_rng();
                    for j in (0..4096).step_by(8) {
                        let val = seed ^ (j as u64);
                        write_buf[j..j+8].copy_from_slice(&val.to_le_bytes());
                    }

                    if self.pagefile.write_block(block_id, &write_buf).is_ok() {
                        self.page_written_kb += 4;
                    } else {
                        self.errors += 1;
                        self.page_err_count += 1;
                    }

                    let idx = (block_id as usize) % 100;
                    self.update_heat(idx, 0.75);
                }
            }
            7 => { // PHASE 7: Pagefile Sequential Read Stress
                let max_test_blocks = self.pagefile.max_blocks().min(4096);
                let blocks_to_read = 4;

                for _ in 0..blocks_to_read {
                    let block_id = self.page_block_cursor;
                    self.page_block_cursor = if max_test_blocks > 0 {
                        (self.page_block_cursor + 1) % max_test_blocks
                    } else {
                        0
                    };

                    if self.pagefile.read_block(block_id).is_ok() {
                        self.page_read_kb += 4;
                    } else {
                        self.errors += 1;
                        self.page_err_count += 1;
                    }

                    let idx = (block_id as usize) % 100;
                    self.update_heat(idx, 0.75);
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
                '5' | 'p' | 'P' => self.test_phase = 5,
                '6' | 'w' | 'W' => self.test_phase = 6,
                '7' | 'r' | 'R' => self.test_phase = 7,
                '0' | 's' | 'S' => self.test_phase = 0,
                'c' | 'C' => {
                    self.errors = 0;
                    self.page_err_count = 0;
                }
                _ => {}
            }
        }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        graphics.fill_rect(x, y, 200, 290, 0x050508);

        // Render Heat Map (10x10 grid)
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

        // Overlay Text Label
        let label = match self.test_phase {
            1 => "RUNNING: ALU_STRESS",
            2 => "RUNNING: FPU_STRESS",
            3 => "RUNNING: MEM_MARCH",
            4 => "RUNNING: MEM_FILL",
            5 => "RUNNING: PAGE_RW_TEST",
            6 => "RUNNING: PAGE_WRITE",
            7 => "RUNNING: PAGE_READ",
            _ => "SYSTEM TEST IDLE",
        };
        graphics.draw_text_bg(x + 5, y + 4, label, 0x00FFFF, 0x001420);

        // Statistics Summary
        graphics.draw_text(x + 5, y + 204, &format!("INT: {} | FPU: {}", self.cpu_int_score, self.fpu_score), 0xAAAAAA);

        let smblen = self.mem_raw_buf.len() * 8;
        let vec_text = if smblen / 1000 < 1000 {
            format!("{}KB", smblen / 1000)
        } else {
            format!("{}MB", smblen / 1000 / 1000)
        };
        graphics.draw_text(x + 5, y + 218, &format!("MEM: {}MB | VEC: {}", self.mem_verified_mb, vec_text), 0xAAAAAA);

        let page_text = if self.page_written_kb >= 1024 || self.page_read_kb >= 1024 {
            format!("PAGE: W:{}M R:{}M", self.page_written_kb / 1024, self.page_read_kb / 1024)
        } else {
            format!("PAGE: W:{}K R:{}K", self.page_written_kb, self.page_read_kb)
        };
        graphics.draw_text(x + 5, y + 232, &page_text, 0x00D0FF);

        let verif_text = if self.page_verified_kb >= 1024 {
            format!("{}MB", self.page_verified_kb / 1024)
        } else {
            format!("{}KB", self.page_verified_kb)
        };
        graphics.draw_text(x + 5, y + 246, &format!("BLK: #{} | VRF: {}", self.page_block_cursor, verif_text), 0x88CC88);

        let err_color = if self.errors > 0 { 0xFF4444 } else { 0x66FF66 };
        graphics.draw_text(x + 5, y + 260, &format!("ERRORS: {} (PG: {})", self.errors, self.page_err_count), err_color);

        graphics.draw_text(x + 5, y + 274, "[1-4]Sys [5]PgRW [0]Stop", 0x557799);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}