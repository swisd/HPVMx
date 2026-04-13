use crate::Color;
use crate::hpvm_log;
use crate::hpvm_warn;

pub fn error(message: &str) {
    hpvm_warn!("MicroC", "!! {}", message)
}