use core::fmt::Write;
use core::ptr::null_mut;
use uefi::proto::console::text::Color;
// Import both protocols
use uefi::proto::console::pointer::Pointer;
use uefi::proto::console::pointer::Pointer as SimplePointer;
use uefi::proto::console::gop; // Assuming you're using uefi-services for simplicity
use uefi::proto::console::gop::{GraphicsOutput, ModeInfo};

pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
}

macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            use core::fmt::Write;
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, $start);
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
        })
    }
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
        if let Ok(handle) = uefi::boot::get_handle_for_protocol::<SimplePointer>() {
            if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<SimplePointer>(handle) {
                if let Ok(Some(state)) = mouse.read_state() {
                    // Add relative movement to existing position
                    self.x = self.x.saturating_add(state.relative_movement[0]);
                    self.y = self.y.saturating_add(state.relative_movement[1]);

                    // Clamp to the 120x40 grid
                    self.x = self.x.max(0).min(640);
                    self.y = self.y.max(0).min(480);

                    self.left_button = state.button[0];
                    self.right_button = state.button[1];
                }
            }
        }
    }

    pub fn render(&self, stdout: &mut uefi::proto::console::text::Output) {
        let cursor_char = if self.left_button { "✔" } else { "▶" };
        let _ = write!(stdout, "\x1b[{};{}H{}", self.y + 1, self.x + 1, cursor_char);
    }
}

#[allow(dead_code)]
use uefi::system;
use uefi_raw::protocol::console::GraphicsOutputModeInformation;
use uefi_raw::table::boot::BootServices;
use uefi_raw::table::system::SystemTable;

// Windows NT Color Palette
pub struct WinNTColors;

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

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Rect { x, y, width, height }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}

pub struct Graphics;

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

    pub fn get_cursor(cursor: Cursor) {
        Cursor::try_update_relative(cursor)
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