use crate::{message, Color};
use crate::hpvm_log;
use alloc::vec::Vec;
use core::ptr::addr_of_mut;
use crate::{filesystem, hpvm_warn};
use crate::pm::{PackageManager, Package, PackageType};



pub fn command(parts: &Vec<&str>, package_manager: &mut PackageManager) {
    match parts[1] {

        "list" => {
            package_manager.list_packages();
        }

        "reload" => {
            package_manager.load_registry();
        }

        "verify" => {
            if !parts.len() == 3 {
                message!("\n", "usage: pm verify [package-name]");
            }
            package_manager.verify_dependencies(parts[2]);
        }

        "version" => {
            message!("", "{}", package_manager.get_version());
        }

        _ => { hpvm_warn!("pm", "no arg named {}", parts[1]) }
    }
}





