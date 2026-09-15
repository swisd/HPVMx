//! Minimal host-controlled kernel for a container virtual machine.

use crate::vmm::container::ContainerId;
use crate::vmm::container_engine::ContainerEngine;
use crate::vmm::cvmbus::{CvmBus, CvmBusMessage};
use crate::vmm::interface::{PortIoError, PortIoInterface, PortOwner};
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CvmKernelError { Container(&'static str), Port(PortIoError) }

pub struct CvmKernel {
    pub cvm_id: u32,
    pub bus: CvmBus,
    pub containers: ContainerEngine,
    pub ports: PortIoInterface,
}

impl CvmKernel {
    pub fn new(cvm_id: u32) -> Self {
        vdebug_autoprefix!("CvmKernel: initialized for CVM {}", cvm_id);
        Self { cvm_id, bus: CvmBus::new(cvm_id), containers: ContainerEngine::new(), ports: PortIoInterface::new(PortOwner::Cvm(cvm_id)) }
    }
    /// Process one isolated container operation from CVMBUS.
    pub fn service_one(&mut self) -> Result<Option<ContainerId>, CvmKernelError> {
        let Some(message) = self.bus.receive() else { return Ok(None); };
        vdebug_autoprefix!("CvmKernel: servicing CVMBUS message for CVM {}", self.cvm_id);
        match message {
            CvmBusMessage::CreateContainer(spec) => self.containers.create(spec).map(Some).map_err(CvmKernelError::Container),
            CvmBusMessage::StartContainer(id) => { self.containers.start(id).map_err(CvmKernelError::Container)?; Ok(Some(id)) }
            CvmBusMessage::StopContainer(id) => { self.containers.stop(id).map_err(CvmKernelError::Container)?; Ok(Some(id)) }
            CvmBusMessage::DeleteContainer(id) => { self.containers.delete(id).map_err(CvmKernelError::Container)?; Ok(Some(id)) }
            CvmBusMessage::PortWrite { container_id, port, data } => {
                let container = self.containers.get_mut(container_id).ok_or(CvmKernelError::Container("container not found"))?;
                container.ports.write(port, &data).map_err(CvmKernelError::Port)?;
                Ok(Some(container_id))
            }
        }
    }
}
