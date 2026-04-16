//! Package Management system.
//!
//! This module handles package installation, dependency management,
//! and command execution for system packages.

mod manager;
mod commands;

pub(crate) use manager::{PackageManager, Package, PackageType};
pub(crate) use commands::command;