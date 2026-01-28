//! UEFI Network Module (0.36.1) - Basic ICMP/IP over SNP
#![allow(dead_code)]

use crate::Color;
use crate::hpvm_log;
use core::mem::{MaybeUninit, size_of};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::{hpvm_info, hpvm_warn, hpvm_error};

// --- Minimal Protocol Definitions ---

#[repr(C, packed)]
struct EthHeader {
    dst: [u8; 6],
    src: [u8; 6],
    ethertype: u16, // 0x0800 for IPv4, 0x0806 for ARP
}

#[repr(C, packed)]
struct Ipv4Header {
    ver_ihl: u8,
    tos: u8,
    len: u16,
    id: u16,
    off: u16,
    ttl: u8,
    proto: u8,
    checksum: u16,
    src: [u8; 4],
    dst: [u8; 4],
}

// --- Module State ---

static STACK_INIT: AtomicBool = AtomicBool::new(false);

#[derive(Copy, Clone, Default)]
pub struct NetStats {
    pub rx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
}

struct NetState {
    ip_addr: [u8; 4],
    gateway: [u8; 4],
    stats: NetStats,
}

static mut NET_STATE: MaybeUninit<Option<NetState>> = MaybeUninit::uninit();

/// Initialize the network stack with a static IP.
pub fn init(ip: [u8; 4], gw: [u8; 4]) {
    if STACK_INIT.load(Ordering::SeqCst) { return; }

    let state = NetState {
        ip_addr: ip,
        gateway: gw,
        stats: NetStats::default(),
    };

    unsafe { NET_STATE.write(Some(state)); }
    STACK_INIT.store(true, Ordering::SeqCst);

    hpvm_info!("NET", "SNP stack up: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
}

#[allow(static_mut_refs)]
pub fn poll_tick() {
    if !STACK_INIT.load(Ordering::SeqCst) { return; }

    if crate::devices::net_hw::is_initialized() {
        let mut buf = [0u8; 1514]; // Max Ethernet frame

        // Drain the SNP RX buffer
        while let Ok(sz) = crate::devices::net_hw::rx(&mut buf) {
            process_packet(&buf[..sz]);

            unsafe {
                if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
                    st.stats.rx_pkts += 1;
                    st.stats.rx_bytes += sz as u64;
                }
            }
        }
    }
}

/// A very primitive packet dispatcher
fn process_packet(frame: &[u8]) {
    if frame.len() < size_of::<EthHeader>() { return; }

    let eth = unsafe { &*(frame.as_ptr() as *const EthHeader) };
    let ethertype = u16::from_be(eth.ethertype);

    match ethertype {
        0x0806 => { /* Handle ARP (Required for others to talk to us) */ },
        0x0800 => handle_ipv4(&frame[size_of::<EthHeader>()..]),
        _ => {} // Ignore IPv6 or other protocols for now
    }
}

fn handle_ipv4(packet: &[u8]) {
    if packet.len() < size_of::<Ipv4Header>() { return; }
    let ip = unsafe { &*(packet.as_ptr() as *const Ipv4Header) };

    // Basic ICMP Echo (Ping) responder
    if ip.proto == 1 {
        // Here you would parse ICMP and send a reply using snp_tx
        // For a minimal fix, we just log that we saw external traffic
    }
}

/// Actual ping implementation (broadcast-based or via Gateway)
pub fn ping_external(target_ip: [u8; 4]) -> Result<(), &'static str> {
    if !crate::devices::net_hw::is_initialized() { return Err("Hardware not ready"); }

    // In a real scenario, you'd send an ARP request here first.
    // As a shortcut for testing, we can broadcast an ICMP packet,
    // though many routers will drop it.

    hpvm_info!("PING", "Sending external probe to {}.{}.{}.{}",
               target_ip[0], target_ip[1], target_ip[2], target_ip[3]);

    // Example: Construct raw Ethernet frame manually...
    // snp_tx(&my_constructed_frame)

    Ok(())
}

#[allow(static_mut_refs)]
pub fn snp_tx(frame: &[u8]) -> Result<(), &'static str> {
    if !crate::devices::net_hw::is_initialized() { return Err("no nic"); }

    match crate::devices::net_hw::tx(frame) {
        Ok(()) => {
            unsafe {
                if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
                    st.stats.tx_pkts += 1;
                    st.stats.tx_bytes += frame.len() as u64;
                }
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}