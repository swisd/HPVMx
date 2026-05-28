use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use font8x8::UnicodeFonts;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{Key, ModifiersState, NamedKey};
use winit::window::{Window, WindowId};

const EXPORT_XML_PATH: &str = "pixel_designer_export.xml";
const EXPORT_RUST_PATH: &str = "pixel_designer_export.rs";
const LEFT_PANEL_W: usize = 230;
const RIGHT_PANEL_W: usize = 330;
const TOP_BAR_H: usize = 40;
const TOOL_ROW_H: usize = 22;
const TOOL_LIST_Y: usize = 72;
const PROP_LIST_Y: usize = 136;
const MIN_ELEMENT_SIZE: usize = 4;

#[derive(Debug, Clone, Copy)]
struct ResolutionPreset {
    label: &'static str,
    width: usize,
    height: usize,
}

const RESOLUTION_PRESETS: &[ResolutionPreset] = &[
    ResolutionPreset {
        label: "1280x720",
        width: 1280,
        height: 720,
    },
    ResolutionPreset {
        label: "1280x800",
        width: 1280,
        height: 800,
    },
    ResolutionPreset {
        label: "1024x768",
        width: 1024,
        height: 768,
    },
    ResolutionPreset {
        label: "800x600",
        width: 800,
        height: 600,
    },
    ResolutionPreset {
        label: "1920x1080",
        width: 1920,
        height: 1080,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgKind {
    Usize,
    I32,
    U64,
    F64,
    F32,
    Bool,
    String,
    Color,
    DataRef,
}

#[derive(Debug, Clone, Copy)]
struct ArgSpec {
    name: &'static str,
    kind: ArgKind,
    default: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct ToolSpec {
    category: &'static str,
    name: &'static str,
    func: &'static str,
    xml_name: &'static str,
    args: &'static [ArgSpec],
    default_w: usize,
    default_h: usize,
}

const EMPTY_ARGS: &[ArgSpec] = &[];
const PIXEL_ARGS: &[ArgSpec] = &[ArgSpec {
    name: "color",
    kind: ArgKind::Color,
    default: "0xFFFFFF",
}];
const LINE_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "x2",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "y2",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
];
const ADV_LINE_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "x2",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "y2",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
    ArgSpec {
        name: "thickness",
        kind: ArgKind::Usize,
        default: "3",
    },
    ArgSpec {
        name: "style",
        kind: ArgKind::Color,
        default: "0xFFFFFFFF",
    },
];
const RECT_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "100",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0x444444",
    },
];
const ADV_RECT_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "100",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
    ArgSpec {
        name: "thickness",
        kind: ArgKind::Usize,
        default: "1",
    },
    ArgSpec {
        name: "style",
        kind: ArgKind::Color,
        default: "0xFFFFFFFF",
    },
];
const LABEL_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "text",
        kind: ArgKind::String,
        default: "Label",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
];
const BG_LABEL_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "text",
        kind: ArgKind::String,
        default: "Label",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
    ArgSpec {
        name: "bg",
        kind: ArgKind::Color,
        default: "0x000000",
    },
];
const BUTTON_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "120",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "30",
    },
    ArgSpec {
        name: "text",
        kind: ArgKind::String,
        default: "Click Me",
    },
    ArgSpec {
        name: "is_focused",
        kind: ArgKind::Bool,
        default: "false",
    },
];
const CHECKBOX_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "checked",
        kind: ArgKind::Bool,
        default: "false",
    },
    ArgSpec {
        name: "blocked",
        kind: ArgKind::Bool,
        default: "false",
    },
    ArgSpec {
        name: "disabled",
        kind: ArgKind::Bool,
        default: "false",
    },
    ArgSpec {
        name: "text",
        kind: ArgKind::String,
        default: "Option",
    },
];
const TRISTATE_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "text",
        kind: ArgKind::String,
        default: "Tristate",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0xFFFFFF",
    },
    ArgSpec {
        name: "some",
        kind: ArgKind::Bool,
        default: "false",
    },
    ArgSpec {
        name: "checked",
        kind: ArgKind::Bool,
        default: "false",
    },
];
const RADIO_ARGS: &[ArgSpec] = &[ArgSpec {
    name: "checked",
    kind: ArgKind::Bool,
    default: "false",
}];
const SLIDER_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "150",
    },
    ArgSpec {
        name: "value",
        kind: ArgKind::Usize,
        default: "50",
    },
    ArgSpec {
        name: "max",
        kind: ArgKind::Usize,
        default: "100",
    },
    ArgSpec {
        name: "vertical",
        kind: ArgKind::Bool,
        default: "false",
    },
];
const DIAL_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "radius",
        kind: ArgKind::Usize,
        default: "30",
    },
    ArgSpec {
        name: "value",
        kind: ArgKind::Usize,
        default: "25",
    },
    ArgSpec {
        name: "max",
        kind: ArgKind::Usize,
        default: "100",
    },
];
const SPINBOX_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "80",
    },
    ArgSpec {
        name: "value",
        kind: ArgKind::I32,
        default: "0",
    },
    ArgSpec {
        name: "label",
        kind: ArgKind::String,
        default: "Label",
    },
];
const DOUBLE_SPINBOX_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "100",
    },
    ArgSpec {
        name: "value",
        kind: ArgKind::F64,
        default: "0.0",
    },
    ArgSpec {
        name: "precision",
        kind: ArgKind::Usize,
        default: "2",
    },
];
const PROGRESS_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "200",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "25",
    },
    ArgSpec {
        name: "value",
        kind: ArgKind::Usize,
        default: "30",
    },
    ArgSpec {
        name: "max",
        kind: ArgKind::Usize,
        default: "100",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0x00FF00",
    },
];
const LINE_GRAPH_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "300",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "150",
    },
    ArgSpec {
        name: "max_val",
        kind: ArgKind::U64,
        default: "100",
    },
    ArgSpec {
        name: "color",
        kind: ArgKind::Color,
        default: "0x00FFFF",
    },
    ArgSpec {
        name: "len",
        kind: ArgKind::Usize,
        default: "10",
    },
];
const LCD_ARGS: &[ArgSpec] = &[ArgSpec {
    name: "value",
    kind: ArgKind::String,
    default: "0000",
}];
const VIEW_SIZE_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "220",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "180",
    },
];
const TREE_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "250",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "220",
    },
    ArgSpec {
        name: "root",
        kind: ArgKind::DataRef,
        default: "root",
    },
];
const TREE_ICON_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "250",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "220",
    },
    ArgSpec {
        name: "root",
        kind: ArgKind::DataRef,
        default: "root",
    },
    ArgSpec {
        name: "icon",
        kind: ArgKind::DataRef,
        default: "ICON_DATA",
    },
];
const EDGE_ARGS: &[ArgSpec] = &[ArgSpec {
    name: "max_strength",
    kind: ArgKind::F32,
    default: "1.0",
}];
const ICON_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "16",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "16",
    },
    ArgSpec {
        name: "icon_id",
        kind: ArgKind::DataRef,
        default: "ICON_DATA",
    },
];
const WINDOW_ARGS: &[ArgSpec] = &[
    ArgSpec {
        name: "width",
        kind: ArgKind::Usize,
        default: "1280",
    },
    ArgSpec {
        name: "height",
        kind: ArgKind::Usize,
        default: "800",
    },
];

const TOOLS: &[ToolSpec] = &[
    ToolSpec {
        category: "Shapes",
        name: "Pixel",
        func: "draw_pixel",
        xml_name: "Pixel",
        args: PIXEL_ARGS,
        default_w: 8,
        default_h: 8,
    },
    ToolSpec {
        category: "Shapes",
        name: "Line",
        func: "draw_line",
        xml_name: "Line",
        args: LINE_ARGS,
        default_w: 60,
        default_h: 60,
    },
    ToolSpec {
        category: "Shapes",
        name: "Advanced Line",
        func: "draw_line_adv",
        xml_name: "AdvLine",
        args: ADV_LINE_ARGS,
        default_w: 60,
        default_h: 60,
    },
    ToolSpec {
        category: "Shapes",
        name: "Rectangle",
        func: "fill_rect",
        xml_name: "FillRect",
        args: RECT_ARGS,
        default_w: 100,
        default_h: 50,
    },
    ToolSpec {
        category: "Shapes",
        name: "Outline",
        func: "draw_rect_outline",
        xml_name: "RectOutline",
        args: RECT_ARGS,
        default_w: 100,
        default_h: 50,
    },
    ToolSpec {
        category: "Shapes",
        name: "Advanced Outline",
        func: "draw_rect_outline_adv",
        xml_name: "AdvRectOutline",
        args: ADV_RECT_ARGS,
        default_w: 100,
        default_h: 50,
    },
    ToolSpec {
        category: "Widgets",
        name: "Label",
        func: "draw_text",
        xml_name: "Label",
        args: LABEL_ARGS,
        default_w: 96,
        default_h: 18,
    },
    ToolSpec {
        category: "Widgets",
        name: "Background Label",
        func: "draw_text_bg",
        xml_name: "BackgroundLabel",
        args: BG_LABEL_ARGS,
        default_w: 120,
        default_h: 18,
    },
    ToolSpec {
        category: "Widgets",
        name: "Button",
        func: "draw_button",
        xml_name: "Button",
        args: BUTTON_ARGS,
        default_w: 120,
        default_h: 30,
    },
    ToolSpec {
        category: "Widgets",
        name: "Checkbox",
        func: "draw_checkbox",
        xml_name: "Checkbox",
        args: CHECKBOX_ARGS,
        default_w: 150,
        default_h: 18,
    },
    ToolSpec {
        category: "Widgets",
        name: "Tristate Checkbox",
        func: "draw_tristate_checkbox",
        xml_name: "TristateCheckbox",
        args: TRISTATE_ARGS,
        default_w: 165,
        default_h: 18,
    },
    ToolSpec {
        category: "Widgets",
        name: "Radio Button",
        func: "draw_radio_button",
        xml_name: "RadioButton",
        args: RADIO_ARGS,
        default_w: 18,
        default_h: 18,
    },
    ToolSpec {
        category: "Widgets",
        name: "Slider",
        func: "draw_slider",
        xml_name: "Slider",
        args: SLIDER_ARGS,
        default_w: 150,
        default_h: 20,
    },
    ToolSpec {
        category: "Widgets",
        name: "Dial",
        func: "draw_dial",
        xml_name: "Dial",
        args: DIAL_ARGS,
        default_w: 60,
        default_h: 60,
    },
    ToolSpec {
        category: "Widgets",
        name: "Spinbox",
        func: "draw_spinbox",
        xml_name: "SpinBox",
        args: SPINBOX_ARGS,
        default_w: 120,
        default_h: 24,
    },
    ToolSpec {
        category: "Widgets",
        name: "Double Spinbox",
        func: "draw_double_spinbox",
        xml_name: "DoubleSpinBox",
        args: DOUBLE_SPINBOX_ARGS,
        default_w: 100,
        default_h: 24,
    },
    ToolSpec {
        category: "Widgets",
        name: "Progress Bar",
        func: "draw_progress_bar",
        xml_name: "ProgressBar",
        args: PROGRESS_ARGS,
        default_w: 200,
        default_h: 25,
    },
    ToolSpec {
        category: "Charts",
        name: "Line Graph",
        func: "draw_line_graph",
        xml_name: "LineGraph",
        args: LINE_GRAPH_ARGS,
        default_w: 300,
        default_h: 150,
    },
    ToolSpec {
        category: "Charts",
        name: "LCD Number",
        func: "draw_lcd_number",
        xml_name: "LCDNumber",
        args: LCD_ARGS,
        default_w: 72,
        default_h: 24,
    },
    ToolSpec {
        category: "Views",
        name: "List View",
        func: "draw_list_view",
        xml_name: "ListView",
        args: VIEW_SIZE_ARGS,
        default_w: 220,
        default_h: 180,
    },
    ToolSpec {
        category: "Views",
        name: "Table View",
        func: "draw_table_view",
        xml_name: "TableView",
        args: VIEW_SIZE_ARGS,
        default_w: 360,
        default_h: 160,
    },
    ToolSpec {
        category: "Views",
        name: "Tree View",
        func: "draw_tree_view",
        xml_name: "TreeView",
        args: TREE_ARGS,
        default_w: 250,
        default_h: 220,
    },
    ToolSpec {
        category: "Views",
        name: "Icon Tree View",
        func: "draw_tree_view_icon",
        xml_name: "TreeViewIcon",
        args: TREE_ICON_ARGS,
        default_w: 250,
        default_h: 220,
    },
    ToolSpec {
        category: "Shaders",
        name: "Scanlines",
        func: "apply_scanlines",
        xml_name: "ScanlineFX",
        args: EMPTY_ARGS,
        default_w: 160,
        default_h: 70,
    },
    ToolSpec {
        category: "Shaders",
        name: "Dither",
        func: "apply_dither",
        xml_name: "DitherFX",
        args: EMPTY_ARGS,
        default_w: 160,
        default_h: 70,
    },
    ToolSpec {
        category: "Shaders",
        name: "Glitch",
        func: "apply_glitch",
        xml_name: "GlitchFX",
        args: EMPTY_ARGS,
        default_w: 160,
        default_h: 70,
    },
    ToolSpec {
        category: "Shaders",
        name: "Edge Aberration",
        func: "apply_edge_aberration",
        xml_name: "EdgeAberrationFX",
        args: EDGE_ARGS,
        default_w: 160,
        default_h: 70,
    },
    ToolSpec {
        category: "Media",
        name: "Icon",
        func: "draw_icon",
        xml_name: "Icon",
        args: ICON_ARGS,
        default_w: 32,
        default_h: 32,
    },
    ToolSpec {
        category: "Misc",
        name: "Header",
        func: "HEADER",
        xml_name: "Header",
        args: WINDOW_ARGS,
        default_w: 420,
        default_h: 80,
    },
    ToolSpec {
        category: "Misc",
        name: "Footer",
        func: "FOOTER",
        xml_name: "Footer",
        args: WINDOW_ARGS,
        default_w: 420,
        default_h: 55,
    },
    ToolSpec {
        category: "Misc",
        name: "Window",
        func: "WINDOW",
        xml_name: "Window",
        args: WINDOW_ARGS,
        default_w: 420,
        default_h: 260,
    },
];

pub struct DesktopPixelGraphics<'a> {
    buffer: &'a mut [u8],
    width: usize,
    height: usize,
}

impl<'a> DesktopPixelGraphics<'a> {
    pub fn new(buffer: &'a mut [u8], width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height,
        }
    }

    pub fn resolution(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let idx = (y * self.width + x) * 4;
        if idx + 3 < self.buffer.len() {
            self.buffer[idx] = ((color >> 16) & 0xFF) as u8;
            self.buffer[idx + 1] = ((color >> 8) & 0xFF) as u8;
            self.buffer[idx + 2] = (color & 0xFF) as u8;
            self.buffer[idx + 3] = 255;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let max_x = x.saturating_add(w).min(self.width);
        let max_y = y.saturating_add(h).min(self.height);
        for py in y..max_y {
            for px in x..max_x {
                self.set_pixel(px, py, color);
            }
        }
    }

    pub fn draw_rect_outline(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        if w == 0 || h == 0 {
            return;
        }

        let max_x = x.saturating_add(w).min(self.width);
        let max_y = y.saturating_add(h).min(self.height);
        if x >= max_x || y >= max_y {
            return;
        }

        for px in x..max_x {
            self.set_pixel(px, y, color);
            self.set_pixel(px, max_y - 1, color);
        }
        for py in y..max_y {
            self.set_pixel(x, py, color);
            self.set_pixel(max_x - 1, py, color);
        }
    }

    pub fn draw_line(&mut self, mut x0: isize, mut y0: isize, x1: isize, y1: isize, color: u32) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.set_pixel(x0 as usize, y0 as usize, color);
            }
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: u32) {
        for (i, c) in text.chars().enumerate() {
            if let Some(glyph) = font8x8::BASIC_FONTS.get(c) {
                for (gy, row) in glyph.iter().enumerate() {
                    for gx in 0..8 {
                        if (row >> gx) & 1 == 1 {
                            self.set_pixel(x + i * 8 + gx, y + gy, color);
                        }
                    }
                }
            }
        }
    }

    fn draw_text_bg(&mut self, x: usize, y: usize, text: &str, color: u32, bg: u32) {
        self.fill_rect(x, y, text.chars().count() * 8, 8, bg);
        self.draw_text(x, y, text, color);
    }

    fn draw_button(&mut self, x: usize, y: usize, w: usize, h: usize, label: &str, focused: bool) {
        self.fill_rect(x, y, w, h, if focused { 0x3A3A45 } else { 0x24242B });
        self.draw_rect_outline(x, y, w, h, 0x00FFCC);
        self.draw_text(x + 10, y + h.saturating_sub(8) / 2, label, 0xFFFFFF);
    }

    fn draw_checkbox(
        &mut self,
        x: usize,
        y: usize,
        checked: bool,
        blocked: bool,
        disabled: bool,
        label: &str,
    ) {
        let fg = if disabled { 0x666666 } else { 0xFFFFFF };
        let mark = if blocked { 0xFF5555 } else { 0x00FF88 };
        self.fill_rect(x, y, 12, 12, 0x202027);
        self.draw_rect_outline(x, y, 12, 12, if disabled { 0x555555 } else { 0x9A9AA5 });
        if checked || blocked {
            self.draw_line(
                x as isize + 2,
                y as isize + 6,
                x as isize + 5,
                y as isize + 9,
                mark,
            );
            self.draw_line(
                x as isize + 5,
                y as isize + 9,
                x as isize + 10,
                y as isize + 2,
                mark,
            );
        }
        self.draw_text(x + 18, y + 2, label, fg);
    }

    fn draw_radio(&mut self, x: usize, y: usize, checked: bool) {
        self.draw_rect_outline(x + 2, y + 2, 12, 12, 0xAAAAAA);
        if checked {
            self.fill_rect(x + 5, y + 5, 6, 6, 0x00FFCC);
        }
    }

    fn draw_slider(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        value: usize,
        max: usize,
        vertical: bool,
    ) {
        let safe_max = max.max(1);
        if vertical {
            self.draw_line(
                x as isize + 8,
                y as isize,
                x as isize + 8,
                y.saturating_add(w) as isize,
                0x8A8A95,
            );
            let handle_y = y + (w * value.min(safe_max) / safe_max);
            self.fill_rect(x + 2, handle_y.saturating_sub(4), 12, 8, 0x00FFCC);
        } else {
            self.draw_line(
                x as isize,
                y as isize + 8,
                x.saturating_add(w) as isize,
                y as isize + 8,
                0x8A8A95,
            );
            let handle_x = x + (w * value.min(safe_max) / safe_max);
            self.fill_rect(handle_x.saturating_sub(4), y + 2, 8, 12, 0x00FFCC);
        }
    }

    fn draw_progress_bar(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        value: usize,
        max: usize,
        color: u32,
    ) {
        self.fill_rect(x, y, w, h, 0x18181E);
        self.draw_rect_outline(x, y, w, h, 0x8A8A95);
        if w > 2 && h > 2 {
            let fill_w = (w - 2) * value.min(max.max(1)) / max.max(1);
            self.fill_rect(x + 1, y + 1, fill_w, h - 2, color);
        }
    }

    fn draw_dial(&mut self, x: usize, y: usize, radius: usize, value: usize, max: usize) {
        let d = radius * 2;
        self.draw_rect_outline(x, y, d, d, 0x888888);
        let cx = x + radius;
        let cy = y + radius;
        let angle = (value.min(max.max(1)) as f32 / max.max(1) as f32) * std::f32::consts::PI * 1.5
            + std::f32::consts::PI * 0.75;
        let px = cx as f32 + angle.cos() * radius.saturating_sub(6) as f32;
        let py = cy as f32 + angle.sin() * radius.saturating_sub(6) as f32;
        self.draw_line(cx as isize, cy as isize, px as isize, py as isize, 0x00FFCC);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropertyKey {
    X,
    Y,
    Width,
    Height,
    Arg(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DragMode {
    None,
    Move,
    Resize,
}

#[derive(Debug, Clone)]
struct DesignerElement {
    tool_idx: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    values: Vec<String>,
}

impl DesignerElement {
    fn new(tool_idx: usize, x: usize, y: usize) -> Self {
        let tool = &TOOLS[tool_idx];
        let values = tool
            .args
            .iter()
            .map(|arg| arg.default.to_string())
            .collect::<Vec<_>>();
        let mut element = Self {
            tool_idx,
            x,
            y,
            width: tool.default_w,
            height: tool.default_h,
            values,
        };
        element.sync_size_from_args();
        element
    }

    fn tool(&self) -> &'static ToolSpec {
        &TOOLS[self.tool_idx]
    }

    fn value(&self, name: &str) -> Option<&str> {
        self.tool()
            .args
            .iter()
            .position(|arg| arg.name == name)
            .and_then(|idx| self.values.get(idx).map(String::as_str))
    }

    fn usize_value(&self, name: &str, fallback: usize) -> usize {
        self.value(name).and_then(parse_usize).unwrap_or(fallback)
    }

    fn bool_value(&self, name: &str, fallback: bool) -> bool {
        self.value(name)
            .map(|value| value.eq_ignore_ascii_case("true") || value == "1")
            .unwrap_or(fallback)
    }

    fn color_value(&self, name: &str, fallback: u32) -> u32 {
        self.value(name).and_then(parse_color).unwrap_or(fallback)
    }

    fn string_value(&self, name: &str, fallback: &str) -> String {
        self.value(name).unwrap_or(fallback).to_string()
    }

    fn sync_size_from_args(&mut self) {
        if let Some(width) = self.value("width").and_then(parse_usize) {
            self.width = width.max(MIN_ELEMENT_SIZE);
        }
        if let Some(height) = self.value("height").and_then(parse_usize) {
            self.height = height.max(MIN_ELEMENT_SIZE);
        }
        if let Some(radius) = self.value("radius").and_then(parse_usize) {
            self.width = radius.max(1) * 2;
            self.height = radius.max(1) * 2;
        }
        if self.tool().func == "draw_line" || self.tool().func == "draw_line_adv" {
            let x2 = self.usize_value("x2", self.x + self.width);
            let y2 = self.usize_value("y2", self.y + self.height);
            self.width = x2.abs_diff(self.x).max(MIN_ELEMENT_SIZE);
            self.height = y2.abs_diff(self.y).max(MIN_ELEMENT_SIZE);
        }
    }

    fn sync_args_from_size(&mut self) {
        let args = self.tool().args;
        for (idx, arg) in args.iter().enumerate() {
            if arg.name == "width" {
                self.values[idx] = self.width.to_string();
            } else if arg.name == "height" {
                self.values[idx] = self.height.to_string();
            } else if arg.name == "radius" {
                self.values[idx] = (self.width.min(self.height) / 2).max(1).to_string();
            } else if arg.name == "x2" {
                self.values[idx] = self.x.saturating_add(self.width).to_string();
            } else if arg.name == "y2" {
                self.values[idx] = self.y.saturating_add(self.height).to_string();
            }
        }
    }
}

pub struct PixelDesignerApp {
    elements: Vec<DesignerElement>,
    selected_idx: Option<usize>,
    selected_tool_idx: usize,
    selected_property: usize,
    tool_scroll: usize,
    property_scroll: usize,
    grid_size: usize,
    canvas_x: usize,
    canvas_y: usize,
    canvas_w: usize,
    canvas_h: usize,
    drag_mode: DragMode,
    drag_offset_x: i32,
    drag_offset_y: i32,
    previous_mouse_down: bool,
    modifiers: ModifiersState,
    target_resolution_idx: usize,
    show_context_outline: bool,
    status: String,
}

impl PixelDesignerApp {
    pub fn new(screen_w: usize, screen_h: usize) -> Self {
        let mut app = Self {
            elements: Vec::new(),
            selected_idx: None,
            selected_tool_idx: 8,
            selected_property: 0,
            tool_scroll: 0,
            property_scroll: 0,
            grid_size: 20,
            canvas_x: 0,
            canvas_y: 0,
            canvas_w: 0,
            canvas_h: 0,
            drag_mode: DragMode::None,
            drag_offset_x: 0,
            drag_offset_y: 0,
            previous_mouse_down: false,
            modifiers: ModifiersState::empty(),
            target_resolution_idx: 0,
            show_context_outline: true,
            status: "Click tools to add. Drag to move; drag white handle to resize.".to_string(),
        };
        app.resize(screen_w, screen_h);
        app.spawn_element(8, app.canvas_x + 40, app.canvas_y + 40);
        app
    }

    pub fn resize(&mut self, screen_w: usize, screen_h: usize) {
        self.canvas_x = LEFT_PANEL_W + 10;
        self.canvas_y = TOP_BAR_H + 10;
        self.canvas_w = screen_w
            .saturating_sub(LEFT_PANEL_W + RIGHT_PANEL_W + 30)
            .max(260);
        self.canvas_h = screen_h.saturating_sub(TOP_BAR_H + 20).max(220);
        self.clamp_elements();
    }

    fn clamp_elements(&mut self) {
        let min_x = self.canvas_x;
        let min_y = self.canvas_y;
        let max_x = self.canvas_x.saturating_add(self.canvas_w);
        let max_y = self.canvas_y.saturating_add(self.canvas_h);
        for el in &mut self.elements {
            el.width = el.width.min(self.canvas_w).max(MIN_ELEMENT_SIZE);
            el.height = el.height.min(self.canvas_h).max(MIN_ELEMENT_SIZE);
            el.x = el.x.clamp(min_x, max_x.saturating_sub(el.width));
            el.y = el.y.clamp(min_y, max_y.saturating_sub(el.height));
            el.sync_args_from_size();
        }
    }

    fn spawn_element(&mut self, tool_idx: usize, x: usize, y: usize) {
        let mut element = DesignerElement::new(tool_idx, x, y);
        element.x = element.x.clamp(
            self.canvas_x,
            self.canvas_x
                .saturating_add(self.canvas_w)
                .saturating_sub(element.width),
        );
        element.y = element.y.clamp(
            self.canvas_y,
            self.canvas_y
                .saturating_add(self.canvas_h)
                .saturating_sub(element.height),
        );
        element.sync_args_from_size();
        self.elements.push(element);
        self.selected_idx = Some(self.elements.len() - 1);
        self.selected_property = 0;
        self.status = format!("Added {}", TOOLS[tool_idx].name);
    }

    fn property_count(&self) -> usize {
        self.selected_element()
            .map(|el| 4 + el.tool().args.len())
            .unwrap_or(0)
    }

    fn target_resolution(&self) -> ResolutionPreset {
        RESOLUTION_PRESETS[self.target_resolution_idx.min(RESOLUTION_PRESETS.len() - 1)]
    }

    fn cycle_resolution(&mut self) {
        self.target_resolution_idx = (self.target_resolution_idx + 1) % RESOLUTION_PRESETS.len();
        let preset = self.target_resolution();
        self.status = format!("Planning resolution set to {}", preset.label);
    }

    fn set_resolution_preset(&mut self, idx: usize) {
        if idx < RESOLUTION_PRESETS.len() {
            self.target_resolution_idx = idx;
            self.status = format!(
                "Planning resolution set to {}",
                RESOLUTION_PRESETS[idx].label
            );
        }
    }

    fn toggle_context_outline(&mut self) {
        self.show_context_outline = !self.show_context_outline;
        self.status = if self.show_context_outline {
            "Planning outline shown".to_string()
        } else {
            "Planning outline hidden".to_string()
        };
    }

    fn selected_element(&self) -> Option<&DesignerElement> {
        self.selected_idx.and_then(|idx| self.elements.get(idx))
    }

    fn selected_element_mut(&mut self) -> Option<&mut DesignerElement> {
        self.selected_idx.and_then(|idx| self.elements.get_mut(idx))
    }

    fn property_key(&self, prop_idx: usize) -> Option<PropertyKey> {
        let el = self.selected_element()?;
        match prop_idx {
            0 => Some(PropertyKey::X),
            1 => Some(PropertyKey::Y),
            2 => Some(PropertyKey::Width),
            3 => Some(PropertyKey::Height),
            n if n < 4 + el.tool().args.len() => Some(PropertyKey::Arg(n - 4)),
            _ => None,
        }
    }

    fn property_name_value(&self, prop_idx: usize) -> Option<(String, String)> {
        let el = self.selected_element()?;
        match self.property_key(prop_idx)? {
            PropertyKey::X => Some(("x".to_string(), el.x.to_string())),
            PropertyKey::Y => Some(("y".to_string(), el.y.to_string())),
            PropertyKey::Width => Some(("width".to_string(), el.width.to_string())),
            PropertyKey::Height => Some(("height".to_string(), el.height.to_string())),
            PropertyKey::Arg(idx) => {
                let arg = el.tool().args[idx];
                Some((arg.name.to_string(), el.values[idx].clone()))
            }
        }
    }

    fn set_property_text(&mut self, prop_idx: usize, value: String) {
        let Some(key) = self.property_key(prop_idx) else {
            return;
        };
        let canvas_x = self.canvas_x;
        let canvas_y = self.canvas_y;
        let canvas_w = self.canvas_w;
        let canvas_h = self.canvas_h;
        if let Some(el) = self.selected_element_mut() {
            match key {
                PropertyKey::X => {
                    if let Some(v) = parse_usize(&value) {
                        el.x = v;
                    }
                }
                PropertyKey::Y => {
                    if let Some(v) = parse_usize(&value) {
                        el.y = v;
                    }
                }
                PropertyKey::Width => {
                    if let Some(v) = parse_usize(&value) {
                        el.width = v.max(MIN_ELEMENT_SIZE);
                    }
                }
                PropertyKey::Height => {
                    if let Some(v) = parse_usize(&value) {
                        el.height = v.max(MIN_ELEMENT_SIZE);
                    }
                }
                PropertyKey::Arg(idx) => {
                    if idx < el.values.len() {
                        el.values[idx] = value;
                        el.sync_size_from_args();
                    }
                }
            }
            el.x = el.x.clamp(
                canvas_x,
                canvas_x.saturating_add(canvas_w).saturating_sub(el.width),
            );
            el.y = el.y.clamp(
                canvas_y,
                canvas_y.saturating_add(canvas_h).saturating_sub(el.height),
            );
            el.sync_args_from_size();
        }
    }

    fn nudge_property(&mut self, amount: i32) {
        let prop = self.selected_property;
        let Some((_, current)) = self.property_name_value(prop) else {
            return;
        };
        let next = match self.property_key(prop) {
            Some(PropertyKey::Arg(idx)) => {
                let Some(el) = self.selected_element() else {
                    return;
                };
                let arg = el.tool().args[idx];
                match arg.kind {
                    ArgKind::Bool => {
                        if current.eq_ignore_ascii_case("true") {
                            "false".to_string()
                        } else {
                            "true".to_string()
                        }
                    }
                    ArgKind::String | ArgKind::DataRef => current,
                    ArgKind::Color => format!(
                        "0x{:06X}",
                        parse_color(&current)
                            .unwrap_or(0)
                            .saturating_add(amount.max(1) as u32 * 0x10101)
                            & 0xFFFFFF
                    ),
                    ArgKind::I32 => (current.parse::<i32>().unwrap_or(0) + amount).to_string(),
                    ArgKind::F32 | ArgKind::F64 => format!(
                        "{:.1}",
                        current.parse::<f64>().unwrap_or(0.0) + amount as f64
                    ),
                    ArgKind::Usize | ArgKind::U64 => (parse_usize(&current).unwrap_or(0) as i32
                        + amount)
                        .max(0)
                        .to_string(),
                }
            }
            Some(_) => (parse_usize(&current).unwrap_or(0) as i32 + amount)
                .max(0)
                .to_string(),
            None => return,
        };
        self.set_property_text(prop, next);
    }

    fn append_property_char(&mut self, c: char) {
        let prop = self.selected_property;
        let Some((_, mut current)) = self.property_name_value(prop) else {
            return;
        };
        current.push(c);
        self.set_property_text(prop, current);
    }

    fn backspace_property(&mut self) {
        let prop = self.selected_property;
        let Some((_, mut current)) = self.property_name_value(prop) else {
            return;
        };
        current.pop();
        self.set_property_text(prop, current);
    }

    fn clear_property(&mut self) {
        let prop = self.selected_property;
        let Some(key) = self.property_key(prop) else {
            return;
        };
        let value = match key {
            PropertyKey::Arg(idx) => self
                .selected_element()
                .map(|el| el.tool().args[idx].default.to_string())
                .unwrap_or_default(),
            _ => "0".to_string(),
        };
        self.set_property_text(prop, value);
    }

    pub fn process_mouse_interaction(&mut self, mx: usize, my: usize, mouse_down: bool) {
        let just_pressed = mouse_down && !self.previous_mouse_down;
        self.previous_mouse_down = mouse_down;

        if !mouse_down {
            self.drag_mode = DragMode::None;
            return;
        }

        if self.drag_mode != DragMode::None {
            self.update_drag(mx, my);
            return;
        }

        if !just_pressed {
            return;
        }

        if mx < LEFT_PANEL_W {
            if my >= TOOL_LIST_Y {
                let row = (my - TOOL_LIST_Y) / TOOL_ROW_H + self.tool_scroll;
                if row < TOOLS.len() {
                    self.selected_tool_idx = row;
                    let offset = 40 + self.elements.len() * 16;
                    self.spawn_element(row, self.canvas_x + offset, self.canvas_y + offset);
                }
            }
            return;
        }

        if mx >= self.canvas_x + self.canvas_w + 10 {
            if (72..=90).contains(&my) {
                self.cycle_resolution();
                return;
            }

            if (94..=112).contains(&my) {
                self.toggle_context_outline();
                return;
            }

            if my >= PROP_LIST_Y {
                let row = (my - PROP_LIST_Y) / 18 + self.property_scroll;
                if row < self.property_count() {
                    self.selected_property = row;
                    self.status =
                        "Type to edit selected property. Backspace/delete clears characters."
                            .to_string();
                }
            }
            return;
        }

        for (idx, el) in self.elements.iter().enumerate().rev() {
            let on_resize_handle = mx >= el.x.saturating_add(el.width).saturating_sub(8)
                && mx <= el.x.saturating_add(el.width).saturating_add(4)
                && my >= el.y.saturating_add(el.height).saturating_sub(8)
                && my <= el.y.saturating_add(el.height).saturating_add(4);
            if on_resize_handle {
                self.selected_idx = Some(idx);
                self.drag_mode = DragMode::Resize;
                self.selected_property = 2;
                self.update_drag(mx, my);
                return;
            }

            if mx >= el.x
                && mx <= el.x.saturating_add(el.width)
                && my >= el.y
                && my <= el.y.saturating_add(el.height)
            {
                self.selected_idx = Some(idx);
                self.drag_mode = DragMode::Move;
                self.drag_offset_x = el.x as i32 - mx as i32;
                self.drag_offset_y = el.y as i32 - my as i32;
                self.selected_property = 0;
                return;
            }
        }

        self.selected_idx = None;
    }

    fn update_drag(&mut self, mx: usize, my: usize) {
        let canvas_x = self.canvas_x;
        let canvas_y = self.canvas_y;
        let canvas_w = self.canvas_w;
        let canvas_h = self.canvas_h;
        let grid_size = self.grid_size;
        let drag_offset_x = self.drag_offset_x;
        let drag_offset_y = self.drag_offset_y;
        let drag_mode = self.drag_mode;

        if let Some(el) = self.selected_element_mut() {
            match drag_mode {
                DragMode::Move => {
                    let nx = (mx as i32 + drag_offset_x).max(0) as usize;
                    let ny = (my as i32 + drag_offset_y).max(0) as usize;
                    let max_x = canvas_x.saturating_add(canvas_w).saturating_sub(el.width);
                    let max_y = canvas_y.saturating_add(canvas_h).saturating_sub(el.height);
                    el.x = snap(nx, grid_size).clamp(canvas_x, max_x);
                    el.y = snap(ny, grid_size).clamp(canvas_y, max_y);
                }
                DragMode::Resize => {
                    let max_w = canvas_x.saturating_add(canvas_w).saturating_sub(el.x);
                    let max_h = canvas_y.saturating_add(canvas_h).saturating_sub(el.y);
                    el.width =
                        snap(mx.saturating_sub(el.x), grid_size).clamp(MIN_ELEMENT_SIZE, max_w);
                    el.height =
                        snap(my.saturating_sub(el.y), grid_size).clamp(MIN_ELEMENT_SIZE, max_h);
                }
                DragMode::None => {}
            }
            el.sync_args_from_size();
        }
    }

    fn handle_key(&mut self, event: &KeyEvent) {
        if event.state != ElementState::Pressed {
            return;
        }

        match &event.logical_key {
            Key::Named(NamedKey::ArrowDown) => {
                let count = self.property_count().max(1);
                self.selected_property = (self.selected_property + 1).min(count - 1);
            }
            Key::Named(NamedKey::ArrowUp) => {
                self.selected_property = self.selected_property.saturating_sub(1);
            }
            Key::Named(NamedKey::ArrowRight) => self.nudge_property(1),
            Key::Named(NamedKey::ArrowLeft) => self.nudge_property(-1),
            Key::Named(NamedKey::PageDown) => {
                self.tool_scroll = (self.tool_scroll + 6).min(TOOLS.len().saturating_sub(1));
            }
            Key::Named(NamedKey::PageUp) => {
                self.tool_scroll = self.tool_scroll.saturating_sub(6);
            }
            Key::Named(NamedKey::Backspace) => self.backspace_property(),
            Key::Named(NamedKey::Delete) => self.clear_property(),
            Key::Named(NamedKey::Enter) => self.nudge_property(1),
            Key::Character(text) => {
                if self.modifiers.control_key() && text.eq_ignore_ascii_case("e") {
                    self.export_xml();
                    self.export_rust();
                } else if self.modifiers.control_key() && text.eq_ignore_ascii_case("b") {
                    self.toggle_context_outline();
                } else if self.modifiers.control_key() && text.eq_ignore_ascii_case("i") {
                    self.import_xml();
                } else if self.modifiers.control_key() && text.eq_ignore_ascii_case("r") {
                    self.export_rust();
                } else if self.modifiers.control_key() && text.eq_ignore_ascii_case("x") {
                    self.export_xml();
                } else if self.modifiers.control_key() {
                    if let Some(digit) = text.chars().next().and_then(|c| c.to_digit(10)) {
                        if digit > 0 {
                            self.set_resolution_preset(digit as usize - 1);
                        }
                    }
                } else if let Some(c) = text.chars().next() {
                    if text.chars().count() == 1 && !c.is_control() {
                        self.append_property_char(c);
                    }
                }
            }
            _ => {}
        }
    }

    fn export_xml(&mut self) {
        match fs::write(EXPORT_XML_PATH, self.to_xml()) {
            Ok(_) => self.status = format!("Exported XML to {}", EXPORT_XML_PATH),
            Err(err) => self.status = format!("XML export failed: {}", err),
        }
    }

    fn export_rust(&mut self) {
        match fs::write(EXPORT_RUST_PATH, self.to_rust()) {
            Ok(_) => self.status = format!("Exported Rust to {}", EXPORT_RUST_PATH),
            Err(err) => self.status = format!("Rust export failed: {}", err),
        }
    }

    fn import_xml(&mut self) {
        match fs::read_to_string(EXPORT_XML_PATH) {
            Ok(xml) => {
                if let Some(canvas_line) =
                    xml.lines().find(|line| line.trim().starts_with("<Canvas"))
                {
                    let trimmed = canvas_line.trim();
                    if let (Some(width), Some(height)) = (
                        xml_attr(trimmed, "target_width").and_then(|v| parse_usize(&v)),
                        xml_attr(trimmed, "target_height").and_then(|v| parse_usize(&v)),
                    ) {
                        if let Some(idx) = RESOLUTION_PRESETS
                            .iter()
                            .position(|preset| preset.width == width && preset.height == height)
                        {
                            self.target_resolution_idx = idx;
                        }
                    }
                    if let Some(outline) = xml_attr(trimmed, "planning_outline") {
                        self.show_context_outline =
                            outline.eq_ignore_ascii_case("true") || outline == "1";
                    }
                }
                let imported = parse_elements_from_xml(&xml);
                self.elements = imported;
                self.selected_idx = if self.elements.is_empty() {
                    None
                } else {
                    Some(0)
                };
                self.clamp_elements();
                self.status = format!(
                    "Imported {} elements from {}",
                    self.elements.len(),
                    EXPORT_XML_PATH
                );
            }
            Err(err) => self.status = format!("XML import failed: {}", err),
        }
    }

    fn to_xml(&self) -> String {
        let mut xml = String::from("<PixelDesignerLayout>\n");
        let preset = self.target_resolution();
        xml.push_str(&format!(
            "  <Canvas x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" grid=\"{}\" target_width=\"{}\" target_height=\"{}\" target_label=\"{}\" planning_outline=\"{}\" />\n",
            self.canvas_x,
            self.canvas_y,
            self.canvas_w,
            self.canvas_h,
            self.grid_size,
            preset.width,
            preset.height,
            preset.label,
            self.show_context_outline
        ));
        xml.push_str("  <Elements>\n");
        for el in &self.elements {
            let tool = el.tool();
            xml.push_str(&format!(
                "    <{} x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"",
                tool.xml_name, el.x, el.y, el.width, el.height
            ));
            for (arg, value) in tool.args.iter().zip(el.values.iter()) {
                xml.push_str(&format!(" {}=\"{}\"", arg.name, escape_xml(value)));
            }
            xml.push_str(" />\n");
        }
        xml.push_str("  </Elements>\n</PixelDesignerLayout>\n");
        xml
    }

    fn to_rust(&self) -> String {
        let mut rust = String::from("pub fn render_generated_ui(gfx: &mut PixelGraphics) {\n");
        rust.push_str("    let graph_data: [u64; 10] = [8, 18, 14, 32, 28, 44, 52, 48, 70, 64];\n");
        rust.push_str("    let list_items: [&str; 3] = [\"Alpha\", \"Beta\", \"Gamma\"];\n");
        rust.push_str("    let table_headers: [&str; 2] = [\"Name\", \"Value\"];\n");
        rust.push_str("    let table_row_a: [&str; 2] = [\"CPU\", \"42\"];\n");
        rust.push_str("    let table_rows: [&[&str]; 1] = [&table_row_a];\n");
        rust.push_str("    // Tree/icon exports expect caller-provided `root` and `ICON_DATA` symbols when used.\n\n");
        for el in &self.elements {
            rust.push_str("    ");
            rust.push_str(&rust_call_for(el));
            rust.push('\n');
        }
        rust.push_str("}\n");
        rust
    }

    pub fn render_application_frame(&self, gfx: &mut DesktopPixelGraphics) {
        let (scr_w, scr_h) = gfx.resolution();
        gfx.fill_rect(0, 0, scr_w, scr_h, 0x0A0A0C);
        gfx.fill_rect(0, 0, scr_w, TOP_BAR_H, 0x1F1F24);
        gfx.draw_text(15, 12, "PIXEL DESIGNER PRO", 0x00FFCC);
        gfx.draw_text(
            230,
            12,
            "Ctrl+E export all  Ctrl+1..5 resolution  Ctrl+B outline  PgUp/PgDn tools  Arrows edit",
            0xA8A8B2,
        );
        gfx.draw_text(
            scr_w.saturating_sub(RIGHT_PANEL_W) + 12,
            12,
            &clip_text(&self.status, 38),
            0xF0D070,
        );

        self.render_tools(gfx, scr_h);
        self.render_canvas(gfx);
        self.render_properties(gfx, scr_w, scr_h);
    }

    fn render_tools(&self, gfx: &mut DesktopPixelGraphics, scr_h: usize) {
        gfx.fill_rect(
            0,
            TOP_BAR_H + 1,
            LEFT_PANEL_W,
            scr_h.saturating_sub(TOP_BAR_H),
            0x17171A,
        );
        gfx.draw_text(10, 54, "TOOLS", 0xFFFFFF);
        let visible_rows = scr_h.saturating_sub(TOOL_LIST_Y + 8) / TOOL_ROW_H;
        for row in 0..visible_rows {
            let idx = self.tool_scroll + row;
            if idx >= TOOLS.len() {
                break;
            }
            let tool = &TOOLS[idx];
            let y = TOOL_LIST_Y + row * TOOL_ROW_H;
            let selected = idx == self.selected_tool_idx;
            gfx.fill_rect(
                10,
                y,
                LEFT_PANEL_W - 20,
                TOOL_ROW_H - 2,
                if selected { 0x30303A } else { 0x24242B },
            );
            gfx.draw_rect_outline(
                10,
                y,
                LEFT_PANEL_W - 20,
                TOOL_ROW_H - 2,
                if selected { 0x00FFCC } else { 0x3A3A45 },
            );
            gfx.draw_text(
                18,
                y + 6,
                &clip_text(&format!("{} / {}", tool.category, tool.name), 24),
                0xFFFFFF,
            );
        }
    }

    fn render_canvas(&self, gfx: &mut DesktopPixelGraphics) {
        gfx.fill_rect(
            self.canvas_x,
            self.canvas_y,
            self.canvas_w,
            self.canvas_h,
            0x121214,
        );
        gfx.draw_rect_outline(
            self.canvas_x,
            self.canvas_y,
            self.canvas_w,
            self.canvas_h,
            0x32323D,
        );
        gfx.draw_rect_outline(
            self.canvas_x.saturating_sub(1),
            self.canvas_y.saturating_sub(1),
            self.canvas_w + 2,
            self.canvas_h + 2,
            0x00FFCC,
        );

        let mut gx = self.canvas_x;
        while gx < self.canvas_x + self.canvas_w {
            gfx.draw_line(
                gx as isize,
                self.canvas_y as isize,
                gx as isize,
                (self.canvas_y + self.canvas_h) as isize,
                0x1D1D22,
            );
            gx += self.grid_size;
        }
        let mut gy = self.canvas_y;
        while gy < self.canvas_y + self.canvas_h {
            gfx.draw_line(
                self.canvas_x as isize,
                gy as isize,
                (self.canvas_x + self.canvas_w) as isize,
                gy as isize,
                0x1D1D22,
            );
            gy += self.grid_size;
        }

        self.render_planning_outline(gfx);

        for (idx, el) in self.elements.iter().enumerate() {
            render_element_preview(gfx, el);
            if Some(idx) == self.selected_idx {
                gfx.draw_rect_outline(
                    el.x.saturating_sub(2),
                    el.y.saturating_sub(2),
                    el.width + 4,
                    el.height + 4,
                    0xFFFFFF,
                );
                gfx.fill_rect(
                    el.x + el.width.saturating_sub(6),
                    el.y + el.height.saturating_sub(6),
                    10,
                    10,
                    0xFFFFFF,
                );
            }
        }
    }

    fn planning_rect(&self) -> (usize, usize, usize, usize) {
        let preset = self.target_resolution();
        let max_w = self.canvas_w.saturating_sub(36).max(1);
        let max_h = self.canvas_h.saturating_sub(36).max(1);
        let scale_w = max_w as f64 / preset.width as f64;
        let scale_h = max_h as f64 / preset.height as f64;
        let scale = scale_w.min(scale_h).min(1.0).max(0.01);
        let w = (preset.width as f64 * scale) as usize;
        let h = (preset.height as f64 * scale) as usize;
        let x = self.canvas_x + (self.canvas_w.saturating_sub(w)) / 2;
        let y = self.canvas_y + (self.canvas_h.saturating_sub(h)) / 2;
        (x, y, w.max(1), h.max(1))
    }

    fn render_planning_outline(&self, gfx: &mut DesktopPixelGraphics) {
        let (x, y, w, h) = self.planning_rect();
        let preset = self.target_resolution();

        gfx.draw_rect_outline(x, y, w, h, 0x686874);
        gfx.draw_rect_outline(
            x + 1,
            y + 1,
            w.saturating_sub(2),
            h.saturating_sub(2),
            0x2F8C7D,
        );

        let label = format!("{} target", preset.label);
        gfx.draw_text_bg(x + 8, y.saturating_sub(14), &label, 0xCFCFD8, 0x0A0A0C);

        if !self.show_context_outline || w < 120 || h < 90 {
            return;
        }

        let top_h = (h / 10).clamp(18, 42);
        let side_w = (w / 6).clamp(44, 150);
        let pad = (w / 50).clamp(6, 16);
        let content_x = x + side_w + pad;
        let content_y = y + top_h + pad;
        let content_w = w.saturating_sub(side_w + pad * 2);
        let content_h = h.saturating_sub(top_h + pad * 2);

        gfx.fill_rect(x + 1, y + 1, w.saturating_sub(2), top_h, 0x1A1A20);
        gfx.draw_line(
            x as isize,
            (y + top_h) as isize,
            (x + w) as isize,
            (y + top_h) as isize,
            0x3A3A45,
        );
        gfx.draw_text(x + pad, y + top_h / 2, "APP", 0x777782);

        gfx.draw_rect_outline(
            x + pad,
            y + top_h + pad,
            side_w.saturating_sub(pad * 2),
            content_h,
            0x3A3A45,
        );
        for i in 0..4 {
            let row_y = y + top_h + pad + 12 + i * 22;
            if row_y + 8 < y + h {
                gfx.fill_rect(
                    x + pad * 2,
                    row_y,
                    side_w.saturating_sub(pad * 4),
                    8,
                    0x2A2A32,
                );
            }
        }

        gfx.draw_rect_outline(content_x, content_y, content_w, content_h, 0x3A3A45);
        let card_gap = pad;
        let card_w = content_w.saturating_sub(card_gap * 2) / 3;
        for i in 0..3 {
            let card_x = content_x + i * (card_w + card_gap);
            gfx.draw_rect_outline(
                card_x,
                content_y + pad,
                card_w,
                (content_h / 4).max(28),
                0x444450,
            );
        }

        let main_y = content_y + (content_h / 4).max(28) + pad * 2;
        gfx.draw_rect_outline(
            content_x + pad,
            main_y,
            content_w.saturating_sub(pad * 2),
            content_h.saturating_sub(main_y.saturating_sub(content_y) + pad),
            0x444450,
        );
        for i in 0..5 {
            let line_y = main_y + 14 + i * 18;
            if line_y < y + h - pad {
                gfx.draw_line(
                    (content_x + pad * 2) as isize,
                    line_y as isize,
                    (content_x + content_w.saturating_sub(pad * 2)) as isize,
                    line_y as isize,
                    0x2A2A32,
                );
            }
        }
    }

    fn render_properties(&self, gfx: &mut DesktopPixelGraphics, scr_w: usize, scr_h: usize) {
        let x = scr_w.saturating_sub(RIGHT_PANEL_W);
        gfx.fill_rect(
            x,
            TOP_BAR_H + 1,
            RIGHT_PANEL_W,
            scr_h.saturating_sub(TOP_BAR_H),
            0x151518,
        );
        gfx.draw_text(x + 12, 54, "PROPERTIES", 0xFFFFFF);

        let preset = self.target_resolution();
        gfx.fill_rect(x + 10, 72, RIGHT_PANEL_W - 20, 16, 0x202026);
        gfx.draw_rect_outline(x + 10, 72, RIGHT_PANEL_W - 20, 16, 0x3A3A45);
        gfx.draw_text(x + 16, 76, "Resolution", 0xCFCFD8);
        gfx.draw_text(x + 120, 76, preset.label, 0xFFFFFF);

        gfx.fill_rect(x + 10, 94, RIGHT_PANEL_W - 20, 16, 0x202026);
        gfx.draw_rect_outline(x + 10, 94, RIGHT_PANEL_W - 20, 16, 0x3A3A45);
        gfx.draw_text(x + 16, 98, "App Outline", 0xCFCFD8);
        gfx.draw_text(
            x + 120,
            98,
            if self.show_context_outline {
                "shown"
            } else {
                "hidden"
            },
            0xFFFFFF,
        );

        if let Some(el) = self.selected_element() {
            gfx.draw_text(
                x + 12,
                PROP_LIST_Y - 16,
                &clip_text(el.tool().name, 35),
                0x00FFCC,
            );
            let visible_rows = scr_h.saturating_sub(PROP_LIST_Y + 8) / 18;
            for row in 0..visible_rows {
                let prop_idx = self.property_scroll + row;
                let Some((name, value)) = self.property_name_value(prop_idx) else {
                    break;
                };
                let y = PROP_LIST_Y + row * 18;
                let selected = prop_idx == self.selected_property;
                gfx.fill_rect(
                    x + 10,
                    y,
                    RIGHT_PANEL_W - 20,
                    16,
                    if selected { 0x30303A } else { 0x202026 },
                );
                gfx.draw_rect_outline(
                    x + 10,
                    y,
                    RIGHT_PANEL_W - 20,
                    16,
                    if selected { 0x00FFCC } else { 0x34343D },
                );
                gfx.draw_text(x + 16, y + 4, &clip_text(&name, 12), 0xCFCFD8);
                gfx.draw_text(x + 120, y + 4, &clip_text(&value, 23), 0xFFFFFF);
            }
        } else {
            gfx.draw_text(x + 12, 78, "Select an element", 0x777782);
        }
    }
}

fn render_element_preview(gfx: &mut DesktopPixelGraphics, el: &DesignerElement) {
    let color = el.color_value("color", 0x00FFCC);
    match el.tool().func {
        "draw_pixel" => gfx.fill_rect(el.x, el.y, 5, 5, color),
        "draw_line" | "draw_line_adv" => {
            let x2 = el.usize_value("x2", el.x + el.width);
            let y2 = el.usize_value("y2", el.y + el.height);
            gfx.draw_line(
                el.x as isize,
                el.y as isize,
                x2 as isize,
                y2 as isize,
                color,
            );
        }
        "fill_rect" => gfx.fill_rect(el.x, el.y, el.width, el.height, color),
        "draw_rect_outline" | "draw_rect_outline_adv" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, color)
        }
        "draw_text" => gfx.draw_text(el.x, el.y, &el.string_value("text", "Label"), color),
        "draw_text_bg" => gfx.draw_text_bg(
            el.x,
            el.y,
            &el.string_value("text", "Label"),
            color,
            el.color_value("bg", 0),
        ),
        "draw_button" => gfx.draw_button(
            el.x,
            el.y,
            el.width,
            el.height,
            &el.string_value("text", "Click Me"),
            el.bool_value("is_focused", false),
        ),
        "draw_checkbox" => gfx.draw_checkbox(
            el.x,
            el.y,
            el.bool_value("checked", false),
            el.bool_value("blocked", false),
            el.bool_value("disabled", false),
            &el.string_value("text", "Option"),
        ),
        "draw_tristate_checkbox" => {
            gfx.draw_checkbox(
                el.x,
                el.y,
                el.bool_value("checked", false) || el.bool_value("some", false),
                false,
                false,
                &el.string_value("text", "Tristate"),
            );
        }
        "draw_radio_button" => gfx.draw_radio(el.x, el.y, el.bool_value("checked", false)),
        "draw_slider" => gfx.draw_slider(
            el.x,
            el.y,
            el.width,
            el.usize_value("value", 50),
            el.usize_value("max", 100),
            el.bool_value("vertical", false),
        ),
        "draw_dial" => gfx.draw_dial(
            el.x,
            el.y,
            el.usize_value("radius", 30),
            el.usize_value("value", 25),
            el.usize_value("max", 100),
        ),
        "draw_spinbox" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, 24, 0x888888);
            gfx.draw_text(
                el.x + 5,
                el.y + 8,
                &el.value("value").unwrap_or("0").to_string(),
                0xFFFFFF,
            );
            gfx.draw_text(
                el.x + el.width + 5,
                el.y + 8,
                &el.string_value("label", "Label"),
                0xAAAAAA,
            );
        }
        "draw_double_spinbox" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, 24, 0x888888);
            gfx.draw_text(
                el.x + 5,
                el.y + 8,
                el.value("value").unwrap_or("0.0"),
                0xFFFFFF,
            );
        }
        "draw_progress_bar" => gfx.draw_progress_bar(
            el.x,
            el.y,
            el.width,
            el.height,
            el.usize_value("value", 30),
            el.usize_value("max", 100),
            color,
        ),
        "draw_line_graph" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x444444);
            let points = [12, 24, 16, 44, 30, 62, 50, 82, 68, 90];
            for i in 0..points.len() - 1 {
                let x1 = el.x + i * el.width / (points.len() - 1);
                let x2 = el.x + (i + 1) * el.width / (points.len() - 1);
                let y1 = el.y + el.height - points[i] * el.height / 100;
                let y2 = el.y + el.height - points[i + 1] * el.height / 100;
                gfx.draw_line(x1 as isize, y1 as isize, x2 as isize, y2 as isize, color);
            }
        }
        "draw_lcd_number" => {
            gfx.fill_rect(el.x, el.y, el.width, el.height.max(20), 0x001800);
            gfx.draw_text(
                el.x + 4,
                el.y + 6,
                &el.string_value("value", "0000"),
                0x00FF00,
            );
        }
        "draw_list_view" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x888888);
            for (i, item) in ["Alpha", "Beta", "Gamma"].iter().enumerate() {
                gfx.draw_text(el.x + 8, el.y + 10 + i * 18, item, 0xFFFFFF);
            }
        }
        "draw_table_view" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x888888);
            gfx.draw_line(
                el.x as isize,
                (el.y + 24) as isize,
                (el.x + el.width) as isize,
                (el.y + 24) as isize,
                0x888888,
            );
            gfx.draw_line(
                (el.x + el.width / 2) as isize,
                el.y as isize,
                (el.x + el.width / 2) as isize,
                (el.y + el.height) as isize,
                0x555555,
            );
            gfx.draw_text(el.x + 8, el.y + 8, "Name", 0xAAAAAA);
            gfx.draw_text(el.x + el.width / 2 + 8, el.y + 8, "Value", 0xAAAAAA);
        }
        "draw_tree_view" | "draw_tree_view_icon" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x888888);
            gfx.draw_text(el.x + 8, el.y + 10, "v root", 0xFFFF00);
            gfx.draw_text(el.x + 24, el.y + 28, "- child", 0xFFFFFF);
        }
        "draw_icon" => {
            gfx.fill_rect(el.x, el.y, el.width, el.height, 0x222244);
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x00FFCC);
            gfx.draw_line(
                el.x as isize,
                el.y as isize,
                (el.x + el.width) as isize,
                (el.y + el.height) as isize,
                0x00FFCC,
            );
        }
        "HEADER" => {
            gfx.fill_rect(el.x, el.y, el.width, el.height, 0xAA2222);
            gfx.draw_text(el.x + 8, el.y + 8, "HEADER", 0xFFFFFF);
        }
        "FOOTER" => {
            gfx.fill_rect(el.x, el.y, el.width, el.height, 0xAA2222);
            gfx.draw_text(el.x + 8, el.y + 8, "FOOTER", 0xFFFFFF);
        }
        "WINDOW" => {
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0x00FFCC);
            gfx.fill_rect(el.x, el.y, el.width, 20, 0x24242B);
            gfx.draw_text(el.x + 8, el.y + 6, "WINDOW", 0xFFFFFF);
        }
        _ => {
            gfx.fill_rect(el.x, el.y, el.width, el.height, 0x202026);
            gfx.draw_rect_outline(el.x, el.y, el.width, el.height, 0xAA88FF);
            gfx.draw_text(el.x + 8, el.y + 8, el.tool().name, 0xFFFFFF);
        }
    }
}

struct App {
    width: u32,
    height: u32,
    surface_width: u32,
    surface_height: u32,
    window: Option<Arc<Window>>,
    pixels: Option<Mutex<Pixels<'static>>>,
    designer_app: PixelDesignerApp,
    mouse_x: usize,
    mouse_y: usize,
    mouse_pressed: bool,
    last_render: Instant,
}

impl App {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            surface_width: width,
            surface_height: height,
            window: None,
            pixels: None,
            designer_app: PixelDesignerApp::new(width as usize, height as usize),
            mouse_x: 0,
            mouse_y: 0,
            mouse_pressed: false,
            last_render: Instant::now(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Desktop UI Layout Designer")
                .with_inner_size(LogicalSize::new(self.width as f64, self.height as f64))
                .with_min_inner_size(LogicalSize::new(900.0, 520.0))
                .with_resizable(true);

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let size = window.inner_size();
            self.surface_width = size.width.max(1);
            self.surface_height = size.height.max(1);
            self.width = self.surface_width;
            self.height = self.surface_height;
            self.designer_app
                .resize(self.width as usize, self.height as usize);

            let surface_texture = SurfaceTexture::new(size.width, size.height, Arc::clone(&window));
            let pixels = Pixels::new(self.width, self.height, surface_texture)
                .expect("Pixels initialization failed");

            self.window = Some(window);
            self.pixels = Some(Mutex::new(pixels));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                let width = size.width.max(1);
                let height = size.height.max(1);
                self.surface_width = width;
                self.surface_height = height;
                self.width = width;
                self.height = height;
                self.designer_app.resize(width as usize, height as usize);

                if let Some(pixels_mutex) = &self.pixels {
                    let mut guard = pixels_mutex.lock().unwrap();
                    if let Err(err) = guard.resize_surface(width, height) {
                        eprintln!("Surface resize error: {}", err);
                        event_loop.exit();
                        return;
                    }
                    if let Err(err) = guard.resize_buffer(width, height) {
                        eprintln!("Buffer resize error: {}", err);
                        event_loop.exit();
                        return;
                    }
                }
                window.request_redraw();
            }
            WindowEvent::CursorMoved { position, .. } => {
                let sx = self.width as f64 / self.surface_width.max(1) as f64;
                let sy = self.height as f64 / self.surface_height.max(1) as f64;
                self.mouse_x = (position.x.max(0.0) * sx) as usize;
                self.mouse_y = (position.y.max(0.0) * sy) as usize;
                self.designer_app.process_mouse_interaction(
                    self.mouse_x,
                    self.mouse_y,
                    self.mouse_pressed,
                );
                window.request_redraw();
            }
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => {
                self.mouse_pressed = state == ElementState::Pressed;
                self.designer_app.process_mouse_interaction(
                    self.mouse_x,
                    self.mouse_y,
                    self.mouse_pressed,
                );
                window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.designer_app.handle_key(&event);
                window.request_redraw();
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.designer_app.modifiers = modifiers.state();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels_mutex) = &self.pixels {
                    let mut guard = pixels_mutex.lock().unwrap();
                    let frame = guard.frame_mut();
                    let mut gfx =
                        DesktopPixelGraphics::new(frame, self.width as usize, self.height as usize);
                    self.designer_app.render_application_frame(&mut gfx);

                    if let Err(err) = guard.render() {
                        eprintln!("Render error: {}", err);
                        event_loop.exit();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if self.last_render.elapsed() >= Duration::from_millis(16) {
            self.last_render = Instant::now();
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::new(1280, 720);
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn parse_usize(value: &str) -> Option<usize> {
    value.trim().parse::<usize>().ok()
}

fn parse_color(value: &str) -> Option<u32> {
    let value = value.trim();
    if let Some(hex) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        u32::from_str_radix(hex, 16).ok()
    } else {
        value.parse::<u32>().ok()
    }
}

fn snap(value: usize, grid_size: usize) -> usize {
    if grid_size <= 1 {
        value
    } else {
        (value / grid_size) * grid_size
    }
}

fn clip_text(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        text.to_string()
    } else {
        text.chars()
            .take(max_chars.saturating_sub(1))
            .collect::<String>()
            + "~"
    }
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn unescape_xml(value: &str) -> String {
    value
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}

fn xml_attr(line: &str, name: &str) -> Option<String> {
    let needle = format!("{}=\"", name);
    let start = line.find(&needle)? + needle.len();
    let end = line[start..].find('"')? + start;
    Some(unescape_xml(&line[start..end]))
}

fn parse_elements_from_xml(xml: &str) -> Vec<DesignerElement> {
    let mut elements = Vec::new();
    for line in xml.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('<')
            || trimmed.starts_with("</")
            || trimmed.starts_with("<Pixel")
            || trimmed.starts_with("<Canvas")
            || trimmed.starts_with("<Elements")
        {
            continue;
        }
        let tag_end = trimmed[1..]
            .find(|c: char| c == ' ' || c == '/' || c == '>')
            .map(|idx| idx + 1)
            .unwrap_or(1);
        let tag = &trimmed[1..tag_end];
        let Some(tool_idx) = TOOLS.iter().position(|tool| tool.xml_name == tag) else {
            continue;
        };
        let x = xml_attr(trimmed, "x")
            .and_then(|v| parse_usize(&v))
            .unwrap_or(260);
        let y = xml_attr(trimmed, "y")
            .and_then(|v| parse_usize(&v))
            .unwrap_or(100);
        let mut element = DesignerElement::new(tool_idx, x, y);
        element.width = xml_attr(trimmed, "width")
            .and_then(|v| parse_usize(&v))
            .unwrap_or(element.width);
        element.height = xml_attr(trimmed, "height")
            .and_then(|v| parse_usize(&v))
            .unwrap_or(element.height);
        for (idx, arg) in element.tool().args.iter().enumerate() {
            if let Some(value) = xml_attr(trimmed, arg.name) {
                element.values[idx] = value;
            }
        }
        element.sync_size_from_args();
        elements.push(element);
    }
    elements
}

fn rust_arg(arg: ArgSpec, value: &str) -> String {
    match arg.kind {
        ArgKind::String => format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\"")),
        ArgKind::Bool => {
            if value.eq_ignore_ascii_case("true") || value == "1" {
                "true".to_string()
            } else {
                "false".to_string()
            }
        }
        ArgKind::Color => format!("0x{:X}", parse_color(value).unwrap_or(0)),
        ArgKind::DataRef => {
            if arg.name == "root" {
                "&root".to_string()
            } else if arg.name == "icon" || arg.name == "icon_id" {
                "&ICON_DATA".to_string()
            } else {
                value.to_string()
            }
        }
        ArgKind::F32 => format!("{}f32", value),
        ArgKind::F64 => value.to_string(),
        ArgKind::I32 | ArgKind::U64 | ArgKind::Usize => value.to_string(),
    }
}

fn rust_call_for(el: &DesignerElement) -> String {
    let tool = el.tool();
    match tool.func {
        "draw_line_graph" => format!(
            "gfx.draw_line_graph({}, {}, {}, {}, &graph_data, {}, {}, {});",
            el.x,
            el.y,
            el.width,
            el.height,
            el.value("max_val").unwrap_or("100"),
            rust_arg(LINE_GRAPH_ARGS[3], el.value("color").unwrap_or("0x00FFFF")),
            el.value("len").unwrap_or("10")
        ),
        "draw_list_view" => format!(
            "gfx.draw_list_view({}, {}, {}, {}, &list_items, None);",
            el.x, el.y, el.width, el.height
        ),
        "draw_table_view" => format!(
            "gfx.draw_table_view({}, {}, {}, {}, &table_headers, &table_rows);",
            el.x, el.y, el.width, el.height
        ),
        "HEADER" | "FOOTER" | "WINDOW" => format!(
            "// {} region at {}, {} size {}x{}",
            tool.name, el.x, el.y, el.width, el.height
        ),
        "apply_scanlines" | "apply_dither" | "apply_glitch" => format!("gfx.{}();", tool.func),
        "apply_edge_aberration" => format!(
            "gfx.apply_edge_aberration({});",
            el.value("max_strength").unwrap_or("1.0")
        ),
        _ => {
            let mut args = vec![el.x.to_string(), el.y.to_string()];
            for (arg, value) in tool.args.iter().zip(el.values.iter()) {
                args.push(rust_arg(*arg, value));
            }
            format!("gfx.{}({});", tool.func, args.join(", "))
        }
    }
}
