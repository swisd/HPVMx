//! Device management and drivers.
//!
//! This module provides interfaces for various hardware devices,
//! such as network cards and timers.

pub mod net;
pub mod net_hw;
pub mod net_stack;
pub mod timer;
pub mod audio;
pub mod hda;
#[cfg(feature = "ps2_mouse")]
pub mod ps2;