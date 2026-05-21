use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use font8x8::UnicodeFonts;
use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

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

        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;

        if idx + 3 < self.buffer.len() {
            self.buffer[idx] = r;
            self.buffer[idx + 1] = g;
            self.buffer[idx + 2] = b;
            self.buffer[idx + 3] = 255;
        }
    }

    pub fn fill_rect(&mut self, x: usize, y: usize, w: usize, h: usize, color: u32) {
        let max_x = (x + w).min(self.width);
        let max_y = (y + h).min(self.height);

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

        for px in x..(x + w) {
            self.set_pixel(px, y, color);
            self.set_pixel(px, y + h - 1, color);
        }

        for py in y..(y + h) {
            self.set_pixel(x, py, color);
            self.set_pixel(x + w - 1, py, color);
        }
    }

    pub fn draw_line(
        &mut self,
        mut x0: isize,
        mut y0: isize,
        x1: isize,
        y1: isize,
        color: u32,
    ) {
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

    pub fn draw_button(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        label: &str,
        focused: bool,
    ) {
        self.fill_rect(x, y, w, h, if focused { 0x3A3A45 } else { 0x24242B });
        self.draw_rect_outline(x, y, w, h, 0x00FFCC);
        self.draw_text(x + 10, y + (h / 2) - 4, label, 0xFFFFFF);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DesignerToolType {
    FillRect,
    Button,
}

impl DesignerToolType {
    pub fn xml_tag(&self) -> &'static str {
        match self {
            DesignerToolType::FillRect => "FillRect",
            DesignerToolType::Button => "Button",
        }
    }
}

pub struct DesignerElement {
    pub tool_type: DesignerToolType,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub label: String,
    pub color: u32,
}

pub struct PixelDesignerApp {
    pub elements: Vec<DesignerElement>,
    pub selected_idx: Option<usize>,
    pub grid_size: usize,
    pub canvas_x: usize,
    pub canvas_y: usize,
    pub canvas_w: usize,
    pub canvas_h: usize,
    pub is_dragging: bool,
    pub drag_offset_x: i32,
    pub drag_offset_y: i32,
    pub previous_mouse_down: bool,
}

impl PixelDesignerApp {
    pub fn new(screen_w: usize, screen_h: usize) -> Self {
        Self {
            elements: Vec::new(),
            selected_idx: None,
            grid_size: 20,
            canvas_x: 210,
            canvas_y: 50,
            canvas_w: screen_w.saturating_sub(460),
            canvas_h: screen_h.saturating_sub(70),
            is_dragging: false,
            drag_offset_x: 0,
            drag_offset_y: 0,
            previous_mouse_down: false,
        }
    }

    pub fn spawn_element(&mut self, tool: DesignerToolType, x: usize, y: usize) {
        let (w, h) = match tool {
            DesignerToolType::FillRect => (120, 60),
            DesignerToolType::Button => (100, 40),
        };

        self.elements.push(DesignerElement {
            tool_type: tool,
            x,
            y,
            width: w,
            height: h,
            label: String::from("Component"),
            color: 0x00FFCC,
        });
    }

    pub fn process_mouse_interaction(
        &mut self,
        mx: usize,
        my: usize,
        mouse_down: bool,
    ) {
        let just_pressed = mouse_down && !self.previous_mouse_down;
        self.previous_mouse_down = mouse_down;

        if !mouse_down {
            self.is_dragging = false;
            return;
        }

        if self.is_dragging {
            if let Some(idx) = self.selected_idx {
                let el = &mut self.elements[idx];

                let nx = (mx as i32 + self.drag_offset_x).max(0) as usize;
                let ny = (my as i32 + self.drag_offset_y).max(0) as usize;

                el.x = ((nx / self.grid_size) * self.grid_size)
                    .clamp(self.canvas_x, self.canvas_x + self.canvas_w - el.width);

                el.y = ((ny / self.grid_size) * self.grid_size)
                    .clamp(self.canvas_y, self.canvas_y + self.canvas_h - el.height);
            }

            return;
        }

        if !just_pressed {
            return;
        }

        for (idx, el) in self.elements.iter().enumerate().rev() {
            if mx >= el.x
                && mx <= el.x + el.width
                && my >= el.y
                && my <= el.y + el.height
            {
                self.selected_idx = Some(idx);
                self.is_dragging = true;
                self.drag_offset_x = el.x as i32 - mx as i32;
                self.drag_offset_y = el.y as i32 - my as i32;
                return;
            }
        }

        if mx >= 10 && mx <= 190 && my >= 80 && my <= 104 {
            self.spawn_element(
                DesignerToolType::FillRect,
                self.canvas_x + 40,
                self.canvas_y + 40,
            );
            return;
        }

        if mx >= 10 && mx <= 190 && my >= 110 && my <= 134 {
            self.spawn_element(
                DesignerToolType::Button,
                self.canvas_x + 40,
                self.canvas_y + 120,
            );
        }
    }

    pub fn render_application_frame(&self, gfx: &mut DesktopPixelGraphics) {
        let (scr_w, scr_h) = gfx.resolution();

        gfx.fill_rect(0, 0, scr_w, scr_h, 0x0A0A0C);

        gfx.fill_rect(0, 0, scr_w, 40, 0x1F1F24);
        gfx.draw_text(15, 12, "PIXEL DESIGNER", 0x00FFCC);

        gfx.fill_rect(0, 41, 200, scr_h - 41, 0x17171A);

        gfx.draw_text(10, 55, "TOOLS", 0xFFFFFF);

        gfx.fill_rect(10, 80, 180, 24, 0x24242B);
        gfx.draw_rect_outline(10, 80, 180, 24, 0x3A3A45);
        gfx.draw_text(18, 88, "FillRect", 0xFFFFFF);

        gfx.fill_rect(10, 110, 180, 24, 0x24242B);
        gfx.draw_rect_outline(10, 110, 180, 24, 0x3A3A45);
        gfx.draw_text(18, 118, "Button", 0xFFFFFF);

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

        for (idx, el) in self.elements.iter().enumerate() {
            match el.tool_type {
                DesignerToolType::FillRect => {
                    gfx.fill_rect(el.x, el.y, el.width, el.height, el.color);
                }
                DesignerToolType::Button => {
                    gfx.draw_button(el.x, el.y, el.width, el.height, &el.label, false);
                }
            }

            if Some(idx) == self.selected_idx {
                gfx.draw_rect_outline(
                    el.x.saturating_sub(2),
                    el.y.saturating_sub(2),
                    el.width + 4,
                    el.height + 4,
                    0xFFFFFF,
                );

                gfx.fill_rect(
                    el.x + el.width - 5,
                    el.y + el.height - 5,
                    8,
                    8,
                    0xFFFFFF,
                );
            }
        }
    }
}

struct App {
    width: u32,
    height: u32,
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
        let mut designer_app = PixelDesignerApp::new(width as usize, height as usize);

        designer_app.spawn_element(DesignerToolType::Button, 260, 100);

        Self {
            width,
            height,
            window: None,
            pixels: None,
            designer_app,
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
                .with_resizable(false);

            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

            let size = window.inner_size();

            let surface_texture =
                SurfaceTexture::new(size.width, size.height, Arc::clone(&window));

            let pixels = Pixels::new(self.width, self.height, surface_texture)
                .expect("Pixels initialization failed");

            self.window = Some(window);
            self.pixels = Some(Mutex::new(pixels));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_x = position.x as usize;
                self.mouse_y = position.y as usize;

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

            WindowEvent::RedrawRequested => {
                if let Some(pixels_mutex) = &self.pixels {
                    let mut guard = pixels_mutex.lock().unwrap();

                    let frame = guard.frame_mut();

                    let mut gfx = DesktopPixelGraphics::new(
                        frame,
                        self.width as usize,
                        self.height as usize,
                    );

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
