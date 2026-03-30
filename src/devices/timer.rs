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