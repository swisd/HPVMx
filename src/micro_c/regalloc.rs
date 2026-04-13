use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;

pub struct RegisterAllocator {
    regs: Vec<String>,
    map: HashMap<String, String>,
    next: usize,
}

impl RegisterAllocator {
    pub fn new(regs: Vec<String>) -> Self {
        Self {
            regs,
            map: HashMap::new(),
            next: 0,
        }
    }

    pub fn alloc(&mut self, temp: &str) -> String {
        if let Some(r) = self.map.get(temp) {
            return r.clone();
        }

        let reg = self.regs[self.next % self.regs.len()].clone();
        self.next += 1;

        self.map.insert(temp.to_string(), reg.clone());
        reg
    }
}