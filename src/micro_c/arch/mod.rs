pub(crate) mod win64;
pub(crate) mod arm64;
pub(crate) mod x86_64_raw;

use alloc::string::String;
use crate::ir::IRInst;

pub trait Architecture {
    fn emit_program(&mut self, ir: &[IRInst]) -> String;
}