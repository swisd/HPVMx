#![allow(dead_code, non_snake_case, non_upper_case_globals)]

//! PyQt6/Qt6-Style UI Architecture & Widgets for PixelGraphics.
//!
//! Provides modern, clean, object-oriented/builder-style widget calling conventions
//! modeled after Qt6/PyQt6, rendering on top of `PixelGraphics`.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::f64::consts::PI;
use libm::{cos, floor, sin};
use crate::ui::pixel_graphics::{PixelGraphics, TreeViewNode};

/// 32-bit ARGB/RGB Color representation with helper utilities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QColor {
    pub argb: u32,
}

impl QColor {
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            argb: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32) | 0xFF000000,
        }
    }

    pub const fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            argb: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub const fn from_hex(hex: u32) -> Self {
        if hex <= 0x00FFFFFF {
            Self { argb: hex | 0xFF000000 }
        } else {
            Self { argb: hex }
        }
    }

    pub const fn raw(&self) -> u32 {
        self.argb
    }

    pub const fn rgb(&self) -> u32 {
        self.argb & 0x00FFFFFF
    }

    pub const fn r(&self) -> u8 {
        ((self.argb >> 16) & 0xFF) as u8
    }

    pub const fn g(&self) -> u8 {
        ((self.argb >> 8) & 0xFF) as u8
    }

    pub const fn b(&self) -> u8 {
        (self.argb & 0xFF) as u8
    }

    pub const fn a(&self) -> u8 {
        ((self.argb >> 24) & 0xFF) as u8
    }

    pub fn lighter(&self, factor: u16) -> Self {
        let f = factor as u32;
        let r = core::cmp::min((self.r() as u32 * f) / 100, 255) as u8;
        let g = core::cmp::min((self.g() as u32 * f) / 100, 255) as u8;
        let b = core::cmp::min((self.b() as u32 * f) / 100, 255) as u8;
        Self::from_rgba(r, g, b, self.a())
    }

    pub fn darker(&self, factor: u16) -> Self {
        let f = factor as u32;
        let r = (self.r() as u32 * 100 / f.max(1)).min(255) as u8;
        let g = (self.g() as u32 * 100 / f.max(1)).min(255) as u8;
        let b = (self.b() as u32 * 100 / f.max(1)).min(255) as u8;
        Self::from_rgba(r, g, b, self.a())
    }

    pub fn with_alpha(&self, alpha: u8) -> Self {
        Self::from_rgba(self.r(), self.g(), self.b(), alpha)
    }
}

impl From<u32> for QColor {
    fn from(val: u32) -> Self {
        Self::from_hex(val)
    }
}

impl Into<u32> for QColor {
    fn into(self) -> u32 {
        self.rgb()
    }
}

/// Predefined Standard & Modern Palette Colors.
pub mod Qt {
    use super::QColor;

    pub const Transparent: QColor = QColor::from_rgba(0, 0, 0, 0);
    pub const Black: QColor = QColor::from_rgb(0, 0, 0);
    pub const White: QColor = QColor::from_rgb(255, 255, 255);
    pub const DarkGray: QColor = QColor::from_rgb(128, 128, 128);
    pub const Gray: QColor = QColor::from_rgb(160, 160, 160);
    pub const LightGray: QColor = QColor::from_rgb(192, 192, 192);
    pub const Red: QColor = QColor::from_rgb(255, 0, 0);
    pub const Green: QColor = QColor::from_rgb(0, 255, 0);
    pub const Blue: QColor = QColor::from_rgb(0, 0, 255);
    pub const Cyan: QColor = QColor::from_rgb(0, 255, 255);
    pub const Magenta: QColor = QColor::from_rgb(255, 0, 255);
    pub const Yellow: QColor = QColor::from_rgb(255, 255, 0);
    pub const DarkRed: QColor = QColor::from_rgb(128, 0, 0);
    pub const DarkGreen: QColor = QColor::from_rgb(0, 128, 0);
    pub const DarkBlue: QColor = QColor::from_rgb(0, 0, 128);
    pub const DarkCyan: QColor = QColor::from_rgb(0, 128, 128);
    pub const DarkMagenta: QColor = QColor::from_rgb(128, 0, 128);
    pub const DarkYellow: QColor = QColor::from_rgb(128, 128, 0);

    // Modern Fusion / Qt6 Palette Constants
    pub const Primary: QColor = QColor::from_rgb(37, 99, 235);      // 0x2563EB Vibrant Blue
    pub const PrimaryDark: QColor = QColor::from_rgb(29, 78, 216);  // 0x1D4ED8
    pub const PrimaryHover: QColor = QColor::from_rgb(59, 130, 246); // 0x3B82F6
    pub const Success: QColor = QColor::from_rgb(22, 163, 74);      // 0x16A34A Emerald Green
    pub const Warning: QColor = QColor::from_rgb(234, 88, 12);      // 0xEA580C Amber
    pub const Danger: QColor = QColor::from_rgb(220, 38, 38);       // 0xDC2626 Crimson
    pub const Info: QColor = QColor::from_rgb(6, 182, 212);         // 0x06B6D4 Cyan

    pub const WindowBg: QColor = QColor::from_rgb(15, 18, 24);      // 0x0F1218
    pub const CardBg: QColor = QColor::from_rgb(24, 28, 38);        // 0x181C26
    pub const CardBgAlt: QColor = QColor::from_rgb(30, 35, 48);     // 0x1E2330
    pub const InputBg: QColor = QColor::from_rgb(17, 19, 24);       // 0x111318
    pub const Border: QColor = QColor::from_rgb(56, 65, 82);        // 0x384152
    pub const BorderLight: QColor = QColor::from_rgb(79, 89, 111);  // 0x4F596F
    pub const TextPrimary: QColor = QColor::from_rgb(248, 250, 252);// 0xF8FAFC
    pub const TextSecondary: QColor = QColor::from_rgb(203, 213, 225);// 0xCBD5E1
    pub const TextMuted: QColor = QColor::from_rgb(148, 163, 184);  // 0x94A3B8
}

/// 2D Point
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QPoint {
    pub x: i32,
    pub y: i32,
}

impl QPoint {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// 2D Size
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QSize {
    pub width: usize,
    pub height: usize,
}

impl QSize {
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

/// 2D Rectangle
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct QRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl QRect {
    pub const fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }

    pub const fn left(&self) -> usize { self.x }
    pub const fn top(&self) -> usize { self.y }
    pub const fn right(&self) -> usize { self.x + self.width }
    pub const fn bottom(&self) -> usize { self.y + self.height }

    pub fn contains(&self, px: usize, py: usize) -> bool {
        px >= self.x && px < self.x + self.width && py >= self.y && py < self.y + self.height
    }
}

/// Alignment flags
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QtAlignment {
    AlignLeft,
    AlignRight,
    AlignCenter,
    AlignTop,
    AlignBottom,
    AlignVCenter,
}

/// Orientation flags
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QtOrientation {
    Horizontal,
    Vertical,
}

/// Checkbox states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QtCheckState {
    Unchecked,
    PartiallyChecked,
    Checked,
}

/// Widget State
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QtWidgetState {
    Normal,
    Hover,
    Pressed,
    Disabled,
    Focused,
}

// =========================================================================
// PyQt6 / Qt6 Widgets Builders
// =========================================================================

/// QPushButton - Push Button Widget with Qt6 Fusion styling
#[derive(Clone, Debug)]
pub struct QPushButton<'a> {
    pub rect: QRect,
    pub text: &'a str,
    pub focused: bool,
    pub enabled: bool,
    pub checkable: bool,
    pub checked: bool,
    pub flat: bool,
    pub icon: Option<&'a [u32]>,
    pub icon_size: usize,
    pub custom_bg: Option<QColor>,
    pub custom_fg: Option<QColor>,
}

impl<'a> QPushButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            rect: QRect::new(0, 0, 100, 26),
            text,
            focused: false,
            enabled: true,
            checkable: false,
            checked: false,
            flat: false,
            icon: None,
            icon_size: 16,
            custom_bg: None,
            custom_fg: None,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn geometry(self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.setGeometry(x, y, width, height)
    }

    pub fn setText(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn setFocus(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    pub fn focused(self, focused: bool) -> Self {
        self.setFocus(focused)
    }

    pub fn setEnabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn setCheckable(mut self, checkable: bool) -> Self {
        self.checkable = checkable;
        self
    }

    pub fn setChecked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn setFlat(mut self, flat: bool) -> Self {
        self.flat = flat;
        self
    }

    pub fn setIcon(mut self, icon: &'a [u32], size: usize) -> Self {
        self.icon = Some(icon);
        self.icon_size = size;
        self
    }

    pub fn setStyleSheet(mut self, bg: QColor, fg: QColor) -> Self {
        self.custom_bg = Some(bg);
        self.custom_fg = Some(fg);
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        let is_active = self.focused || (self.checkable && self.checked);
        let bg_top = if !self.enabled {
            0x2A2D34
        } else if let Some(bg) = self.custom_bg {
            bg.rgb()
        } else if is_active {
            0x2563EB // Primary Blue
        } else {
            0x323846 // Slate dark
        };

        let bg_bottom = if !self.enabled {
            0x22242B
        } else if let Some(bg) = self.custom_bg {
            bg.darker(115).rgb()
        } else if is_active {
            0x1D4ED8 // Primary Dark
        } else {
            0x242832
        };

        let border_color = if !self.enabled {
            0x3A3E48
        } else if is_active {
            0x60A5FA // Highlight blue
        } else {
            0x454D5F
        };

        let text_color = if !self.enabled {
            0x6B7280
        } else if let Some(fg) = self.custom_fg {
            fg.rgb()
        } else {
            0xFFFFFF
        };

        if !self.flat {
            // Fill gradient with rounded corners
            pg.fill_gradient_v(x, y, w, h, bg_top, bg_bottom);
            pg.draw_rounded_rect_outline(x, y, w, h, 3, border_color);

            // Subtle 3D top highlight line for tactile depth
            if self.enabled && h > 4 {
                let highlight_color = if is_active { 0x60A5FA } else { 0x4E576B };
                pg.draw_line(x + 2, y + 1, x + w.saturating_sub(3), y + 1, highlight_color);
            }
        }

        // Draw Icon if present
        let mut text_offset_x = 0;
        if let Some(icon_data) = self.icon {
            let icon_x = x + 6;
            let icon_y = y + (h.saturating_sub(self.icon_size)) / 2;
            pg.draw_icon(icon_x, icon_y, self.icon_size, self.icon_size, icon_data);
            text_offset_x = self.icon_size + 6;
        }

        // Center Text
        let text_w = self.text.len() * 8;
        let avail_w = w.saturating_sub(text_offset_x);
        let text_x = if avail_w > text_w {
            x + text_offset_x + (avail_w - text_w) / 2
        } else {
            x + text_offset_x + 4
        };
        let text_y = if h > 16 { y + (h - 16) / 2 } else { y + 2 };
        pg.draw_text(text_x, text_y, self.text, text_color);
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QCheckBox - Check Box Widget with Qt6 vector checkmark
#[derive(Clone, Debug)]
pub struct QCheckBox<'a> {
    pub point: QPoint,
    pub text: &'a str,
    pub check_state: QtCheckState,
    pub enabled: bool,
    pub blocked: bool,
    pub text_color: Option<QColor>,
}

impl<'a> QCheckBox<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            point: QPoint::new(0, 0),
            text,
            check_state: QtCheckState::Unchecked,
            enabled: true,
            blocked: false,
            text_color: None,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize) -> Self {
        self.point = QPoint::new(x as i32, y as i32);
        self
    }

    pub fn setChecked(mut self, checked: bool) -> Self {
        self.check_state = if checked { QtCheckState::Checked } else { QtCheckState::Unchecked };
        self
    }

    pub fn setCheckState(mut self, state: QtCheckState) -> Self {
        self.check_state = state;
        self
    }

    pub fn setEnabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn setBlocked(mut self, blocked: bool) -> Self {
        self.blocked = blocked;
        self
    }

    pub fn setTextColor(mut self, color: QColor) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y) = (self.point.x as usize, self.point.y as usize);
        let size = 14usize;

        let bg_box = if !self.enabled {
            0x242830
        } else if self.blocked {
            0xDC2626 // Crimson blocked
        } else if self.check_state == QtCheckState::Checked {
            0x2563EB // Primary Blue
        } else if self.check_state == QtCheckState::PartiallyChecked {
            0x1E40AF // Deep Blue
        } else {
            0x181B22 // Dark Input
        };

        let border_color = if !self.enabled {
            0x3A404D
        } else if self.blocked {
            0xEF4444
        } else if self.check_state != QtCheckState::Unchecked {
            0x60A5FA
        } else {
            0x4B5568
        };

        let text_color = if !self.enabled {
            0x6B7280
        } else if let Some(c) = self.text_color {
            c.rgb()
        } else {
            0xF8FAFC
        };

        // Draw Checkbox Box Frame
        pg.fill_rounded_rect(x, y, size, size, 2, bg_box);
        pg.draw_rounded_rect_outline(x, y, size, size, 2, border_color);

        // Vector Checkmark / Icon
        if self.blocked {
            // Crisp White X
            pg.draw_line_adv(x + 3, y + 3, x + 10, y + 10, 0xFFFFFF, 2, 0xFFFFFFFF);
            pg.draw_line_adv(x + 10, y + 3, x + 3, y + 10, 0xFFFFFF, 2, 0xFFFFFFFF);
        } else if self.check_state == QtCheckState::Checked {
            // Crisp Vector Tick Mark (Short tick + long tick)
            pg.draw_line_adv(x + 3, y + 7, x + 6, y + 10, 0xFFFFFF, 2, 0xFFFFFFFF);
            pg.draw_line_adv(x + 6, y + 10, x + 11, y + 4, 0xFFFFFF, 2, 0xFFFFFFFF);
        } else if self.check_state == QtCheckState::PartiallyChecked {
            // Indeterminate Dash in middle
            pg.fill_rect(x + 3, y + 5, 8, 4, 0xFFFFFF);
        }

        // Draw Label Text
        if !self.text.is_empty() {
            pg.draw_text(x + size + 6, y - 1, self.text, text_color);
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QRadioButton - Radio Button Widget with smooth circular vector rendering
#[derive(Clone, Debug)]
pub struct QRadioButton<'a> {
    pub point: QPoint,
    pub text: &'a str,
    pub checked: bool,
    pub enabled: bool,
    pub text_color: Option<QColor>,
}

impl<'a> QRadioButton<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            point: QPoint::new(0, 0),
            text,
            checked: false,
            enabled: true,
            text_color: None,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize) -> Self {
        self.point = QPoint::new(x as i32, y as i32);
        self
    }

    pub fn setChecked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn setEnabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn setTextColor(mut self, color: QColor) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y) = (self.point.x as usize, self.point.y as usize);
        let cx = x + 7;
        let cy = y + 7;

        let bg_color = if !self.enabled { 0x242830 } else { 0x181B22 };
        let border_color = if !self.enabled {
            0x3A404D
        } else if self.checked {
            0x2563EB // Primary Blue
        } else {
            0x4B5568
        };
        let dot_color = if !self.enabled { 0x6B7280 } else { 0x2563EB };
        let text_color = if !self.enabled {
            0x6B7280
        } else if let Some(c) = self.text_color {
            c.rgb()
        } else {
            0xF8FAFC
        };

        // Draw Outer Circle
        pg.fill_circle(cx, cy, 6, bg_color);
        pg.draw_circle(cx, cy, 6, border_color);

        // Draw Center Pip when checked
        if self.checked {
            pg.fill_circle(cx, cy, 3, dot_color);
        }

        // Draw Label Text
        if !self.text.is_empty() {
            pg.draw_text(x + 18, y - 1, self.text, text_color);
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QProgressBar - Modern Progress Bar with gloss highlight and percentage text
#[derive(Clone, Debug)]
pub struct QProgressBar {
    pub rect: QRect,
    pub min: usize,
    pub max: usize,
    pub value: usize,
    pub color: QColor,
    pub text_visible: bool,
}

impl QProgressBar {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 150, 20),
            min: 0,
            max: 100,
            value: 0,
            color: Qt::Primary,
            text_visible: true,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setRange(mut self, min: usize, max: usize) -> Self {
        self.min = min;
        self.max = max.max(min + 1);
        self
    }

    pub fn setValue(mut self, value: usize) -> Self {
        self.value = value;
        self
    }

    pub fn setColor(mut self, color: QColor) -> Self {
        self.color = color;
        self
    }

    pub fn setTextVisible(mut self, visible: bool) -> Self {
        self.text_visible = visible;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        let range = self.max.saturating_sub(self.min).max(1);
        let clamped_val = self.value.saturating_sub(self.min).min(range);
        let pct = (clamped_val * 100) / range;

        // Draw Recessed Track
        pg.fill_rounded_rect(x, y, w, h, 2, 0x12151C);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x384152);

        // Fill Progress Segment
        let inner_w = w.saturating_sub(4);
        let inner_h = h.saturating_sub(4);
        let fill_w = (inner_w * clamped_val) / range;

        if fill_w > 0 && inner_h > 0 {
            let bar_color = self.color.rgb();
            let bar_top = self.color.lighter(115).rgb();
            pg.fill_gradient_v(x + 2, y + 2, fill_w, inner_h, bar_top, bar_color);

            // Top gloss highlight
            if inner_h >= 6 {
                pg.draw_line(x + 2, y + 2, x + 2 + fill_w.saturating_sub(1), y + 2, self.color.lighter(130).rgb());
            }
        }

        // Percentage Text
        if self.text_visible && h >= 16 && w >= 50 {
            let pct_text = alloc::format!("{}%", pct);
            let tw = pct_text.len() * 8;
            let tx = x + (w.saturating_sub(tw)) / 2;
            let ty = y + (h.saturating_sub(16)) / 2;
            // Shadow
            pg.draw_text(tx + 1, ty + 1, &pct_text, 0x000000);
            pg.draw_text(tx, ty, &pct_text, 0xFFFFFF);
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QSlider - Horizontal/Vertical Slider with smooth track and beveled knob handle
#[derive(Clone, Debug)]
pub struct QSlider {
    pub rect: QRect,
    pub orientation: QtOrientation,
    pub min: usize,
    pub max: usize,
    pub value: usize,
}

impl QSlider {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 150, 20),
            orientation: QtOrientation::Horizontal,
            min: 0,
            max: 100,
            value: 0,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, length: usize, thickness: usize) -> Self {
        self.rect = QRect::new(x, y, length, thickness);
        self
    }

    pub fn setOrientation(mut self, orientation: QtOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn setRange(mut self, min: usize, max: usize) -> Self {
        self.min = min;
        self.max = max.max(min + 1);
        self
    }

    pub fn setValue(mut self, value: usize) -> Self {
        self.value = value;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        let range = self.max.saturating_sub(self.min).max(1);
        let clamped = self.value.saturating_sub(self.min).min(range);

        match self.orientation {
            QtOrientation::Horizontal => {
                let track_y = y + h / 2 - 2;
                let track_h = 4;
                // Groove
                pg.fill_rounded_rect(x, track_y, w, track_h, 2, 0x141820);
                pg.draw_rounded_rect_outline(x, track_y, w, track_h, 2, 0x384152);

                // Active highlight
                let active_w = (w * clamped) / range;
                if active_w > 0 {
                    pg.fill_rounded_rect(x, track_y, active_w, track_h, 2, 0x2563EB);
                }

                // Handle Thumb Knob
                let hx = x + active_w;
                let hy = y + h / 2;
                pg.fill_circle(hx, hy, 7, 0x242832);
                pg.draw_circle(hx, hy, 7, 0x60A5FA);
                pg.fill_circle(hx, hy, 3, 0x60A5FA);
            }
            QtOrientation::Vertical => {
                let track_x = x + w / 2 - 2;
                let track_w = 4;
                // Groove
                pg.fill_rounded_rect(track_x, y, track_w, h, 2, 0x141820);
                pg.draw_rounded_rect_outline(track_x, y, track_w, h, 2, 0x384152);

                // Active highlight from bottom up
                let active_h = (h * clamped) / range;
                if active_h > 0 {
                    pg.fill_rounded_rect(track_x, y + h - active_h, track_w, active_h, 2, 0x2563EB);
                }

                // Handle Thumb Knob
                let hx = x + w / 2;
                let hy = y + h - active_h;
                pg.fill_circle(hx, hy, 7, 0x242832);
                pg.draw_circle(hx, hy, 7, 0x60A5FA);
                pg.fill_circle(hx, hy, 3, 0x60A5FA);
            }
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QSpinBox - Integer SpinBox Widget with vector up/down buttons
#[derive(Clone, Debug)]
pub struct QSpinBox<'a> {
    pub rect: QRect,
    pub min: i32,
    pub max: i32,
    pub value: i32,
    pub prefix: &'a str,
    pub suffix: &'a str,
}

impl<'a> QSpinBox<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 70, 24),
            min: 0,
            max: 100,
            value: 0,
            prefix: "",
            suffix: "",
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setRange(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn setValue(mut self, value: i32) -> Self {
        self.value = value.clamp(self.min, self.max);
        self
    }

    pub fn setPrefix(mut self, prefix: &'a str) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn setSuffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        let btn_w = 18usize.min(w / 3);

        // Inset text box
        pg.fill_rounded_rect(x, y, w, h, 2, 0x111318);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x3E4758);

        // Dividers
        let btn_x = x + w - btn_w;
        pg.draw_line(btn_x, y, btn_x, y + h, 0x3E4758);
        pg.draw_line(btn_x, y + h / 2, x + w, y + h / 2, 0x3E4758);

        // Button backgrounds
        pg.fill_rect(btn_x + 1, y + 1, btn_w - 2, h / 2 - 1, 0x252B37);
        pg.fill_rect(btn_x + 1, y + h / 2 + 1, btn_w - 2, h / 2 - 2, 0x252B37);

        // Vector Up Arrow (▲)
        let mid_x = btn_x + btn_w / 2;
        let up_y = y + h / 4;
        let up_poly = [
            (mid_x, up_y.saturating_sub(3)),
            (mid_x.saturating_sub(4), up_y + 2),
            (mid_x + 4, up_y + 2),
        ];
        pg.polygon_fill(&up_poly, 0xCBD5E1);

        // Vector Down Arrow (▼)
        let down_y = y + (h * 3) / 4;
        let down_poly = [
            (mid_x.saturating_sub(4), down_y.saturating_sub(2)),
            (mid_x + 4, down_y.saturating_sub(2)),
            (mid_x, down_y + 3),
        ];
        pg.polygon_fill(&down_poly, 0xCBD5E1);

        // Value text
        let mut display = alloc::format!("{}{}", self.prefix, self.value);
        if !self.suffix.is_empty() {
            display.push_str(self.suffix);
        }
        let ty = y + (h.saturating_sub(16)) / 2;
        pg.draw_text(x + 6, ty, &display, 0xFFFFFF);
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QDoubleSpinBox - Floating Point SpinBox Widget
#[derive(Clone, Debug)]
pub struct QDoubleSpinBox<'a> {
    pub rect: QRect,
    pub min: f64,
    pub max: f64,
    pub value: f64,
    pub decimals: usize,
    pub suffix: &'a str,
}

impl<'a> QDoubleSpinBox<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 80, 24),
            min: 0.0,
            max: 100.0,
            value: 0.0,
            decimals: 2,
            suffix: "",
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setRange(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn setValue(mut self, value: f64) -> Self {
        self.value = if value < self.min { self.min } else if value > self.max { self.max } else { value };
        self
    }

    pub fn setDecimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn setSuffix(mut self, suffix: &'a str) -> Self {
        self.suffix = suffix;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        let btn_w = 18usize.min(w / 3);

        // Format float accurately
        let mut s = String::new();
        let int_part = self.value as i64;
        s.push_str(&int_part.to_string());
        if self.decimals > 0 {
            s.push('.');
            let mut frac = self.value.abs() - floor(self.value.abs());
            for _ in 0..self.decimals {
                frac *= 10.0;
                let digit = (frac as i64) % 10;
                s.push_str(&digit.to_string());
            }
        }
        if !self.suffix.is_empty() {
            s.push_str(self.suffix);
        }

        // Inset text box
        pg.fill_rounded_rect(x, y, w, h, 2, 0x111318);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x3E4758);

        // Dividers
        let btn_x = x + w - btn_w;
        pg.draw_line(btn_x, y, btn_x, y + h, 0x3E4758);
        pg.draw_line(btn_x, y + h / 2, x + w, y + h / 2, 0x3E4758);

        // Button backgrounds
        pg.fill_rect(btn_x + 1, y + 1, btn_w - 2, h / 2 - 1, 0x252B37);
        pg.fill_rect(btn_x + 1, y + h / 2 + 1, btn_w - 2, h / 2 - 2, 0x252B37);

        // Up Arrow (▲)
        let mid_x = btn_x + btn_w / 2;
        let up_y = y + h / 4;
        let up_poly = [
            (mid_x, up_y.saturating_sub(3)),
            (mid_x.saturating_sub(4), up_y + 2),
            (mid_x + 4, up_y + 2),
        ];
        pg.polygon_fill(&up_poly, 0xCBD5E1);

        // Down Arrow (▼)
        let down_y = y + (h * 3) / 4;
        let down_poly = [
            (mid_x.saturating_sub(4), down_y.saturating_sub(2)),
            (mid_x + 4, down_y.saturating_sub(2)),
            (mid_x, down_y + 3),
        ];
        pg.polygon_fill(&down_poly, 0xCBD5E1);

        let ty = y + (h.saturating_sub(16)) / 2;
        pg.draw_text(x + 6, ty, &s, 0xFFFFFF);
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QDial - Rotary Dial Widget with notch ticks and indicator pointer
#[derive(Clone, Debug)]
pub struct QDial {
    pub point: QPoint,
    pub radius: usize,
    pub min: usize,
    pub max: usize,
    pub value: usize,
    pub notches_visible: bool,
}

impl QDial {
    pub fn new() -> Self {
        Self {
            point: QPoint::new(0, 0),
            radius: 16,
            min: 0,
            max: 100,
            value: 0,
            notches_visible: true,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, radius: usize) -> Self {
        self.point = QPoint::new(x as i32, y as i32);
        self.radius = radius;
        self
    }

    pub fn setRange(mut self, min: usize, max: usize) -> Self {
        self.min = min;
        self.max = max.max(min + 1);
        self
    }

    pub fn setValue(mut self, value: usize) -> Self {
        self.value = value;
        self
    }

    pub fn setNotchesVisible(mut self, visible: bool) -> Self {
        self.notches_visible = visible;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y) = (self.point.x as usize, self.point.y as usize);
        let r = self.radius;
        let cx = x + r;
        let cy = y + r;
        let range = self.max.saturating_sub(self.min).max(1);
        let fraction = (self.value.saturating_sub(self.min).min(range)) as f64 / range as f64;

        let start_angle = 0.75 * PI;
        let sweep = 1.5 * PI;
        let needle_angle = start_angle + fraction * sweep;

        // 1. Draw Perimeter Notches (Ticks)
        if self.notches_visible && r >= 10 {
            let ticks = 11;
            let current_tick = (fraction * (ticks - 1) as f64) as usize;
            for i in 0..ticks {
                let a = start_angle + (i as f64 / (ticks - 1) as f64) * sweep;
                let r_in = r as f64 + 1.0;
                let r_out = r as f64 + 4.0;
                let tx1 = (cx as f64 + r_in * cos(a)) as usize;
                let ty1 = (cy as f64 + r_in * sin(a)) as usize;
                let tx2 = (cx as f64 + r_out * cos(a)) as usize;
                let ty2 = (cy as f64 + r_out * sin(a)) as usize;
                let tick_color = if i <= current_tick { 0x38BDF8 } else { 0x475569 };
                pg.draw_line(tx1, ty1, tx2, ty2, tick_color);
            }
        }

        // 2. Draw Dial Body
        let inner_r = r.saturating_sub(2).max(4);
        pg.fill_circle(cx, cy, inner_r, 0x1E232E);
        pg.draw_circle(cx, cy, inner_r, 0x475569);

        // 3. Draw Needle Pointer
        let tip_r = inner_r as f64 - 2.0;
        let tx = (cx as f64 + tip_r * cos(needle_angle)) as usize;
        let ty = (cy as f64 + tip_r * sin(needle_angle)) as usize;
        pg.draw_line_adv(cx, cy, tx, ty, 0x38BDF8, 2, 0xFFFFFFFF);

        // 4. Center Cap
        pg.fill_circle(cx, cy, 3, 0x334155);
        pg.draw_circle(cx, cy, 3, 0x64748B);
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QLCDNumber - Digital 7-Segment Multimeter/Dashboard Display Widget
#[derive(Clone, Debug)]
pub struct QLCDNumber<'a> {
    pub rect: QRect,
    pub text: &'a str,
    pub color: QColor,
    pub segment_style: u8,
}

impl<'a> QLCDNumber<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 140, 32),
            text: "0",
            color: QColor::from_rgb(0, 255, 68), // Green glow
            segment_style: 0,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn display(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }

    pub fn setColor(mut self, color: QColor) -> Self {
        self.color = color;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        // Recessed Dark Glass Plate
        pg.fill_rounded_rect(x, y, w, h, 3, 0x0A120A);
        pg.draw_rounded_rect_outline(x, y, w, h, 3, 0x1E2B1E);

        let lit_color = self.color.rgb();
        let ghost_color = 0x0C1F0C; // Dim ghost unlit segment

        // Segment geometry per digit
        let margin_x = 6usize;
        let margin_y = 5usize;
        let avail_w = w.saturating_sub(margin_x * 2);
        let avail_h = h.saturating_sub(margin_y * 2);

        let digit_count = self.text.len().max(1);
        let char_w = (avail_w / digit_count).max(6).min(avail_h * 2 / 3);
        let spacing = 3usize;
        let digit_w = char_w.saturating_sub(spacing).max(4);
        let digit_h = avail_h;
        let t = (digit_h / 8).max(2); // Bar thickness

        for (i, c) in self.text.chars().enumerate() {
            let dx = x + margin_x + i * char_w;
            let dy = y + margin_y;

            if dx + digit_w > x + w { break; }

            if c == '.' {
                // Decimal point dot
                let dot_sz = t.max(2);
                let dot_x = dx + 1;
                let dot_y = dy + digit_h - dot_sz - 1;
                pg.fill_rect(dot_x, dot_y, dot_sz, dot_sz, lit_color);
                continue;
            } else if c == ':' {
                // Colon dots
                let dot_sz = t.max(2);
                let dot_x = dx + digit_w / 2;
                let dot_y1 = dy + digit_h / 3;
                let dot_y2 = dy + (digit_h * 2) / 3;
                pg.fill_rect(dot_x, dot_y1, dot_sz, dot_sz, lit_color);
                pg.fill_rect(dot_x, dot_y2, dot_sz, dot_sz, lit_color);
                continue;
            }

            // 7-segment bitmask: bit0=A(top), bit1=B(top-right), bit2=C(bot-right),
            // bit3=D(bot), bit4=E(bot-left), bit5=F(top-left), bit6=G(mid)
            let mask: u8 = match c {
                '0' => 0b00111111,
                '1' => 0b00000110,
                '2' => 0b01011011,
                '3' => 0b01001111,
                '4' => 0b01100110,
                '5' => 0b01101101,
                '6' => 0b01111101,
                '7' => 0b00000111,
                '8' => 0b01111111,
                '9' => 0b01101111,
                '-' => 0b01000000,
                'A' | 'a' => 0b01110111,
                'B' | 'b' => 0b01111100,
                'C' | 'c' => 0b00111001,
                'D' | 'd' => 0b01011110,
                'E' | 'e' => 0b01111001,
                'F' | 'f' => 0b01110001,
                ' ' => 0b00000000,
                _ => 0b01000000,
            };

            let seg_h_v = (digit_h.saturating_sub(3 * t)) / 2;

            // Segment A (Top horizontal)
            let col_a = if (mask & (1 << 0)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx + t, dy, digit_w.saturating_sub(2 * t), t, col_a);

            // Segment B (Top right vertical)
            let col_b = if (mask & (1 << 1)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx + digit_w.saturating_sub(t), dy + t, t, seg_h_v, col_b);

            // Segment C (Bottom right vertical)
            let col_c = if (mask & (1 << 2)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx + digit_w.saturating_sub(t), dy + digit_h / 2 + t / 2, t, seg_h_v, col_c);

            // Segment D (Bottom horizontal)
            let col_d = if (mask & (1 << 3)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx + t, dy + digit_h.saturating_sub(t), digit_w.saturating_sub(2 * t), t, col_d);

            // Segment E (Bottom left vertical)
            let col_e = if (mask & (1 << 4)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx, dy + digit_h / 2 + t / 2, t, seg_h_v, col_e);

            // Segment F (Top left vertical)
            let col_f = if (mask & (1 << 5)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx, dy + t, t, seg_h_v, col_f);

            // Segment G (Middle horizontal)
            let col_g = if (mask & (1 << 6)) != 0 { lit_color } else { ghost_color };
            pg.fill_rect(dx + t, dy + digit_h / 2 - t / 2, digit_w.saturating_sub(2 * t), t, col_g);
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QListView - Modern List View Widget with zebra stripes and accent highlights
#[derive(Clone, Debug)]
pub struct QListView<'a> {
    pub rect: QRect,
    pub items: &'a [&'a str],
    pub selected_index: Option<usize>,
    pub item_height: usize,
}

impl<'a> QListView<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 200, 100),
            items: &[],
            selected_index: None,
            item_height: 22,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setItems(mut self, items: &'a [&'a str]) -> Self {
        self.items = items;
        self
    }

    pub fn setCurrentIndex(mut self, index: Option<usize>) -> Self {
        self.selected_index = index;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        pg.fill_rounded_rect(x, y, w, h, 2, 0x141820);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x333E50);

        let row_h = self.item_height;
        let mut curr_y = y + 2;

        for (i, &item) in self.items.iter().enumerate() {
            if curr_y + row_h > y + h - 2 { break; }

            let is_selected = Some(i) == self.selected_index;
            let row_bg = if is_selected {
                0x1D4ED8 // Primary Blue
            } else if i % 2 == 1 {
                0x181D26 // Zebra alternate
            } else {
                0x141820
            };

            pg.fill_rect(x + 2, curr_y, w.saturating_sub(4), row_h, row_bg);

            if is_selected {
                // Left accent bar
                pg.fill_rect(x + 2, curr_y, 3, row_h, 0x60A5FA);
            }

            let text_color = if is_selected { 0xFFFFFF } else { 0xE2E8F0 };
            pg.draw_text(x + 10, curr_y + 3, item, text_color);
            curr_y += row_h;
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QTableView - Table View Widget with column headers and zebra striping
#[derive(Clone, Debug)]
pub struct QTableView<'a> {
    pub rect: QRect,
    pub headers: &'a [&'a str],
    pub rows: Vec<&'a [&'a str]>,
    pub row_height: usize,
}

impl<'a> QTableView<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 250, 100),
            headers: &[],
            rows: Vec::new(),
            row_height: 20,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setHorizontalHeaderLabels(mut self, headers: &'a [&'a str]) -> Self {
        self.headers = headers;
        self
    }

    pub fn setRows(mut self, rows: Vec<&'a [&'a str]>) -> Self {
        self.rows = rows;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        pg.fill_rounded_rect(x, y, w, h, 2, 0x141720);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x333E50);

        let col_count = self.headers.len().max(1);
        let col_w = w / col_count;
        let row_h = self.row_height;

        // Header
        pg.fill_rect(x + 1, y + 1, w.saturating_sub(2), row_h, 0x222938);
        for (c_idx, &hdr) in self.headers.iter().enumerate() {
            let cx = x + c_idx * col_w;
            pg.draw_text(cx + 6, y + 2, hdr, 0x94A3B8);
            if c_idx > 0 {
                pg.draw_line(cx, y + 1, cx, y + h.saturating_sub(1), 0x333E50);
            }
        }
        pg.draw_line(x + 1, y + row_h, x + w.saturating_sub(1), y + row_h, 0x333E50);

        // Rows
        for (r_idx, row) in self.rows.iter().enumerate() {
            let ry = y + row_h + r_idx * row_h;
            if ry + row_h > y + h - 1 { break; }

            let bg = if r_idx % 2 == 1 { 0x1A1F2B } else { 0x141720 };
            pg.fill_rect(x + 1, ry + 1, w.saturating_sub(2), row_h.saturating_sub(1), bg);

            for (c_idx, &cell) in row.iter().enumerate() {
                if c_idx >= col_count { break; }
                let cx = x + c_idx * col_w;
                pg.draw_text(cx + 6, ry + 2, cell, 0xF1F5F9);
            }
            pg.draw_line(x + 1, ry + row_h, x + w.saturating_sub(1), ry + row_h, 0x242C3B);
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QTreeView - Hierarchical Tree View with branches and chevron expand/collapse icons
#[derive(Clone, Debug)]
pub struct QTreeView<'a> {
    pub rect: QRect,
    pub root: Option<&'a TreeViewNode<'a>>,
    pub icon: Option<&'a [u32]>,
}

impl<'a> QTreeView<'a> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 200, 150),
            root: None,
            icon: None,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setRoot(mut self, root: &'a TreeViewNode<'a>) -> Self {
        self.root = Some(root);
        self
    }

    pub fn setIcon(mut self, icon: &'a [u32]) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        if w == 0 || h == 0 { return; }

        pg.fill_rounded_rect(x, y, w, h, 2, 0x141820);
        pg.draw_rounded_rect_outline(x, y, w, h, 2, 0x333E50);

        if let Some(root) = self.root {
            if let Some(icon) = self.icon {
                pg.draw_tree_view_icon(x, y, w, h, root, icon);
            } else {
                pg.draw_tree_view(x, y, w, h, root);
            }
        }
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QLineSeries / QChart - High Resolution Chart with reference grid and area fill
#[derive(Clone, Debug)]
pub struct QLineSeries<'a, T: Into<u64> + Copy> {
    pub rect: QRect,
    pub data: &'a [T],
    pub max_value: u64,
    pub color: QColor,
    pub window_size: usize,
    pub grid_visible: bool,
    pub area_fill: bool,
}

impl<'a, T: Into<u64> + Copy> QLineSeries<'a, T> {
    pub fn new() -> Self {
        Self {
            rect: QRect::new(0, 0, 200, 60),
            data: &[],
            max_value: 100,
            color: Qt::Primary,
            window_size: 60,
            grid_visible: true,
            area_fill: true,
        }
    }

    pub fn setGeometry(mut self, x: usize, y: usize, width: usize, height: usize) -> Self {
        self.rect = QRect::new(x, y, width, height);
        self
    }

    pub fn setData(mut self, data: &'a [T]) -> Self {
        self.data = data;
        self
    }

    pub fn setMaxValue(mut self, max: u64) -> Self {
        self.max_value = max;
        self
    }

    pub fn setColor(mut self, color: QColor) -> Self {
        self.color = color;
        self
    }

    pub fn setWindowSize(mut self, len: usize) -> Self {
        self.window_size = len;
        self
    }

    pub fn setGridVisible(mut self, visible: bool) -> Self {
        self.grid_visible = visible;
        self
    }

    pub fn setAreaFill(mut self, fill: bool) -> Self {
        self.area_fill = fill;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics) {
        let (x, y, w, h) = (self.rect.x, self.rect.y, self.rect.width, self.rect.height);
        pg.draw_line_graph(x, y, w, h, self.data, self.max_value, self.color.rgb(), self.window_size);
    }

    pub fn draw(&self, pg: &mut PixelGraphics) {
        self.render(pg);
    }
}

/// QToast / QToolTip - Floating Acrylic Notification Toast
#[derive(Clone, Debug)]
pub struct QToast<'a> {
    pub text: &'a str,
    pub y_offset: usize,
    pub color: QColor,
}

impl<'a> QToast<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            y_offset: 0,
            color: Qt::Success,
        }
    }

    pub fn setOffset(mut self, y: usize) -> Self {
        self.y_offset = y;
        self
    }

    pub fn setColor(mut self, color: QColor) -> Self {
        self.color = color;
        self
    }

    pub fn render(&self, pg: &mut PixelGraphics, duration_frames: &mut usize) {
        pg.draw_toast(self.text, duration_frames, self.y_offset);
    }
}

/// QPainter - Painter Canvas Wrapper for PixelGraphics
pub struct QPainter<'a> {
    pub pg: &'a mut PixelGraphics,
    pub pen_color: QColor,
    pub brush_color: QColor,
}

impl<'a> QPainter<'a> {
    pub fn new(pg: &'a mut PixelGraphics) -> Self {
        Self {
            pg,
            pen_color: Qt::White,
            brush_color: Qt::Transparent,
        }
    }

    pub fn setPen(&mut self, color: QColor) {
        self.pen_color = color;
    }

    pub fn setBrush(&mut self, color: QColor) {
        self.brush_color = color;
    }

    pub fn drawRect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        self.pg.draw_rect_outline(x, y, w, h, self.pen_color.rgb());
    }

    pub fn fillRect(&mut self, x: usize, y: usize, w: usize, h: usize) {
        self.pg.fill_rect(x, y, w, h, self.brush_color.rgb());
    }

    pub fn drawRoundedRect(&mut self, x: usize, y: usize, w: usize, h: usize, radius: usize) {
        self.pg.draw_rounded_rect_outline(x, y, w, h, radius, self.pen_color.rgb());
    }

    pub fn fillRoundedRect(&mut self, x: usize, y: usize, w: usize, h: usize, radius: usize) {
        self.pg.fill_rounded_rect(x, y, w, h, radius, self.brush_color.rgb());
    }

    pub fn drawLine(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.pg.draw_line(x1, y1, x2, y2, self.pen_color.rgb());
    }

    pub fn drawText(&mut self, x: usize, y: usize, text: &str) {
        self.pg.draw_text(x, y, text, self.pen_color.rgb());
    }

    pub fn drawCircle(&mut self, cx: usize, cy: usize, radius: usize) {
        self.pg.draw_circle(cx, cy, radius, self.pen_color.rgb());
    }

    pub fn fillCircle(&mut self, cx: usize, cy: usize, radius: usize) {
        self.pg.fill_circle(cx, cy, radius, self.brush_color.rgb());
    }
}
