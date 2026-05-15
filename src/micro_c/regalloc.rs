//! Simple register allocator.
//!
//! This module provides a basic register allocation strategy for backends,
//! mapping temporaries from the IR to physical registers.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;

/// Manages the mapping of IR temporaries to physical registers.
pub struct RegisterAllocator {
    regs: Vec<String>,
    map: HashMap<String, String>,
    next: usize,
}

impl RegisterAllocator {
    /// Creates a new RegisterAllocator with a set of available registers.
    pub fn new(regs: Vec<String>) -> Self {
        Self {
            regs,
            map: HashMap::new(),
            next: 0,
        }
    }

    /// Allocates a register for a given temporary.
    ///
    /// If the temporary already has an allocated register, it is returned.
    /// Otherwise, a new register is assigned from the available pool using
    /// a simple round-robin strategy.
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