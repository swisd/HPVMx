use core::fmt::Write;
use core::time::Duration;
use uefi::proto::console::text::Color;
use uefi::{StatusExt, Identify};
use uefi::proto::console::pointer::Pointer;
use uefi_raw::protocol::console::AbsolutePointerProtocol;
use crate::message;

#[repr(transparent)]
struct AbsolutePointer(AbsolutePointerProtocol);
unsafe impl uefi::Identify for AbsolutePointer {
    const GUID: uefi::Guid = AbsolutePointerProtocol::GUID;
}
impl uefi::proto::Protocol for AbsolutePointer {}

#[allow(dead_code)]
impl AbsolutePointer {
    fn read_state(&mut self) -> uefi::Result<Option<uefi_raw::protocol::console::AbsolutePointerState>> {
        let mut state = uefi_raw::protocol::console::AbsolutePointerState {
            current_x: 0,
            current_y: 0,
            current_z: 0,
            active_buttons: 0,
        };
        match unsafe { (self.0.get_state)(&mut self.0, &mut state) } {
            uefi::Status::SUCCESS => Ok(Some(state)),
            uefi::Status::NOT_READY => Ok(None),
            status => Err(status.into()),
        }
    }

    fn mode(&self) -> &uefi_raw::protocol::console::AbsolutePointerMode {
        unsafe { &*self.0.mode }
    }
    
    fn reset(&mut self, extended: bool) -> uefi::Result {
        unsafe { (self.0.reset)(&mut self.0, extended.into()) }.to_result()
    }
}
 // Assuming you're using uefi-services for simplicity

#[allow(dead_code)]
pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
}

#[allow(dead_code)]
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

    pub fn update_from_mouse(&mut self, screen_width: usize, screen_height: usize) {
        // Try absolute first (modern environments/VMs)
        if self.try_update_absolute(screen_width, screen_height) {
            return;
        }
        // Fallback to relative
        self.try_update_relative(screen_width, screen_height);
    }

    fn try_update_absolute(&mut self, screen_width: usize, screen_height: usize) -> bool {
        let Ok(handle) = uefi::boot::get_handle_for_protocol::<AbsolutePointer>() else { return false };
        let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<AbsolutePointer>(handle) else { return false };

        if let Ok(Some(state)) = mouse.read_state() {
            let mode = mouse.mode();
            
            // Map absolute coordinates to screen coordinates
            // Formula: screen_pos = (abs_pos - abs_min) * screen_max / (abs_max - abs_min)
            let abs_x = state.current_x as f32;
            let abs_y = state.current_y as f32;
            
            let min_x = mode.absolute_min_x as f32;
            let min_y = mode.absolute_min_y as f32;
            let max_x = mode.absolute_max_x as f32;
            let max_y = mode.absolute_max_y as f32;

            if max_x > min_x && max_y > min_y {
                self.x = (((abs_x - min_x) * (screen_width as f32 - 1.0)) / (max_x - min_x)) as i32;
                self.y = (((abs_y - min_y) * (screen_height as f32 - 1.0)) / (max_y - min_y)) as i32;
            }

            self.left_button = state.active_buttons & 0x1 != 0;
            self.right_button = state.active_buttons & 0x2 != 0;
            return true;
        }
        false
    }

    // Absolute pointer path not used in this target environment

    fn try_update_relative(&mut self, screen_width: usize, screen_height: usize) {
        let Ok(handle) = uefi::boot::get_handle_for_protocol::<Pointer>() else {
            return;
        };
        let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(handle) else {
            return;
        };

        if let Ok(Some(state)) = mouse.read_state() {
            let mode = mouse.mode();
            
            // X and Y resolution can be used to scale movement
            // resolution is in counts/mm. If 0, it's not supported.
            // We use a base sensitivity and then scale if resolution is available.
            let mut dx = state.relative_movement[0] as f32;
            let mut dy = state.relative_movement[1] as f32;

            // Sensitivity factor - adjust as needed. 
            // In some environments, 1:1 is too slow.
            let sensitivity = 1.5f32;
            dx *= sensitivity;
            dy *= sensitivity;

            // Optional: apply resolution scaling if available
            if mode.resolution[0] != 0 {
                // Adjusting based on resolution might help on real hardware
                // but for now we keep it simple to ensure basic movement first.
            }

            if dx != 0.0 || dy != 0.0 {
                self.x = (self.x + dx as i32).clamp(0, screen_width as i32 - 1);
                self.y = (self.y + dy as i32).clamp(0, screen_height as i32 - 1);
            }

            self.left_button = state.button[0];
            self.right_button = state.button[1];
        }
    }

    pub fn render(&self, stdout: &mut uefi::proto::console::text::Output) {
        let _cursor_char = if self.left_button { "+" } else { "*" };
        stdout.enable_cursor(true).expect("cursor issue 0");
        stdout.set_cursor_position(self.x as usize, self.y as usize).expect("cursor issue 1");
        //let _ = write!(stdout, "[{};{}H{}", self.y + 1, self.x + 1, cursor_char);
    }

    pub fn debug_mouse() {
        use uefi::proto::console::pointer::Pointer;
        use uefi_raw::protocol::console::AbsolutePointerProtocol;

        message!("\n", "--- Mouse Debug ---");

        // Check relative pointer
        let handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&Pointer::GUID));
        match handles {
            Ok(h) => {
                message!("", "Found {} handles with Simple Pointer protocol", h.as_slice().len());
                for (i, handle) in h.as_slice().iter().enumerate() {
                    if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(*handle) {
                        let mode = mouse.mode();
                        message!("", "  [{}] Resolution: [{}, {}, {}], Buttons: [{}, {}]", 
                            i, mode.resolution[0], mode.resolution[1], mode.resolution[2],
                            mode.has_button[0], mode.has_button[1]);
                    } else {
                        message!("", "  [{}] Failed to open protocol", i);
                    }
                }
            }
            Err(_) => message!("", "No Simple Pointer protocol found"),
        }

        // Check absolute pointer
        let handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&AbsolutePointerProtocol::GUID));
        match handles {
            Ok(h) => {
                message!("", "Found {} handles with Absolute Pointer protocol", h.as_slice().len());
                for (i, handle) in h.as_slice().iter().enumerate() {
                    // Use our local wrapper
                    if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<AbsolutePointer>(*handle) {
                        let mode = mouse.mode();
                        message!("", "  [{}] Range: [{}..{}, {}..{}, {}..{}], Buttons: {:?}", 
                            i, mode.absolute_min_x, mode.absolute_max_x,
                            mode.absolute_min_y, mode.absolute_max_y,
                            mode.absolute_min_z, mode.absolute_max_z,
                            mode.attributes);
                    } else {
                        message!("", "  [{}] Failed to open protocol", i);
                    }
                }
            }
            Err(_) => message!("", "No Absolute Pointer protocol found"),
        }

        message!("", "Polling data... Press any key to stop.");
        
        loop {
            // Poll relative
            if let Ok(handle) = uefi::boot::get_handle_for_protocol::<Pointer>() {
                if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(handle) {
                    if let Ok(Some(state)) = mouse.read_state() {
                        if state.relative_movement[0] != 0 || state.relative_movement[1] != 0 || state.button[0] || state.button[1] {
                            message!("", "REL: dx={}, dy={}, btn=[{}, {}]", 
                                state.relative_movement[0], state.relative_movement[1],
                                state.button[0], state.button[1]);
                        }
                    } else if let Ok(None) = mouse.read_state() {
                        uefi::boot::stall(Duration::from_millis(10));
                    }

                    //else {
                    //     message!("", "no state rel  {:#?}", mouse.read_state());
                    // }
                }
            }

            // Poll absolute
            if let Ok(handle) = uefi::boot::get_handle_for_protocol::<AbsolutePointer>() {
                if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<AbsolutePointer>(handle) {
                    if let Ok(Some(state)) = mouse.read_state() {
                        message!("", "ABS: x={}, y={}, btn={}", 
                            state.current_x, state.current_y, state.active_buttons);
                    } else if let Ok(None) = mouse.read_state() {
                        uefi::boot::stall(Duration::from_millis(10));
                    }


                    //else {
                    //     message!("", "no state abs  {:#?}", mouse.read_state());
                    // }
                }
            }

            // Check for keypress to exit
            let key = uefi::system::with_stdin(|i| i.read_key());
            if let Ok(Some(_)) = key {
                break;
            }

            uefi::boot::stall(core::time::Duration::from_millis(100)); // 100ms
        }
    }
}

#[allow(dead_code)]
use uefi::system;

// Windows NT Color Palette
pub struct WinNTColors;

#[allow(dead_code)]
impl WinNTColors {
    pub const TITLE_BAR: (Color, Color) = (Color::White, Color::Blue);
    pub const DIALOG_BG: (Color, Color) = (Color::Black, Color::LightGray);
    pub const BUTTON_NORMAL: (Color, Color) = (Color::Black, Color::LightGray);
    pub const BUTTON_HIGHLIGHT: (Color, Color) = (Color::White, Color::LightGray);
    pub const BUTTON_SHADOW: (Color, Color) = (Color::LightGray, Color::DarkGray);
    pub const TEXT_NORMAL: (Color, Color) = (Color::Black, Color::LightGray);
    pub const BORDER_LIGHT: Color = Color::White;
    pub const BORDER_DARK: Color = Color::DarkGray;
    pub const TASKBAR_BG: (Color, Color) = (Color::Black, Color::DarkGray);
}

pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[allow(dead_code)]
impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect { x, y, width, height }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

#[allow(dead_code)]
pub struct Graphics;

#[allow(dead_code)]
impl Graphics {
    /// Clear screen with background color
    pub fn clear_screen(bg: Color) {
        system::with_stdout(|stdout| {
            let _ = stdout.set_color(Color::White, bg);
            let _ = stdout.clear();
        });
    }

    /// Set cursor position (column, row)
    pub fn set_cursor(col: usize, row: usize) {
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(col, row);
        });
    }

    /// Draw a titled box (window frame)
    pub fn draw_box(rect: &Rect, title: &str, active: bool) {
        let title_color = if active {
            WinNTColors::TITLE_BAR
        } else {
            (Color::White, Color::DarkGray)
        };

        // Top border
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x, rect.y);
            let _ = stdout.set_color(title_color.0, title_color.1);

            // Top-left corner + title
            let _ = write!(stdout, "┌");
            for _ in 1..rect.width.saturating_sub(2) {
                let _ = write!(stdout, "─");
            }
            let _ = write!(stdout, "┐");
        });

        // Title bar
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x + 1, rect.y);
            let padding = (rect.width.saturating_sub(2).saturating_sub(title.len())) / 2;
            for _ in 0..padding {
                let _ = write!(stdout, " ");
            }
            let _ = write!(stdout, "{}", title);
        });

        // Side borders
        for i in 1..rect.height.saturating_sub(1) {
            system::with_stdout(|stdout| {
                let _ = stdout.set_cursor_position(rect.x, rect.y + i);
                let _ = stdout.set_color(Color::Black, Color::LightGray);
                let _ = write!(stdout, "│");

                for _ in 1..rect.width.saturating_sub(1) {
                    let _ = write!(stdout, " ");
                }

                let _ = write!(stdout, "│");
            });
        }

        // Bottom border
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x, rect.y + rect.height - 1);
            let _ = stdout.set_color(Color::Black, Color::LightGray);
            let _ = write!(stdout, "└");
            for _ in 1..rect.width.saturating_sub(2) {
                let _ = write!(stdout, "─");
            }
            let _ = write!(stdout, "┘");
        });
    }

    /// Draw a 3D button
    pub fn draw_button(rect: &Rect, label: &str, focused: bool) {
        let (text_fg, text_bg) = if focused {
            WinNTColors::BUTTON_HIGHLIGHT
        } else {
            WinNTColors::BUTTON_NORMAL
        };

        // Button background
        for i in rect.y..rect.y + rect.height {
            system::with_stdout(|stdout| {
                let _ = stdout.set_cursor_position(rect.x, i);
                let _ = stdout.set_color(text_fg, text_bg);
                for _ in 0..rect.width {
                    let _ = write!(stdout, " ");
                }
            });
        }

        // Button text centered
        system::with_stdout(|stdout| {
            let padding = (rect.width.saturating_sub(label.len())) / 2;
            let _ = stdout.set_cursor_position(rect.x + padding, rect.y + rect.height / 2);
            let _ = stdout.set_color(text_fg, text_bg);
            let _ = write!(stdout, "{}", label);
        });
    }

    /// Draw a text input field
    pub fn draw_textbox(rect: &Rect, text: &str, focused: bool) {
        let bg = if focused { Color::White } else { Color::LightGray };

        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x, rect.y);
            let _ = stdout.set_color(Color::Black, bg);

            // Draw border
            let _ = write!(stdout, "┌");
            for _ in 1..rect.width.saturating_sub(2) {
                let _ = write!(stdout, "─");
            }
            let _ = write!(stdout, "┐");
        });

        // Content
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x, rect.y + 1);
            let _ = stdout.set_color(Color::Black, bg);
            let _ = write!(stdout, "│ ");

            if text.len() < rect.width.saturating_sub(4) {
                let _ = write!(stdout, "{}", text);
                for _ in text.len()..rect.width.saturating_sub(4) {
                    let _ = write!(stdout, " ");
                }
            } else {
                let _ = write!(stdout, "{}", &text[..rect.width.saturating_sub(4)]);
            }

            let _ = write!(stdout, " │");
        });

        // Bottom border
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(rect.x, rect.y + 2);
            let _ = stdout.set_color(Color::Black, bg);
            let _ = write!(stdout, "└");
            for _ in 1..rect.width.saturating_sub(2) {
                let _ = write!(stdout, "─");
            }
            let _ = write!(stdout, "┘");
        });
    }

    /// Draw a menu bar
    pub fn draw_menu_bar(items: &[&str]) {
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(0, 0);
            let _ = stdout.set_color(Color::Black, Color::LightGray);

            for item in items {
                let _ = write!(stdout, " {} ", item);
            }

            // Fill rest of line
            let _ = write!(stdout, "{}", " ".repeat(80));
        });
    }

    /// Draw taskbar at bottom
    pub fn draw_taskbar(time: &str) {
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(0, 24);
            let _ = stdout.set_color(Color::Black, Color::LightGray);

            let _ = write!(stdout, "Start");

            // Fill middle
            for _ in 0..70 {
                let _ = write!(stdout, " ");
            }

            // Time on right
            let _ = write!(stdout, "{}", time);
        });
    }

    /// Draw a scrollable list
    pub fn draw_list(rect: &Rect, items: &[&str], selected: usize) {
        // Draw list box
        for i in rect.y..rect.y + rect.height {
            system::with_stdout(|stdout| {
                let _ = stdout.set_cursor_position(rect.x, i);
                let _ = stdout.set_color(Color::Black, Color::LightGray);

                let item_idx = i - rect.y - 1;
                if i == rect.y {
                    let _ = write!(stdout, "┌");
                    for _ in 1..rect.width.saturating_sub(2) {
                        let _ = write!(stdout, "─");
                    }
                    let _ = write!(stdout, "┐");
                } else if i == rect.y + rect.height - 1 {
                    let _ = write!(stdout, "└");
                    for _ in 1..rect.width.saturating_sub(2) {
                        let _ = write!(stdout, "─");
                    }
                    let _ = write!(stdout, "┘");
                } else {
                    let _ = write!(stdout, "│");

                    if item_idx < items.len() {
                        let item = items[item_idx];
                        let bg = if item_idx == selected {
                            Color::Blue
                        } else {
                            Color::LightGray
                        };

                        let _ = stdout.set_color(Color::White, bg);

                        if item.len() < rect.width - 2 {
                            let _ = write!(stdout, "{}", item);
                            for _ in item.len()..rect.width - 2 {
                                let _ = write!(stdout, " ");
                            }
                        } else {
                            let _ = write!(stdout, "{}", &item[..rect.width - 2]);
                        }
                    } else {
                        for _ in 0..rect.width - 2 {
                            let _ = write!(stdout, " ");
                        }
                    }

                    let _ = write!(stdout, "│");
                }
            });
        }
    }

    /// Print text at position
    pub fn print_at(col: usize, row: usize, text: &str, fg: Color, bg: Color) {
        system::with_stdout(|stdout| {
            let _ = stdout.set_cursor_position(col, row);
            let _ = stdout.set_color(fg, bg);
            let _ = write!(stdout, "{}", text);
        });
    }

    // pub fn get_resolution(boot_services: &BootServices) -> Result<(u3, u), E> {
    //     let gop_handle = boot_services.locate_protocol::<GraphicsOutput>()?;
    //     let mut gop = boot_services.open_protocol::<GraphicsOutput>(
    //         gop_handle,
    //         Some(Handle::null()), // For reading/querying
    //         uefi::OpenProtocolAttributes::GetProtocol,
    //     )?;
    //
    //     let mut best_mode_info = None;
    //     let mut max_pixels = 0;
    //
    //     // Get number of modes
    //     let mode_count = gop.mode().maxMode;
    //
    //     for mode_index in 0..mode_count {
    //         let mode = gop.query_mode(mode_index)?;
    //         let resolution = (mode.info().horizontal_resolution, mode.info().vertical_resolution);
    //         let pixels = resolution.0 * resolution.1;
    //
    //         // Find the largest resolution (or desired mode)
    //         if pixels > max_pixels {
    //             max_pixels = pixels;
    //             best_mode_info = Some(resolution);
    //         }
    //     }
    //
    //     best_mode_info.ok_or_else(|| uefi::Status::NOT_FOUND.into())
    // }

}