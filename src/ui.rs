use alloc::string::String;
use core::fmt::Write;
use uefi::proto::console::text::{Key, ScanCode};
use uefi::proto::console::text::Color;

pub struct UI {
    selected_index: usize,
    menu_items: [&'static str; 5],
}

impl UI {
    pub fn new() -> Self {
        UI {
            selected_index: 0,
            menu_items: [
                "System Info",
                "File Manager",
                "Memory Info",
                "Device List",
                "Exit UI",
            ],
        }
    }

    pub fn run(&mut self) {
        uefi::system::with_stdout(|s| {
            let _ = s.clear();
        });

        loop {
            self.render();
            self.handle_input();

            if self.selected_index == 4 {
                // Exit UI
                break;
            }
        }

        uefi::system::with_stdout(|s| {
            let _ = s.clear();
        });
    }

    fn render(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, " HPVMx     v0.1.0                                                                                                         00:00 \n");
            let _ = write!(stdout, "\n\n");
            for (i, item) in self.menu_items.iter().enumerate() {
                if i == self.selected_index {
                    let _ = stdout.set_color(Color::Black, Color::LightGray);
                    let _ = write!(stdout, "  ► {} ◄\n", item);
                    let _ = stdout.set_color(Color::White, Color::Black);
                } else {
                    let _ = write!(stdout, "    {}\n", item);
                }
            }

            let _ = write!(stdout, "\n  [↑↓] Navigate | [Enter] Select\n");
        });
    }

    fn handle_input(&mut self) {
        loop {
            let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
            uefi::boot::wait_for_event(&mut events).unwrap();

            if let Some(key) = uefi::system::with_stdin(|i| i.read_key().unwrap()) {
                match key {
                    Key::Special(ScanCode::UP) => {
                        if self.selected_index > 0 {
                            self.selected_index -= 1;
                        } else {
                            self.selected_index = self.menu_items.len() - 1;
                        }
                        return;
                    }
                    Key::Special(ScanCode::DOWN) => {
                        self.selected_index = (self.selected_index + 1) % self.menu_items.len();
                        return;
                    }
                    Key::Printable(c) => {
                        let ch = char::from(c);
                        if ch == '\r' || ch == '\n' {
                            self.execute_selection();
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn execute_selection(&self) {
        match self.selected_index {
            0 => self.show_system_info(),
            1 => self.show_file_manager(),
            2 => self.show_memory_info(),
            3 => self.show_device_list(),
            _ => {}
        }
    }

    fn show_system_info(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, "\n\nSystem Information\n");
            let _ = write!(stdout, "═════════════════════\n\n");
            let _ = write!(stdout, "HPVMx Version: {}\n", env!("CARGO_PKG_VERSION"));
            let _ = write!(stdout, "\n[Press any key to return...]\n");
        });
        self.wait_for_key();
    }

    fn show_file_manager(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, "\n\nFile Manager (Not Implemented)\n");
            let _ = write!(stdout, "═════════════════════════════════\n\n");
            let _ = write!(stdout, "[Press any key to return...]\n");
        });
        self.wait_for_key();
    }

    fn show_memory_info(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, "\n\nMemory Information\n");
            let _ = write!(stdout, "═════════════════════\n\n");
            let _ = write!(stdout, "[Press any key to return...]\n");
        });
        self.wait_for_key();
    }

    fn show_device_list(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, "\n\nDevice List\n");
            let _ = write!(stdout, "═════════════\n\n");
            let _ = write!(stdout, "[Press any key to return...]\n");
        });
        self.wait_for_key();
    }

    fn network_settings(&self) {
        uefi::system::with_stdout(|stdout| {
            let _ = stdout.clear();
            let _ = write!(stdout, "\n\nNetwork Settings\n");
            let _ = write!(stdout, "═════════════\n\n");
            let _ = write!(stdout, "[Press any key to return...]\n");
        });
        self.wait_for_key();
    }

    fn wait_for_key(&self) {
        loop {
            let mut events = [uefi::system::with_stdin(|i| i.wait_for_key_event().unwrap())];
            uefi::boot::wait_for_event(&mut events).unwrap();

            if uefi::system::with_stdin(|i| i.read_key().unwrap()).is_some() {
                break;
            }
        }
    }
}