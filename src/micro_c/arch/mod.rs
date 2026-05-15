//! Architecture-specific backends for code generation.
//!
//! This module defines the [`Architecture`] trait that all backends must
//! implement to translate IR into target assembly.

pub(crate) mod win64;
pub(crate) mod arm64;
pub(crate) mod x86_64_raw;

use alloc::string::String;
use crate::ir::IRInst;

/// Common trait for all target architectures.
pub trait Architecture {
    /// Translates a sequence of IR instructions into target assembly code.
    fn emit_program(&mut self, ir: &[IRInst]) -> String;
}