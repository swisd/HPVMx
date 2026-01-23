
//! Built-in predicates and command execution for LogicLang

use alloc::format;
use super::parser::Term;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub trait CommandExecutor {
    fn execute(&self, cmd: &str, args: &[Term]) -> Result<String, String>;
}

pub struct HPVMxCommandExecutor;

impl CommandExecutor for HPVMxCommandExecutor {
    fn execute(&self, cmd: &str, args: &[Term]) -> Result<String, String> {
        match cmd {
            // File system commands
            "ls" => Ok("ls executed".to_string()),
            "mkdir" => {
                if let Some(Term::Atom(name)) = args.get(0) {
                    Ok(format!("mkdir {} executed", name))
                } else {
                    Err("mkdir requires directory name".to_string())
                }
            }
            "rm" => {
                if let Some(Term::Atom(file)) = args.get(0) {
                    Ok(format!("rm {} executed", file))
                } else {
                    Err("rm requires file name".to_string())
                }
            }
            "cat" => {
                if let Some(Term::Atom(file)) = args.get(0) {
                    Ok(format!("cat {} executed", file))
                } else {
                    Err("cat requires file name".to_string())
                }
            }

            // VM commands
            "vm_create" => {
                if let (Some(Term::Atom(name)), Some(Term::Integer(mem)), Some(Term::Integer(cpus))) =
                    (args.get(0), args.get(1), args.get(2))
                {
                    Ok(format!("vm_create {} {} {} executed", name, mem, cpus))
                } else {
                    Err("vm_create requires name, memory, and cpus".to_string())
                }
            }
            "vm_start" => {
                if let Some(Term::Integer(id)) = args.get(0) {
                    Ok(format!("vm_start {} executed", id))
                } else {
                    Err("vm_start requires VM ID".to_string())
                }
            }
            "vm_stop" => {
                if let Some(Term::Integer(id)) = args.get(0) {
                    Ok(format!("vm_stop {} executed", id))
                } else {
                    Err("vm_stop requires VM ID".to_string())
                }
            }

            // Boot commands
            "boot" => {
                if let (Some(Term::Integer(id)), Some(Term::Atom(path))) = (args.get(0), args.get(1)) {
                    Ok(format!("boot {} {} executed", id, path))
                } else {
                    Err("boot requires VM ID and path".to_string())
                }
            }

            // Print command
            "print" => {
                let output = args
                    .iter()
                    .map(|t| term_to_string(t))
                    .collect::<Vec<_>>()
                    .join(" ");
                Ok(output)
            }

            _ => Err(format!("unknown command: {}", cmd)),
        }
    }
}

fn term_to_string(term: &Term) -> String {
    match term {
        Term::Atom(a) => a.clone(),
        Term::Variable(v) => format!("_{}", v),
        Term::Integer(i) => i.to_string(),
        Term::String(s) => s.clone(),
        Term::Compound(f, args) => {
            let args_str = args
                .iter()
                .map(term_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", f, args_str)
        }
        Term::List(elements) => {
            let elements_str = elements
                .iter()
                .map(term_to_string)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", elements_str)
        }
        Term::Nil => "[]".to_string(),
    }
}