//! UEFI Simple Network Protocol (SNP) bring-up for HPVMx
//! Minimal hardware binding so higher-level networking knows a NIC exists.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};
use log::{info, warn, error};
use uefi::boot::{self, ScopedProtocol, SearchType};
use uefi::proto::network::snp::SimpleNetwork;
use uefi::Identify;

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

#[inline]
pub fn is_initialized() -> bool { NET_INITIALIZED.load(Ordering::SeqCst) }

pub fn get_info() -> Option<NetHwInfo> {
    unsafe { NET_INFO }
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
            warn!("net-hw: no SNP handles present");
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

            unsafe { NET_INFO = Some(NetHwInfo { mac, mac_len, mtu, media_present, state: 2 }); }
            NET_INITIALIZED.store(true, Ordering::SeqCst);

            info!(
                "net-hw: SNP initialized: MAC={} MTU={} media_present={:?}",
                format_mac(&mac, mac_len), mtu, media_present
            );
            return Ok(());
        }
    }

    error!("net-hw: SNP handles found but none could be initialized");
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
