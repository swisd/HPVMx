//! Container virtual machine (CVM): a VM unit with an isolated container kernel (Container Box).
//! Models the CVM environment: Container Box -> CVM Kernel -> CVM Interface -> CVMBus -> Container I/O.

use crate::vdebug_autoprefix;
use crate::vmm::container::{ContainerId, ContainerSpec};
use crate::vmm::cvm_kernel::{CvmKernel, CvmKernelError, CvmProcessId, CvmProcessState};
use crate::vmm::cvmbus::CvmBusMessage;
use crate::vmm::interface::{PortIoInterface, PortOwner};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvmState {
    Created,
    Running,
    Stopped,
    Deleted,
}

pub struct ContainerVirtualMachine {
    pub id: u32,
    pub state: CvmState,
    pub kernel: CvmKernel,
    /// CVM-facing ports are distinct from container ports and VM VMBUS.
    pub ports: PortIoInterface,
}

impl ContainerVirtualMachine {
    pub fn new(id: u32) -> Self {
        vdebug_autoprefix!(
            "ContainerVirtualMachine: initialized CVM (Container Box) with ID {}",
            id
        );
        Self {
            id,
            state: CvmState::Created,
            kernel: CvmKernel::new(id),
            ports: PortIoInterface::new(PortOwner::Cvm(id)),
        }
    }
    pub fn start(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: starting CVM {}", self.id);
        match self.state {
            CvmState::Created | CvmState::Stopped => {
                self.state = CvmState::Running;
                Ok(())
            }
            CvmState::Running => Err("CVM is already running"),
            CvmState::Deleted => Err("CVM was deleted"),
        }
    }
    pub fn stop(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: stopping CVM {}", self.id);
        if self.state != CvmState::Running {
            return Err("CVM is not running");
        }
        self.state = CvmState::Stopped;
        Ok(())
    }
    pub fn delete(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: deleting CVM {}", self.id);
        if self.state == CvmState::Running {
            return Err("cannot delete a running CVM");
        }
        self.state = CvmState::Deleted;
        Ok(())
    }

    pub fn execute_container(
        &mut self,
        container_id: ContainerId,
        command: &str,
    ) -> Result<(), CvmKernelError> {
        self.ensure_running()?;
        self.kernel.execute_container(container_id, command)
    }

    pub fn create_container(&mut self, spec: ContainerSpec) -> Result<ContainerId, CvmKernelError> {
        self.ensure_running()?;
        match self
            .kernel
            .send_and_service(CvmBusMessage::CreateContainer(spec))?
        {
            Some(id) => Ok(id),
            None => Err(CvmKernelError::Container(
                "CVMBUS create command was not serviced",
            )),
        }
    }

    pub fn start_container(&mut self, container_id: ContainerId) -> Result<(), CvmKernelError> {
        self.service_container_message(CvmBusMessage::StartContainer(container_id), container_id)
    }

    pub fn stop_container(&mut self, container_id: ContainerId) -> Result<(), CvmKernelError> {
        self.service_container_message(CvmBusMessage::StopContainer(container_id), container_id)
    }

    pub fn delete_container(&mut self, container_id: ContainerId) -> Result<(), CvmKernelError> {
        self.service_container_message(CvmBusMessage::DeleteContainer(container_id), container_id)
    }

    pub fn drain_container_terminal(
        &mut self,
        container_id: ContainerId,
        max_len: usize,
    ) -> Result<alloc::vec::Vec<u8>, &'static str> {
        self.kernel.containers.drain_terminal(container_id, max_len)
    }

    pub fn load_process(
        &mut self,
        container_id: ContainerId,
        program: &[u8],
        memory_bytes: usize,
    ) -> Result<CvmProcessId, CvmKernelError> {
        self.ensure_running()
            .map_err(|_| CvmKernelError::Container("CVM must be running to load a process"))?;
        self.kernel
            .load_process(container_id, program, memory_bytes)
    }

    pub fn run_next_process(
        &mut self,
    ) -> Result<Option<(CvmProcessId, CvmProcessState)>, CvmKernelError> {
        self.ensure_running()
            .map_err(|_| CvmKernelError::Container("CVM must be running to schedule processes"))?;
        self.kernel.run_next_process()
    }

    pub fn reap_process(&mut self, process_id: CvmProcessId) -> Result<(), CvmKernelError> {
        self.ensure_running()
            .map_err(|_| CvmKernelError::Container("CVM must be running to reap a process"))?;
        self.kernel.reap_process(process_id)
    }

    fn service_container_message(
        &mut self,
        message: CvmBusMessage,
        expected_id: ContainerId,
    ) -> Result<(), CvmKernelError> {
        self.ensure_running()?;
        match self.kernel.send_and_service(message)? {
            Some(id) if id == expected_id => Ok(()),
            Some(_) => Err(CvmKernelError::Container(
                "CVMBUS serviced an unexpected container",
            )),
            None => Err(CvmKernelError::Container("CVMBUS command was not serviced")),
        }
    }

    fn ensure_running(&self) -> Result<(), CvmKernelError> {
        if self.state == CvmState::Running {
            Ok(())
        } else {
            Err(CvmKernelError::Container(
                "CVM must be running to service containers",
            ))
        }
    }
}
