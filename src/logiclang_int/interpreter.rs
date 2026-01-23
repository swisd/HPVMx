// src/logiclang_int/interpreter.rs


macro_rules! hpvm_log {
    ($color:expr, $prefix:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            // Bring the trait into scope INSIDE the closure
            //use uefi::proto::console::text::Output;
            use core::fmt::Write;

            // let old_attribute = stdout.get_attribute().ok();

            // Set prefix color
            let _ = stdout.set_color($color, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, "[{}] ", $prefix);

            // Reset to white for message
            match $color {
                Color::Yellow => {}
                Color::Red => {}
                _ => {let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);}
            }
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);

            // Restore original attributes if they existed
            // if let Some(attr) = old_attribute {
            //     let _ = stdout.set_attribute(attr);
            // }
        })
    };
}

macro_rules! message {
    ($start:expr, $($arg:tt)*) => {
        uefi::system::with_stdout(|stdout| {
            use core::fmt::Write;
            let _ = stdout.set_color(uefi::proto::console::text::Color::White, uefi::proto::console::text::Color::Black);
            let _ = write!(stdout, $start);
            let _ = write!(stdout, $($arg)*);
            let _ = write!(stdout, "\n");
        })
    }
}

macro_rules! hpvm_info {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::LightCyan, $tag, $($arg)*) };
}

macro_rules! hpvm_warn {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Yellow, $tag, $($arg)*) };
}

// Added this to stop the "unused macro" warning
macro_rules! hpvm_error {
    ($tag:expr, $($arg:tt)*) => { hpvm_log!(Color::Red, $tag, $($arg)*) };
}

use alloc::boxed::Box;
use crate::Color;
use alloc::string::String;
use alloc::vec::Vec;
use crate::logiclang_int::{Lexer, LogicError, Parser};
use alloc::collections::BTreeMap as HashMap;
use crate::logiclang_int::parser::Clause;

pub struct LogicInterpreter {
    clauses: Vec<Vec<String>>,
}

impl LogicInterpreter {
    pub fn new(clauses: Vec<Vec<String>>) -> Self {
        LogicInterpreter { clauses }
    }

    pub fn interpret(text: String) -> Result<Vec<Clause>, LogicError> {
        let mut lexer = Lexer::new(&text);
        let tokens = lexer.tokenize();
        match tokens {
            Ok(tokens) => {
                let mut parser = Parser::new(tokens);
                parser.parse()
            }
            Err(error) => {
                Err(LogicError::string_to_logicerror(error))
            }
        }
        
    }

    // pub fn execute_query(&mut self, query: &str) -> Option<HashMap<String, String>> {
    //     let tokens = Lexer::new(query).tokenize();
    //     match tokens {
    //         Ok(tokens) => {
    //             self.clauses.retain(|c| {
    //                 // Simplified clause matching logic
    //                 // This should be replaced with actual unification logic
    //                 true
    //             });
    //             Some(self.interpret_query(&tokens))
    //         }
    //         Err(e) => {
    //             hpvm_error!("LogicLang", "parse error: {}", e.message());
    //             None
    //         }
    //     }
    // }

    fn interpret_query(&self, tokens: &[String]) -> HashMap<String, String> {
        let mut result = HashMap::new();

        result
    }

    fn unify_clause(&self, clause: &[String], query: &[String]) -> bool {
        if clause.len() != query.len() {
            return false;
        }

        for (c, q) in clause.iter().zip(query.iter()) {
            // Basic unification logic - replace with actual implementation
            if c != q {
                return false;
            }
        }

        true
    }
}


// match self.current_char() {
// 'n' => string.push('\n'),
// 't' => string.push('\t'),
// 'r' => string.push('\r'),
// '"' => string.push('"'),
// '\\' => string.push('\\'),
// _ => string.push(self.current_char()),
// }
// } else {
// string.push(self.current_char());
// }
// self.position += 1;
// }
//
// if self.position >= self.input.len() {
// return Err("unterminated string".to_string());
// }
//
// self.position += 1; // Skip closing quote
// Ok(Token::String(string))
// }
//
// fn read_quoted_atom(&mut self) -> Result<Token, String> {
//     self.position += 1; // Skip opening quote
//     let mut atom = String::new();
//
//     while self.position < self.input.len() && self.current_char() != '\'' {
//         atom.push(self.current_char());
//         self.position += 1;
//     }
//
//     if self.position >= self.input.len() {
//         return Err("unterminated atom".to_string());
//     }
//
//     self.position += 1; // Skip closing quote
//     Ok(Token::Atom(atom))
// }
//
// fn read_identifier(&mut self) -> String {
//     let mut ident = String::new();
//
//     while self.position < self.input.len() {
//         let ch = self.current_char();
//         if ch.is_alphanumeric() || ch == '_' {
//             ident.push(ch);
//             self.position += 1;
//         } else {
//             break;
//         }
//     }
//
//     ident
// }
//
// fn read_number(&mut self) -> Result<Token, String> {
//     let mut number = String::new();
//
//     if self.current_char() == '-' {
//         number.push('-');
//         self.position += 1;
//     }
//
//     while self.position < self.input.len() && self.current_char().is_ascii_digit() {
//         number.push(self.current_char());
//         self.position += 1;
//     }
//
//     number
//         .parse::<i64>()
//         .map(Token::Integer)
//         .map_err(|_| "invalid number".to_string())
// }
//
// fn skip_whitespace_and_comments(&mut self) {
//     while self.position < self.input.len() {
//         let ch = self.current_char();
//
//         if ch.is_whitespace() {
//             self.position += 1;
//         } else if ch == '%' {
//             // Line comment
//             while self.position < self.input.len() && self.current_char() != '\n' {
//                 self.position += 1;
//             }
//         } else if ch == '/' && self.peek_char() == Some('*') {
//             // Block comment
//             self.position += 2;
//             while self.position + 1 < self.input.len() {
//                 if self.current_char() == '*' && self.peek_char() == Some('/') {
//                     self.position += 2;
//                     break;
//                 }
//                 self.position += 1;
//             }
//         } else {
//             break;
//         }
//     }
// }
//
// fn current_char(&self) -> char {
//     self.input
//         .get(self.position)
//         .copied()
//         .unwrap_or('\0')
// }
//
// fn peek_char(&self) -> Option<char> {
//     self.input.get(self.position + 1).copied()
// }
// }