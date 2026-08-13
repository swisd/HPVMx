//! Intel High Definition Audio (HDA) driver for HPVMx
//!
//! This driver supports basic HDA controllers and codecs, providing
//! PCM audio output capabilities.

use alloc::vec::Vec;
use crate::hardware::pci::{self, PciDeviceInfo};

/// HDA Register Offsets
const GCAP: u16 = 0x00;   // Global Capabilities
const GCTL: u16 = 0x08;   // Global Control
const WAKEEN: u16 = 0x0C; // Wake Enable
const STATESTS: u16 = 0x0E; // State Status
const INTCTL: u16 = 0x20; // Interrupt Control
const INTSTS: u16 = 0x24; // Interrupt Status
const CORBLBASE: u16 = 0x40; // CORB Lower Base Address
const CORBUBASE: u16 = 0x44; // CORB Upper Base Address
const CORBWP: u16 = 0x48;   // CORB Write Pointer
const CORBRP: u16 = 0x4A;   // CORB Read Pointer
const CORBCTL: u16 = 0x4C;  // CORB Control
const CORBSTS: u16 = 0x4D;  // CORB Status
const CORBSIZE: u16 = 0x4E; // CORB Size
const RIRBLBASE: u16 = 0x50; // RIRB Lower Base Address
const RIRBUBASE: u16 = 0x54; // RIRB Upper Base Address
const RIRBWP: u16 = 0x58;   // RIRB Write Pointer
const RIRBCTL: u16 = 0x5C;  // RIRB Control
const RIRBSTS: u16 = 0x5D;  // RIRB Status
const RIRBSIZE: u16 = 0x5E; // RIRB Size
const DPLBASE: u16 = 0x70;  // DMA Position Lower Base
const DPUBASE: u16 = 0x74;  // DMA Position Upper Base

/// CORB/RIRB Entry
type CorbEntry = u32;
#[repr(C)]
struct RirbEntry {
    response: u32,
    resp_ex: u32,
}

/// Global HDA State
struct HdaState {
    pci_info: Option<PciDeviceInfo>,
    bar0: u64,
    initialized: bool,
    corb_addr: u64,
    rirb_addr: u64,
}

static mut HDA_STATE: HdaState = HdaState {
    pci_info: None,
    bar0: 0,
    initialized: false,
    corb_addr: 0,
    rirb_addr: 0,
};

pub fn init() -> Result<(), &'static str> {
    unsafe {
        if HDA_STATE.initialized {
            return Ok(());
        }

        // Find HDA device
        let devices = pci::scan_bus();
        let hda_dev = devices.iter().find(|d| d.class_id == 0x04 && d.subclass_id == 0x03);

        if let Some(dev) = hda_dev {
            HDA_STATE.pci_info = Some(dev.clone());
            
            // Read BAR0 (Memory Mapped Register Base)
            let bar0_low = pci::pci_config_read_u32(dev.bus, dev.device, dev.function, 0x10);
            let bar0_high = if (bar0_low & 0x6) == 0x4 {
                pci::pci_config_read_u32(dev.bus, dev.device, dev.function, 0x14)
            } else {
                0
            };
            
            let bar0 = ((bar0_high as u64) << 32) | (bar0_low as u64 & !0xF);
            HDA_STATE.bar0 = bar0;

            // Enable Memory Space and Bus Mastering
            let command = pci::pci_config_read_u16(dev.bus, dev.device, dev.function, 0x04);
            pci::pci_config_write_u16(dev.bus, dev.device, dev.function, 0x04, command | 0x06);

            // Reset Controller
            write_reg32(GCTL, 0);
            while (read_reg32(GCTL) & 1) != 0 {} // Wait for reset
            
            write_reg32(GCTL, 1);
            while (read_reg32(GCTL) & 1) == 0 {} // Wait for controller to be ready

            // Wait for codecs to report status
            crate::devices::timer::sleep_ms(100);

            // Setup CORB/RIRB
            setup_corb_rirb()?;

            HDA_STATE.initialized = true;
            Ok(())
        } else {
            Err("HDA Controller not found")
        }
    }
}

unsafe fn setup_corb_rirb() -> Result<(), &'static str> {
    // Stop CORB and RIRB DMA engines
    write_reg8(CORBCTL, 0);
    write_reg8(RIRBCTL, 0);
    while (read_reg8(CORBCTL) & 2) != 0 {}
    while (read_reg8(RIRBCTL) & 2) != 0 {}

    // We'll use a fixed memory location for now (1MB for simplicity, should be properly allocated)
    // In a real OS, we'd use a proper physical memory allocator.
    let base_phys: u64 = 0x2000000; // 32MB mark, hopefully safe
    HDA_STATE.corb_addr = base_phys;
    HDA_STATE.rirb_addr = base_phys + 1024;

    // Set Base Addresses
    write_reg32(CORBLBASE, HDA_STATE.corb_addr as u32);
    write_reg32(CORBUBASE, (HDA_STATE.corb_addr >> 32) as u32);
    write_reg32(RIRBLBASE, HDA_STATE.rirb_addr as u32);
    write_reg32(RIRBUBASE, (HDA_STATE.rirb_addr >> 32) as u32);

    // Set CORB/RIRB size to 256 entries
    write_reg8(CORBSIZE, 0x02); // 256 entries
    write_reg8(RIRBSIZE, 0x02); // 256 entries

    // Reset Write/Read Pointers
    write_reg16(CORBWP, 0);
    write_reg16(CORBRP, 0x8000); // Reset RP
    while (read_reg16(CORBRP) & 0x8000) == 0 {}
    write_reg16(CORBRP, 0);
    while (read_reg16(CORBRP) & 0x7FF) != 0 {}

    write_reg16(RIRBWP, 0x8000); // Reset WP

    // Start CORB and RIRB DMA engines
    write_reg8(CORBCTL, 0x02);
    write_reg8(RIRBCTL, 0x02);

    Ok(())
}

unsafe fn send_verb(codec: u8, node: u8, verb: u32) -> Result<u32, &'static str> {
    let corb_wp = (read_reg16(CORBWP) & 0xFF) as u16;
    let next_wp = (corb_wp + 1) & 0xFF;

    let corb_ptr = HDA_STATE.corb_addr as *mut u32;
    let val = ((codec as u32) << 28) | ((node as u32) << 20) | (verb & 0xFFFFF);
    
    core::ptr::write_volatile(corb_ptr.add(next_wp as usize), val);
    write_reg16(CORBWP, next_wp);

    // Wait for response in RIRB
    let mut timeout = 1000;
    while timeout > 0 {
        let rirb_wp = (read_reg16(RIRBWP) & 0xFF) as u16;
        if rirb_wp == next_wp {
            let rirb_ptr = HDA_STATE.rirb_addr as *const RirbEntry;
            let entry = core::ptr::read_volatile(rirb_ptr.add(next_wp as usize));
            return Ok(entry.response);
        }
        crate::devices::timer::sleep_ms(1);
        timeout -= 1;
    }

    Err("HDA Verb timeout")
}

pub fn play_tone(frequency_hz: u32, duration_ms: u64) -> Result<(), &'static str> {
    if !is_available() {
        return Err("HDA not available");
    }

    // This is a placeholder for actual HDA DMA playback.
    // For now, we will just log that HDA is being used.
    // In a real implementation, we would set up DMA buffers and streams here.
    
    // Fallback to PC Speaker for actual sound until DMA is fully implemented
    crate::devices::audio::play_tone(frequency_hz, duration_ms);
    
    Ok(())
}

pub fn is_available() -> bool {
    unsafe { HDA_STATE.initialized }
}

/// Helper to read HDA register
unsafe fn read_reg32(offset: u16) -> u32 {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::read_volatile(addr as *const u32)
}

/// Helper to write HDA register
unsafe fn write_reg32(offset: u16, val: u32) {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::write_volatile(addr as *mut u32, val)
}

unsafe fn read_reg16(offset: u16) -> u16 {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::read_volatile(addr as *const u16)
}

unsafe fn write_reg16(offset: u16, val: u16) {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::write_volatile(addr as *mut u16, val)
}

unsafe fn read_reg8(offset: u16) -> u8 {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::read_volatile(addr as *const u8)
}

unsafe fn write_reg8(offset: u16, val: u8) {
    let addr = HDA_STATE.bar0 + offset as u64;
    core::ptr::write_volatile(addr as *mut u8, val)
}
