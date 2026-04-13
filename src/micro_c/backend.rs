use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use serde::Deserialize;
//use std::fs;

// #[derive(Debug, Deserialize)]
// pub struct TargetSpec {
//     pub registers: Vec<String>,
//     pub instructions: HashMap<String, String>,
// }

// pub fn load_target(path: &str) -> TargetSpec {
//     if let Ok(text) = fs::read_to_string(path) {
//         serde_yaml::from_str(&text).unwrap()
//     } else {
//         panic!("cannot find specified target of '{}'. are you sure the path is correct?", path)
//     }
// }