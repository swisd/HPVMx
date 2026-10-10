//! Container lifecycle manager for host kernel and CVM execution environments.

use crate::vdebug_autoprefix;
use crate::vmm::container::{Container, ContainerId, ContainerSpec, ContainerState};
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

pub struct ContainerEngine {
    next_id: ContainerId,
    containers: BTreeMap<ContainerId, Container>,
}

impl ContainerEngine {
    pub fn new() -> Self {
        vdebug_autoprefix!("ContainerEngine: initialized");
        Self {
            next_id: 1,
            containers: BTreeMap::new(),
        }
    }
    pub fn create(&mut self, spec: ContainerSpec) -> Result<ContainerId, &'static str> {
        let id = self.next_id;
        vdebug_autoprefix!(
            "ContainerEngine: creating container {} with name '{}'",
            id,
            spec.name
        );
        let container = Container::new(id, spec)?;
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or("container ID space exhausted")?;
        self.containers.insert(id, container);
        Ok(id)
    }
    pub fn start(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: starting container {}", id);
        let container = self.containers.get_mut(&id).ok_or("container not found")?;
        match container.state {
            ContainerState::Created | ContainerState::Stopped => {
                container.state = ContainerState::Running;
                Ok(())
            }
            ContainerState::Running => Err("container is already running"),
            ContainerState::Deleted => Err("container was deleted"),
        }
    }
    pub fn stop(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: stopping container {}", id);
        let container = self.containers.get_mut(&id).ok_or("container not found")?;
        match container.state {
            ContainerState::Running => {
                container.state = ContainerState::Stopped;
                Ok(())
            }
            ContainerState::Created | ContainerState::Stopped => Err("container is not running"),
            ContainerState::Deleted => Err("container was deleted"),
        }
    }
    pub fn delete(&mut self, id: ContainerId) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerEngine: deleting container {}", id);
        let container = self.containers.get(&id).ok_or("container not found")?;
        if container.state == ContainerState::Running {
            return Err("cannot delete a running container");
        }
        self.containers.remove(&id);
        Ok(())
    }
    /// Run a bounded built-in command without launching a host process.
    pub fn exec(&mut self, id: ContainerId, command: &str) -> Result<(), &'static str> {
        let output = self.builtin_output(id, command)?;
        self.containers
            .get_mut(&id)
            .ok_or("container not found")?
            .write_terminal(&output)
    }

    pub(crate) fn builtin_output(
        &self,
        id: ContainerId,
        command: &str,
    ) -> Result<Vec<u8>, &'static str> {
        let container = self.containers.get(&id).ok_or("container not found")?;
        if container.state != ContainerState::Running {
            return Err("container must be running to execute a command");
        }
        if command.len() > 4096 || command.as_bytes().contains(&0) {
            return Err("container command is invalid or exceeds 4096 bytes");
        }

        let trimmed = command.trim();
        let (program, arguments) = trimmed
            .split_once(char::is_whitespace)
            .map(|(program, args)| (program, args.trim_start()))
            .unwrap_or((trimmed, ""));
        let output = match program {
            "" => return Err("container command is empty"),
            "help" => String::from("builtins: echo help id pwd uname true\n"),
            "echo" => format!("{}\n", arguments),
            "id" => format!("uid=0(root) gid=0(root) container={}\n", id),
            "pwd" => String::from("/\n"),
            "uname" => String::from("HPVMx-container 0.1.0 no-host-exec\n"),
            "true" => String::new(),
            _ => return Err("command is not present in the container built-in runtime"),
        };
        Ok(output.into_bytes())
    }

    pub fn drain_terminal(
        &mut self,
        id: ContainerId,
        max_len: usize,
    ) -> Result<Vec<u8>, &'static str> {
        self.containers
            .get_mut(&id)
            .map(|container| container.drain_terminal(max_len))
            .ok_or("container not found")
    }
    pub fn get(&self, id: ContainerId) -> Option<&Container> {
        self.containers.get(&id)
    }
    pub fn get_mut(&mut self, id: ContainerId) -> Option<&mut Container> {
        self.containers.get_mut(&id)
    }
    pub fn list(&self) -> Vec<(ContainerId, ContainerState)> {
        self.containers
            .iter()
            .map(|(id, c)| (*id, c.state))
            .collect()
    }
}

impl Default for ContainerEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vmm::container::{ContainerResources, ContainerSpec};

    fn test_spec() -> ContainerSpec {
        ContainerSpec {
            name: String::from("test"),
            image: String::from("builtin:test"),
            resources: ContainerResources {
                cpu_shares: 100,
                memory_mb: 16,
            },
        }
    }

    #[test]
    fn only_running_containers_can_execute_builtins() {
        let mut engine = ContainerEngine::new();
        let id = engine.create(test_spec()).unwrap();
        assert_eq!(
            engine.exec(id, "echo hello"),
            Err("container must be running to execute a command")
        );
        engine.start(id).unwrap();
        assert_eq!(engine.exec(id, "echo hello"), Ok(()));
        assert_eq!(engine.drain_terminal(id, 32).unwrap(), b"hello\n");
        assert_eq!(
            engine.exec(id, "host-command"),
            Err("command is not present in the container built-in runtime")
        );
    }
}
