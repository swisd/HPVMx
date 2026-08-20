// #![no_std]
// #![no_main]
//
// use core::hint::spin_loop;
// use uefi::prelude::*;
// use uefi::println;
//
// #[entry]
// fn main() -> Status {
//     uefi::helpers::init().unwrap();
//
//     println!("HPVMx UEFI test app started.");
//     println!("Entering infinite loop for VM run validation...");
//
//     loop {
//         spin_loop();
//     }
// }