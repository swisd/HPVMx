
//! # HPVMx Scripting Engine
//!
//! A lightweight scripting language for HPVMx that supports:
//! - **Variables**: `set x = 42`
//! - **Arguments**: `%%0`, `%%1`, `%%2` (argc/argv style)
//! - **If/elif/else**: Conditional branching
//! - **For/while loops**: Iteration with break/continue
//! - **Commands**: Execute terminal commands
//! - **Echo**: Output values
//! - **Advanced errors**: Detailed diagnostics with source context and hints

pub mod error;
pub mod lexer;
pub mod ast;
pub mod parser;
pub mod interpreter;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use error::{ScriptError, ScriptErrorKind, ScriptResult};
use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use crate::vdebug_autoprefix;

/// Parse and execute a script from source code.
///
/// # Arguments
/// - `source`: The script source code
/// - `file`: The script file name (for error reporting)
/// - `arguments`: Positional arguments (%%1, %%2, etc.)
///
/// # Returns
/// The exit code if the script called `exit`, or `None` if it completed normally.
pub fn run_script(source: &str, file: &str, arguments: Vec<String>) -> ScriptResult<Option<i64>> {
    // Phase 1: Lex
    let tokens = Lexer::new(source, file).tokenize()?;
    
    // Phase 2: Parse
    let script = Parser::new(tokens, file, source).parse()?;
    
    // Phase 3: Execute
    let mut interp = Interpreter::new(arguments);
    let result = interp.execute_script(&script)?;
    
    Ok(result)
}

/// Run a script from a file path, reading from the filesystem.
pub fn run_script_from_file(path: &str, arguments: Vec<String>) -> ScriptResult<Option<i64>> {
    let source = crate::filesystem::FileSystem::read_file_to_string(path).map_err(|e| {
        ScriptError::new(
            ScriptErrorKind::FileError,
            alloc::format!("Failed to read script file '{}': {}", path, e),
            path,
            0,
            0,
        ).with_hint(String::from("Ensure the file exists and is readable"))
    })?;
    
    run_script(&source, path, arguments)
}

/// Print a formatted script error to the terminal.
pub fn print_script_error(error: &ScriptError) {
    let detailed = error.format_detailed();
    vdebug_autoprefix!(4, "{}", format!("{}: {}", error.kind.tag(), detailed.trim()));
}
