//! Container instance and its host-enforced resource limits.
//! Models the Container execution environment: Container -> CVS Interface -> Container Engine.

use crate::vdebug_autoprefix;
use crate::vmm::interface::{PortIoInterface, PortOwner};
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

pub type ContainerId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerState {
    Created,
    Running,
    Stopped,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerResources {
    pub cpu_shares: u32,
    pub memory_mb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerSpec {
    pub name: String,
    pub image: String,
    pub resources: ContainerResources,
}

pub struct Container {
    pub id: ContainerId,
    pub spec: ContainerSpec,
    pub state: ContainerState,
    pub ports: PortIoInterface,
    terminal_output: VecDeque<u8>,
    terminal_output_limit: usize,
}

impl Container {
    pub fn new(id: ContainerId, spec: ContainerSpec) -> Result<Self, &'static str> {
        if spec.name.is_empty() || spec.image.is_empty() {
            return Err("container name and image are required");
        }
        if spec.resources.cpu_shares == 0 || spec.resources.memory_mb == 0 {
            return Err("container resources must be non-zero");
        }
        vdebug_autoprefix!(
            0,
            "Container: initialized container {} (name='{}', image='{}')",
            id,
            spec.name,
            spec.image
        );
        Ok(Self {
            id,
            spec,
            state: ContainerState::Created,
            ports: PortIoInterface::new(PortOwner::Container(id)),
            terminal_output: VecDeque::new(),
            terminal_output_limit: 16 * 1024,
        })
    }

    pub(crate) fn write_terminal(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if self.terminal_output.len().saturating_add(bytes.len()) > self.terminal_output_limit {
            return Err("container terminal output buffer is full");
        }
        self.terminal_output.extend(bytes.iter().copied());
        Ok(())
    }

    pub fn drain_terminal(&mut self, max_len: usize) -> Vec<u8> {
        let count = core::cmp::min(max_len, self.terminal_output.len());
        self.terminal_output.drain(..count).collect()
    }
}
