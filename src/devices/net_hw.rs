//! UEFI Simple Network Protocol (SNP) bring-up for HPVMx
//! Minimal hardware binding so higher-level networking knows a NIC exists.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};
use crate::Color;
use crate::hpvm_log;
use crate::hpvm_info;
use crate::hpvm_warn;
use crate::hpvm_error;
use crate::message;
use log::{info, warn, error};
use uefi::boot::{self, HandleBuffer, ScopedProtocol, SearchType};
use uefi::proto::network::snp::SimpleNetwork;
use uefi::{Identify, Handle};
use uefi_raw::MacAddress;

static NET_INITIALIZED: AtomicBool = AtomicBool::new(false);


#[derive(Copy, Clone)]
pub struct NetHwInfo {
    pub mac: [u8; 32],
    pub mac_len: usize,
    pub mtu: u32,
    pub media_present: bool,
    pub state: u32,
}

// Keep minimal and avoid extra deps; use a simple static mut for single-writer init()
static mut NET_INFO: Option<NetHwInfo> = None;
static mut NIC_HANDLE: Option<Handle> = None;

#[inline]
pub fn is_initialized() -> bool { NET_INITIALIZED.load(Ordering::SeqCst) }

pub fn get_info() -> Option<NetHwInfo> {
    unsafe { NET_INFO }
}

#[inline]
pub fn nic_handle() -> Option<Handle> { unsafe { NIC_HANDLE } }

/// Convenience helper to open the SNP protocol on the selected NIC.
pub fn snp_open() -> Option<ScopedProtocol<SimpleNetwork>> {
    if let Some(h) = nic_handle() {
        match boot::open_protocol_exclusive::<SimpleNetwork>(h) {
            Ok(p) => Some(p),
            Err(_) => None,
        }
    } else {
        None
    }
}

/// Current link status (best-effort).
pub fn link_up() -> bool {
    if let Some(mut snp) = snp_open() {
        let m = snp.mode();
        let present: bool = m.media_present.into();
        present
    } else {
        get_info().map(|i| i.media_present).unwrap_or(false)
    }
}

/// Transmit a raw Ethernet frame via SNP (best-effort).
pub fn tx(frame: &[u8]) -> Result<(), &'static str> {
    let Some(mut snp) = snp_open() else { return Err("no snp"); };
    // Safety: UEFI SNP expects DMA-safe buffer; firmware copies internally.
    match snp.transmit(0, frame, None, None, None) {
        Ok(_) => Ok(()),
        Err(_) => Err("tx failed"),
    }
}

/// Receive a raw Ethernet frame into the provided buffer. Returns length if a packet was received.
pub fn rx(buf: &mut [u8]) -> Result<usize, &'static str> {
    let Some(mut snp) = snp_open() else { return Err("no snp"); };
    match snp.receive(buf, None, None, None, None) {
        Ok(sz) => Ok(sz),
        Err(_) => Err("rx none"),
    }
}

/// Try to locate a NIC via UEFI SNP and initialize it (Start + Initialize).
/// Returns Ok if at least one device was started and initialized.
pub fn init() -> Result<(), &'static str> {
    if is_initialized() && get_info().is_some() {
        return Ok(());
    }

    let handles = match boot::locate_handle_buffer(SearchType::ByProtocol(&SimpleNetwork::GUID)) {
        Ok(buf) => buf,
        Err(_) => {
            hpvm_warn!("NETHW", "net-hw: no SNP handles present");
            return Err("no nic found");
        }
    };

    for &h in handles.as_slice().iter() {
        let snp_scoped: Result<ScopedProtocol<SimpleNetwork>, _> = boot::open_protocol_exclusive(h);
        if let Ok(snp) = snp_scoped {
            let snp_ref: &SimpleNetwork = &snp;
            // Best-effort start/initialize
            let _ = snp_ref.start();
            let _ = snp_ref.initialize(0, 0);

            let mode = snp_ref.mode();
            let mut mac = [0u8; 32];
            let mac_len = (mode.hw_address_size as usize).min(32);
            let src = &mode.current_address.0[..mac_len];
            mac[..mac_len].copy_from_slice(src);
            let mtu = mode.max_packet_size as u32;
            let media_present: bool = mode.media_present.into();

            unsafe {
                NET_INFO = Some(NetHwInfo { mac, mac_len, mtu, media_present, state: 2 });
                NIC_HANDLE = Some(h);
            }
            NET_INITIALIZED.store(true, Ordering::SeqCst);

            hpvm_info!("NETHW",
                "SNP initialized: MAC={} MTU={} media_present={:?}",
                format_mac(&mac, mac_len), mtu, media_present
            );
            return Ok(());
        }
    }

    hpvm_error!("NETHW", ": SNP handles found but none could be initialized");
    Err("no usable nic")
}

fn format_mac(mac: &[u8;32], len: usize) -> alloc::string::String {
    let mut out = alloc::string::String::new();
    let n = len.min(32).min(8);
    for i in 0..n {
        if i > 0 { out.push(':'); }
        out.push_str(&alloc::format!("{:02x}", mac[i]));
    }
    out
}

pub(crate) fn get_mac() -> [u8; 6] {
    todo!()
}