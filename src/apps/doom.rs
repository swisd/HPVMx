use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};
use crate::message;

// unsafe extern "C" {
//     fn doomgeneric_key_pressed(key: u8);
//     fn doomgeneric_key_released(key: u8);
//     fn doomgeneric_tick();
//     fn doom_main();
// }
// 
// // Manually resolve the symbol at link time without defining it in Rust
// unsafe extern "C" {
//     #[link_name = "DG_ScreenBuffer"]
//     static DG_SCREEN_BUFFER_PTR: *mut u32;
// }


pub struct DoomApp {
    // DOOM manages its own state, we just hold the handle
}

impl DoomApp {
    pub fn new() -> Self {
        static mut INITIALIZED: bool = false;
        // unsafe {
        //     if !INITIALIZED {
        //         message!("\n", "DOOM: Initializing...");
        //         doom_main();
        //         message!("\n", "DOOM: doom_main returned");
        //         INITIALIZED = true;
        //     }
        // }
        Self {}
    }
}

impl AppInfo for DoomApp {
    fn name(&self) -> &str {
        "DOOM"
    }

    fn version(&self) -> &str {
        "1.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::DOOM_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (320, 200)
    }
}

impl Runnable for DoomApp {
    fn logic(&mut self, _vars: &mut Vec<String>) {
        // We call into the C-exported "doom_tick" function
        // unsafe {
        //     doomgeneric_tick();
        // }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        // DOOM produces a 320x200 buffer. 
        // We grab the pointer to that buffer and blit it to our PixelGraphics
        // unsafe {
        //     graphics.draw_buffer_at(x, y, 320, 200, DG_SCREEN_BUFFER_PTR);
        // }
    }

    fn input(&mut self, key: Key) {

        // Doom keys (doomgeneric expected values or common C-doom mapping)
        // Usually: 
        // 0x1d: CTRL (Fire)
        // 0x38: ALT (Strafe)
        // 0x39: SPACE (Open)
        // 0x48: UP, 0x50: DOWN, 0x4b: LEFT, 0x4d: RIGHT
        // 0x01: ESC
        // 0x1c: ENTER

        // match key {
        //     Key::Printable(c) => {
        //         let ch = u16::from(c) as u8 as char;
        //         match ch.to_ascii_lowercase() {
        //             ' ' => unsafe { doomgeneric_key_pressed(0x39); doomgeneric_key_released(0x39); }, // Space (Open)
        //             'f' | 'e' => unsafe { doomgeneric_key_pressed(0x1d); doomgeneric_key_released(0x1d); }, // Ctrl (Fire) - mapped to F/E
        //             '\r' => unsafe { doomgeneric_key_pressed(0x1c); doomgeneric_key_released(0x1c); }, // Enter
        //             '\x1b' => unsafe { doomgeneric_key_pressed(0x01); doomgeneric_key_released(0x01); }, // Esc
        //             'w' => unsafe { doomgeneric_key_pressed(0x48); doomgeneric_key_released(0x48); }, // Up
        //             's' => unsafe { doomgeneric_key_pressed(0x50); doomgeneric_key_released(0x50); }, // Down
        //             'a' => unsafe { doomgeneric_key_pressed(0x4b); doomgeneric_key_released(0x4b); }, // Left
        //             'd' => unsafe { doomgeneric_key_pressed(0x4d); doomgeneric_key_released(0x4d); }, // Right
        //             _ => {}
        //         }
        //     }
        //     Key::Special(scancode) => {
        //         // uefi scancodes
        //         match scancode {
        //             uefi::proto::console::text::ScanCode::UP => unsafe { doomgeneric_key_pressed(0x48); doomgeneric_key_released(0x48); },
        //             uefi::proto::console::text::ScanCode::DOWN => unsafe { doomgeneric_key_pressed(0x50); doomgeneric_key_released(0x50); },
        //             uefi::proto::console::text::ScanCode::LEFT => unsafe { doomgeneric_key_pressed(0x4b); doomgeneric_key_released(0x4b); },
        //             uefi::proto::console::text::ScanCode::RIGHT => unsafe { doomgeneric_key_pressed(0x4d); doomgeneric_key_released(0x4d); },
        //             uefi::proto::console::text::ScanCode::ESCAPE => unsafe { doomgeneric_key_pressed(0x01); doomgeneric_key_released(0x01); },
        //             _ => {}
        //         }
        //     }
        // }
    }
}