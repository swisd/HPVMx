pub type BYTEARRAY = [u8; 16];
pub type WORDARRAY = [u16; 8];
pub type DWORDARRAY = [u32; 4];
pub type QWORDARRAY = [u64; 2];

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
    let _ = uefi::boot::stall(core::time::Duration::from_millis(ms));
}