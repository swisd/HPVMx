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
            0x00 => "Unclassified Device",
            0x01 => "Mass Storage Controller",
            0x02 => "Network Controller",
            0x03 => "Display Controller",
            0x04 => "Multimedia Controller",
            0x05 => "Memory Controller",
            0x06 => "Bridge Device",
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
            _ => "Unknown Controller",
        }
    }

    pub fn subclass_name(&self) -> &'static str {
        match (self.class_id, self.subclass_id) {
            // Mass Storage
            (0x01, 0x00) => "SCSI Bus Controller",
            (0x01, 0x01) => "IDE Interface",
            (0x01, 0x02) => "Floppy Disk Controller",
            (0x01, 0x03) => "IPI Bus Controller",
            (0x01, 0x04) => "RAID Controller",
            (0x01, 0x05) => "ATA Controller",
            (0x01, 0x06) => "SATA Controller (AHCI)",
            (0x01, 0x07) => "Serial Attached SCSI (SAS)",
            (0x01, 0x08) => "Non-Volatile Memory (NVMe)",
            (0x01, 0x80) => "Mass Storage Device",

            // Network
            (0x02, 0x00) => "Ethernet Controller",
            (0x02, 0x01) => "Token Ring Controller",
            (0x02, 0x02) => "FDDI Network Controller",
            (0x02, 0x03) => "ATM Network Controller",
            (0x02, 0x04) => "ISDN Controller",
            (0x02, 0x05) => "WorldFip Controller",
            (0x02, 0x06) => "PICMG Controller",
            (0x02, 0x07) => "InfiniBand Controller",
            (0x02, 0x08) => "Fabric Controller",
            (0x02, 0x80) => "Network Interface",

            // Display
            (0x03, 0x00) => "VGA Compatible Controller",
            (0x03, 0x01) => "XGA Controller",
            (0x03, 0x02) => "3D Controller",
            (0x03, 0x80) => "Display Controller",

            // Multimedia
            (0x04, 0x00) => "Video Device",
            (0x04, 0x01) => "Audio Device",
            (0x04, 0x02) => "Telephony Device",
            (0x04, 0x03) => "High Definition Audio",
            (0x04, 0x80) => "Multimedia Controller",

            // Bridge
            (0x06, 0x00) => "Host Bridge",
            (0x06, 0x01) => "ISA Bridge",
            (0x06, 0x02) => "EISA Bridge",
            (0x06, 0x03) => "MicroChannel Bridge",
            (0x06, 0x04) => "PCI-to-PCI Bridge",
            (0x06, 0x05) => "PCMCIA Bridge",
            (0x06, 0x06) => "CardBus Bridge",
            (0x06, 0x07) => "RACEway Bridge",
            (0x06, 0x08) => "Semi-Transparent Bridge",
            (0x06, 0x09) => "InfiniBand-to-PCI Bridge",
            (0x06, 0x80) => "System Bridge",

            // Communication
            (0x07, 0x00) => "Serial Controller (16550 UART)",
            (0x07, 0x01) => "Parallel Port",
            (0x07, 0x02) => "Multiport Serial",
            (0x07, 0x03) => "Modem Controller",
            (0x07, 0x04) => "GPIB (IEEE 488.1/2)",
            (0x07, 0x05) => "Smart Card",
            (0x07, 0x80) => "Communication Device",

            // System Peripheral
            (0x08, 0x00) => "Interrupt Controller (PIC/APIC)",
            (0x08, 0x01) => "DMA Controller",
            (0x08, 0x02) => "System Timer (PIT/HPET)",
            (0x08, 0x03) => "Real Time Clock (RTC)",
            (0x08, 0x04) => "PCI Hot-Plug Controller",
            (0x08, 0x05) => "SD Host Controller",
            (0x08, 0x06) => "IOMMU Device",
            (0x08, 0x80) => "System Peripheral",

            // Input
            (0x09, 0x00) => "Keyboard Controller",
            (0x09, 0x01) => "Digitizer Pen",
            (0x09, 0x02) => "Mouse Controller",
            (0x09, 0x03) => "Scanner Controller",
            (0x09, 0x04) => "Gameport Controller",
            (0x09, 0x80) => "Input Device",

            // Serial Bus
            (0x0C, 0x00) => "FireWire (IEEE 1394)",
            (0x0C, 0x01) => "ACCESS Bus",
            (0x0C, 0x02) => "SSA",
            (0x0C, 0x03) => match self.interface_id {
                0x00 => "USB Controller (UHCI)",
                0x10 => "USB Controller (OHCI)",
                0x20 => "USB 2.0 (EHCI)",
                0x30 => "USB 3.0 (xHCI)",
                0x40 => "USB4 Host Controller",
                _ => "USB Controller",
            },
            (0x0C, 0x04) => "Fibre Channel",
            (0x0C, 0x05) => "SMBus Controller",
            (0x0C, 0x06) => "InfiniBand",
            (0x0C, 0x07) => "IPMI Interface",
            (0x0C, 0x08) => "SERCOS Interface",
            (0x0C, 0x09) => "CANbus",
            (0x0C, 0x80) => "Serial Bus",

            // Wireless
            (0x0D, 0x00) => "iRDA Controller",
            (0x0D, 0x01) => "Consumer IR",
            (0x0D, 0x10) => "RF Controller",
            (0x0D, 0x11) => "Bluetooth Adapter",
            (0x0D, 0x12) => "Broadband Controller",
            (0x0D, 0x20) => "802.11a WiFi Adapter",
            (0x0D, 0x21) => "802.11b/g/n/ac/ax WiFi Adapter",
            (0x0D, 0x80) => "Wireless Controller",

            // Encryption
            (0x10, 0x00) => "Network Encryption",
            (0x10, 0x10) => "Entertainment Encryption",
            (0x10, 0x80) => "Encryption Controller",

            _ => self.class_name(),
        }
    }

    pub fn vendor_name(&self) -> String {
        match self.vendor_id {
            0x8086 => String::from("Intel Corp."),
            0x10DE => String::from("NVIDIA Corp."),
            0x1002 => String::from("AMD/ATI"),
            0x1022 => String::from("AMD"),
            0x1234 => String::from("QEMU/Bochs"),
            0x15AD => String::from("VMware Inc."),
            0x80EE => String::from("VirtualBox"),
            0x1AF4 => String::from("Red Hat (Virtio)"),
            0x10EC => String::from("Realtek Semi."),
            0x14E4 => String::from("Broadcom Inc."),
            0x11AB => String::from("Marvell Tech."),
            0x1013 => String::from("Cirrus Logic"),
            0x1412 => String::from("VIA Tech."),
            0x1039 => String::from("SiS Corp."),
            0x10B7 => String::from("3Com Corp."),
            0x104C => String::from("Texas Instruments"),
            0x105A => String::from("Promise Tech."),
            0x1000 => String::from("LSI Logic/Broadcom"),
            0x144D => String::from("Samsung Electronics"),
            0x15B7 => String::from("Western Digital/SanDisk"),
            0x1344 => String::from("Micron Technology"),
            0x1987 => String::from("Phison Electronics"),
            0x1E0F => String::from("KIOXIA Corp."),
            0x14C3 => String::from("MediaTek Inc."),
            0x168C => String::from("Qualcomm Atheros"),
            0x106B => String::from("Apple Inc."),
            0x1414 => String::from("Microsoft Corp."),
            0x1106 => String::from("VIA/S3 Graphics"),
            0x104A => String::from("STMicroelectronics"),
            0x13F6 => String::from("C-Media Electronics"),
            0x1274 => String::from("Ensoniq"),
            0x1102 => String::from("Creative Labs"),
            0x1B36 => String::from("Red Hat / QEMU PCIe"),
            _ => format!("Vendor 0x{:04X}", self.vendor_id),
        }
    }

    pub fn device_name(&self) -> String {
        match (self.vendor_id, self.device_id) {
            // Virtio
            (0x1AF4, 0x1000) => String::from("Virtio Network Adapter"),
            (0x1AF4, 0x1001) => String::from("Virtio Block Storage Device"),
            (0x1AF4, 0x1002) => String::from("Virtio Memory Balloon"),
            (0x1AF4, 0x1003) => String::from("Virtio Console Device"),
            (0x1AF4, 0x1004) => String::from("Virtio SCSI Controller"),
            (0x1AF4, 0x1005) => String::from("Virtio Entropy RNG"),
            (0x1AF4, 0x1009) => String::from("Virtio 9P Transport"),
            (0x1AF4, 0x1041) => String::from("Virtio 1.0 Network Adapter"),
            (0x1AF4, 0x1042) => String::from("Virtio 1.0 Block Storage Device"),
            (0x1AF4, 0x1044) => String::from("Virtio 1.0 Entropy RNG"),
            (0x1AF4, 0x1045) => String::from("Virtio 1.0 Memory Balloon"),
            (0x1AF4, 0x1048) => String::from("Virtio 1.0 SCSI Controller"),
            (0x1AF4, 0x1050) => String::from("Virtio 1.0 GPU Display Device"),
            (0x1AF4, 0x1052) => String::from("Virtio 1.0 Input Controller"),
            (0x1AF4, 0x1053) => String::from("Virtio 1.0 Socket Device"),

            // Intel
            (0x8086, 0x1237) => String::from("440FX 82441FX PMC Host Bridge"),
            (0x8086, 0x7000) => String::from("82371SB PIIX3 ISA Bridge"),
            (0x8086, 0x7010) => String::from("82371SB PIIX3 IDE Controller"),
            (0x8086, 0x7111) => String::from("82371AB/EB/MB PIIX4 IDE Controller"),
            (0x8086, 0x7113) => String::from("82371AB/EB/MB PIIX4 ACPI / Power"),
            (0x8086, 0x100E) => String::from("82540EM Gigabit Ethernet NIC"),
            (0x8086, 0x100F) => String::from("82545EM Gigabit Ethernet NIC"),
            (0x8086, 0x10D3) => String::from("82574L Gigabit Network Connection"),
            (0x8086, 0x1539) => String::from("I211 Gigabit Network Connection"),
            (0x8086, 0x15F3) => String::from("I225-V 2.5GbE Network Controller"),
            (0x8086, 0x2918) => String::from("82801IB (ICH9) LPC Interface Controller"),
            (0x8086, 0x2922) => String::from("82801IR 6-port SATA AHCI Controller"),
            (0x8086, 0x293E) => String::from("82801I (ICH9) High Definition Audio"),
            (0x8086, 0x2934) => String::from("82801I (ICH9) USB UHCI Controller"),
            (0x8086, 0x293A) => String::from("82801I (ICH9) USB2 EHCI Controller"),
            (0x8086, 0x1E31) => String::from("7 Series USB 3.0 xHCI Controller"),
            (0x8086, 0x0166) => String::from("3rd Gen HD Graphics 4000"),
            (0x8086, 0x0412) => String::from("4th Gen HD Graphics 4600"),
            (0x8086, 0x3E92) => String::from("Coffee Lake UHD Graphics 630"),

            // QEMU / Bochs
            (0x1234, 0x1111) => String::from("QEMU/Bochs VGA Display Adapter"),
            (0x1234, 0x0001) => String::from("QEMU Test / Diagnostic Device"),
            (0x1234, 0x0002) => String::from("QEMU PCI-PCI Bridge"),

            // VMware
            (0x15AD, 0x0405) => String::from("VMware SVGA II Adapter"),
            (0x15AD, 0x0710) => String::from("VMware SVGA3D Accelerator"),
            (0x15AD, 0x0770) => String::from("VMware USB2 EHCI Controller"),
            (0x15AD, 0x0790) => String::from("VMware PCIe Root Port"),
            (0x15AD, 0x07A0) => String::from("VMware VMXNET3 Ethernet Adapter"),
            (0x15AD, 0x07E0) => String::from("VMware PVSCSI Storage Controller"),
            (0x15AD, 0x0801) => String::from("VMware NVMe Controller"),

            // Realtek
            (0x10EC, 0x8139) => String::from("Realtek RTL8139 Fast Ethernet"),
            (0x10EC, 0x8168) => String::from("Realtek RTL8168/8111 PCIe Gigabit Ethernet"),
            (0x10EC, 0x8169) => String::from("Realtek RTL8169 Gigabit Ethernet"),
            (0x10EC, 0x8821) => String::from("Realtek RTL8821CE 802.11ac WiFi Adapter"),

            // QEMU PCIe
            (0x1B36, 0x0001) => String::from("QEMU PCI-PCI Bridge"),
            (0x1B36, 0x0004) => String::from("QEMU PCIe Root Port"),
            (0x1B36, 0x0008) => String::from("QEMU PCIe Expander Bridge"),
            (0x1B36, 0x000C) => String::from("QEMU PCIe-to-PCI Bridge"),

            _ => format!("{} {}", self.vendor_name(), self.subclass_name()),
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

pub fn pci_config_read_u32(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
                  ((func as u32) << 8) | (offset as u32 & 0xFC) | 0x80000000;
    
    unsafe {
        out_l(0xCF8, address);
        in_l(0xCFC)
    }
}

pub fn pci_config_read_u16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    let val = pci_config_read_u32(bus, slot, func, offset);
    ((val >> ((offset & 2) * 8)) & 0xFFFF) as u16
}

pub fn pci_config_read_u8(bus: u8, slot: u8, func: u8, offset: u8) -> u8 {
    let val = pci_config_read_u32(bus, slot, func, offset);
    ((val >> ((offset & 3) * 8)) & 0xFF) as u8
}

pub fn pci_config_write_u32(bus: u8, slot: u8, func: u8, offset: u8, val: u32) {
    let address = ((bus as u32) << 16) | ((slot as u32) << 11) |
                  ((func as u32) << 8) | (offset as u32 & 0xFC) | 0x80000000;
    
    unsafe {
        out_l(0xCF8, address);
        out_l(0xCFC, val);
    }
}

pub fn pci_config_write_u16(bus: u8, slot: u8, func: u8, offset: u8, val: u16) {
    let mut old_val = pci_config_read_u32(bus, slot, func, offset);
    let shift = (offset & 2) * 8;
    old_val &= !(0xFFFF << shift);
    old_val |= (val as u32) << shift;
    pci_config_write_u32(bus, slot, func, offset, old_val);
}

pub fn pci_config_write_u8(bus: u8, slot: u8, func: u8, offset: u8, val: u8) {
    let mut old_val = pci_config_read_u32(bus, slot, func, offset);
    let shift = (offset & 3) * 8;
    old_val &= !(0xFF << shift);
    old_val |= (val as u32) << shift;
    pci_config_write_u32(bus, slot, func, offset, old_val);
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
