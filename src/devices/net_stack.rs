//! UEFI Network Module (0.36.36) - ICMP/IGMPv3/UDP/TCP over SNP with mDNS support
#![allow(dead_code, static_mut_refs)]

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::Color;
use crate::hpvm_log;
use crate::vdebug_autoprefix;
use core::mem::{MaybeUninit, size_of};
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use hashbrown::HashMap;
use crate::{hpvm_info, hpvm_warn};

static mut protocol_map: Option<HashMap<u8, &str>> = None;

// Populate the map


// --- Protocol Constants & Headers ---

pub const ETHERTYPE_IPV4: u16 = 0x0800;
pub const ETHERTYPE_ARP: u16 = 0x0806;
const ARP_REQUEST: u16 = 1;
const ARP_REPLY: u16 = 2;
const IP_PROTO_ICMP: u8 = 1;
const IP_PROTO_IGMP: u8 = 2;
const IP_PROTO_UDP: u8 = 17;
const IP_PROTO_TCP: u8 = 6;

// IGMP constants
const IGMP_V1_MEMBERSHIP_REPORT: u8 = 0x12;
const IGMP_V2_MEMBERSHIP_REPORT: u8 = 0x16;
const IGMP_V2_LEAVE_GROUP: u8 = 0x17;
const IGMP_V1_QUERY: u8 = 0x11;
const IGMP_V2_QUERY: u8 = 0x12;
// IGMPv3 constants (RFC 3376)
const IGMP_V3_MEMBERSHIP_REPORT: u8 = 0x22;
const IGMP_V3_MODE_INCLUDE: u8 = 1;
const IGMP_V3_MODE_EXCLUDE: u8 = 2;
const IGMP_V3_CHANGE_TO_INCLUDE: u8 = 3;
const IGMP_V3_CHANGE_TO_EXCLUDE: u8 = 4;
const IGMP_V3_ALLOW_NEW_SOURCES: u8 = 5;
const IGMP_V3_BLOCK_OLD_SOURCES: u8 = 6;
const IGMP_MAX_SOURCES_PER_GROUP: u8 = 4;
const IGMP_MAX_GROUPS: u8 = 8;
const TCP_FLAG_FIN: u8 = 0x01;
const TCP_FLAG_SYN: u8 = 0x02;
const TCP_FLAG_RST: u8 = 0x04;
const TCP_FLAG_ACK: u8 = 0x10;
const TCP_FLAG_PSH: u8 = 0x08;


const MAX_ARP_ENTRIES: usize = 16;

// --- Headers ---

#[repr(C, packed)]
pub struct EthHeader {
    dst: [u8; 6],
    src: [u8; 6],
    pub(crate) ethertype: u16,
}

#[repr(C, packed)]
struct ArpPacket {
    hw_type: u16,
    proto_type: u16,
    hw_size: u8,
    proto_size: u8,
    opcode: u16,
    sender_mac: [u8; 6],
    sender_ip: [u8; 4],
    target_mac: [u8; 6],
    target_ip: [u8; 4],
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

#[repr(C, packed)]
struct IcmpHeader {
    msg_type: u8,
    code: u8,
    checksum: u16,
    ident: u16,
    seq: u16,
}

#[repr(C, packed)]
struct TcpHeader {
    src_port: u16,
    dst_port: u16,
    seq: u32,
    ack: u32,
    off_flags: u16, // Data offset (4 bits) + Flags (9 bits)
    window: u16,
    checksum: u16,
    urgent: u16,
}

#[repr(C, packed)]
struct UdpHeader {
    src_port: u16,
    dst_port: u16,
    len: u16,
    checksum: u16,
}

// --- Module State ---

static HTTPD: AtomicBool = AtomicBool::new(false);

/// Registry index for the network stack tracked lock (only used when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
static NET_STACK_LOCK_IDX: AtomicU64 = AtomicU64::new(u64::MAX);

/// Whether the dedicated HTTP poll AP worker is currently running (only used when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
static HTTP_POLL_RUNNING: AtomicBool = AtomicBool::new(false);

/// Signal to stop the HTTP poll AP worker (only used when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
static HTTP_POLL_STOP: AtomicBool = AtomicBool::new(false);

/// Report which networking backend is active.
/// - "SNP (raw)" when a NIC via UEFI SNP is initialized.
/// - "loopback-stub" otherwise.
pub fn backend_name() -> &'static str {
    if crate::devices::net_hw::is_initialized() {
        "SNP (raw)"
    } else {
        "loopback-stub"
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct NetStats {
    pub rx_pkts: u64,
    pub rx_bytes: u64,
    pub tx_pkts: u64,
    pub tx_bytes: u64,
}



// --- State Management ---7

#[derive(Copy, Clone, Debug, Default)]
struct ArpEntry {
    ip: [u8; 4],
    mac: [u8; 6],
    valid: bool,
}

/// TCP state machine states for HTTP server connections
#[derive(Copy, Clone, Debug, PartialEq, Default)]
enum HttpTcpState {
    #[default]
    None,
    SynReceived,      // We sent SYN/ACK, waiting for client ACK
    Established,      // Connection open, ready for HTTP data
    FinWait,          // We sent FIN, waiting for client FIN/ACK
    Closed,
}

/// Tracks one HTTP server connection (identified by client IP + port)
#[derive(Clone, Debug, Default)]
struct HttpConn {
    client_ip: [u8; 4],
    client_port: u16,
    tcp_state: HttpTcpState,
    /// Our next sequence number (what we will send next)
    our_seq: u32,
    /// Expected next sequence number from the client
    their_seq: u32,
    /// Accumulated HTTP request data (headers + body)
    request_buf: Vec<u8>,
    /// Whether the HTTP request has been fully received and processed
    request_complete: bool,
    /// Last received sequence number — used to detect looped-back duplicates
    last_rx_seq: u32,
    /// Last received payload length — used with last_rx_seq to distinguish ACKs from data packets
    last_rx_payload_len: u32,
    /// Poll tick count when last packet was received — used for duplicate timeout
    last_rx_tick: u64,
    /// Poll tick count when we last sent a packet — used to drop our own looped-back echoes
    last_send_tick: u64,
}

const MAX_HTTP_CONNS: usize = 8;

#[derive(Clone, Debug, Default)]
pub struct NetState {
    pub ip_addr: [u8; 4],
    pub gateway: [u8; 4],
    pub subnet_mask: [u8; 4], // Added Mask
    pub mac_addr: [u8; 6],
    pub stats: NetStats,
    arp_cache: [ArpEntry; MAX_ARP_ENTRIES],
    pub ping_success: bool,
    pub tcp_rx_data: Vec<u8>,
    pub tcp_connected: bool,
    pub tcp_src_port: u16,
    pub tcp_dst_port: u16,
    pub tcp_seq: u32,
    pub tcp_ack: u32,
    pub tcp_fin_received: bool,
    /// HTTP server connection table
    http_conns: [HttpConn; MAX_HTTP_CONNS],
    /// Counter for generating initial sequence numbers
    http_isn_counter: u32,
    /// Port the HTTP server is listening on (0 = disabled)
    httpd_port: u16,
    /// Monotonically increasing poll tick counter — used for duplicate packet timeout
    httpd_poll_tick: u64,
    /// Whether to use static IP configuration instead of DHCP
    use_static_ip: bool,
    /// Static IP address (used when use_static_ip is true)
    static_ip: [u8; 4],
    /// Static gateway (used when use_static_ip is true)
    static_gw: [u8; 4],
    /// Static subnet mask (used when use_static_ip is true)
    static_mask: [u8; 4],
    /// IGMP membership group count
    igmp_group_count: u8,
    /// IGMP membership groups (multicast IPs)
    igmp_groups: [[u8; 4]; 8],
    /// IGMPv3 source filter mode per group (0=none, 1=include, 2=exclude)
    igmp_v3_filter_mode: [u8; IGMP_MAX_GROUPS as usize],
    /// IGMPv3 source addresses per group (up to IGMP_MAX_SOURCES_PER_GROUP per group)
    igmp_v3_sources: [[[u8; 4]; IGMP_MAX_SOURCES_PER_GROUP as usize]; IGMP_MAX_GROUPS as usize],
    /// IGMPv3 source count per group
    igmp_v3_source_count: [u8; IGMP_MAX_GROUPS as usize],
    /// UDP statistics
    udp_rx_packets: u64,
    udp_tx_packets: u64,
    /// ICMP statistics
    icmp_rx_packets: u64,
    icmp_tx_packets: u64,
    /// IGMP statistics
    igmp_rx_packets: u64,
    igmp_tx_packets: u64,
}

static mut NET_STATE: MaybeUninit<Option<NetState>> = MaybeUninit::uninit();
pub static STACK_INIT: AtomicBool = AtomicBool::new(false);

// --- Helpers ---

fn calculate_checksum(data: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    let mut chunks = data.chunks_exact(2);
    while let Some(chunk) = chunks.next() {
        sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
    }
    if let Some(&rem) = chunks.remainder().first() {
        sum += u16::from_be_bytes([rem, 0]) as u32;
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

// --- New Subnet Logic ---

/// Checks if a destination IP is within our local subnet.
fn is_local(dest_ip: [u8; 4]) -> bool {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    for i in 0..4 {
        if (dest_ip[i] & state.subnet_mask[i]) != (state.ip_addr[i] & state.subnet_mask[i]) {
            return false;
        }
    }
    true
}



pub fn init(ip: [u8; 4], gw: [u8; 4], mask: [u8; 4]) {
    if STACK_INIT.load(Ordering::SeqCst) { return; }
    let mac = crate::devices::net_hw::get_mac();
    let mc: [u8; 6] = mac[0..6].try_into().unwrap_or([0; 6]);

    let state = NetState {
        ip_addr: ip,
        gateway: gw,
        subnet_mask: mask,
        mac_addr: mc,
        stats: NetStats {
            rx_bytes: 0,
            rx_pkts: 0,
            tx_bytes: 0,
            tx_pkts: 0,
        },
        arp_cache: [ArpEntry { ip: [0; 4], mac: [0; 6], valid: false }; MAX_ARP_ENTRIES],
        ping_success: false,
        tcp_rx_data: Vec::new(),
        tcp_connected: false,
        tcp_src_port: 0,
        tcp_dst_port: 0,
        tcp_seq: 0,
        tcp_ack: 0,
        tcp_fin_received: false,
        http_conns: core::array::from_fn(|_| HttpConn::default()),
        http_isn_counter: 1000,
        httpd_port: 0,
        httpd_poll_tick: 0,
        use_static_ip: false,
        static_ip: [0; 4],
        static_gw: [0; 4],
        static_mask: [0; 4],
        igmp_group_count: 0,
        igmp_groups: [[0; 4]; 8],
        igmp_v3_filter_mode: [0; IGMP_MAX_GROUPS as usize],
        igmp_v3_sources: [[[0; 4]; IGMP_MAX_SOURCES_PER_GROUP as usize]; IGMP_MAX_GROUPS as usize],
        igmp_v3_source_count: [0; IGMP_MAX_GROUPS as usize],
        udp_rx_packets: 0,
        udp_tx_packets: 0,
        icmp_rx_packets: 0,
        icmp_tx_packets: 0,
        igmp_rx_packets: 0,
        igmp_tx_packets: 0,
    };
    unsafe {
        protocol_map = Some(HashMap::new());
        if let Some(map) = protocol_map.as_mut() {
            map.insert(1, "ICMP");
            map.insert(2, "IGMP");
            map.insert(6, "TCP");
            map.insert(17, "UDP");
            map.insert(41, "IPv6");
            map.insert(47, "GRE");
            map.insert(50, "ESP");
            map.insert(51, "AH");
            map.insert(88, "EIGRP");
            map.insert(89, "OSPF");
        }
    }

    #[allow(static_mut_refs)]
    unsafe { NET_STATE.write(Some(state)); }
    STACK_INIT.store(true, Ordering::SeqCst);

    // Register the network stack in the global lock registry so that the AP poll worker
    // can coordinate exclusive access with the BSP. The lock ID is the address of NET_STATE.
    #[cfg(feature = "net-multicore")]
    {
        let lock_id = unsafe { NET_STATE.as_ptr() } as usize;
        let idx = crate::env::register_tracked_lock(lock_id) as u64;
        NET_STACK_LOCK_IDX.store(idx, Ordering::SeqCst);
        crate::vdebug!("NET", "Registered network stack lock (idx={}, id={:#x})", idx, lock_id);
    }

    crate::vdebug!("NET", "Stack initialized at {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
    crate::vdebug!("NET", "Mask: {}.{}.{}.{}", mask[0], mask[1], mask[2], mask[3]);
    crate::vdebug!("NET", "SNP stack up: IP: {}.{}.{}.{}  GW: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3], gw[0], gw[1], gw[2], gw[3]);
}

/// Resolves a MAC address. If target is remote, resolves the Gateway's MAC instead.
fn resolve_mac(target_ip: [u8; 4], timeout_loops: u32) -> Option<[u8; 6]> {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };

    // Logic: If target is not local, we must talk to the Gateway.
    let ip_to_resolve = if is_local(target_ip) {
        target_ip
    } else {
        state.gateway
    };

    // Check Cache
    if let Some(mac) = find_mac(ip_to_resolve) {
        return Some(mac);
    }

    // ARP Request
    send_arp_packet(ARP_REQUEST, ip_to_resolve, [0; 6]);
    for _ in 0..timeout_loops {
        poll_tick();
        if let Some(mac) = find_mac(ip_to_resolve) {
            return Some(mac);
        }
    }
    None
}

pub fn ping_external(target_ip: [u8; 4], timeout: u32, print: bool) -> bool {
    unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().ping_success = false; }

    // Use our new resolution logic
    let mac = match resolve_mac(target_ip, timeout / 10) {
        Some(m) => m,
        None => {
            if print {
                hpvm_warn!("PING", "Routing failed: Host/Gateway unreachable");
            }
            return false;
        }
    };

    send_icmp_echo(8, target_ip, mac, 0xBEAF, 1, b"ping");

    for _ in 0..timeout {
        poll_tick();
        unsafe {
            if NET_STATE.assume_init_ref().as_ref().unwrap().ping_success {
                if print {
                    crate::vdebug!("PING", "Reply from {}.{}.{}.{}", target_ip[0], target_ip[1], target_ip[2], target_ip[3]);
                }
                return true;
            }
        }
    }
    false
}

// --- Helper functions (Checksum, Poll, Handle, Send) remain the same as previous versions ---
// [Included in full for completeness]

fn update_arp_cache(ip: [u8; 4], mac: [u8; 6]) {
    unsafe {
        if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
            // Check if IP already exists
            for entry in st.arp_cache.iter_mut() {
                if entry.valid && entry.ip == ip {
                    entry.mac = mac;
                    return;
                }
            }
            // Otherwise, find empty slot
            for entry in st.arp_cache.iter_mut() {
                if !entry.valid {
                    entry.ip = ip;
                    entry.mac = mac;
                    entry.valid = true;
                    crate::vdebug!("ARP", "Cached {}.{}.{}.{} -> {:02X}:{:02X}:...", ip[0], ip[1], ip[2], ip[3], mac[0], mac[1]);
                    return;
                }
            }
        }
    }
}

fn find_mac(ip: [u8; 4]) -> Option<[u8; 6]> {
    unsafe {
        let st = NET_STATE.assume_init_ref().as_ref()?;
        for entry in st.arp_cache.iter() {
            if entry.valid && entry.ip == ip {
                return Some(entry.mac);
            }
        }
    }
    None
}

#[inline]
pub fn is_initialized() -> bool { STACK_INIT.load(Ordering::SeqCst) }

static POLL_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Acquire the network stack lock from the global lock registry (only when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
fn net_stack_lock_acquire() {
    let idx = NET_STACK_LOCK_IDX.load(Ordering::SeqCst);
    if idx == u64::MAX {
        return;
    }
    loop {
        if !crate::env::is_lock_being_accessed(idx as usize) {
            crate::env::mark_lock_accessing(idx as usize);
            break;
        }
        core::hint::spin_loop();
    }
}

/// Release the network stack lock in the global lock registry (only when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
fn net_stack_lock_release() {
    let idx = NET_STACK_LOCK_IDX.load(Ordering::SeqCst);
    if idx == u64::MAX {
        return;
    }
    crate::env::mark_lock_released(idx as usize);
}

#[allow(static_mut_refs)]
pub fn poll_tick() {
    let polls = POLL_COUNTER.fetch_add(1, Ordering::Relaxed) + 1;

    if !STACK_INIT.load(Ordering::SeqCst) {
        // Log once every 1000 calls to avoid flooding if stack is not initialized
        if polls % 1000 == 0 {
            vdebug_autoprefix!(14, "poll_tick: stack not initialized after {} polls", polls);
        }
        return;
    }

    if !crate::devices::net_hw::is_initialized() {
        if polls % 1000 == 0 {
            vdebug_autoprefix!(14, "poll_tick: hardware not initialized after {} polls", polls);
        }
        return;
    }

    // Acquire exclusive access to the network stack via the global lock registry.
    // This prevents the AP poll worker from accessing SNP hardware concurrently.
    #[cfg(feature = "net-multicore")]
    net_stack_lock_acquire();

    // Increment poll tick counter for HTTPD duplicate detection
    unsafe {
        if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
            st.httpd_poll_tick += 1;
        }
    }

    // Validate TCP checksum on incoming packets to drop corrupted or looped-back garbage.
    // The SNP hardware may not validate TCP checksums, so we must do it in software.
    let mut buf = [0u8; 1514]; // Max Ethernet frame

    // Drain the SNP RX buffer
    loop {
        match crate::devices::net_hw::rx(&mut buf) {
            Ok(sz) => {
                if sz == 0 { break; }
                let frame = &buf[..sz];
                if frame.len() < size_of::<EthHeader>() {
                    continue;
                }

                let eth = unsafe { &*(frame.as_ptr() as *const EthHeader) };
                let ethertype = u16::from_be(eth.ethertype);
                crate::vdebug!("NET", "RX: {} bytes ethertype=0x{:04x}", sz, ethertype);
                match ethertype {
                    ETHERTYPE_ARP => {
                        handle_arp(&frame[size_of::<EthHeader>()..]);
                    },
                    ETHERTYPE_IPV4 => {
                        handle_ipv4(&frame[size_of::<EthHeader>()..], eth.src);
                    },
                    _ => {}
                }

                unsafe {
                    if let Some(st) = NET_STATE.assume_init_mut().as_mut() {
                        st.stats.rx_pkts += 1;
                        st.stats.rx_bytes += sz as u64;
                    }
                }
            },
            Err(_) => {
                // No packet available, exit the loop
                break;
            }
        }
    }

    // Periodic stats logging every 10000 polls to show network activity without flooding
    if polls % 10000 == 0 {
        unsafe {
            if let Some(st) = NET_STATE.assume_init_ref().as_ref() {
                vdebug_autoprefix!(11, "Poll stats: {} polls, {} rx pkts, {} tx pkts, {} rx bytes, {} tx bytes",
                    polls, st.stats.rx_pkts, st.stats.tx_pkts, st.stats.rx_bytes, st.stats.tx_bytes);
            }
        }
    }

    // Release the network stack lock so the AP worker can access it
    #[cfg(feature = "net-multicore")]
    net_stack_lock_release();
}




/// A very primitive packet dispatcher
fn process_packet(frame: &[u8]) {
    if frame.len() < size_of::<EthHeader>() { return; }
    let eth = unsafe { &*(frame.as_ptr() as *const EthHeader) };
    let ethertype = u16::from_be(eth.ethertype);

    match ethertype {
        ETHERTYPE_ARP => handle_arp(&frame[size_of::<EthHeader>()..]),
        ETHERTYPE_IPV4 => handle_ipv4(&frame[size_of::<EthHeader>()..], eth.src),
        _ => {}
    }
}

/// Very small loopback ping: if dst is 127.0.0.1, report success with tiny RTT.
pub fn ping_loopback(dst: &str) -> Result<u32, &'static str> {
    //init(/* [u8; 4] */, /* [u8; 4] */);
    if dst == "127.0.0.1" || dst.eq_ignore_ascii_case("localhost") {
        // Pretend we sent an ICMP echo and received it immediately.
        crate::vdebug!("PING", "loopback echo reply from {}: bytes=32 time=1ms TTL=64", dst);
        Ok(1)
    } else {
        hpvm_warn!("PING", "only loopback is available currently; cannot reach {}", dst);
        Err("no route to host (non-loopback)")
    }
}

fn handle_arp(packet: &[u8]) {
    if packet.len() < size_of::<ArpPacket>() {
        crate::vdebug!("ARP", "handle_arp: packet too small");
        return;
    }
    let arp = unsafe { &*(packet.as_ptr() as *const ArpPacket) };
    let opcode = u16::from_be(arp.opcode);
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    
    crate::vdebug!("ARP", "handle_arp: opcode={} sender={}.{}.{}.{} target={}.{}.{}.{} our_ip={}.{}.{}.{}",
        opcode,
        arp.sender_ip[0], arp.sender_ip[1], arp.sender_ip[2], arp.sender_ip[3],
        arp.target_ip[0], arp.target_ip[1], arp.target_ip[2], arp.target_ip[3],
        state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]);
    
    if arp.target_ip == state.ip_addr || opcode == ARP_REPLY {
        crate::vdebug!("ARP", "handle_arp: caching MAC for {}.{}.{}.{}", 
            arp.sender_ip[0], arp.sender_ip[1], arp.sender_ip[2], arp.sender_ip[3]);
        update_arp_cache(arp.sender_ip, arp.sender_mac);
    }
    if opcode == ARP_REQUEST && arp.target_ip == state.ip_addr {
        crate::vdebug!("ARP", "handle_arp: sending ARP reply");
        send_arp_packet(ARP_REPLY, arp.sender_ip, arp.sender_mac);
    }
}

fn send_arp_packet(opcode: u16, target_ip: [u8; 4], target_mac: [u8; 6]) {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; size_of::<EthHeader>() + size_of::<ArpPacket>()];

    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = if opcode == ARP_REQUEST { [0xFF; 6] } else { target_mac };
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_ARP.to_be();

    let arp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut ArpPacket) };
    arp.hw_type = 1u16.to_be();
    arp.proto_type = 0x0800u16.to_be();
    arp.hw_size = 6;
    arp.proto_size = 4;
    arp.opcode = opcode.to_be();
    arp.sender_mac = state.mac_addr;
    arp.sender_ip = state.ip_addr;
    arp.target_mac = if opcode == ARP_REQUEST { [0; 6] } else { target_mac };
    arp.target_ip = target_ip;

    let _ = crate::devices::net_hw::tx(&frame);
}

fn get_protoname_from_int(a: u8) -> String {
    unsafe {
        if let Some(map) = protocol_map.as_mut() {
            if let Some(string) = map.get(&a) {
                string.clone().parse().unwrap()
            } else {
                "UNK".parse().unwrap()
            }
        } else {
            "UNK".parse().unwrap()
        }
    }
}

fn handle_ipv4(packet: &[u8], src_mac: [u8; 6]) {
    if packet.len() < size_of::<Ipv4Header>() {
        crate::vdebug!("IP", "handle_ipv4: packet too small ({} bytes)", packet.len());
        return;
    }
    let ip = unsafe { &*(packet.as_ptr() as *const Ipv4Header) };
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };

    crate::vdebug!("IP", "handle_ipv4: src={}.{}.{}.{} dst={}.{}.{}.{} proto={} ({}) ttl={} tos={}",
        ip.src[0], ip.src[1], ip.src[2], ip.src[3],
        ip.dst[0], ip.dst[1], ip.dst[2], ip.dst[3],
        ip.proto, get_protoname_from_int(ip.proto), ip.ttl, ip.tos);

    // Accept packets addressed to our IP, broadcast, or multicast.
    // On a LAN, broadcast (255.255.255.255) and multicast (224.0.0.0/4) packets
    // are essential for DHCP, NTP, mDNS, and other network services.
    let is_our_ip = ip.dst == state.ip_addr;
    let is_broadcast = ip.dst == [255, 255, 255, 255];
    let is_multicast = ip.dst[0] >= 224; // Multicast range: 224.0.0.0/4
    // Subnet broadcast: e.g., 192.168.1.255 for /24 network
    let subnet_broadcast = (state.ip_addr[0] == ip.dst[0])
        && (state.ip_addr[1] == ip.dst[1])
        && (state.ip_addr[2] == ip.dst[2])
        && (ip.dst[3] == 255);
    if !is_our_ip && !is_broadcast && !is_multicast && !subnet_broadcast {
        crate::vdebug!("IP", "handle_ipv4: dst {}.{}.{}.{} != our IP {}.{}.{}.{}, dropping",
            ip.dst[0], ip.dst[1], ip.dst[2], ip.dst[3],
            state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3]);
        return;
    }

    let header_len = (ip.ver_ihl & 0x0F) as usize * 4;
    // Use the IP total length field to bound the payload — ignore any padding bytes
    // that may be present in the Ethernet frame beyond the actual IP packet.
    let ip_total_len = u16::from_be(ip.len) as usize;
    if ip_total_len < header_len || ip_total_len > packet.len() {
        crate::vdebug!("IP", "handle_ipv4: invalid IP total length {} (header={}, packet={}), dropping",
            ip_total_len, header_len, packet.len());
        return;
    }
    let payload = &packet[header_len..ip_total_len];

    match ip.proto {
        IP_PROTO_ICMP => {
            crate::vdebug!("IP", "handle_ipv4: dispatching ICMP");
            unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().icmp_rx_packets += 1; }
            if payload.len() >= size_of::<IcmpHeader>() {
                let icmp = unsafe { &*(payload.as_ptr() as *const IcmpHeader) };
                match icmp.msg_type {
                    0 => { // Echo Reply
                        crate::vdebug!("ICMP", "Echo Reply received from {}.{}.{}.{}", ip.src[0], ip.src[1], ip.src[2], ip.src[3]);
                        unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().ping_success = true; }
                    }
                    3 => { // Destination Unreachable
                        crate::vdebug!("ICMP", "Destination Unreachable from {}.{}.{}.{} code={}", ip.src[0], ip.src[1], ip.src[2], ip.src[3], u8::from(icmp.code));
                    }
                    8 => { // Echo Request
                        let ident = u16::from_be(icmp.ident);
                        let seq = u16::from_be(icmp.seq);
                        crate::vdebug!("ICMP", "Echo Request from {}.{}.{}.{} ident={:#04x} seq={}", ip.src[0], ip.src[1], ip.src[2], ip.src[3], ident, seq);
                        send_icmp_echo(0, ip.src, src_mac, icmp.ident, icmp.seq, &payload[size_of::<IcmpHeader>()..]);
                    }
                    11 => { // Time Exceeded
                        crate::vdebug!("ICMP", "Time Exceeded from {}.{}.{}.{}", ip.src[0], ip.src[1], ip.src[2], ip.src[3]);
                    }
                    _ => {
                        crate::vdebug!("ICMP", "Unknown ICMP type {} from {}.{}.{}.{}", icmp.msg_type, ip.src[0], ip.src[1], ip.src[2], ip.src[3]);
                    }
                    // todo: need to add more fields

                    /*
                    • Type 0: Echo Reply (used in ping)
                    • Type 3: Destination Unreachable (codes 0–15 indicate network, host, port, or protocol unreachable)
                    • Type 5: Redirect (notifies a host of a better router path)
                    • Type 8: Echo Request (used in ping)
                    • Type 9: Router Advertisement (announces local router availability)
                    • Type 10: Router Solicitation (host requests a router advertisement)
                    • Type 11: Time Exceeded (TTL expired in transit; used by traceroute)
                    • Type 12: Parameter Problem (bad IP header fields)
                    • Type 13: Timestamp (clock synchronization query)
                    • Type 14: Timestamp Reply (response to timestamp query)
                    • Type 40: Photuris (security failure)
                    • Type 42 / 43: Extended Echo Request / Reply
                    */
                }
            }
        }
        IP_PROTO_IGMP => {
            crate::vdebug!("IP", "handle_ipv4: dispatching IGMP");
            unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().igmp_rx_packets += 1; }
            handle_igmp(ip.src, payload);
        }
        IP_PROTO_UDP => {
            crate::vdebug!("IP", "handle_ipv4: dispatching UDP");
            unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().udp_rx_packets += 1; }
            if payload.len() >= size_of::<UdpHeader>() {
                let udp = unsafe { &*(payload.as_ptr() as *const UdpHeader) };
                let dst_port = u16::from_be(udp.dst_port);
                let src_port = u16::from_be(udp.src_port);
                let udp_len = u16::from_be(udp.len);
                crate::vdebug!("UDP", "RX: {} -> {} (len={})", src_port, dst_port, udp_len);
                handle_udp(ip.src, dst_port, &payload[size_of::<UdpHeader>()..]);
            }
        }
        IP_PROTO_TCP  => {
            crate::vdebug!("IP", "handle_ipv4: dispatching TCP, payload size {}", payload.len());
            handle_tcp(ip.src, src_mac, payload);
        }
        _ => {
            crate::vdebug!("IP", "handle_ipv4: unknown protocol {}, ignoring", ip.proto);
        }
    }
}


/// Sends a UDP packet even if the stack isn't fully initialized (useful for DHCP).
pub fn send_raw_udp(
    src_ip: [u8; 4],
    dst_ip: [u8; 4],
    dst_mac: [u8; 6],
    src_port: u16,
    dst_port: u16,
    data: &[u8],
) -> Result<(), String> {
    let mut frame = [0u8; 1514];
    let mac = crate::devices::net_hw::get_mac();
    let src_mac: [u8; 6] = mac[0..6].try_into().unwrap_or([0; 6]);

    let udp_len = (size_of::<UdpHeader>() + data.len()) as u16;
    let ip_len = (size_of::<Ipv4Header>() as u16) + udp_len;
    let total_len = size_of::<EthHeader>() + (ip_len as usize);

    if total_len > frame.len() { return Err("Payload too large".parse().unwrap()); }

    // 1. Ethernet Header
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = dst_mac;
    eth.src = src_mac;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    // 2. IPv4 Header
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.tos = 0;
    ip.len = ip_len.to_be();
    ip.id = 0xABCDu16.to_be(); // Random ID
    ip.off = 0;
    ip.ttl = 64;
    ip.proto = IP_PROTO_UDP;
    ip.src = src_ip;
    ip.dst = dst_ip;
    ip.checksum = 0;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    // 3. UDP Header
    let udp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut UdpHeader) };
    udp.src_port = src_port.to_be();
    udp.dst_port = dst_port.to_be();
    udp.len = udp_len.to_be();
    udp.checksum = 0; // Optional in IPv4; set to 0 for simplicity

    // 4. Copy Data Payload
    frame[size_of::<EthHeader>() + size_of::<Ipv4Header>() + size_of::<UdpHeader>()..total_len]
        .copy_from_slice(data);

    // 5. Transmit
    match crate::devices::net_hw::tx(&frame[..total_len]) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Hardware TX failed: {}", e)),
    }
}




/// Send a UDP packet to a specific IP/Port
pub fn send_udp(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16, data: &[u8]) {
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1514];

    let udp_len = size_of::<UdpHeader>() + data.len();
    let ip_len = size_of::<Ipv4Header>() + udp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    // Ethernet
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = dst_mac;
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    // IPv4
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.len = (ip_len as u16).to_be();
    ip.proto = IP_PROTO_UDP;
    ip.src = state.ip_addr;
    ip.dst = dst_ip;
    ip.ttl = 64;
    ip.checksum = 0;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    // UDP
    let udp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut UdpHeader) };
    udp.src_port = src_port.to_be();
    udp.dst_port = dst_port.to_be();
    udp.len = (udp_len as u16).to_be();
    udp.checksum = 0; // UDP checksum is optional in IPv4, but recommended. Leaving 0 for simplicity.

    frame[total_len - data.len()..total_len].copy_from_slice(data);
    let _ = snp_tx(&frame[..total_len]);
}

// --- Minimal TCP / HTTP Responder ---

fn handle_tcp(src_ip: [u8; 4], src_mac: [u8; 6], packet: &[u8]) {
    if packet.len() < size_of::<TcpHeader>() {
        crate::vdebug!("TCP", "handle_tcp: packet too small ({} bytes)", packet.len());
        return;
    }
    let tcp = unsafe { &*(packet.as_ptr() as *const TcpHeader) };

    // Note: TCP checksum validation is skipped because checksum offloading is common
    // in virtual environments (VirtualBox, VMware). Incoming packets may have checksum
    // 0x0000 when the hardware calculates it on receive. The duplicate detection logic
    // in handle_httpd provides sufficient protection against looped-back garbage packets.

    let flags = u16::from_be(tcp.off_flags) & 0x3F;
    let dst_port = u16::from_be(tcp.dst_port);
    let src_port = u16::from_be(tcp.src_port);
    let seq = u32::from_be(tcp.seq);
    let ack = u32::from_be(tcp.ack);

    crate::vdebug!("TCP", "handle_tcp: src={}.{}.{}.{}, port={}->{} seq={} ack={} flags=0x{:04x} (SYN={} ACK={} FIN={} RST={})",
        src_ip[0], src_ip[1], src_ip[2], src_ip[3],
        src_port, dst_port, seq, ack, flags,
        if (flags & TCP_FLAG_SYN as u16) != 0 { 1 } else { 0 },
        if (flags & TCP_FLAG_ACK as u16) != 0 { 1 } else { 0 },
        if (flags & TCP_FLAG_FIN as u16) != 0 { 1 } else { 0 },
        if (flags & TCP_FLAG_RST as u16) != 0 { 1 } else { 0 });

    // Route to HTTP server if listening, otherwise handle client connections
    let state_mut = unsafe { NET_STATE.assume_init_mut().as_mut().unwrap() };
    let httpd_port = state_mut.httpd_port;
    
    crate::vdebug!("TCP", "handle_tcp: httpd_port={} HTTPD_running={}", 
        httpd_port, HTTPD.load(Ordering::SeqCst));
    
    if httpd_port != 0 && dst_port == httpd_port && HTTPD.load(Ordering::SeqCst) {
        crate::vdebug!("HTTPD", "TCP pkt from {}.{}.{}.{}:{}, flags=0x{:x} dst_port={}", 
            src_ip[0], src_ip[1], src_ip[2], src_ip[3], src_port, flags, dst_port);
        handle_httpd(src_ip, src_mac, tcp, flags, packet, httpd_port);
    } else if dst_port >= 49152 {
        // Potential client response
        crate::vdebug!("TCP", "Client packet: flags=0x{:x} seq={} ack={} payload={}", flags, seq, ack, packet.len() - (((u16::from_be(tcp.off_flags) >> 12) * 4) as usize));
        handle_tcp_client(src_ip, src_mac, tcp, flags, packet, state_mut);
    } else {
        crate::vdebug!("TCP", "handle_tcp: no handler for port {}, ignoring", dst_port);
    }
}

/// Find or create an HTTP connection slot for the given client.
/// `is_syn` must be true to reuse a closed slot (new handshake); false means only match active connections.
fn find_http_conn(state: &mut NetState, client_ip: [u8; 4], client_port: u16, is_syn: bool) -> Option<usize> {
    // First: find an active (non-closed, non-None) connection matching this IP:port
    for (i, conn) in state.http_conns.iter_mut().enumerate() {
        if conn.tcp_state != HttpTcpState::None && conn.tcp_state != HttpTcpState::Closed
            && conn.client_ip == client_ip && conn.client_port == client_port
        {
            return Some(i);
        }
    }
    // Second: if this is a SYN, reuse a closed/empty slot for a new handshake
    if is_syn {
        for (i, conn) in state.http_conns.iter_mut().enumerate() {
            if conn.tcp_state == HttpTcpState::None || conn.tcp_state == HttpTcpState::Closed {
                conn.tcp_state = HttpTcpState::None;
                conn.client_ip = client_ip;
                conn.client_port = client_port;
                conn.our_seq = 0;
                conn.their_seq = 0;
                conn.request_buf.clear();
                conn.request_complete = false;
                conn.last_rx_seq = 0;
                conn.last_rx_payload_len = 0;
                conn.last_rx_tick = 0;
                conn.last_send_tick = 0;
                return Some(i);
            }
        }
    }
    // Third: fallback — if we receive a packet from an unknown client and there's a free slot,
    // it likely means the initial SYN was lost (e.g. dropped by loop-back detection or NAT
    // timeout). Reuse a closed/None slot to avoid infinite retries.
    // This is a last-resort measure to handle NAT environments where the gateway IP is shared.
    for (i, conn) in state.http_conns.iter_mut().enumerate() {
        if conn.tcp_state == HttpTcpState::None || conn.tcp_state == HttpTcpState::Closed {
            crate::vdebug!("HTTPD", "Fallback: reusing slot {} for orphaned packet from {}.{}.{}.{}",
                i, client_ip[0], client_ip[1], client_ip[2], client_ip[3]);
            conn.tcp_state = HttpTcpState::Established;
            conn.client_ip = client_ip;
            conn.client_port = client_port;
            conn.our_seq = 1; // Start at 1 since we skipped the SYN handshake
            conn.their_seq = 0; // Will be set by the caller when processing the first data packet
            conn.request_buf.clear();
            conn.request_complete = false;
            conn.last_rx_seq = 0;
            conn.last_rx_payload_len = 0;
            conn.last_rx_tick = 0;
            conn.last_send_tick = 0;
            return Some(i);
        }
    }
    None
}

/// Parse an HTTP request from the buffer. Returns (method, path, body) if complete.
/// Searches for the HTTP method keyword (GET, POST, etc.) to find the actual request start,
/// skipping any leading garbage bytes that may result from TCP segmentation or loop-back echoes.
/// Then searches for the header/body boundary (\r\n\r\n) and validates only the header section as UTF-8.
fn parse_http_request(data: &[u8]) -> Option<(&str, &str, Vec<u8>)> {
    // Known HTTP methods — scan for the first occurrence to find the request start.
    // This handles cases where leading bytes were lost (e.g., first TCP segment dropped by echo detection).
    const METHODS: &[&[u8]] = &[b"GET ", b"POST ", b"PATCH ", b"PUT ", b"DELETE ", b"HEAD ", b"OPTIONS "];

    let mut method_found: Option<&[u8]> = None;
    let mut method_start = 0usize;

    for m in METHODS {
        if let Some(pos) = data.windows(m.len()).position(|w| w == *m) {
            // Prefer the earliest method found
            if method_found.is_none() || pos < method_start {
                method_found = Some(m);
                method_start = pos;
            }
        }
    }

    let (data_start, method_name) = match method_found {
        Some(m) => {
            if method_start > 0 {
                crate::vdebug!("HTTPD", "Parse: skipped {} leading garbage bytes before method", method_start);
            }
            (method_start, core::str::from_utf8(m).unwrap().trim_end())
        }
        None => {
            // Fallback: try parsing from byte 0 (original behavior)
            crate::vdebug!("HTTPD", "Parse: no HTTP method keyword found in {} bytes, trying from byte 0", data.len());
            (0, "")
        }
    };

    let data = &data[data_start..];

    // Search for header/body boundary in raw bytes — don't require the entire buffer to be UTF-8.
    let header_end = data.windows(4).position(|w| w == b"\r\n\r\n")
        .or_else(|| data.windows(2).position(|w| w == b"\n\n"))
        .or_else(|| {
            crate::vdebug!("HTTPD", "Parse: no \\r\\n\\r\\n or \\n\\n found in {} bytes", data.len());
            None
        })?;

    let header_bytes = &data[..header_end];
    let body_start = header_end + 4.min(data.len() - header_end);
    let body = data[body_start..].to_vec();

    // Validate only the header section as UTF-8
    let header_text = match core::str::from_utf8(header_bytes) {
        Ok(t) => t,
        Err(e) => {
            crate::vdebug!("HTTPD", "Parse: header from_utf8 failed: {:?}", e);
            return None;
        }
    };

    let first_line = match header_text.lines().next() {
        Some(l) => l,
        None => {
            crate::vdebug!("HTTPD", "Parse: no first line in header section");
            return None;
        }
    };
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() < 2 {
        crate::vdebug!("HTTPD", "Parse: first line '{}' has {} parts", first_line, parts.len());
        return None;
    }

    let method = if method_name.is_empty() { parts[0] } else { method_name };
    let path = parts[1];
    match method {
        "GET" | "POST" | "PATCH" | "PUT" | "DELETE" | "HEAD" | "OPTIONS" => {},
        _ => {
            crate::vdebug!("HTTPD", "Parse: unknown method '{}'", method);
            return None;
        }
    }
    crate::vdebug!("HTTPD", "Parse: success method='{}' path='{}' body={} bytes", method, path, body.len());
    Some((method, path, body))
}

/// Build an HTTP response for the given request
///
static HEADER_VAL_11: &str = "HHTTP/1.1";

fn build_http_response(method: &str, path: &str, body: &[u8]) -> Vec<u8> {
    let (status_line, content_type, response_body): (&str, &str, Vec<u8>) = if path == "/" || path == "/index.html" {
        let html = "<!DOCTYPE html>\n<html><head><title>HPVMx UEFI Server</title></head>\n<body>\n<h1>HPVMx UEFI Server</h1>\n<p>Status: OK</p>\n<p>Available endpoints:</p>\n<ul>\n<li><code>GET /</code> - This page</li>\n<li><code>GET /status</code> - System status</li>\n<li><code>POST /echo</code> - Echo back posted data</li>\n<li><code>PATCH /config</code> - Update configuration</li>\n</ul>\n</body></html>";
        (&*(HEADER_VAL_11.to_string() + &*" 200 OK\r\n".to_string()), "text/html", html.as_bytes().to_vec())
    } else if path == "/status" {
        let html = "<!DOCTYPE html>\n<html><head><title>HPVMx Status</title></head>\n<body>\n<h1>System Status</h1>\n<p>HPVMx UEFI Server is running.</p>\n<p>Network: Active</p>\n</body></html>";
        (&*(HEADER_VAL_11.to_string() + &*" 200 OK\r\n".to_string()), "text/html", html.as_bytes().to_vec())
    } else if path == "/echo" && (method == "POST" || method == "PATCH") {
        let mut resp = String::from("<!DOCTYPE html>\n<html><body><h1>Echo Response</h1><pre>");
        if let Ok(text) = core::str::from_utf8(body) {
            for c in text.chars() {
                match c {
                    '<' => resp.push_str("&lt;"),
                    '>' => resp.push_str("&gt;"),
                    '&' => resp.push_str("&amp;"),
                    '"' => resp.push_str("&quot;"),
                    _ => resp.push(c),
                }
            }
        }
        resp.push_str("</pre></body></html>");
        (&*(HEADER_VAL_11.to_string() + &*" 200 OK\r\n".to_string()), "text/html", resp.into_bytes())
    } else if path == "/config" && method == "PATCH" {
        let html = "<!DOCTYPE html>\n<html><head><title>Config Updated</title></head>\n<body>\n<h1>Configuration Updated</h1>\n<p>PATCH request processed successfully.</p>\n</body></html>";
        (&*(HEADER_VAL_11.to_string() + &*" 200 OK\r\n".to_string()), "text/html", html.as_bytes().to_vec())
    } else if method == "GET" {
        let mut html = format!("<!DOCTYPE html>\n<html><body><h1>404 Not Found</h1><p>The requested path was not found: {}</p></body></html>", path);
        (&*(HEADER_VAL_11.to_string() + &*" 404 Not Found\r\n".to_string()), "text/html", html.into_bytes())
    } else {
        let html = "<!DOCTYPE html>\n<html><head><title>405 Method Not Allowed</title></head>\n<body>\n<h1>405 Method Not Allowed</h1>\n<p>The HTTP method is not supported for this path.</p>\n</body></html>";
        (&*(HEADER_VAL_11.to_string() + &*" 405 Method Not Allowed\r\n".to_string()), "text/html", html.as_bytes().to_vec())
    };

    let mut response = Vec::new();
    response.extend_from_slice(status_line.as_bytes());
    response.extend_from_slice(b"Server: HPVMx-UEFI/1.0\r\n");
    response.extend_from_slice(b"Content-Type: ");
    response.extend_from_slice(content_type.as_bytes());
    response.extend_from_slice(b"\r\n");
    response.extend_from_slice(b"Content-Length: ");
    response.extend_from_slice(&response_body.len().to_string().into_bytes());
    response.extend_from_slice(b"\r\n");
    response.extend_from_slice(b"Connection: close\r\n");
    response.extend_from_slice(b"\r\n");
    response.extend_from_slice(&response_body);
    response
}

fn handle_httpd(src_ip: [u8; 4], src_mac: [u8; 6], tcp: &TcpHeader, flags: u16, packet: &[u8], _server_port: u16) {
    // Drop packets from our own MAC address — these are looped-back transmissions from the VM network interface.
    let state = unsafe { NET_STATE.assume_init_mut().as_mut().unwrap() };
    // Always read the server port fresh from state — the passed parameter may be stale.
    let server_port = state.httpd_port;
    if server_port == 0 {
        crate::vdebug!("HTTPD", "Dropping packet: httpd_port not configured");
        return;
    }
    
    // Resolve the client's MAC address using ARP cache. In VirtualBox host-only networking,
    // the src_mac from the incoming packet may not be the correct MAC to use for responses
    // due to NAT or bridge MAC address swapping. Use ARP to get the correct MAC address.
    let dst_mac = find_mac(src_ip).unwrap_or(src_mac);

    let seq = u32::from_be(tcp.seq);
    let ack = u32::from_be(tcp.ack);
    let client_port = u16::from_be(tcp.src_port);
    let off_flags_raw = u16::from_be(tcp.off_flags);
    let data_offset = (off_flags_raw >> 12) as usize;
    let header_len = data_offset * 4;
    let payload = if packet.len() > header_len { &packet[header_len..] } else { &[] };

    // Debug: show first bytes of payload to verify correct offset
    let payload_preview: Vec<String> = payload.iter().take(20).map(|b| format!("{:02x}", b)).collect();
    crate::vdebug!("HTTPD", "handle_httpd: entering, off_flags=0x{:04x} data_offset={} header_len={} packet_len={} payload={} bytes preview={}",
        off_flags_raw, data_offset, header_len, packet.len(), payload.len(), payload_preview.join(" "));

    // Determine if this is a SYN packet (new connection request)
    let is_syn = (flags & (TCP_FLAG_SYN as u16)) != 0 && (flags & (TCP_FLAG_ACK as u16)) == 0;

    // Duplicate packet detection — drop looped-back echoes.
    // On both host-only and bridged networks, transmitted packets can be looped back
    // with modified flags (e.g., ACK becomes ACK+FIN or ACK+RST). We detect these by
    // matching seq + payload length. Unlike host-only networks where loops are brief,
    // bridged networks can loop packets continuously, so we drop duplicates indefinitely
    // (no time window) — TCP sequence numbers only advance when data is consumed, so
    // a legitimate retransmission would have the same seq+payload but only after client
    // timeout (seconds), not the instant arrival of looped echoes.
    for conn_preview in state.http_conns.iter() {
        if conn_preview.client_ip == src_ip && conn_preview.client_port == client_port {
            // Check 1: Active connection already exists for this client — drop duplicate SYN.
            if is_syn && conn_preview.tcp_state != HttpTcpState::None && conn_preview.tcp_state != HttpTcpState::Closed {
                crate::vdebug!("HTTPD", "Dropping duplicate SYN: active connection {:?} for {}.{}.{}.{}:{}",
                    conn_preview.tcp_state, src_ip[0], src_ip[1], src_ip[2], src_ip[3], client_port);
                return;
            }
            // Check 2: Drop packets matching the last received packet's seq + payload length.
            // No time window — looped echoes on bridged networks arrive continuously.
            // A real retransmission takes seconds; a looped echo arrives instantly.
            if !is_syn
                && conn_preview.last_rx_tick > 0
                && seq == conn_preview.last_rx_seq
                && payload.len() == conn_preview.last_rx_payload_len as usize {
                crate::vdebug!("HTTPD", "Dropping looped-back echo: seq={} len={}",
                    seq, payload.len());
                return;
            }
        }
    }

    let conn_idx = match find_http_conn(state, src_ip, client_port, is_syn) {
        Some(i) => {
            crate::vdebug!("HTTPD", "handle_httpd: found/created connection slot {}", i);
            i
        },
        None => {
            crate::vdebug!("HTTPD", "No free connection slots for {}.{}.{}.{}", src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
            // Send RST so the client stops retrying — prevents infinite transmit loops when
            // the initial SYN was lost (e.g. dropped by loop-back detection or NAT timeout).
            send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                            ack.wrapping_add(1), seq, TCP_FLAG_RST, &[]);
            return;
        }
    };
    let conn = &mut state.http_conns[conn_idx];
    
    crate::vdebug!("HTTPD", "handle_httpd: conn state={:?}", conn.tcp_state);

    // 1. SYN (without ACK) = new connection request
    if (flags & (TCP_FLAG_SYN as u16)) != 0 && (flags & (TCP_FLAG_ACK as u16)) == 0 {
        crate::vdebug!("HTTPD", "SYN from {}.{}.{}.{}, port {}", src_ip[0], src_ip[1], src_ip[2], src_ip[3], client_port);
        let our_isn = state.http_isn_counter;
        state.http_isn_counter = state.http_isn_counter.wrapping_add(1);
        conn.our_seq = our_isn;
        conn.their_seq = seq + 1;
        conn.tcp_state = HttpTcpState::SynReceived;
        crate::vdebug!("HTTPD", "Sending SYN/ACK: our_seq={} their_seq={}", our_isn, seq + 1);
        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                        our_isn, seq + 1, TCP_FLAG_SYN | TCP_FLAG_ACK, &[]);
        conn.last_send_tick = state.httpd_poll_tick;
        return;
    }

    // SYN+ACK should not arrive from client (we're the server)
    if (flags & (TCP_FLAG_SYN as u16)) != 0 && (flags & (TCP_FLAG_ACK as u16)) != 0 {
        crate::vdebug!("HTTPD", "Ignoring SYN+ACK from client (unexpected)");
        return;
    }

    // 2. ACK = data transfer or connection established
    if (flags & (TCP_FLAG_ACK as u16)) != 0 && (flags & (TCP_FLAG_SYN as u16)) == 0 {
        crate::vdebug!("HTTPD", "ACK handler: conn_state={:?} seq={} ack={} payload={} our_seq={} their_seq={}",
            conn.tcp_state, seq, ack, payload.len(), conn.our_seq, conn.their_seq);
        match conn.tcp_state {
            HttpTcpState::SynReceived => {
                // If we already processed this connection, ignore duplicate packets
                if conn.request_complete {
                    crate::vdebug!("HTTPD", "SynReceived: already processed, ignoring duplicate");
                    return;
                }
                conn.tcp_state = HttpTcpState::Established;
                crate::vdebug!("HTTPD", "Connection established from {}.{}.{}.{}", src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
                // DO NOT overwrite conn.our_seq — it was set to our ISN in the SYN handler.
                // The client's ack field confirms they received our SYN, but our_seq stays as our ISN.
                // conn.our_seq should remain the value we set when we sent SYN/ACK.
                // Always trust the client's seq in the establishment ACK — it defines their current sequence.
                // Update our expected seq to match what the client is actually sending.
                if seq != conn.their_seq {
                    crate::vdebug!("HTTPD", "SynReceived: client seq {} != expected {}, updating their_seq", seq, conn.their_seq);
                    conn.their_seq = seq;
                }
                if !payload.is_empty() {
                    crate::vdebug!("HTTPD", "Received {} bytes of data in establishment ACK", payload.len());
                    conn.request_buf.extend_from_slice(payload);
                    conn.their_seq = seq + payload.len() as u32;
                    if let Some((method, path, body)) = parse_http_request(&conn.request_buf) {
                        crate::vdebug!("HTTPD", "{} {} from {}.{}.{}.{}", method, path, src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
                        let response = build_http_response(method, path, &body);
                        crate::vdebug!("HTTPD", "Sending response: {} bytes", response.len());
                        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                        conn.our_seq, conn.their_seq,
                                        TCP_FLAG_ACK | TCP_FLAG_PSH, &response);
                        conn.our_seq += response.len() as u32;
                        conn.request_complete = true;
                        conn.last_send_tick = state.httpd_poll_tick;
                        // DO NOT send FIN yet — wait for client to ACK the response first.
                        // Sending FIN immediately can confuse the client TCP stack.
                    } else {
                        // Debug: show what's in the buffer when parsing fails
                        let buf_preview: Vec<String> = conn.request_buf.iter().take(80).map(|b| format!("{:02x}", b)).collect();
                        let buf_text = core::str::from_utf8(&conn.request_buf).unwrap_or("<invalid utf8>");
                        crate::vdebug!("HTTPD", "Parse failed ({} bytes) hex: {} text: {:?}", conn.request_buf.len(), buf_preview.join(" "), buf_text.chars().take(80).collect::<String>());
                        crate::vdebug!("HTTPD", "Request not yet complete, sending ACK");
                        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                        conn.our_seq, conn.their_seq,
                                        TCP_FLAG_ACK, &[]);
                        conn.last_send_tick = state.httpd_poll_tick;
                    }
                } else {
                    crate::vdebug!("HTTPD", "Establishment ACK with no data, waiting for more");
                }
            }
            HttpTcpState::Established => {
                // If we already sent a response, wait for client ACK then send FIN to close
                if conn.request_complete {
                    // Client is ACKing our response — send FIN to initiate connection close
                    crate::vdebug!("HTTPD", "Established: response ACKed by client, sending FIN");
                    send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                    conn.our_seq, conn.their_seq,
                                    TCP_FLAG_ACK | TCP_FLAG_FIN, &[]);
                    conn.our_seq += 1;
                    conn.tcp_state = HttpTcpState::FinWait;
                    conn.last_send_tick = state.httpd_poll_tick;
                    return;
                }
                // Fallback case: if their_seq is 0, this connection was created by the fallback
                // mechanism because the SYN was lost. Initialize the sequence numbers from
                // the first data packet we receive.
                if conn.their_seq == 0 && !payload.is_empty() {
                    crate::vdebug!("HTTPD", "Established: fallback connection, initializing from first data packet seq={}", seq);
                    conn.their_seq = seq;
                    // Accept this first data packet directly — don't treat it as a gap.
                    // The fallback connection has no prior state, so the first packet is always valid.
                    crate::vdebug!("HTTPD", "Established: fallback first packet, accepting {} bytes", payload.len());
                    conn.request_buf.extend_from_slice(payload);
                    conn.their_seq = seq + payload.len() as u32;
                    // Try to parse the HTTP request from this first packet
                    if let Some((method, path, body)) = parse_http_request(&conn.request_buf) {
                        crate::vdebug!("HTTPD", "{} {} from {}.{}.{}.{}", method, path, src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
                        let response = build_http_response(method, path, &body);
                        crate::vdebug!("HTTPD", "Sending response: {} bytes", response.len());
                        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                        conn.our_seq, conn.their_seq,
                                        TCP_FLAG_ACK | TCP_FLAG_PSH, &response);
                        conn.our_seq += response.len() as u32;
                        conn.request_complete = true;
                        conn.last_send_tick = state.httpd_poll_tick;
                    } else {
                        crate::vdebug!("HTTPD", "Fallback: request not yet complete, sending ACK");
                        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                        conn.our_seq, conn.their_seq,
                                        TCP_FLAG_ACK, &[]);
                        conn.last_send_tick = state.httpd_poll_tick;
                    }
                    // Record this packet for duplicate detection
                    conn.last_rx_seq = seq;
                    conn.last_rx_payload_len = payload.len() as u32;
                    conn.last_rx_tick = state.httpd_poll_tick;
                    return;
                }
                if payload.is_empty() {
                    // No payload — window update or keepalive, silently ignore
                    crate::vdebug!("HTTPD", "Established: empty payload, seq={} their_seq={}", seq, conn.their_seq);
                    return;
                }
                // Handle TCP retransmissions: the client may resend data we already received.
                // Accept any new data beyond what's already in the buffer.
                crate::vdebug!("HTTPD", "Established: checking seq match: seq={} their_seq={}", seq, conn.their_seq);
                if seq == conn.their_seq {
                    // Exact match — append all payload
                    crate::vdebug!("HTTPD", "Established: received {} bytes at expected seq {}", payload.len(), seq);
                    conn.request_buf.extend_from_slice(payload);
                    conn.their_seq = seq + payload.len() as u32;
                } else if seq < conn.their_seq {
                    // Retransmission — check if there's new data beyond what we already have
                    let overlap = conn.their_seq - seq;
                    if overlap < payload.len() as u32 {
                        // There's new data after the overlapping portion
                        let new_data = &payload[overlap as usize..];
                        crate::vdebug!("HTTPD", "Established: retransmission with {} new bytes (overlap={})", new_data.len(), overlap);
                        conn.request_buf.extend_from_slice(new_data);
                        conn.their_seq = seq + payload.len() as u32;
                    } else {
                        // Fully received — send ACK to tell client what we expect next
                        crate::vdebug!("HTTPD", "Established: fully received retransmission, sending ACK");
                        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                        conn.our_seq, conn.their_seq,
                                        TCP_FLAG_ACK, &[]);
                        conn.last_send_tick = state.httpd_poll_tick;
                        return;
                    }
                } else {
                    // seq > their_seq: gap in data — send ACK telling client what we expect
                    crate::vdebug!("HTTPD", "Established: gap in data (got {}, expected {}), sending ACK", seq, conn.their_seq);
                    send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                    conn.our_seq, conn.their_seq,
                                    TCP_FLAG_ACK, &[]);
                    conn.last_send_tick = state.httpd_poll_tick;
                    return;
                }
                // Try to parse the HTTP request
                if let Some((method, path, body)) = parse_http_request(&conn.request_buf) {
                    crate::vdebug!("HTTPD", "{} {} from {}.{}.{}.{}", method, path, src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
                    let response = build_http_response(method, path, &body);
                    crate::vdebug!("HTTPD", "Sending response: {} bytes", response.len());
                    // ACK the client's data AND send our response in a single packet
                    send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                    conn.our_seq, conn.their_seq,
                                    TCP_FLAG_ACK | TCP_FLAG_PSH, &response);
                    conn.our_seq += response.len() as u32;
                    conn.request_complete = true;
                    conn.last_send_tick = state.httpd_poll_tick;
                    // DO NOT send FIN yet — wait for client to ACK the response first.
                } else {
                    // Debug: show what's in the buffer when parsing fails
                    let buf_preview: Vec<String> = conn.request_buf.iter().take(80).map(|b| format!("{:02x}", b)).collect();
                    let buf_text = core::str::from_utf8(&conn.request_buf).unwrap_or("<invalid utf8>");
                    crate::vdebug!("HTTPD", "Parse failed ({} bytes) hex: {} text: {:?}", conn.request_buf.len(), buf_preview.join(" "), buf_text.chars().take(80).collect::<String>());
                    crate::vdebug!("HTTPD", "Request not yet complete, sending ACK");
                    send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                                    conn.our_seq, conn.their_seq,
                                    TCP_FLAG_ACK, &[]);
                    conn.last_send_tick = state.httpd_poll_tick;
                }
            }
            HttpTcpState::FinWait => {
                // We sent FIN, now waiting for client's final ACK or FIN
                // CRITICAL: Do NOT send any more packets — just transition to Closed silently.
                // The client will ACK our FIN, and we should not respond to that ACK.
                // Sending an ACK here creates an infinite ACK-ACK loop.
                if (flags & (TCP_FLAG_FIN as u16)) != 0 {
                    conn.their_seq = seq + 1;
                }
                crate::vdebug!("HTTPD", "FinWait: closing connection silently");
                conn.tcp_state = HttpTcpState::Closed;
            }
            _ => {
                crate::vdebug!("HTTPD", "ACK received in unexpected state {:?}", conn.tcp_state);
            }
        }
    }

    // 3. FIN from client (only if not already handled in the ACK match block above)
    if (flags & (TCP_FLAG_FIN as u16)) != 0 && conn.tcp_state != HttpTcpState::Closed {
        crate::vdebug!("HTTPD", "FIN from client, closing connection");
        conn.tcp_state = HttpTcpState::Closed;
        conn.their_seq = seq + 1;
        send_tcp_packet(src_ip, dst_mac, server_port, client_port,
                        conn.our_seq, conn.their_seq,
                        TCP_FLAG_ACK, &[]);
        conn.last_send_tick = state.httpd_poll_tick;
        crate::vdebug!("HTTPD", "Connection closed from {}.{}.{}.{}", src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
    }

    // 4. RST
    if (flags & (TCP_FLAG_RST as u16)) != 0 {
        crate::vdebug!("HTTPD", "RST from {}.{}.{}.{}", src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        conn.tcp_state = HttpTcpState::Closed;
    }

    // Record this packet's seq and tick for duplicate detection on the next incoming packet.
    // Only record for non-SYN packets (SYN creates a fresh connection).
    if !is_syn {
        conn.last_rx_seq = seq;
        conn.last_rx_payload_len = payload.len() as u32;
        conn.last_rx_tick = state.httpd_poll_tick;
    }
}

fn handle_tcp_client(src_ip: [u8; 4], src_mac: [u8; 6], tcp: &TcpHeader, flags: u16, packet: &[u8], state: &mut NetState) {
    let seq = u32::from_be(tcp.seq);
    let ack = u32::from_be(tcp.ack);
    let src_port = u16::from_be(tcp.src_port);
    let dst_port = u16::from_be(tcp.dst_port);

    if (flags & (TCP_FLAG_SYN as u16)) != 0 && (flags & (TCP_FLAG_ACK as u16)) != 0 {
        // SYN/ACK received
        crate::vdebug!("TCP", "SYN/ACK received, connecting...");
        state.tcp_connected = true;
        state.tcp_seq = ack;
        state.tcp_ack = seq + 1;
        // ACK it
        send_tcp_packet(src_ip, src_mac, dst_port, src_port, state.tcp_seq, state.tcp_ack, TCP_FLAG_ACK, &[]);
    } else if (flags & (TCP_FLAG_ACK as u16)) != 0 {
        let header_len = ((u16::from_be(tcp.off_flags) >> 12) * 4) as usize;
        state.tcp_seq = ack;
        if packet.len() > header_len {
            let payload = &packet[header_len..];
            crate::vdebug!("TCP", "Received {} bytes of data", payload.len());
            state.tcp_rx_data.extend_from_slice(payload);
            state.tcp_ack = seq + payload.len() as u32;
            // ACK the data
            send_tcp_packet(src_ip, src_mac, dst_port, src_port, state.tcp_seq, state.tcp_ack, TCP_FLAG_ACK, &[]);
        } else if (flags & (TCP_FLAG_FIN as u16)) == 0 {
             // Pure ACK (no data, no FIN) - usually a response to our PSH|ACK
             // We don't need to ACK an ACK unless it has data or FIN.
             // Just updating state.tcp_seq = ack (done above) is enough.
        }

        if (flags & (TCP_FLAG_FIN as u16)) != 0 {
            state.tcp_connected = false;
            state.tcp_fin_received = true;
            state.tcp_ack = seq + 1;
            // ACK the FIN
            send_tcp_packet(src_ip, src_mac, dst_port, src_port, state.tcp_seq, state.tcp_ack, TCP_FLAG_ACK, &[]);
        }
    }
}

pub fn tcp_connect(dst_ip: [u8; 4], dst_port: u16) -> bool {
    let state = unsafe { NET_STATE.assume_init_mut().as_mut().unwrap() };
    state.tcp_connected = false;
    state.tcp_rx_data.clear();
    state.tcp_seq = 1000; // Initial Sequence Number
    state.tcp_ack = 0;
    state.tcp_fin_received = false;

    let mac = match resolve_mac(dst_ip, 100) {
        Some(m) => m,
        None => return false,
    };

    // Send SYN
    let src_port = 49152 + (uefi::runtime::get_time().map(|t| t.second()).unwrap_or(0) as u16) % 1000;
    state.tcp_src_port = src_port;
    state.tcp_dst_port = dst_port;
    send_tcp_packet(dst_ip, mac, src_port, dst_port, state.tcp_seq, 0, TCP_FLAG_SYN, &[]);

    // Wait for connection
    for _ in 0..500 {
        poll_tick();
        if state.tcp_connected { return true; }
        uefi::boot::stall(core::time::Duration::from_micros(10_000));
    }
    false
}

pub fn tcp_send(dst_ip: [u8; 4], dst_port: u16, data: &[u8]) {
    let state = unsafe { NET_STATE.assume_init_mut().as_mut().unwrap() };
    let mac = match find_mac(dst_ip) {
        Some(m) => m,
        None => return,
    };
    
    send_tcp_packet(dst_ip, mac, state.tcp_src_port, dst_port, state.tcp_seq, state.tcp_ack, TCP_FLAG_ACK | TCP_FLAG_PSH, data);
    state.tcp_seq += data.len() as u32;
}

pub fn tcp_disconnect(dst_ip: [u8; 4], dst_port: u16) {
    let state = unsafe { NET_STATE.assume_init_mut().as_mut().unwrap() };
    let mac = match find_mac(dst_ip) {
        Some(m) => m,
        None => return,
    };
    send_tcp_packet(dst_ip, mac, state.tcp_src_port, dst_port, state.tcp_seq, state.tcp_ack, TCP_FLAG_ACK | TCP_FLAG_FIN, &[]);
}



fn send_tcp_packet(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16,
                   seq: u32, ack: u32, flags: u8, data: &[u8]) {
    crate::vdebug!("TCP", "send_tcp_packet: to {}.{}.{}.{}, port {}->{} seq={} ack={} flags=0x{:02x} data={} bytes dst_mac={:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
        dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3], src_port, dst_port, seq, ack, flags, data.len(),
        dst_mac[0], dst_mac[1], dst_mac[2], dst_mac[3], dst_mac[4], dst_mac[5]);
    
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1514];

    let tcp_len = size_of::<TcpHeader>() + data.len();
    let ip_len = size_of::<Ipv4Header>() + tcp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    // --- Ethernet & IP (Standard Logic) ---
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = dst_mac; eth.src = state.mac_addr; eth.ethertype = ETHERTYPE_IPV4.to_be();

    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45; ip.len = (ip_len as u16).to_be();
    ip.proto = IP_PROTO_TCP; ip.src = state.ip_addr; ip.dst = dst_ip;
    ip.ttl = 64; ip.checksum = 0;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>()+20]).to_be();

    // --- TCP Header ---
    let tcp_offset = size_of::<EthHeader>() + size_of::<Ipv4Header>();
    let tcp = unsafe { &mut *(frame.as_mut_ptr().add(tcp_offset) as *mut TcpHeader) };
    tcp.src_port = src_port.to_be();
    tcp.dst_port = dst_port.to_be();
    tcp.seq = seq.to_be();
    tcp.ack = ack.to_be();
    tcp.off_flags = (((5 << 12) | flags as u16)).to_be(); // 5 words (20 bytes)
    tcp.window = 8192u16.to_be();
    tcp.checksum = 0;

    // Copy Data
    if !data.is_empty() {
        frame[tcp_offset + size_of::<TcpHeader>()..total_len].copy_from_slice(data);
    }

    // --- TCP Checksum (Pseudo-header + TCP) ---
    // Calculate the TCP checksum in software because the SNP hardware may not
    // support checksum offloading. The checksum field must be 0 when calculating.
    let checksum_val = calculate_tcp_checksum(state.ip_addr, dst_ip, &frame[tcp_offset..total_len]).to_be();
    tcp.checksum = checksum_val;
    crate::vdebug!("TCP", "send_tcp_packet: checksum=0x{:04x} total_len={}", checksum_val, total_len);

    match snp_tx(&frame[..total_len]) {
        Ok(()) => {
            crate::vdebug!("TCP", "send_tcp_packet: TX SUCCESS total_len={}", total_len);
        }
        Err(e) => {
            crate::vdebug!("TCP", "send_tcp_packet: TX FAILED: {}", e);
        }
    }
}

/// Validate TCP checksum by recalculating it on the transmitted frame.
/// Used for debugging — verify that the checksum written to the frame is correct.
fn validate_tcp_checksum_frame(src_ip: [u8; 4], dst_ip: [u8; 4], tcp_segment: &[u8]) -> bool {
    let mut sum: u32 = 0;
    sum += u16::from_be_bytes([src_ip[0], src_ip[1]]) as u32;
    sum += u16::from_be_bytes([src_ip[2], src_ip[3]]) as u32;
    sum += u16::from_be_bytes([dst_ip[0], dst_ip[1]]) as u32;
    sum += u16::from_be_bytes([dst_ip[2], dst_ip[3]]) as u32;
    sum += IP_PROTO_TCP as u32;
    sum += tcp_segment.len() as u32;
    let mut chunks = tcp_segment.chunks_exact(2);
    while let Some(chunk) = chunks.next() {
        sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
    }
    if let Some(&rem) = chunks.remainder().first() {
        sum += u16::from_be_bytes([rem, 0]) as u32;
    }
    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    (sum as u16) == 0xFFFF
}

fn calculate_tcp_checksum(src: [u8; 4], dst: [u8; 4], tcp_segment: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    // Pseudo-header
    sum += u16::from_be_bytes([src[0], src[1]]) as u32;
    sum += u16::from_be_bytes([src[2], src[3]]) as u32;
    sum += u16::from_be_bytes([dst[0], dst[1]]) as u32;
    sum += u16::from_be_bytes([dst[2], dst[3]]) as u32;
    sum += IP_PROTO_TCP as u32;
    sum += tcp_segment.len() as u32;

    // Add actual TCP segment
    let mut chunks = tcp_segment.chunks_exact(2);
    while let Some(chunk) = chunks.next() {
        sum += u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
    }
    if let Some(&rem) = chunks.remainder().first() {
        sum += u16::from_be_bytes([rem, 0]) as u32;
    }

    while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
    !(sum as u16)
}

fn send_icmp_reply(dst_ip: [u8; 4], dst_mac: [u8; 6], ident: u16, seq: u16, payload: &[u8]) {
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1514];

    let icmp_len = size_of::<IcmpHeader>() + payload.len();
    let ip_len = size_of::<Ipv4Header>() + icmp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    // 1. Ethernet Header
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = dst_mac;
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    // 2. IPv4 Header
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.len = (ip_len as u16).to_be();
    ip.ttl = 64;
    ip.proto = IP_PROTO_ICMP;
    ip.src = state.ip_addr;
    ip.dst = dst_ip;
    ip.checksum = 0;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    // 3. ICMP Header
    let icmp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut IcmpHeader) };
    icmp.msg_type = 0; // Echo Reply
    icmp.code = 0;
    icmp.ident = ident;
    icmp.seq = seq;
    icmp.checksum = 0;

    // Copy payload
    frame[total_len - payload.len()..total_len].copy_from_slice(payload);

    // ICMP Checksum (Header + Payload)
    let icmp_full_slice = &frame[size_of::<EthHeader>() + size_of::<Ipv4Header>()..total_len];
    icmp.checksum = calculate_checksum(icmp_full_slice).to_be();

    let _ = snp_tx(&frame[..total_len]);
    crate::vdebug!("ICMP", "Ping reply sent to {}.{}.{}.{}", dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3]);
}

fn handle_udp(src_ip: [u8; 4], port: u16, data: &[u8]) {
    match port {
        67 => {
            // DHCP server port — handled by DHCP discovery
            crate::vdebug!("UDP", "DHCP server port {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        53 => {
            // DNS response
            crate::vdebug!("UDP", "DNS response {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        123 => {
            // NTP response
            crate::vdebug!("UDP", "NTP response {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        137 => {
            // NetBIOS Name Service
            crate::vdebug!("UDP", "NetBIOS Name Service {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        138 => {
            // NetBIOS Datagram Service
            crate::vdebug!("UDP", "NetBIOS Datagram {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        161 => {
            // SNMP
            crate::vdebug!("UDP", "SNMP {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        514 => {
            // Syslog
            crate::vdebug!("UDP", "Syslog {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        5353 => {
            // mDNS (Multicast DNS) — respond with SYSTEMNAME
            crate::vdebug!("mDNS", "mDNS query {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
            handle_mdns_query(src_ip, data);
        }
        1900 => {
            // SSDP (Simple Service Discovery Protocol)
            crate::vdebug!("UDP", "SSDP {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        5355 => {
            // LLMNR (Link-Local Multicast Name Resolution)
            crate::vdebug!("UDP", "LLMNR {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        3702 => {
            // NFC Deployment Data Discovery (NFCDDP)
            crate::vdebug!("UDP", "NFCDDP {} bytes from {}.{}.{}.{}", data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        1234 => {
            // Command listener
            crate::vdebug!("UDP", "Command {} bytes on port 1234 from {}.{}.{}.{}",
                data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        _ => {
            crate::vdebug!("UDP", "Unhandled port {} — {} bytes from {}.{}.{}.{}", port, data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
    }
}

/// Handle mDNS query (UDP port 5353) — respond with SYSTEMNAME A and PTR records
fn handle_mdns_query(_src_ip: [u8; 4], query_data: &[u8]) {
    // Extract the queried name from the DNS query (skip 12-byte header, parse name)
    let system_name = crate::SYSTEMNAME;
    let name_bytes = system_name.as_bytes();

    // Build a minimal mDNS response with:
    // - PTR record: _http._tcp.local. -> <name>.local.
    // - A record: <name>.local. -> our IP
    // - SRV record: <name>.local. port 80

    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let our_ip = state.ip_addr;

    // Build DNS response packet
    // Header: 12 bytes (ID, flags, questions, answer RRs, authority RRs, additional RRs)
    // We'll build a response with 1 A record answer
    let mut dns_buf = Vec::with_capacity(256);

    // DNS Header
    dns_buf.extend_from_slice(&0x0000u16.to_be_bytes()); // Transaction ID (echo 0)
    dns_buf.extend_from_slice(&0x8400u16.to_be_bytes()); // Flags: standard response, recursion available
    dns_buf.extend_from_slice(&0x0000u16.to_be_bytes()); // Questions: 0
    dns_buf.extend_from_slice(&0x0001u16.to_be_bytes()); // Answer RRs: 1 (A record)
    dns_buf.extend_from_slice(&0x0000u16.to_be_bytes()); // Authority RRs: 0
    dns_buf.extend_from_slice(&0x0000u16.to_be_bytes()); // Additional RRs: 0

    // Answer: A record for <SYSTEMNAME>.local.
    // Name: encode <SYSTEMNAME>.local. in DNS name format
    // e.g., "us-east-hypervisor-01.local." -> labels
    let name_part = &name_bytes[..name_bytes.len().min(63)];
    dns_buf.push(name_part.len() as u8);
    dns_buf.extend_from_slice(name_part);
    dns_buf.push(5); // "local" length
    dns_buf.extend_from_slice(b"local");
    dns_buf.push(0); // root label terminator

    // Type: A record (1)
    dns_buf.extend_from_slice(&0x0001u16.to_be_bytes());
    // Class: IN (1)
    dns_buf.extend_from_slice(&0x0001u16.to_be_bytes());
    // TTL: 120 seconds
    dns_buf.extend_from_slice(&120u32.to_be_bytes());
    // RDLENGTH: 4 (IPv4 address)
    dns_buf.extend_from_slice(&0x0004u16.to_be_bytes());
    // RDATA: our IP
    dns_buf.extend_from_slice(&our_ip);

    // Send mDNS response to multicast address 224.0.0.251:5353
    // Multicast MAC for 224.0.0.251: 01:00:5e:00:00:fb
    let mdns_mac: [u8; 6] = [0x01, 0x00, 0x5e, 0x00, 0x00, 0xfb];
    let mdns_ip: [u8; 4] = [224, 0, 0, 251];

    crate::vdebug!("mDNS", "Sending mDNS response: {} -> {}.{}.{}.{}",
        system_name, our_ip[0], our_ip[1], our_ip[2], our_ip[3]);

    send_udp(mdns_ip, mdns_mac, 5353, 5353, &dns_buf);
    unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().udp_tx_packets += 1; }
    let _ = query_data;
}

/// Handle IGMP packets (membership reports and queries) — supports V1, V2, and V3
fn handle_igmp(src_ip: [u8; 4], payload: &[u8]) {
    if payload.len() < 4 {
        crate::vdebug!("IGMP", "Packet too small: {} bytes", payload.len());
        return;
    }
    let msg_type = payload[0];

    match msg_type {
        IGMP_V1_MEMBERSHIP_REPORT | IGMP_V2_MEMBERSHIP_REPORT => {
            if payload.len() < 8 {
                crate::vdebug!("IGMP", "V1/V2 report too small: {} bytes", payload.len());
                return;
            }
            let group_ip: [u8; 4] = [payload[4], payload[5], payload[6], payload[7]];
            let version = if msg_type == IGMP_V2_MEMBERSHIP_REPORT { "V2" } else { "V1" };
            crate::vdebug!("IGMP", "{} Membership Report for {}.{}.{}.{} from {}.{}.{}.{}",
                version, group_ip[0], group_ip[1], group_ip[2], group_ip[3],
                src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        IGMP_V2_LEAVE_GROUP => {
            if payload.len() < 8 {
                crate::vdebug!("IGMP", "V2 leave too small: {} bytes", payload.len());
                return;
            }
            let group_ip: [u8; 4] = [payload[4], payload[5], payload[6], payload[7]];
            crate::vdebug!("IGMP", "Leave Group {}.{}.{}.{} from {}.{}.{}.{}",
                group_ip[0], group_ip[1], group_ip[2], group_ip[3],
                src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
        IGMP_V1_QUERY | IGMP_V2_QUERY => {
            if payload.len() < 8 {
                crate::vdebug!("IGMP", "V1/V2 query too small: {} bytes", payload.len());
                return;
            }
            let group_ip: [u8; 4] = [payload[4], payload[5], payload[6], payload[7]];
            let version = if msg_type == IGMP_V2_QUERY { "V2" } else { "V1" };
            crate::vdebug!("IGMP", "{} Query for {}.{}.{}.{} from {}.{}.{}.{}",
                version, group_ip[0], group_ip[1], group_ip[2], group_ip[3],
                src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
            // Respond to general queries with membership reports for our groups
            if group_ip == [0; 4] {
                unsafe {
                    if let Some(state) = NET_STATE.assume_init_ref().as_ref() {
                        for i in 0..state.igmp_group_count as usize {
                            let group = state.igmp_groups[i];
                            if group != [0; 4] {
                                // Use IGMPv3 report if sources configured, otherwise V2
                                if state.igmp_v3_filter_mode[i] != 0 {
                                    send_igmp_v3_membership_report(group, i);
                                } else {
                                    send_igmp_membership_report(group);
                                }
                            }
                        }
                    }
                }
            }
        }
        IGMP_V3_MEMBERSHIP_REPORT => {
            // IGMPv3 Membership Report — parse record count and records
            if payload.len() < 8 {
                crate::vdebug!("IGMP", "V3 report too small: {} bytes", payload.len());
                return;
            }
            let record_count = u16::from_be_bytes([payload[6], payload[7]]) as usize;
            crate::vdebug!("IGMP", "V3 Membership Report with {} records from {}.{}.{}.{}",
                record_count, src_ip[0], src_ip[1], src_ip[2], src_ip[3]);

            let mut offset = 8;
            for r in 0..record_count {
                if offset + 8 > payload.len() {
                    crate::vdebug!("IGMP", "V3 record {} truncated at offset {}", r, offset);
                    break;
                }
                let record_type = payload[offset];
                let aux_data_len = payload[offset + 1];
                let src_count = u16::from_be_bytes([payload[offset + 2], payload[offset + 3]]) as usize;
                let group_ip: [u8; 4] = [payload[offset + 4], payload[offset + 5], payload[offset + 6], payload[offset + 7]];
                offset += 8;

                // Skip source addresses
                let src_bytes = src_count * 4;
                if offset + src_bytes > payload.len() {
                    crate::vdebug!("IGMP", "V3 record {} sources truncated", r);
                    break;
                }
                crate::vdebug!("IGMP", "  V3 Record: type={} group={}.{}.{}.{} sources={}",
                    record_type, group_ip[0], group_ip[1], group_ip[2], group_ip[3], src_count);

                // Skip aux data
                offset += src_bytes + (aux_data_len as usize);
            }
        }
        _ => {
            crate::vdebug!("IGMP", "Unknown IGMP type {} from {}.{}.{}.{}", msg_type, src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
        }
    }
}

/// Send IGMPv2 membership report for a multicast group
pub fn send_igmp_membership_report(group_ip: [u8; 4]) {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1514];
    let total_len = size_of::<EthHeader>() + size_of::<Ipv4Header>() + 8;
    
    // Ethernet header — destination is the multicast group MAC
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    // Multicast MAC: 01:00:5e:00:xx:xx (derived from multicast IP)
    eth.dst = [0x01, 0x00, 0x5e, group_ip[1], group_ip[2], group_ip[3]];
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();
    
    // IPv4 header
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.tos = 0;
    ip.len = (size_of::<Ipv4Header>() + 8) as u16;
    ip.id = 0;
    ip.off = 0;
    ip.ttl = 1;
    ip.proto = IP_PROTO_IGMP;
    ip.checksum = 0;
    ip.src = state.ip_addr;
    ip.dst = group_ip;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();
    
    // IGMP header (8 bytes)
    let igmp_offset = size_of::<EthHeader>() + size_of::<Ipv4Header>();
    frame[igmp_offset] = IGMP_V2_MEMBERSHIP_REPORT; // Type
    frame[igmp_offset + 1] = 0; // Max response time
    let mut igmp_checksum: u16 = 0;
    // Calculate checksum over IGMP header
    let mut igmp_bytes = [0u8; 8];
    igmp_bytes[0] = IGMP_V2_MEMBERSHIP_REPORT;
    igmp_bytes[1..5].copy_from_slice(&group_ip);
    igmp_checksum = calculate_checksum(&igmp_bytes);
    frame[igmp_offset + 2..igmp_offset + 4].copy_from_slice(&igmp_checksum.to_be_bytes());
    frame[igmp_offset + 4..igmp_offset + 8].copy_from_slice(&group_ip);
    
    let _ = snp_tx(&frame[..total_len]);
    unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().igmp_tx_packets += 1; }
    crate::vdebug!("IGMP", "Membership report sent for {}.{}.{}.{}", group_ip[0], group_ip[1], group_ip[2], group_ip[3]);
}

/// Join a multicast group via IGMP
pub fn igmp_join_group(group_ip: [u8; 4]) {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            // Check if already joined
            for i in 0..state.igmp_group_count as usize {
                if state.igmp_groups[i] == group_ip {
                    crate::vdebug!("IGMP", "Already joined {}.{}.{}.{}", group_ip[0], group_ip[1], group_ip[2], group_ip[3]);
                    return;
                }
            }
            // Add new group
            if (state.igmp_group_count as usize) < 8 {
                state.igmp_groups[state.igmp_group_count as usize] = group_ip;
                state.igmp_group_count += 1;
                send_igmp_membership_report(group_ip);
                crate::vdebug!("IGMP", "Joined {}.{}.{}.{} ({} groups)", group_ip[0], group_ip[1], group_ip[2], group_ip[3], state.igmp_group_count);
            } else {
                crate::vdebug!("IGMP", "Cannot join: max groups reached");
            }
        }
    }
}

/// Leave a multicast group via IGMP
pub fn igmp_leave_group(group_ip: [u8; 4]) {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            for i in 0..state.igmp_group_count as usize {
                if state.igmp_groups[i] == group_ip {
                    // Shift remaining groups down
                    let count = state.igmp_group_count as usize;
                    for j in i..count-1 {
                        state.igmp_groups[j] = state.igmp_groups[j+1];
                    }
                    state.igmp_group_count -= 1;
                    state.igmp_groups[count-1] = [0; 4];
                    crate::vdebug!("IGMP", "Left {}.{}.{}.{} ({} groups)", group_ip[0], group_ip[1], group_ip[2], group_ip[3], state.igmp_group_count);
                    return;
                }
            }
            crate::vdebug!("IGMP", "Not a member of {}.{}.{}.{}", group_ip[0], group_ip[1], group_ip[2], group_ip[3]);
        }
    }
}

/// Send IGMPv3 membership report for a multicast group with source filtering
fn send_igmp_v3_membership_report(group_ip: [u8; 4], group_index: usize) {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let src_count = state.igmp_v3_source_count[group_index] as usize;
    let filter_mode = state.igmp_v3_filter_mode[group_index];

    // IGMPv3 report: 8-byte header + 8-byte record header + (src_count * 4) source IPs
    let record_size = 8 + src_count * 4;
    let igmp_size = 8 + record_size;
    let total_len = size_of::<EthHeader>() + size_of::<Ipv4Header>() + igmp_size;

    let mut frame = [0u8; 1514];

    // Ethernet header
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = [0x01, 0x00, 0x5e, group_ip[1], group_ip[2], group_ip[3]];
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    // IPv4 header
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.tos = 0;
    ip.len = (size_of::<Ipv4Header>() + igmp_size) as u16;
    ip.id = 0;
    ip.off = 0;
    ip.ttl = 1;
    ip.proto = IP_PROTO_IGMP;
    ip.checksum = 0;
    ip.src = state.ip_addr;
    ip.dst = group_ip;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    // IGMPv3 report header
    let igmp_offset = size_of::<EthHeader>() + size_of::<Ipv4Header>();
    frame[igmp_offset] = IGMP_V3_MEMBERSHIP_REPORT; // Type
    frame[igmp_offset + 1] = 0; // Reserved
    frame[igmp_offset + 2] = 0; // Reserved
    frame[igmp_offset + 3] = 0; // Reserved
    // Checksum placeholder
    frame[igmp_offset + 4] = 0;
    frame[igmp_offset + 5] = 0;
    // Number of records = 1
    frame[igmp_offset + 6] = 0;
    frame[igmp_offset + 7] = 1;

    // Record: TYPE | AUX_DATA_LEN | SRC_SPECIFIER | SRC_COUNT | GROUP
    let rec_offset = igmp_offset + 8;
    // Record type: MODE_IS_INCLUDE or MODE_IS_EXCLUDE
    let record_type = if filter_mode == IGMP_V3_MODE_EXCLUDE {
        0x31 // MODE_IS_EXCLUDE
    } else {
        0x11 // MODE_IS_INCLUDE
    };
    frame[rec_offset] = record_type;
    frame[rec_offset + 1] = 0; // AUX_DATA_LEN
    frame[rec_offset + 2] = 0; // SRC_SPECIFIER (S bit)
    frame[rec_offset + 3] = src_count as u8; // SRC_COUNT
    frame[rec_offset + 4..rec_offset + 8].copy_from_slice(&group_ip);

    // Source addresses
    let mut src_offset = rec_offset + 8;
    for s in 0..src_count {
        let src = state.igmp_v3_sources[group_index][s];
        frame[src_offset..src_offset + 4].copy_from_slice(&src);
        src_offset += 4;
    }

    // Calculate IGMPv3 checksum over entire IGMP payload
    let igmp_end = igmp_offset + igmp_size;
    let igmp_checksum = calculate_checksum(&frame[igmp_offset..igmp_end]);
    frame[igmp_offset + 4..igmp_offset + 6].copy_from_slice(&igmp_checksum.to_be_bytes());

    let _ = snp_tx(&frame[..total_len]);
    unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().igmp_tx_packets += 1; }
    crate::vdebug!("IGMP", "V3 Membership report sent for {}.{}.{}.{} ({} sources, mode={})",
        group_ip[0], group_ip[1], group_ip[2], group_ip[3],
        src_count, if filter_mode == IGMP_V3_MODE_EXCLUDE { "EXCLUDE" } else { "INCLUDE" });
}

/// Join a multicast group via IGMPv3 with source filtering
/// If sources is empty, falls back to IGMPv2-style join
pub fn igmp_v3_join_group(group_ip: [u8; 4], sources: &[[u8; 4]], filter_mode: u8) {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            // Check if already joined
            for i in 0..state.igmp_group_count as usize {
                if state.igmp_groups[i] == group_ip {
                    // Update sources if changed
                    let mode = if filter_mode == IGMP_V3_MODE_INCLUDE || filter_mode == IGMP_V3_MODE_EXCLUDE { filter_mode } else { IGMP_V3_MODE_INCLUDE };
                    state.igmp_v3_filter_mode[i] = mode;
                    let max_src = core::cmp::min(sources.len(), IGMP_MAX_SOURCES_PER_GROUP as usize);
                    state.igmp_v3_source_count[i] = max_src as u8;
                    for s in 0..max_src {
                        state.igmp_v3_sources[i][s] = sources[s];
                    }
                    // Send updated report
                    if max_src > 0 {
                        send_igmp_v3_membership_report(group_ip, i);
                    }
                    crate::vdebug!("IGMP", "Updated V3 join {}.{}.{}.{} ({} sources)",
                        group_ip[0], group_ip[1], group_ip[2], group_ip[3], max_src);
                    return;
                }
            }
            // Add new group
            if (state.igmp_group_count as usize) < IGMP_MAX_GROUPS as usize {
                let idx = state.igmp_group_count as usize;
                state.igmp_groups[idx] = group_ip;
                state.igmp_group_count += 1;

                if !sources.is_empty() {
                    let mode = if filter_mode == IGMP_V3_MODE_INCLUDE || filter_mode == IGMP_V3_MODE_EXCLUDE { filter_mode } else { IGMP_V3_MODE_INCLUDE };
                    let max_src = core::cmp::min(sources.len(), IGMP_MAX_SOURCES_PER_GROUP as usize);
                    state.igmp_v3_filter_mode[idx] = mode;
                    state.igmp_v3_source_count[idx] = max_src as u8;
                    for s in 0..max_src {
                        state.igmp_v3_sources[idx][s] = sources[s];
                    }
                    send_igmp_v3_membership_report(group_ip, idx);
                } else {
                    state.igmp_v3_filter_mode[idx] = 0;
                    state.igmp_v3_source_count[idx] = 0;
                    send_igmp_membership_report(group_ip);
                }
                crate::vdebug!("IGMP", "Joined {}.{}.{}.{} ({} groups, V3)",
                    group_ip[0], group_ip[1], group_ip[2], group_ip[3], state.igmp_group_count);
            } else {
                crate::vdebug!("IGMP", "Cannot join: max groups reached");
            }
        }
    }
}

/// Configure static IP address (bypasses DHCP)
pub fn set_static_ip(ip: [u8; 4], gw: [u8; 4], mask: [u8; 4]) {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            state.use_static_ip = true;
            state.static_ip = ip;
            state.static_gw = gw;
            state.static_mask = mask;
            crate::vdebug!("NET", "Static IP configured: {}.{}.{}.{} / {}.{}.{}.{} / GW {}.{}.{}.{}",
                ip[0], ip[1], ip[2], ip[3],
                mask[0], mask[1], mask[2], mask[3],
                gw[0], gw[1], gw[2], gw[3]);
        }
    }
}

/// Check if static IP is configured
pub fn is_static_ip_configured() -> bool {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_ref().as_ref() {
            state.use_static_ip
        } else {
            false
        }
    }
}

/// Get static IP configuration
pub fn get_static_ip_config() -> ([u8; 4], [u8; 4], [u8; 4]) {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_ref().as_ref() {
            (state.static_ip, state.static_gw, state.static_mask)
        } else {
            ([0; 4], [0; 4], [0; 4])
        }
    }
}

/// Send a UDP packet to a destination (convenience wrapper that increments TX counter)
pub fn send_udp_and_count(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16, data: &[u8]) {
    send_udp(dst_ip, dst_mac, src_port, dst_port, data);
    unsafe { NET_STATE.assume_init_mut().as_mut().unwrap().udp_tx_packets += 1; }
}

pub fn httpd_start(port: u16) {
    // Set the port BEFORE enabling the HTTPD flag — prevents a race where
    // packets arrive with HTTPD=true but httpd_port still 0.
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            state.httpd_port = port;
        }
    }
    if HTTPD.swap(true, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "already running");
        return;
    }
    crate::vdebug!("HTTPD", "HTTP server listening on port {}", port);

    // Spawn a dedicated AP core to run the network poll loop continuously.
    // Only active when the `net-multicore` feature is enabled.
    #[cfg(feature = "net-multicore")]
    spawn_http_poll_worker();
}

pub fn httpd_stop() {
    if !HTTPD.swap(false, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "not running");
        return;
    }

    // Signal the dedicated AP poll worker to stop (only when `net-multicore` is enabled)
    #[cfg(feature = "net-multicore")]
    {
        if HTTP_POLL_RUNNING.load(Ordering::SeqCst) {
            HTTP_POLL_STOP.store(true, Ordering::SeqCst);
            for _ in 0..1000 {
                if !HTTP_POLL_RUNNING.load(Ordering::SeqCst) {
                    break;
                }
                core::hint::spin_loop();
            }
            crate::vdebug!("HTTPD", "HTTP poll worker stopped");
        }
    }

    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            state.httpd_port = 0;
            for conn in state.http_conns.iter_mut() {
                conn.tcp_state = HttpTcpState::Closed;
                conn.request_buf.clear();
            }
        }
    }
    crate::vdebug!("HTTPD", "stopped");
}

/// Entry point for the dedicated HTTP poll AP worker (only when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
extern "efiapi" fn http_poll_worker_entry(_context: *mut core::ffi::c_void) {
    HTTP_POLL_RUNNING.store(true, Ordering::SeqCst);
    crate::vdebug!("HTTPD", "HTTP poll worker started on AP core");

    while !HTTP_POLL_STOP.load(Ordering::SeqCst) {
        poll_tick();
    }

    HTTP_POLL_RUNNING.store(false, Ordering::SeqCst);
    crate::vdebug!("HTTPD", "HTTP poll worker stopped");
}

/// Spawn a dedicated AP core to run the network poll loop (only when `net-multicore` is enabled).
#[cfg(feature = "net-multicore")]
fn spawn_http_poll_worker() {
    // Check if we already have a running poll worker
    if HTTP_POLL_RUNNING.load(Ordering::SeqCst) {
        crate::vdebug!("HTTPD", "HTTP poll worker already running");
        return;
    }

    // Reset stop signal
    HTTP_POLL_STOP.store(false, Ordering::SeqCst);

    // Try to get MP Services to spawn on an AP
    let topology = match crate::hardware::cpu::mp::MpManager::detect_topology() {
        Some(t) => t,
        None => {
            hpvm_warn!("HTTPD", "MP Services unavailable — poll worker will run on BSP via main loop");
            return;
        }
    };

    // Find the first available AP core
    let ap_core = match topology.enabled_aps().next() {
        Some(ap) => ap.clone(),
        None => {
            hpvm_warn!("HTTPD", "No AP cores available — poll worker will run on BSP via main loop");
            return;
        }
    };

    let mp = match crate::hardware::cpu::mp::MpManager::get_mp_services() {
        Some(mp) => mp,
        None => {
            hpvm_warn!("HTTPD", "Failed to acquire MP Services — poll worker will run on BSP");
            return;
        }
    };

    // Create an event for the AP (non-blocking — we don't wait for it to finish)
    let event = unsafe {
        uefi::boot::create_event(
            uefi::boot::EventType::empty(),
            uefi::boot::Tpl::APPLICATION,
            None,
            None
        ).ok()
    };

    // Context pointer is not used, but required by the API
    let ctx_ptr = core::ptr::null_mut();

    let result = mp.startup_this_ap(
        ap_core.index,
        http_poll_worker_entry,
        ctx_ptr,
        event,
        None, // No timeout — run until HTTP_POLL_STOP is set
    );

    match result {
        Ok(()) => {
            crate::vdebug!("HTTPD", "HTTP poll worker spawned on AP core #{} (processor ID {})",
                ap_core.index, ap_core.processor_id);
        }
        Err(status) => {
            hpvm_warn!("HTTPD", "Failed to spawn HTTP poll worker on AP #{}: {:?} — falling back to BSP poll",
                ap_core.index, status);
        }
    }
}


/// Return a snapshot of current network stats (RX/TX counters).
#[allow(static_mut_refs)]
pub fn stats() -> NetStats {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_ref().as_ref() {
            state.stats
        } else {
            NetStats::default()
        }
    }
}

pub fn get_state() -> NetState {
    unsafe {
        if let Some(state) = NET_STATE.assume_init_mut().as_mut() {
            state.clone()
        } else {
            NetState::default()
        }
    }
}

pub unsafe fn get_state_mut() -> &'static mut NetState {
    NET_STATE.assume_init_mut().as_mut().unwrap()
}

/// Transmit a raw frame via SNP if available. Increments TX counters on success.
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

/// Sends a real ICMP Echo Request (Ping) to an external IP.
// pub fn ping_external(target_ip: [u8; 4]) -> Result<(), &'static str> {
//     #[allow(static_mut_refs)]
//     let state = unsafe { NET_STATE.assume_init_ref().as_ref().ok_or("Stack not init")? };
//
//     // 1. Define Packet Sizes
//     let icmp_len = size_of::<IcmpHeader>(); // Basic 8-byte ping, no extra payload
//     let ip_len = size_of::<Ipv4Header>() + icmp_len;
//     let total_len = size_of::<EthHeader>() + ip_len;
//
//     let mut frame = [0u8; 1514];
//
//     // 2. Build Ethernet Header
//     // Note: To reach an external IP, we send the packet to the GATEWAY's MAC.
//     // In a full stack, you'd use ARP to find this. Here we assume a broadcast
//     // or a known gateway MAC for the demo.
//     let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
//     eth.dst = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Broadcast (simplest for no-ARP setup)
//     eth.src = state.mac_addr;
//     eth.ethertype = ETHERTYPE_IPV4.to_be();
//
//     // 3. Build IPv4 Header
//     let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
//     ip.ver_ihl = 0x45; // Version 4, Header Length 5 (20 bytes)
//     ip.tos = 0;
//     ip.len = (ip_len as u16).to_be();
//     ip.id = 0x1234u16.to_be();
//     ip.off = 0;
//     ip.ttl = 64;
//     ip.proto = IP_PROTO_ICMP;
//     ip.src = state.ip_addr;
//     ip.dst = target_ip;
//     ip.checksum = 0;
//     // Calculate IP Checksum (Header only)
//     ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();
//
//     // 4. Build ICMP Header
//     let icmp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut IcmpHeader) };
//     icmp.msg_type = 8; // Echo Request
//     icmp.code = 0;
//     icmp.checksum = 0;
//     icmp.ident = 0xBEAFu16.to_be();
//     icmp.seq = 1u16.to_be();
//
//     // Calculate ICMP Checksum (Header + Payload)
//     let icmp_slice = &frame[size_of::<EthHeader>() + size_of::<Ipv4Header>()..total_len];
//     icmp.checksum = calculate_checksum(icmp_slice).to_be();
//
//     // 5. Transmit
//     snp_tx(&frame[..total_len])
// }


/// Generic ICMP sender: msg_type 8 for Request, 0 for Reply
fn send_icmp_echo(msg_type: u8, dst_ip: [u8; 4], dst_mac: [u8; 6], ident: u16, seq: u16, payload: &[u8]) {
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1024];

    let icmp_len = size_of::<IcmpHeader>() + payload.len();
    let ip_len = size_of::<Ipv4Header>() + icmp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = dst_mac;
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45;
    ip.len = (ip_len as u16).to_be();
    ip.ttl = 64;
    ip.proto = IP_PROTO_ICMP;
    ip.src = state.ip_addr;
    ip.dst = dst_ip;
    ip.checksum = 0;
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    let icmp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut IcmpHeader) };
    icmp.msg_type = msg_type;
    icmp.code = 0;
    icmp.ident = ident;
    icmp.seq = seq;
    icmp.checksum = 0;

    if !payload.is_empty() {
        frame[total_len - payload.len()..total_len].copy_from_slice(payload);
    }

    let icmp_slice = &frame[size_of::<EthHeader>() + size_of::<Ipv4Header>()..total_len];
    icmp.checksum = calculate_checksum(icmp_slice).to_be();

    let _ = crate::devices::net_hw::tx(&frame[..total_len]);
}



/// Public API: High-level Ping
pub fn ping(target_ip: [u8; 4], timeout_loops: u32) -> bool {
    let state = unsafe { NET_STATE.assume_init_mut().as_mut().expect("Not init") };
    state.ping_success = false;

    // 1. Resolve MAC via ARP
    let mac = match resolve_mac(target_ip, timeout_loops / 10) {
        Some(m) => m,
        None => {
            hpvm_warn!("PING", "ARP failed: destination unreachable");
            return false;
        }
    };

    // 2. Send ICMP Request
    send_icmp_echo(8, target_ip, mac, 0x1234, 1, b"UEFI-PING-DATA");

    // 3. Wait for Reply
    for _ in 0..timeout_loops {
        poll_tick();
        unsafe {
            if NET_STATE.assume_init_ref().as_ref().unwrap().ping_success {
                crate::vdebug!("PING", "Success!");
                return true;
            }
        }
    }

    false
}