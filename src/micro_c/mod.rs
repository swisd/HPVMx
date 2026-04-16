//! MicroC Compiler.
//!
//! A lightweight C compiler targeting multiple architectures.
//! This module includes lexing, parsing, code generation,
//! and architecture-specific backends.

use alloc::string::String;
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


/// Compiles a MicroC source file to assembly.
pub fn compile_from_file_to_asm(srcpath: String) -> String {
    let source = crate::FileSystem::read_file_to_string(&*srcpath).unwrap();
    let asm = compile(&*source, "x86_64");
    asm
}