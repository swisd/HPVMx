//! UEFI Simple Network Protocol (SNP) bring-up for HPVMx
//! Minimal hardware binding so higher-level networking knows a NIC exists.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};
use crate::Color;
use crate::hpvm_log;
use crate::hpvm_info;
use crate::hpvm_warn;
use crate::hpvm_error;
use uefi::boot::{self, OpenProtocolAttributes, OpenProtocolParams, ScopedProtocol, SearchType};
use uefi::proto::network::snp::SimpleNetwork;
use uefi::{Identify, Handle};
use uefi_raw::protocol::network::snp::{NetworkState, ReceiveFlags};
use core::ffi::c_void;
use core::ops::Deref;

static mut OWNED_SNP: Option<ScopedProtocol<SimpleNetwork>> = None;
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
    if let Some(snp) = snp_open() {
        let m = snp.mode();
        let present: bool = m.media_present.into();
        present
    } else {
        get_info().map(|i| i.media_present).unwrap_or(false)
    }
}

/// Transmit a raw Ethernet frame via SNP (best-effort).
pub fn tx(frame: &[u8]) -> Result<(), &'static str> {
    unsafe {
        if let Some(ref snp_scoped) = OWNED_SNP {
            let snp = &**snp_scoped;

            // 1. Send the frame
            // transmit() returns Result<()>
            if snp.transmit(0, frame, None, None, None).is_err() {
                return Err("tx hardware rejected packet");
            }

            // 2. Poll for the recycled buffer
            // VirtualBox's virtual NIC needs this to "acknowledge" the packet is sent
            let mut timeout = 1000000;
            while timeout > 0 {
                // In 0.36.1, this is the specialized method for tx cleanup
                if let Ok(maybe_buf) = snp.get_recycled_transmit_buffer_status() {
                    // maybe_buf is Option<NonNull<u8>>
                    if maybe_buf.is_some() {
                        return Ok(()); // Hardware is done; slot is free
                    }
                }

                timeout -= 1;
                core::hint::spin_loop();
            }

            Err("tx timeout: hardware didn't return buffer")
        } else {
            Err("no snp")
        }
    }
}

/// Receive a raw Ethernet frame into the provided buffer. Returns length if a packet was received.
pub fn rx(buf: &mut [u8]) -> Result<usize, &'static str> {
    let Some(snp) = snp_open() else { return Err("no snp"); };
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
        let Ok(snp_scoped) = boot::open_protocol_exclusive::<SimpleNetwork>(h) else {
            continue;
        };

        let snp = &*snp_scoped;
        let mode = snp.mode();

        match mode.state {
            NetworkState::STOPPED => { // EfiSimpleNetworkStopped
                if snp.start().is_err() { continue; }
            }
            NetworkState::STARTED => {} // EfiSimpleNetworkStarted - Ready to initialize
            _ => {
                // If already initialized (state 2), reset it to be safe
                let _ = snp.reset(false);
            }
        }

        if let Err(e) = snp.initialize(0, 0) {
            hpvm_warn!("NETHW", "SNP init failed on handle {:?}: {:?}", h, e);
            continue;
        }

        let _ = snp.receive_filters(
            ReceiveFlags::UNICAST | ReceiveFlags::BROADCAST,
            ReceiveFlags::empty(),
            false,   // Don't go into Promiscuous mode
            None     // No hardware MAC list needed
        ).map_err(|_| "Failed to set receive filters")?;

        let mode = snp.mode(); // Refresh mode after init
        let mac_len = (mode.hw_address_size as usize).min(32);
        let mut mac = [0u8; 32];
        mac[..mac_len].copy_from_slice(&mode.current_address.0[..mac_len]);

        // 5. Global State Update
        unsafe {
            NET_INFO = Some(NetHwInfo {
                mac,
                mac_len,
                mtu: mode.max_packet_size,
                media_present: mode.media_present.into(),
                state: 2,
            });

            NIC_HANDLE = Some(h);
            OWNED_SNP = Some(snp_scoped);
        }

        NET_INITIALIZED.store(true, Ordering::SeqCst);
        return Ok(());
    }

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

pub(crate) fn get_mac() -> [u8; 32] {
    let handles = match boot::locate_handle_buffer(SearchType::ByProtocol(&SimpleNetwork::GUID)) {
        Ok(buf) => buf,
        Err(_) => {
            hpvm_warn!("NETHW", "net-hw: no SNP handles present");
            return [0; 32];
        }
    };
    let mut mc: [u8;32] = [0; 32];
    unsafe {
        for &h in handles.as_slice().iter() {
            let snp_scoped: Result<ScopedProtocol<SimpleNetwork>, _> = boot::open_protocol(OpenProtocolParams {
                handle: h,
                agent: boot::image_handle(),
                controller: None,
            }, OpenProtocolAttributes::GetProtocol
            );
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
                //mac
                mc = mac;
            } else { return [0; 32] }
        }
    }
    mc
}