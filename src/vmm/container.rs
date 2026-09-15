//! Container instance and its host-enforced resource limits.
//! Models the Container execution environment: Container -> CVS Interface -> Container Engine.

use alloc::string::String;
use serde::{Deserialize, Serialize};
use crate::vmm::interface::{PortIoInterface, PortOwner};
use crate::vdebug_autoprefix;

pub type ContainerId = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContainerState { Created, Running, Stopped, Deleted }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContainerResources { pub cpu_shares: u32, pub memory_mb: u32 }

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
}

impl Container {
    pub fn new(id: ContainerId, spec: ContainerSpec) -> Result<Self, &'static str> {
        if spec.name.is_empty() || spec.image.is_empty() {
            return Err("container name and image are required");
        }
        if spec.resources.cpu_shares == 0 || spec.resources.memory_mb == 0 {
            return Err("container resources must be non-zero");
        }
        vdebug_autoprefix!(0, "Container: initialized container {} (name='{}', image='{}')", id, spec.name, spec.image);
        Ok(Self {
            id,
            spec,
            state: ContainerState::Created,
            ports: PortIoInterface::new(PortOwner::Container(id)),
        })
    }
}
