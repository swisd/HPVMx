//! Container virtual machine (CVM): a VM unit with an isolated container kernel (Container Box).
//! Models the CVM environment: Container Box -> CVM Kernel -> CVM Interface -> CVMBus -> Container I/O.

use crate::vmm::cvm_kernel::CvmKernel;
use crate::vmm::interface::{PortIoInterface, PortOwner};
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CvmState { Created, Running, Stopped, Deleted }

pub struct ContainerVirtualMachine {
    pub id: u32,
    pub state: CvmState,
    pub kernel: CvmKernel,
    /// CVM-facing ports are distinct from container ports and VM VMBUS.
    pub ports: PortIoInterface,
}

impl ContainerVirtualMachine {
    pub fn new(id: u32) -> Self {
        vdebug_autoprefix!("ContainerVirtualMachine: initialized CVM (Container Box) with ID {}", id);
        Self { id, state: CvmState::Created, kernel: CvmKernel::new(id), ports: PortIoInterface::new(PortOwner::Cvm(id)) }
    }
    pub fn start(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: starting CVM {}", self.id);
        match self.state {
            CvmState::Created | CvmState::Stopped => { self.state = CvmState::Running; Ok(()) },
            CvmState::Running => Err("CVM is already running"),
            CvmState::Deleted => Err("CVM was deleted"),
        }
    }
    pub fn stop(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: stopping CVM {}", self.id);
        if self.state != CvmState::Running { return Err("CVM is not running"); }
        self.state = CvmState::Stopped;
        Ok(())
    }
    pub fn delete(&mut self) -> Result<(), &'static str> {
        vdebug_autoprefix!("ContainerVirtualMachine: deleting CVM {}", self.id);
        if self.state == CvmState::Running { return Err("cannot delete a running CVM"); }
        self.state = CvmState::Deleted;
        Ok(())
    }
}
