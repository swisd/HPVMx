//! PCI bus enumeration and device lookup

use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use crate::types::{PciVendor, PciClass};

#[derive(Debug, Clone)]
pub struct PciDeviceInfo {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_id: u8,
    pub subclass_id: u8,
    pub interface_id: u8,
    pub revision_id: u8,
}

impl PciDeviceInfo {
    pub fn class_name(&self) -> &'static str {
        match self.class_id {
            0x00 => "Unclassified",
            0x01 => "Mass Storage Controller",
            0x02 => "Network Controller",
            0x03 => "Display Controller",
            0x04 => "Multimedia Controller",
            0x05 => "Memory Controller",
            0x06 => "Bridge",
            0x07 => "Communication Controller",
            0x08 => "Generic System Peripheral",
            0x09 => "Input Device Controller",
            0x0A => "Docking Station",
            0x0B => "Processor",
            0x0C => "Serial Bus Controller",
            0x0D => "Wireless Controller",
            0x0E => "Intelligent Controller",
            0x0F => "Satellite Communications Controller",
            0x10 => "Encryption Controller",
            0x11 => "Signal Processing Controller",
            0x12 => "Processing Accelerator",
            0x13 => "Non-Essential Instrumentation",
            0x40 => "Co-Processor",
            0xFF => "Unassigned Class",
            _ => "Unknown",
        }
    }

    pub fn vendor_name(&self) -> String {
        match self.vendor_id {
            0x8086 => String::from("Intel Corporation"),
            0x10DE => String::from("NVIDIA Corporation"),
            0x1002 => String::from("Advanced Micro Devices, Inc. [AMD/ATI]"),
            0x1022 => String::from("Advanced Micro Devices, Inc. [AMD]"),
            0x1234 => String::from("QEMU"),
            0x15AD => String::from("VMware"),
            0x80EE => String::from("VirtualBox"),
            0x1AF4 => String::from("Red Hat, Inc. (Virtio)"),
            _ => format!("Unknown (0x{:04X})", self.vendor_id),
        }
    }
}

pub fn scan_bus() -> Vec<PciDeviceInfo> {
    let mut devices = Vec::new();

    for bus in 0..=255 {
        for dev in 0..32 {
            // Check if device exists by reading vendor ID of function 0
            let vendor_id = pci_config_read_u16(bus as u8, dev as u8, 0, 0);
            if vendor_id == 0xFFFF || vendor_id == 0x0000 {
                continue;
            }

            // Check all 8 functions
            for func in 0..8 {
                let vendor_id = pci_config_read_u16(bus as u8, dev as u8, func as u8, 0);
                if vendor_id == 0xFFFF || vendor_id == 0x0000 {
                    continue;
                }

                let device_id = pci_config_read_u16(bus as u8, dev as u8, func as u8, 2);
                let class_rev = pci_config_read_u32(bus as u8, dev as u8, func as u8, 8);
                
                let revision_id = (class_rev & 0xFF) as u8;
                let interface_id = ((class_rev >> 8) & 0xFF) as u8;
                let subclass_id = ((class_rev >> 16) & 0xFF) as u8;
                let class_id = ((class_rev >> 24) & 0xFF) as u8;

                devices.push(PciDeviceInfo {
                    bus: bus as u8,
                    device: dev as u8,
                    function: func as u8,
                    vendor_id,
                    device_id,
                    class_id,
                    subclass_id,
                    interface_id,
                    revision_id,
                });

                // If function 0 doesn't have bit 7 of header type set, it's a single function device
                if func == 0 {
                    let header_type = pci_config_read_u8(bus as u8, dev as u8, 0, 0x0E);
                    if (header_type & 0x80) == 0 {
                        break;
                    }
                }
            }
        }
    }

    devices
}

fn pci_config_read_u32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
                  ((func as u32) << 8) | (offset as u32 & 0xFC) | 0x80000000;
    
    unsafe {
        out_l(0xCF8, address);
        in_l(0xCFC)
    }
}

fn pci_config_read_u16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let val = pci_config_read_u32(bus, slot, func, offset);
    ((val >> ((offset & 2) * 8)) & 0xFFFF) as u16
}

fn pci_config_read_u8(bus: u8, slot: u8, func: u8, offset: u8) -> u8 {
    let val = pci_config_read_u32(bus, slot, func, offset);
    ((val >> ((offset & 3) * 8)) & 0xFF) as u8
}

// Inline assembly for IO ports
unsafe fn out_l(port: u16, val: u32) {
    core::arch::asm!("out dx, eax", in("dx") port, in("eax") val);
}

unsafe fn in_l(port: u16) -> u32 {
    let val: u32;
    core::arch::asm!("in eax, dx", out("eax") val, in("dx") port);
    val
}
