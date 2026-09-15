//! Adapter between host request processing and the local VMM.

use alloc::string::String;
use alloc::vec::Vec;
use crate::hosting::types::{ConfigPayload, EnvironmentType, VirtComRingBuf};
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostVmError {
    InvalidConfiguration(&'static str),
    Backend(String),
}

pub trait HostVmBackend {
    fn create(&mut self, instance_uuid: &str, config: &ConfigPayload) -> Result<u32, HostVmError>;
    fn start(&mut self, vm_id: u32) -> Result<(), HostVmError>;
    fn stop(&mut self, vm_id: u32) -> Result<(), HostVmError>;
    fn delete(&mut self, vm_id: u32) -> Result<(), HostVmError>;
    fn load_iso(&mut self, vm_id: u32, iso_path: &str) -> Result<(), HostVmError>;

    fn create_with_env(
        &mut self,
        env: EnvironmentType,
        instance_uuid: &str,
        config: &ConfigPayload,
    ) -> Result<u32, HostVmError> {
        match env {
            EnvironmentType::VM => self.create(instance_uuid, config),
            EnvironmentType::Container | EnvironmentType::CVM => {
                // Containers and CVMs map to VM-backed or container engine instances
                vdebug_autoprefix!("Creating {:?} instance for UUID {}", env, instance_uuid);
                self.create(instance_uuid, config)
            }
        }
    }

    fn terminal_write(&mut self, _vm_id: u32, _data: &[u8]) -> Result<usize, HostVmError> {
        Ok(0)
    }

    fn terminal_read(&mut self, _vm_id: u32, _max_len: usize) -> Result<Vec<u8>, HostVmError> {
        Ok(Vec::new())
    }
}

/// Production adapter. It exposes host VMM lifecycle operations.
pub struct HypervisorBackend<'a> {
    hypervisor: &'a mut crate::vmm::HypervisorManager,
    terminal_buffers: hashbrown::HashMap<u32, VirtComRingBuf>,
}

impl<'a> HypervisorBackend<'a> {
    pub fn new(hypervisor: &'a mut crate::vmm::HypervisorManager) -> Self {
        Self {
            hypervisor,
            terminal_buffers: hashbrown::HashMap::new(),
        }
    }
}

impl HostVmBackend for HypervisorBackend<'_> {
    fn create(&mut self, instance_uuid: &str, config: &ConfigPayload) -> Result<u32, HostVmError> {
        vdebug_autoprefix!("HypervisorBackend: creating VM instance for UUID {}", instance_uuid);
        let memory_mb = u32::try_from(config.memory_size)
            .map_err(|_| HostVmError::InvalidConfiguration("memory_size exceeds u32 MB"))?;
        if config.cpu_cores == 0 || memory_mb == 0 {
            return Err(HostVmError::InvalidConfiguration("CPU and memory must be non-zero"));
        }
        let vm_id = self.hypervisor.create_vm(instance_uuid, memory_mb, config.cpu_cores)
            .map_err(|error| HostVmError::Backend(String::from(error)))?;
        self.terminal_buffers.insert(vm_id, VirtComRingBuf::new(4096));
        Ok(vm_id)
    }

    fn start(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("HypervisorBackend: starting VM {}", vm_id);
        self.hypervisor.start_vm(vm_id).map_err(|e| HostVmError::Backend(String::from(e)))
    }

    fn stop(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("HypervisorBackend: stopping VM {}", vm_id);
        self.hypervisor.stop_vm(vm_id).map_err(|e| HostVmError::Backend(String::from(e)))
    }

    fn delete(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("HypervisorBackend: deleting VM {}", vm_id);
        self.terminal_buffers.remove(&vm_id);
        self.hypervisor.delete_vm(vm_id).map_err(|e| HostVmError::Backend(String::from(e)))
    }

    fn load_iso(&mut self, vm_id: u32, iso_path: &str) -> Result<(), HostVmError> {
        vdebug_autoprefix!("HypervisorBackend: loading ISO '{}' into VM {}", iso_path, vm_id);
        self.hypervisor.boot_vm_with_media(vm_id, iso_path).map_err(|e| HostVmError::Backend(String::from(e)))
    }

    fn terminal_write(&mut self, vm_id: u32, data: &[u8]) -> Result<usize, HostVmError> {
        if let Some(buf) = self.terminal_buffers.get_mut(&vm_id) {
            let written = buf.write_in(data);
            vdebug_autoprefix!("HypervisorBackend: VirtCOM write to VM {} ({} bytes)", vm_id, written);
            Ok(written)
        } else {
            Err(HostVmError::InvalidConfiguration("terminal buffer not found for VM"))
        }
    }

    fn terminal_read(&mut self, vm_id: u32, max_len: usize) -> Result<Vec<u8>, HostVmError> {
        if let Some(buf) = self.terminal_buffers.get_mut(&vm_id) {
            let data = buf.read_out(max_len);
            vdebug_autoprefix!("HypervisorBackend: VirtCOM read from VM {} ({} bytes)", vm_id, data.len());
            Ok(data)
        } else {
            Err(HostVmError::InvalidConfiguration("terminal buffer not found for VM"))
        }
    }
}
