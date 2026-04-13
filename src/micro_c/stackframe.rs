use alloc::string::{String, ToString};
use hashbrown::HashMap;

pub struct StackFrame {
    pub(crate) offsets: HashMap<String, i32>,
    next_offset: i32,
}

impl StackFrame {
    pub fn new() -> Self {
        Self {
            offsets: HashMap::new(),
            next_offset: 8,
        }
    }

    pub fn alloc(&mut self, name: &str) -> i32 {
        if let Some(o) = self.offsets.get(name) {
            return *o;
        }

        let offset = self.next_offset;
        self.offsets.insert(name.to_string(), offset);
        self.next_offset += 8;
        offset
    }

    pub fn get(&self, name: &str) -> i32 {
        *self.offsets.get(name)
            .unwrap_or_else(|| panic!("Unknown stack var: {}", name))
    }

    pub fn frame_size(&self) -> i32 {
        ((self.next_offset + 15) / 16) * 16
    }
}