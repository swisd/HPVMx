//! UEFI Simple Network Protocol (SNP) bring-up for HPVMx
//! Minimal hardware binding so higher-level networking knows a NIC exists.

#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};
use crate::{vdebug_autoprefix, Color};
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
    crate::vdebug!("NETHW", "TX called, frame size: {} bytes", frame.len());
    unsafe {
        if let Some(ref snp_scoped) = OWNED_SNP {
            let snp = &**snp_scoped;

            crate::vdebug!("NETHW", "TX: SNP available, calling transmit");
            if snp.transmit(0, frame, None, None, None).is_err() {
                crate::vdebug!("NETHW", "TX: hardware rejected packet");
                return Err("tx hardware rejected packet");
            }

            crate::vdebug!("NETHW", "TX: transmit accepted, waiting for buffer recycle");
            let mut timeout = 1000000;
            while timeout > 0 {
                if let Ok(maybe_buf) = snp.get_recycled_transmit_buffer_status() {
                    if maybe_buf.is_some() {
                        crate::vdebug!("NETHW", "TX: buffer recycled after {} iterations", 1000000 - timeout);
                        return Ok(());
                    }
                }

                timeout -= 1;
                core::hint::spin_loop();
            }

            crate::vdebug!("NETHW", "TX: timeout waiting for buffer recycle");
            Err("tx timeout: hardware didn't return buffer")
        } else {
            crate::vdebug!("NETHW", "TX: FAILED - no SNP protocol");
            Err("no snp")
        }
    }
}

/// Set receive filters on the SNP interface.
/// On bridged LANs, promiscuous mode ensures we receive broadcast DHCP responses.
pub fn set_receive_filters(promiscuous: bool) -> Result<(), &'static str> {
    unsafe {
        if let Some(ref snp_scoped) = OWNED_SNP {
            let snp = &**snp_scoped;
            let filters = ReceiveFlags::UNICAST | ReceiveFlags::BROADCAST;
            let _ = snp.receive_filters(filters, ReceiveFlags::empty(), promiscuous, None);
            crate::vdebug!("NETHW", "Receive filters set: promiscuous={}", promiscuous);
            Ok(())
        } else {
            Err("no snp")
        }
    }
}

/// Receive a raw Ethernet frame into the provided buffer. Returns length if a packet was received.
pub fn rx(buf: &mut [u8]) -> Result<usize, &'static str> {
    unsafe {
        if let Some(ref snp_scoped) = OWNED_SNP {
            let snp = &**snp_scoped;
            match snp.receive(buf, None, None, None, None) {
                Ok(sz) => {
                    Ok(sz)
                },
                Err(_) => {
                    // NOT_READY is the normal state when no packet is in the buffer
                    Err("rx none")
                },
            }
        } else {
            crate::vdebug!("NETHW", "RX: FAILED - no SNP protocol available");
            Err("no snp")
        }
    }
}

/// Try to locate a NIC via UEFI SNP and initialize it (Start + Initialize).
/// Returns Ok if at least one device was started and initialized.
pub fn init() -> Result<(), &'static str> {
    if is_initialized() && get_info().is_some() {
        vdebug_autoprefix!(11, "Network already initialized");
        return Ok(());
    }

    vdebug_autoprefix!(11, "Searching for network adapters...");
    let handles = match boot::locate_handle_buffer(SearchType::ByProtocol(&SimpleNetwork::GUID)) {
        Ok(buf) => buf,
        Err(_) => {
            vdebug_autoprefix!(12, "No SNP handles present - no network adapter detected by UEFI");
            return Err("no nic found");
        }
    };
    vdebug_autoprefix!(11, "Found {} SNP handle(s)", handles.as_slice().len());

    for &h in handles.as_slice().iter() {
        vdebug_autoprefix!(9, "Attempting to open SNP protocol on handle {:?}", h);
        let Ok(snp_scoped) = boot::open_protocol_exclusive::<SimpleNetwork>(h) else {
            vdebug_autoprefix!(14, "Failed to open SNP protocol exclusively on handle {:?}", h);
            continue;
        };
        vdebug_autoprefix!(11, "SNP protocol opened successfully");

        let snp = &*snp_scoped;
        let mode = snp.mode();
        vdebug_autoprefix!(9, "NIC state: {:?}, media present: {}", mode.state, bool::from(mode.media_present));

        match mode.state {
            NetworkState::STOPPED => {
                vdebug_autoprefix!(9, "NIC is STOPPED, calling start()");
                if snp.start().is_err() {
                    vdebug_autoprefix!(14, "NIC start() failed");
                    continue;
                }
                vdebug_autoprefix!(11, "NIC start() succeeded");
            }
            NetworkState::STARTED => {
                vdebug_autoprefix!(9, "NIC is STARTED, ready to initialize");
            }
            _ => {
                vdebug_autoprefix!(9, "NIC already initialized, resetting for safety");
                let _ = snp.reset(false);
            }
        }

        vdebug_autoprefix!(9, "Calling SNP initialize()");
        if let Err(e) = snp.initialize(0, 0) {
            vdebug_autoprefix!(14, "SNP initialize() failed: {:?}", e);
            continue;
        }
        vdebug_autoprefix!(11, "SNP initialize() succeeded");

        vdebug_autoprefix!(9, "Setting receive filters (UNICAST + BROADCAST)");
        let _ = snp.receive_filters(
            ReceiveFlags::UNICAST | ReceiveFlags::BROADCAST,
            ReceiveFlags::empty(),
            false,   // Don't go into Promiscuous mode
            None     // No hardware MAC list needed
        ).map_err(|_| "Failed to set receive filters")?;

        // Wait for the link to come up - virtual NICs (especially VirtualBox) may need time
        // to establish the connection after initialize(). Retry up to 5 seconds.
        vdebug_autoprefix!(9, "Waiting for link to come up (up to 5 seconds)...");
        let mut mode = snp.mode();
        let mut retries = 0;
        while !bool::from(mode.media_present) && retries < 50 {
            uefi::boot::stall(core::time::Duration::from_millis(100));
            mode = snp.mode();
            retries += 1;
        }
        if !bool::from(mode.media_present) {
            vdebug_autoprefix!(14, "Link still down after {} retries - continuing anyway, link may come up later", retries);
        } else {
            vdebug_autoprefix!(10, "Link came up after {} retries", retries);
        }
        let mac_len = (mode.hw_address_size as usize).min(32);
        let mut mac = [0u8; 32];
        mac[..mac_len].copy_from_slice(&mode.current_address.0[..mac_len]);

        // Log the link status before moving snp_scoped
        let link_is_up = bool::from(mode.media_present);
        if link_is_up {
            vdebug_autoprefix!(10, "Link UP");
        } else {
            vdebug_autoprefix!(14, "Link DOWN - packets may not be received");
        }

        // 5. Global State Update
        unsafe {
            NET_INFO = Some(NetHwInfo {
                mac,
                mac_len,
                mtu: mode.max_packet_size,
                media_present: link_is_up,
                state: 2,
            });

            NIC_HANDLE = Some(h);
            OWNED_SNP = Some(snp_scoped);
        }

        NET_INITIALIZED.store(true, Ordering::SeqCst);
        hpvm_info!("NETHW", "Network hardware initialized successfully - MAC: {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
        return Ok(());
    }

    hpvm_error!("NETHW", "No usable NIC found after trying all handles");
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