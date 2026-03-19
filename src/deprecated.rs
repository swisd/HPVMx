#[deprecated]
pub struct Window {
    pub title: String,
    pub rect: Rect,
    pub active: bool,
}

impl Window {
    pub fn new(title: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Window {
            title: String::from(title),
            rect: Rect::new(x, y, width, height),
            active: true,
        }
    }

    pub fn draw(&self) {
        if let Some(pg) = pixel_graphics::PixelGraphics::new() {
            let mut pg = pg;
            pg.fill_rect(self.rect.x * 8, self.rect.y * 16, self.rect.width * 8, self.rect.height * 16, 0xCCCCCC);
            pg.draw_text(self.rect.x * 8 + 4, self.rect.y * 16 + 4, &self.title, 0x000000);
        } else {
            Graphics::draw_box(&self.rect, &self.title, self.active);
        }
    }
}
#[deprecated]
pub struct Button {
    pub label: String,
    pub rect: Rect,
    pub focused: bool,
    pub clicked: bool,
}

impl Button {
    pub fn new(label: &str, x: usize, y: usize, width: usize, height: usize) -> Self {
        Button {
            label: String::from(label),
            rect: Rect::new(x, y, width, height),
            focused: false,
            clicked: false,
        }
    }

    pub fn draw(&self) {
        if let Some(pg) = pixel_graphics::PixelGraphics::new() {
            let mut pg = pg;
            let bg = if self.focused { 0xFFFFFF } else { 0xBBBBBB };
            pg.fill_rect(self.rect.x * 8, self.rect.y * 16, self.rect.width * 8, self.rect.height * 16, bg);
            pg.draw_text(self.rect.x * 8 + 10, self.rect.y * 16 + 8, &self.label, 0x000000);
        } else {
            Graphics::draw_button(&self.rect, &self.label, self.focused);
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        self.rect.contains(x, y)
    }
}
#[deprecated]
pub struct TextBox {
    pub text: String,
    pub rect: Rect,
    pub focused: bool,
    pub max_length: usize,
}

impl TextBox {
    pub fn new(x: usize, y: usize, width: usize, max_length: usize) -> Self {
        TextBox {
            text: String::new(),
            rect: Rect::new(x, y, width, 3),
            focused: false,
            max_length,
        }
    }

    pub fn draw(&self) {
        Graphics::draw_textbox(&self.rect, &self.text, self.focused);
    }

    pub fn add_char(&mut self, ch: char) {
        if self.text.len() < self.max_length {
            self.text.push(ch);
        }
    }

    pub fn backspace(&mut self) {
        self.text.pop();
    }
}
#[deprecated]
pub struct ListBox {
    pub items: Vec<String>,
    pub rect: Rect,
    pub selected: usize,
}

impl ListBox {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        ListBox {
            items: Vec::new(),
            rect: Rect::new(x, y, width, height),
            selected: 0,
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }

    pub fn draw(&self) {
        let items_str: Vec<&str> = self.items.iter().map(|s| s.as_str()).collect();
        Graphics::draw_list(&self.rect, &items_str, self.selected);
    }

    pub fn select_next(&mut self) {
        if self.selected < self.items.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn select_prev(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn get_selected(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }
}
#[deprecated]
pub struct WinNTShell {
    pub windows: Vec<Window>,
    pub buttons: Vec<Button>,
    pub textboxes: Vec<TextBox>,
    pub listbox: Option<ListBox>,
    pub focused_button: usize,
    pub focused_textbox: usize,
    focus_target: FocusTarget,
    exit_requested: bool,
}

#[deprecated]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FocusTarget {
    TextBox,
    Button,
    ListBox,
}

impl WinNTShell {
    pub fn new() -> Self {
        WinNTShell {
            windows: Vec::new(),
            buttons: Vec::new(),
            textboxes: Vec::new(),
            listbox: None,
            focused_button: 0,
            focused_textbox: 0,
            focus_target: FocusTarget::TextBox,
            exit_requested: false,
        }
    }

    pub fn init_desktop(&mut self) {
        // Clear screen with gray background
        Graphics::clear_screen(Color::LightGray);

        // Draw menu bar
        Graphics::draw_menu_bar(&["File", "Edit", "View", "Help"]);

        // Draw taskbar
        Graphics::draw_taskbar("12:00");

        // Create main shell window
        let shell_window = Window::new("HPVMx Shell", 5, 3, 70, 18);
        self.windows.push(shell_window);

        // Create OK and Cancel buttons
        let ok_button = Button::new("OK", 35, 20, 8, 2);
        let cancel_button = Button::new("Cancel", 45, 20, 10, 2);

        self.buttons.push(ok_button);
        self.buttons.push(cancel_button);

        // Create a command input textbox
        let input_box = TextBox::new(7, 5, 66, 50);
        self.textboxes.push(input_box);

        // Create output listbox
        let mut output_list = ListBox::new(7, 8, 66, 10);
        output_list.add_item("HPVMx v0.1.0 - Shell Interface");
        output_list.add_item("Type 'help' for available commands");
        self.listbox = Some(output_list);

        if !self.textboxes.is_empty() {
            self.focus_textbox(0);
        }
        self.exit_requested = false;
    }

    pub fn draw(&self) {
        for window in &self.windows {
            window.draw();
        }

        for button in &self.buttons {
            button.draw();
        }

        for textbox in &self.textboxes {
            textbox.draw();
        }

        if let Some(ref listbox) = self.listbox {
            listbox.draw();
        }
    }

    pub fn focus_button(&mut self, idx: usize) {
        for (i, button) in self.buttons.iter_mut().enumerate() {
            button.focused = i == idx;
        }
        self.focused_button = idx;
        self.focus_target = FocusTarget::Button;
    }

    pub fn focus_textbox(&mut self, idx: usize) {
        for (i, textbox) in self.textboxes.iter_mut().enumerate() {
            textbox.focused = i == idx;
        }
        self.focused_textbox = idx;
        self.focus_target = FocusTarget::TextBox;
    }

    fn focus_listbox(&mut self) {
        self.focus_target = FocusTarget::ListBox;
    }

    fn cycle_focus(&mut self) {
        let has_textbox = !self.textboxes.is_empty();
        let has_button = !self.buttons.is_empty();
        let has_listbox = self.listbox.is_some();

        let mut next = self.focus_target;
        let mut attempts = 0;
        while attempts < 3 {
            next = match next {
                FocusTarget::TextBox => FocusTarget::Button,
                FocusTarget::Button => FocusTarget::ListBox,
                FocusTarget::ListBox => FocusTarget::TextBox,
            };
            if (next == FocusTarget::TextBox && has_textbox)
                || (next == FocusTarget::Button && has_button)
                || (next == FocusTarget::ListBox && has_listbox)
            {
                break;
            }
            attempts += 1;
        }

        match next {
            FocusTarget::TextBox => {
                let idx = self.focused_textbox.min(self.textboxes.len().saturating_sub(1));
                self.focus_textbox(idx);
            }
            FocusTarget::Button => {
                let idx = self.focused_button.min(self.buttons.len().saturating_sub(1));
                self.focus_button(idx);
            }
            FocusTarget::ListBox => self.focus_listbox(),
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Printable(c) => {
                let ch = char::from(c);
                if ch.to_ascii_lowercase() == 'q' {
                    self.exit_requested = true;
                    return;
                }
                if ch == '\t' {
                    self.cycle_focus();
                    return;
                }
                if self.focus_target == FocusTarget::TextBox && self.focused_textbox < self.textboxes.len() {
                    if ch != '\r' && ch != '\n' {
                        self.textboxes[self.focused_textbox].add_char(ch);
                    }
                }
            }
            Key::Special(ScanCode::DELETE) => {
                if self.focus_target == FocusTarget::TextBox && self.focused_textbox < self.textboxes.len() {
                    self.textboxes[self.focused_textbox].backspace();
                }
            }
            Key::Special(ScanCode::UP) => {
                if self.focus_target == FocusTarget::ListBox {
                    if let Some(ref mut listbox) = self.listbox {
                        listbox.select_prev();
                    }
                } else if self.focus_target == FocusTarget::Button && !self.textboxes.is_empty() {
                    self.focus_textbox(self.focused_textbox);
                }
            }
            Key::Special(ScanCode::DOWN) => {
                if self.focus_target == FocusTarget::ListBox {
                    if let Some(ref mut listbox) = self.listbox {
                        listbox.select_next();
                    }
                } else if self.focus_target == FocusTarget::TextBox && self.listbox.is_some() {
                    self.focus_listbox();
                }
            }
            Key::Special(ScanCode::LEFT) => {
                if self.focus_target == FocusTarget::Button && !self.buttons.is_empty() {
                    let next_button = if self.focused_button == 0 {
                        self.buttons.len() - 1
                    } else {
                        self.focused_button - 1
                    };
                    self.focus_button(next_button);
                } else if self.focus_target == FocusTarget::ListBox && !self.buttons.is_empty() {
                    self.focus_button(self.focused_button);
                }
            }
            Key::Special(ScanCode::RIGHT) => {
                if self.focus_target == FocusTarget::Button && !self.buttons.is_empty() {
                    let next_button = (self.focused_button + 1) % self.buttons.len();
                    self.focus_button(next_button);
                } else if self.focus_target == FocusTarget::TextBox && !self.buttons.is_empty() {
                    self.focus_button(self.focused_button);
                }
            }
            Key::Special(ScanCode::END) => {
                self.cycle_focus();
            }
            Key::Special(ScanCode::HOME) => {
                if self.focus_target == FocusTarget::Button && self.focused_button < self.buttons.len() {
                    let label = self.buttons[self.focused_button].label.clone();
                    self.buttons[self.focused_button].clicked = true;
                    if label.eq_ignore_ascii_case("ok") {
                        if let Some(input) = self.get_input() {
                            if !input.is_empty() {
                                self.add_output(&alloc::format!("> {}", input));
                                self.clear_input();
                            }
                        }
                    } else if label.eq_ignore_ascii_case("cancel") {
                        self.clear_input();
                    } else {
                        self.add_output(&alloc::format!("[{}] pressed", label));
                    }
                }
            }
            _ => {}
        }
    }

    pub fn get_input(&self) -> Option<String> {
        if self.textboxes.len() > 0 {
            Some(self.textboxes[0].text.clone())
        } else {
            None
        }
    }

    pub fn clear_input(&mut self) {
        if self.textboxes.len() > 0 {
            self.textboxes[0].text.clear();
        }
    }

    pub fn add_output(&mut self, text: &str) {
        if let Some(ref mut listbox) = self.listbox {
            listbox.add_item(text);
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub  fn process_mouse(){
        //Graphics::get_cursor()
        return;
    }
}