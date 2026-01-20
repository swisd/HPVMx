//! I/O device virtualization and emulation

use alloc::collections::BTreeMap;
use alloc::string::String;

/// Virtual I/O device trait
pub trait VirtioDevice {
    fn device_name(&self) -> &str;
    fn handle_io_read(&self, port: u16, size: u32) -> u32;
    fn handle_io_write(&mut self, port: u16, size: u32, data: u32) -> Result<(), &'static str>;
}

/// Virtual Serial Console device
pub struct VirtualConsole {
    pub name: String,
    pub data_buffer: [u8; 256],
    pub buffer_pos: usize,
}

impl VirtualConsole {
    pub fn new() -> Self {
        Self {
            name: String::from("Virtual Serial Console"),
            data_buffer: [0; 256],
            buffer_pos: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        if self.buffer_pos < 256 {
            self.data_buffer[self.buffer_pos] = byte;
            self.buffer_pos += 1;
        }
    }
}

impl VirtioDevice for VirtualConsole {
    fn device_name(&self) -> &str {
        "Virtual Serial Console"
    }

    fn handle_io_read(&self, _port: u16, _size: u32) -> u32 {
        0
    }

    fn handle_io_write(&mut self, _port: u16, _size: u32, data: u32) -> Result<(), &'static str> {
        let byte = (data & 0xFF) as u8;
        self.write_byte(byte);
        Ok(())
    }
}

/// Virtual device manager
pub struct IoManager {
    devices: BTreeMap<String, usize>,
}

impl IoManager {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
        }
    }

    pub fn register_device(&mut self, name: String) {
        self.devices.insert(name, 0);
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
}