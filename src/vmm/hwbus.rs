//! Hardware Bus (HWBUS) - inspected hardware-manager communication channel.
//!
//! HWBUS models the diagram path between VM-facing virtual hardware and the
//! kernel hardware services such as mapped memory, PCIe, device ports, and GHM.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum HwBusMessage {
    MemoryAccess { gpa: u64, hpa: u64, size: usize, write: bool },
    PciConfig { bus: u8, device: u8, function: u8, offset: u16, write: bool, data: Option<u32> },
    DevicePort { port: u16, size: usize, write: bool, data: Option<Vec<u8>> },
    DmaRequest { device_id: u32, guest_addr: u64, bytes: usize, write: bool },
    InstructionTrace { rip: u64, opcode: u16, mnemonic: String, length: u8 },
    Call { from: u64, to: u64, target_name: Option<String> },
}

pub struct HwBus {
    pub vm_id: u32,
    queue: VecDeque<HwBusMessage>,
}

impl HwBus {
    pub fn new(vm_id: u32) -> Self {
        Self {
            vm_id,
            queue: VecDeque::new(),
        }
    }

    pub fn send_message(&mut self, message: HwBusMessage) {
        self.queue.push_back(message);
    }

    pub fn receive_message(&mut self) -> Option<HwBusMessage> {
        self.queue.pop_front()
    }

    pub fn queued_messages(&self) -> Vec<HwBusMessage> {
        self.queue.iter().cloned().collect()
    }

    pub fn inspect_messages<F>(&self, mut inspector: F)
    where
        F: FnMut(&HwBusMessage) -> bool,
    {
        for message in &self.queue {
            if !inspector(message) {
                break;
            }
        }
    }
}
