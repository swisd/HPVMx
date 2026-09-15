//! Container lifecycle manager for host kernel and CVM execution environments.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use crate::vmm::container::{Container, ContainerId, ContainerSpec, ContainerState};
use crate::vdebug_autoprefix;

pub struct ContainerEngine { next_id: ContainerId, containers: BTreeMap<ContainerId, Container> }

impl ContainerEngine {
    pub fn new() -> Self {
        vdebug_autoprefix!("ContainerEngine: initialized");
        Self { next_id: 1, containers: BTreeMap::new() }
    }
    pub fn create(&mut self, spec: ContainerSpec) -> Result<ContainerId, &'static str> {
        let id = self.next_id;
        vdebug_autoprefix!("ContainerEngine: creating container {} with name '{}'", id, spec.name);
        let container = Container::new(id, spec)?;
        self.next_id = self.next_id.checked_add(1).ok_or("container ID space exhausted")?;
        self.containers.insert(id, container);
        Ok(id)
    }
    pub fn start(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: starting container {}", id);
        let container = self.containers.get_mut(&id).ok_or("container not found")?;
        match container.state {
            ContainerState::Created | ContainerState::Stopped => { container.state = ContainerState::Running; Ok(()) },
            ContainerState::Running => Err("container is already running"),
            ContainerState::Deleted => Err("container was deleted"),
        }
    }
    pub fn stop(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: stopping container {}", id);
        let container = self.containers.get_mut(&id).ok_or("container not found")?;
        match container.state {
            ContainerState::Running => { container.state = ContainerState::Stopped; Ok(()) },
            ContainerState::Created | ContainerState::Stopped => Err("container is not running"),
            ContainerState::Deleted => Err("container was deleted"),
        }
    }
    pub fn delete(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: deleting container {}", id);
        let container = self.containers.get(&id).ok_or("container not found")?;
        if container.state == ContainerState::Running { return Err("cannot delete a running container"); }
        self.containers.remove(&id);
        Ok(())
    }
    pub fn get(&self, id: ContainerId) -> Option<&Container> { self.containers.get(&id) }
    pub fn get_mut(&mut self, id: ContainerId) -> Option<&mut Container> { self.containers.get_mut(&id) }
    pub fn list(&self) -> Vec<(ContainerId, ContainerState)> { self.containers.iter().map(|(id, c)| (*id, c.state)).collect() }
}

impl Default for ContainerEngine { fn default() -> Self { Self::new() } }
