//! Typed port I/O boundary shared by VMs, CVMs, and containers.
//!
//! No unit can access host devices directly through this abstraction. A port
//! must be explicitly registered by the owning VMM component first.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortOwner { Vm(u32), Cvm(u32), Container(u64) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortDirection { Read, Write }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortIoEvent {
    pub owner: PortOwner,
    pub port: u16,
    pub direction: PortDirection,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortIoError { InvalidWidth, UnmappedPort, ReadOnlyPort }

#[derive(Debug, Clone)]
pub(crate) struct PortRegister { value: Vec<u8>, writable: bool }

/// In-memory port model used by virtual device implementations.
pub struct PortIoInterface {
    owner: PortOwner,
    ports: BTreeMap<u16, PortRegister>,
    events: Vec<PortIoEvent>,
}

impl PortIoInterface {
    pub fn new(owner: PortOwner) -> Self {
        vdebug_autoprefix!("PortIoInterface: initialized for owner {:?}", owner);
        Self { owner, ports: BTreeMap::new(), events: Vec::new() }
    }
    pub fn owner(&self) -> PortOwner { self.owner }
    pub fn register_port(&mut self, port: u16, width: usize, writable: bool) -> Result<(), PortIoError> {
        validate_width(width)?;
        vdebug_autoprefix!("PortIoInterface: register port 0x{:04x} (width {}, writable {}) for {:?}", port, width, writable, self.owner);
        self.ports.insert(port, PortRegister { value: alloc::vec![0; width], writable });
        Ok(())
    }
    pub fn read(&mut self, port: u16, width: usize) -> Result<Vec<u8>, PortIoError> {
        validate_width(width)?;
        let value = self.ports.get(&port).ok_or(PortIoError::UnmappedPort)?.value.clone();
        if value.len() != width { return Err(PortIoError::InvalidWidth); }
        vdebug_autoprefix!("PortIoInterface: read from port 0x{:04x} by {:?}", port, self.owner);
        self.events.push(PortIoEvent { owner: self.owner, port, direction: PortDirection::Read, data: value.clone() });
        Ok(value)
    }
    pub fn write(&mut self, port: u16, data: &[u8]) -> Result<(), PortIoError> {
        validate_width(data.len())?;
        let register = self.ports.get_mut(&port).ok_or(PortIoError::UnmappedPort)?;
        if !register.writable { return Err(PortIoError::ReadOnlyPort); }
        if register.value.len() != data.len() { return Err(PortIoError::InvalidWidth); }
        vdebug_autoprefix!("PortIoInterface: write to port 0x{:04x} ({} bytes) by {:?}", port, data.len(), self.owner);
        register.value.copy_from_slice(data);
        self.events.push(PortIoEvent { owner: self.owner, port, direction: PortDirection::Write, data: data.to_vec() });
        Ok(())
    }
    pub fn take_events(&mut self) -> Vec<PortIoEvent> { core::mem::take(&mut self.events) }
}

fn validate_width(width: usize) -> Result<(), PortIoError> {
    if matches!(width, 1 | 2 | 4 | 8) { Ok(()) } else { Err(PortIoError::InvalidWidth) }
}
