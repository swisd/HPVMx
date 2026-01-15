use core::fmt::Write;
use core::ptr::null_mut;
use uefi::proto::console::text::Color;
// Import both protocols
use uefi::proto::console::pointer::Pointer;
//use uefi::proto::pointer::SimplePointer;

pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
}

impl Cursor {
    pub fn new() -> Self {
        Cursor {
            x: 0,
            y: 0,
            visible: true,
            left_button: false,
            right_button: false,
        }
    }

    pub fn update_from_mouse(&mut self) {
        // Strategy: Try Absolute Pointer first, then fall back to Simple (Relative) Pointer
        if !self.try_update_absolute() {
            self.try_update_relative();
        }
    }

    fn try_update_absolute(&mut self) -> bool {
        if let Ok(handle) = uefi::boot::get_handle_for_protocol::<Pointer>() {
            if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(handle) {
                let mode = &mut mouse.mode();
                let Ok(mut mouse2) = uefi::boot::open_protocol_exclusive::<Pointer>(handle) else { todo!() };
                if let Ok(Some(state)) = mouse2.read_state() {
                    // Map absolute hardware coordinates to our 120x40 text grid
                    if mode.resolution[0] > 0 {
                        self.x = ((state.relative_movement[0] as u64 * 120) / mode.resolution[0]) as i32;
                    }
                    if mode.resolution[1] > 0 {
                        self.y = ((state.relative_movement[1] as u64 * 40) / mode.resolution[1]) as i32;
                    }
                    self.left_button = state.button[0];
                    self.right_button = state.button[1];
                    return true; // Successfully updated via Absolute Pointer
                }
            }
        }
        false
    }

    fn try_update_relative(&mut self) {
        // if let Ok(handle) = uefi::boot::get_handle_for_protocol::<SimplePointer>() {
        //     if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<SimplePointer>(handle) {
        //         if let Ok(Some(state)) = mouse.read_state() {
        //             // Add relative movement to existing position
        //             self.x = self.x.saturating_add(state.relative_movement_x);
        //             self.y = self.y.saturating_add(state.relative_movement_y);
        //
        //             // Clamp to the 120x40 grid
        //             self.x = self.x.max(0).min(120);
        //             self.y = self.y.max(0).min(40);
        //
        //             self.left_button = state.left_button;
        //             self.right_button = state.right_button;
        //         }
        //     }
        // }
    }

    pub fn render(&self, stdout: &mut uefi::proto::console::text::Output) {
        let cursor_char = if self.left_button { "✔" } else { "▶" };
        let _ = write!(stdout, "\x1b[{};{}H{}", self.y + 1, self.x + 1, cursor_char);
    }
}