//! Host-side VM management boundary.
//!
//! This module deliberately contains no guest agent or guest operating-system
//! protocol. A network listener can decode REST/JSON or WebSocket traffic into
//! [`IPCMessage`] and submit it to [`VmPort`].

use crate::vdebug_autoprefix;

pub mod interface;
pub mod ap;
pub mod rqproc;
pub mod lut;
pub mod types;
pub mod vmdispatch;

pub use interface::{HostVmBackend, HostVmError, HypervisorBackend};
pub use lut::{CidLut, ClientDataLut, InstanceLut, PortLut};
pub use rqproc::{RequestProcessor, RequestResult};
pub use types::*;
pub use vmdispatch::{GuardError, HeartbeatSource, PortError, SmRingBuf, VmGuard, VmPort, VmPortResponse};

pub fn init_hosting_subsystem() {
    vdebug_autoprefix!("Hosting subsystem initialized (VM, Container, CVM, Port LUT, Client LUT, VirtCOM)");
}
