use crate::Color;
use crate::{hpvm_log, hpvm_warn};

pub struct XorShiftRng {
    state: u64,
}

impl XorShiftRng {
    // Function to create a new RNG with a starting seed
    pub const fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // The core PRNG logic (Xorshift* variant)
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12; // a
        x ^= x << 25; // b
        x ^= x >> 27; // c
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn rand(&mut self, len: u8) -> u64 {
        if len > 19 {
            hpvm_warn!("RNG", "length out of range for u64");
            0
        } else {
            let it = 1;
            let mut num = 0;
            while it <= len {
                num += self.next_u64() * ((10 * it) as u64);
            }
            num
        }
    }

    pub fn rand_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min < max, "min must be less than max");
        let range = max - min;
        min + (self.next_u64() % range)
    }
}