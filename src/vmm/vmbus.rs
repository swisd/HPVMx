//! Virtual Machine Bus (VMBUS) - Serialized and inspected communication channel.
//! All inter-unit communication is serialized and inspected via the VMBUS,
//! ensuring no "side-door" access to system components.

use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub enum VmBusMessage {
    IoRequest { address: u64, size: usize, write: bool, data: Option<Vec<u8>> },
    Interrupt { vector: u8 },
    StorageRequest { sector: u64, count: u32, write: bool, data: Option<Vec<u8>> },
    InstructionTrace { rip: u64, opcode: u16, mnemonic: String, length: u8 },
    Call { from: u64, to: u64, target_name: Option<String> },
}

pub struct VmBus {
    pub vm_id: u32,
    queue: VecDeque<VmBusMessage>,
}

impl VmBus {
    pub fn new(vm_id: u32) -> Self {
        Self {
            vm_id,
            queue: VecDeque::new(),
        }
    }

    /// Send a message over the bus. This is the primary communication method.
    pub fn send_message(&mut self, message: VmBusMessage) {
        // Serialization and inspection would happen here in a real implementation.
        self.queue.push_back(message);
    }

    /// Receive a message from the bus.
    pub fn receive_message(&mut self) -> Option<VmBusMessage> {
        self.queue.pop_front()
    }

    /// Return a snapshot of queued messages for security analysis/training.
    pub fn queued_messages(&self) -> Vec<VmBusMessage> {
        self.queue.iter().cloned().collect()
    }

    /// Inspect all messages currently in the queue (used by Deep Level Security).
    pub fn inspect_messages<F>(&self, mut inspector: F)
    where
        F: FnMut(&VmBusMessage) -> bool,
    {
        for message in &self.queue {
            if !inspector(message) {
                break;
            }
        }
    }
}
