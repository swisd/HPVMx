//! CVM-internal bus (CVMBus). It is separate from VMBUS so container traffic cannot
//! inject VM or host-hardware bus operations.

use alloc::collections::VecDeque;
use alloc::vec::Vec;
use crate::vmm::container::{ContainerId, ContainerSpec};
use crate::vdebug_autoprefix;

#[derive(Debug, Clone)]
pub enum CvmBusMessage {
    CreateContainer(ContainerSpec),
    StartContainer(ContainerId),
    StopContainer(ContainerId),
    DeleteContainer(ContainerId),
    PortWrite { container_id: ContainerId, port: u16, data: Vec<u8> },
}

pub struct CvmBus { pub cvm_id: u32, queue: VecDeque<CvmBusMessage> }

impl CvmBus {
    pub fn new(cvm_id: u32) -> Self {
        vdebug_autoprefix!("CvmBus: initialized for CVM {}", cvm_id);
        Self { cvm_id, queue: VecDeque::new() }
    }
    pub fn send(&mut self, message: CvmBusMessage) {
        vdebug_autoprefix!("CvmBus: queued message for CVM {}", self.cvm_id);
        self.queue.push_back(message);
    }
    pub fn receive(&mut self) -> Option<CvmBusMessage> { self.queue.pop_front() }
    pub fn pending(&self) -> usize { self.queue.len() }
}
