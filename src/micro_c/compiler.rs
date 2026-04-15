use alloc::string::{String, ToString};
use crate::arch::Architecture;
use crate::arch::win64::WIN64Backend;
use crate::arch::arm64::ARM64Backend;
use crate::arch::x86_64_raw::X86_64RawBackend;
use crate::codegen_ir::IRGenerator;
use crate::error::error;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn compile(source: &str, arch: &str) -> String {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);

    let ast = parser.parse_program();

    let mut irgen = IRGenerator::new();
    irgen.gen_program(&ast);

    match arch {
        "win64" => {
            let mut backend = WIN64Backend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        }

        "arm64" => {
            let mut backend = ARM64Backend::new();
            backend.emit_program(&irgen.code)
        }

        "x86_64" => {
            let mut backend = X86_64RawBackend::new(irgen.function_params);
            backend.emit_program(&irgen.code)
        }

        _ => {
            error("Unsupported architecture");
            return "".to_string()
        },
    }
}