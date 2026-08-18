use crate::{hpvm_info, hpvm_log};
use core::fmt::Write;
use core::time::Duration;
use uefi::proto::console::text::Color;
use uefi::{StatusExt, Identify};
use uefi::boot::OpenProtocolAttributes;
use uefi::proto::console::pointer::Pointer;
pub use uefi_raw::protocol::console::AbsolutePointerProtocol;
use crate::{hpvm_warn, message, vdebug};

#[repr(transparent)]
pub struct AbsolutePointer(pub AbsolutePointerProtocol);
unsafe impl Identify for AbsolutePointer {
    const GUID: uefi::Guid = AbsolutePointerProtocol::GUID;
}
impl uefi::proto::Protocol for AbsolutePointer {}

#[allow(dead_code)]
impl AbsolutePointer {
    pub fn read_state(&mut self) -> uefi::Result<Option<uefi_raw::protocol::console::AbsolutePointerState>> {
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

    pub fn mode(&self) -> Option<&uefi_raw::protocol::console::AbsolutePointerMode> {
        if self.0.mode.is_null() {
            None
        } else {
            Some(unsafe { &*self.0.mode })
        }
    }
    
    pub fn reset(&mut self, extended: bool) -> uefi::Result {
        unsafe { (self.0.reset)(&mut self.0, extended.into()) }.to_result()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub x: i32,
    pub y: i32,
    pub visible: bool,
    pub left_button: bool,
    pub right_button: bool,
    pub prev_left_button: bool,
    pub prev_right_button: bool,
    pub poll_count: usize,
}

#[allow(dead_code)]
impl Cursor {
    pub fn new() -> Self {
        Cursor {
            x: 100,
            y: 100,
            visible: true,
            left_button: false,
            right_button: false,
            prev_left_button: false,
            prev_right_button: false,
            poll_count: 0,
        }
    }

    pub fn left_clicked(&self) -> bool {
        self.left_button && !self.prev_left_button
    }

    pub fn left_released(&self) -> bool {
        !self.left_button && self.prev_left_button
    }

    pub fn left_down(&self) -> bool {
        self.left_button
    }

    pub fn right_clicked(&self) -> bool {
        self.right_button && !self.prev_right_button
    }

    pub fn right_released(&self) -> bool {
        !self.right_button && self.prev_right_button
    }

    pub fn right_down(&self) -> bool {
        self.right_button
    }

    pub unsafe fn update_from_mouse(&mut self, screen_width: usize, screen_height: usize) -> bool {
        self.prev_left_button = self.left_button;
        self.prev_right_button = self.right_button;
        self.poll_count = self.poll_count.wrapping_add(1);

        let abs_res = self.try_update_absolute(screen_width, screen_height);
        let rel_res = self.try_update_relative(screen_width, screen_height);

        abs_res || rel_res
    }

    unsafe fn try_update_absolute(&mut self, screen_width: usize, screen_height: usize) -> bool {
        let mut any_updated = false;
        let mut handles_to_try = alloc::vec::Vec::new();
        if let Ok(handles) = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&AbsolutePointerProtocol::GUID)) {
            for handle in handles.iter() {
                handles_to_try.push(*handle);
            }
        }
        if let Ok(stdin_h) = uefi::boot::get_handle_for_protocol::<AbsolutePointer>() {
            if !handles_to_try.contains(&stdin_h) {
                handles_to_try.push(stdin_h);
            }
        }

        for handle in handles_to_try {
            if let Ok(mut mouse) = uefi::boot::open_protocol::<AbsolutePointer>(
                uefi::boot::OpenProtocolParams {
                    handle,
                    agent: uefi::boot::image_handle(),
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            ) {
                if let Ok(Some(state)) = mouse.read_state() {
                    let abs_x = state.current_x as f64;
                    let abs_y = state.current_y as f64;

                    let (min_x, max_x, min_y, max_y) = if let Some(mode) = mouse.mode() {
                        (
                            mode.absolute_min_x as f64,
                            mode.absolute_max_x as f64,
                            mode.absolute_min_y as f64,
                            mode.absolute_max_y as f64,
                        )
                    } else {
                        (0.0, 0.0, 0.0, 0.0)
                    };

                    let target_x = if max_x > min_x {
                        ((abs_x - min_x) * (screen_width.saturating_sub(1) as f64)) / (max_x - min_x)
                    } else if max_x > 0.0 {
                        (abs_x * (screen_width.saturating_sub(1) as f64)) / max_x
                    } else if abs_x <= screen_width as f64 {
                        abs_x
                    } else if abs_x <= 32767.0 {
                        (abs_x * (screen_width.saturating_sub(1) as f64)) / 32767.0
                    } else {
                        (abs_x * (screen_width.saturating_sub(1) as f64)) / 65535.0
                    };

                    let target_y = if max_y > min_y {
                        ((abs_y - min_y) * (screen_height.saturating_sub(1) as f64)) / (max_y - min_y)
                    } else if max_y > 0.0 {
                        (abs_y * (screen_height.saturating_sub(1) as f64)) / max_y
                    } else if abs_y <= screen_height as f64 {
                        abs_y
                    } else if abs_y <= 32767.0 {
                        (abs_y * (screen_height.saturating_sub(1) as f64)) / 32767.0
                    } else {
                        (abs_y * (screen_height.saturating_sub(1) as f64)) / 65535.0
                    };

                    self.x = (target_x as i32).clamp(0, screen_width.saturating_sub(1) as i32);
                    self.y = (target_y as i32).clamp(0, screen_height.saturating_sub(1) as i32);

                    self.left_button = (state.active_buttons & 0x1) != 0;
                    self.right_button = (state.active_buttons & 0x2) != 0;
                    any_updated = true;
                }
            }
        }
        any_updated
    }

    unsafe fn try_update_relative(&mut self, screen_width: usize, screen_height: usize) -> bool {
        let mut any_updated = false;
        let mut handles_to_try = alloc::vec::Vec::new();
        if let Ok(handles) = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&Pointer::GUID)) {
            for handle in handles.iter() {
                handles_to_try.push(*handle);
            }
        }
        if let Ok(stdin_h) = uefi::boot::get_handle_for_protocol::<Pointer>() {
            if !handles_to_try.contains(&stdin_h) {
                handles_to_try.push(stdin_h);
            }
        }

        for handle in handles_to_try {
            if let Ok(mut mouse) = uefi::boot::open_protocol::<Pointer>(
                uefi::boot::OpenProtocolParams {
                    handle,
                    agent: uefi::boot::image_handle(),
                    controller: None,
                },
                OpenProtocolAttributes::GetProtocol,
            ) {
                if let Ok(Some(state)) = mouse.read_state() {
                    let dx = state.relative_movement[0] as f32;
                    let dy = state.relative_movement[1] as f32;

                    if dx != 0.0 || dy != 0.0 {
                        // In UEFI SimplePointer spec:
                        // dx > 0 is Right, dx < 0 is Left.
                        // dy > 0 is Forward/Up, dy < 0 is Backward/Down.
                        // On screen: Y=0 is top, Y=height is bottom.
                        self.x = (self.x + dx as i32).clamp(0, screen_width.saturating_sub(1) as i32);
                        self.y = (self.y - dy as i32).clamp(0, screen_height.saturating_sub(1) as i32);
                        any_updated = true;
                    }

                    if state.button[0] || state.button[1] || any_updated {
                        self.left_button = state.button[0];
                        self.right_button = state.button[1];
                        any_updated = true;
                    }
                }
            }
        }
        any_updated
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

        crate::vdebug!("Mouse", "--- Mouse Debug ---");

        // Check relative pointer
        let handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&Pointer::GUID));
        match handles {
            Ok(h) => {
                crate::vdebug!("Mouse", "Found {} handles with Simple Pointer protocol", h.as_slice().len());
                for (i, handle) in h.as_slice().iter().enumerate() {
                    if let Ok(mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(*handle) {
                        let mode = mouse.mode();
                        crate::vdebug!("Mouse", "  [{}] Resolution: [{}, {}, {}], Buttons: [{}, {}]", 
                            i, mode.resolution[0], mode.resolution[1], mode.resolution[2],
                            mode.has_button[0], mode.has_button[1]);
                    } else {
                        crate::vdebug!("Mouse", "  [{}] Failed to open protocol", i);
                    }
                }
            }
            Err(_) => crate::vdebug!("Mouse", "No Simple Pointer protocol found"),
        }

        // Check absolute pointer
        let handles = uefi::boot::locate_handle_buffer(uefi::boot::SearchType::ByProtocol(&AbsolutePointerProtocol::GUID));
        match handles {
            Ok(h) => {
                crate::vdebug!("Mouse", "Found {} handles with Absolute Pointer protocol", h.as_slice().len());
                for (i, handle) in h.as_slice().iter().enumerate() {
                    // Use our local wrapper
                    if let Ok(mouse) = uefi::boot::open_protocol_exclusive::<AbsolutePointer>(*handle) {
                        if let Some(mode) = mouse.mode() {
                            crate::vdebug!("Mouse", "  [{}] Range: [{}..{}, {}..{}, {}..{}], Buttons: {:?}", 
                                i, mode.absolute_min_x, mode.absolute_max_x,
                                mode.absolute_min_y, mode.absolute_max_y,
                                mode.absolute_min_z, mode.absolute_max_z,
                                mode.attributes);
                        } else {
                            crate::vdebug!("Mouse", "  [{}] No mode reported", i);
                        }
                    } else {
                        crate::vdebug!("Mouse", "  [{}] Failed to open protocol", i);
                    }
                }
            }
            Err(_) => crate::vdebug!("Mouse", "No Absolute Pointer protocol found"),
        }

        crate::vdebug!("Mouse", "Polling data... Press any key to stop.");
        
        loop {
            // Poll relative
            if let Ok(handle) = uefi::boot::get_handle_for_protocol::<Pointer>() {
                if let Ok(mut mouse) = uefi::boot::open_protocol_exclusive::<Pointer>(handle) {
                    if let Ok(Some(state)) = mouse.read_state() {
                        if state.relative_movement[0] != 0 || state.relative_movement[1] != 0 || state.button[0] || state.button[1] {
                            crate::vdebug!("Mouse", "REL: dx={}, dy={}, btn=[{}, {}]", 
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
                        crate::vdebug!("Mouse", "ABS: x={}, y={}, btn={}", 
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
            let key = system::with_stdin(|i| i.read_key());
            if let Ok(Some(_)) = key {
                break;
            }

            uefi::boot::stall(Duration::from_millis(100)); // 100ms
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