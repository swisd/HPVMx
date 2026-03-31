use alloc::vec::Vec;
use crate::filesystem::FileSystem;
use crate::message;
use crate::micro_c::parser::Parser;

pub fn command(parts: &Vec<&str>) {
    match parts[1] {
        "parse" => {
            if parts.len() < 3 {
                message!("\n", "incorrect arg count")
            } else {
                if let Ok(path) = FileSystem::read_file_to_string(&alloc::format!("{}\\{}", FileSystem::get_cwd().unwrap(), parts[2])) {
                    let mut parser = Parser::new(&*path);
                    parser.parse_all(true);
                } else {
                    message!("\n", "could not open file {}", &alloc::format!("{}\\{}", FileSystem::get_cwd().unwrap(), parts[2]))
                }

            }
        }

        _ => {
            message!("\n", "no command '{}'", parts[1]);
        }
    }
}