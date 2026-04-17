use alloc::string::String;
use alloc::vec::Vec;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Runnable};
use crate::ui::pixel_graphics::{icons, PixelGraphics};

#[derive(Clone, Copy, PartialEq)]
enum Direction { Up, Down, Left, Right }

pub struct SnakeApp {
    snake: Vec<(isize, isize)>,
    food: (isize, isize),
    dir: Direction,
    next_dir: Direction,
    game_over: bool,
    move_timer: u8,
}

impl SnakeApp {
    pub fn new() -> Self {
        let mut snake = Vec::new();
        snake.push((10, 10)); // Start in the middle of the 20x20 grid
        Self {
            snake,
            food: (5, 5),
            dir: Direction::Right,
            next_dir: Direction::Right,
            game_over: false,
            move_timer: 0,
        }
    }
}

impl AppInfo for SnakeApp {
    fn name(&self) -> &str {
        "Snake"
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn icon(&self) -> [u32; 1024] {
        icons::SNAKE_32_ICON_DATA
    }

    fn dimensions(&self) -> (usize, usize) {
        (200, 200)
    }
}

impl Runnable for SnakeApp {
    fn logic(&mut self, _vars: &mut Vec<String>) {
        if self.game_over { return; }

        // Control the speed: only move every 5 frames (~12 FPS)
        self.move_timer += 1;
        if self.move_timer < 5 { return; }
        self.move_timer = 0;

        self.dir = self.next_dir;
        let head = self.snake[0];
        let mut new_head = match self.dir {
            Direction::Up    => (head.0, head.1 - 1),
            Direction::Down  => (head.0, head.1 + 1),
            Direction::Left  => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        // Screen wrapping (20x20 grid for a 200x200 window)
        if new_head.0 < 0 { new_head.0 = 19; }
        if new_head.0 > 19 { new_head.0 = 0; }
        if new_head.1 < 0 { new_head.1 = 19; }
        if new_head.1 > 19 { new_head.1 = 0; }

        // Check collision with self
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.insert(0, new_head);

        // Check food collision
        if new_head == self.food {
            // "Generate" new food (ideally use your RNG here)
            self.food = ((new_head.0 + 7) % 20, (new_head.1 + 3) % 20);
        } else {
            self.snake.pop(); // Remove tail if no food eaten
        }
    }

    fn draw(&self, graphics: &mut PixelGraphics, _vars: &Vec<String>, x: usize, y: usize) {
        // Clear background
        graphics.fill_rect(x, y, 200, 200, 0x000000);

        // Draw Food (Red)
        graphics.fill_rect(x + (self.food.0 as usize * 10), y + (self.food.1 as usize * 10), 8, 8, 0xFF0000);

        // Draw Snake (Green)
        for (idx, &(sx, sy)) in self.snake.iter().enumerate() {
            let color = if idx == 0 { 0x00FF00 } else { 0x00AA00 };
            graphics.fill_rect(x + (sx as usize * 10), y + (sy as usize * 10), 9, 9, color);
        }

        if self.game_over {
            // Draw a simple "X" or message if your graphics engine supports it
            graphics.draw_line(x, y, x + 200, y + 200, 0xFFFFFF);
            graphics.draw_line(x + 200, y, x, y + 200, 0xFFFFFF);
        }
    }

    fn input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => match u16::from(c) as u8 as char {
                'w' | 'W' if self.dir != Direction::Down  => self.next_dir = Direction::Up,
                's' | 'S' if self.dir != Direction::Up    => self.next_dir = Direction::Down,
                'a' | 'A' if self.dir != Direction::Right => self.next_dir = Direction::Left,
                'd' | 'D' if self.dir != Direction::Left  => self.next_dir = Direction::Right,
                'r' | 'R' if self.game_over               => *self = Self::new(),
                _ => {}
            },
            _ => {}
        }
    }
}