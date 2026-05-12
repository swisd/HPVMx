//! MicroC Compiler.
//!
//! A lightweight C compiler targeting multiple architectures.
//! This module includes lexing, parsing, code generation,
//! and architecture-specific backends.

use crate::Color;
use crate::hpvm_log;
use alloc::string::String;
use crate::hpvm_error;
use crate::micro_c::compiler::compile;

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod interpreter;
pub mod ir;
pub mod codegen_ir;
pub mod backend;
pub mod regalloc;
pub mod emitter;
pub mod compiler;
pub mod arch;
pub mod stackframe;
pub mod error;
pub mod fs;

/// Compiles a MicroC source file to assembly.
pub fn compile_from_file_to_asm(srcpath: String) -> String {
    if let Ok(source) = crate::FileSystem::read_file_to_string(&*srcpath) {
        let asm = compile(&*source, "x86_64");
        asm
    } else {
        hpvm_error!("micro-c", "could not open file");
        " ".parse().unwrap()
    }
}