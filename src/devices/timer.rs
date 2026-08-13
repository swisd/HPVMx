use crate::types::WORDARRAY;

pub type Time = WORDARRAY;

pub struct Timer {
    time: Time,

}

impl Timer {
    pub fn new() -> Timer {
        Timer { time: [0, 0, 0, 0, 0, 0, 0, 0] }
    }
}

pub fn sleep_ms(ms: u64) {
    let _ = uefi::boot::stall(core::time::Duration::from_millis(ms).as_micros() as usize);
}