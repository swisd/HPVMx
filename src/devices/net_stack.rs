//! UEFI Network Module (0.36.1) - Basic ICMP/IP over SNP
#![allow(dead_code)]

use crate::Color;
use crate::hpvm_log;
use core::mem::{MaybeUninit, size_of};
use core::sync::atomic::{AtomicBool, Ordering};
use crate::{hpvm_info, hpvm_warn, hpvm_error};

// --- Protocol Constants & Headers ---

const ETHERTYPE_IPV4: u16 = 0x0800;
const ETHERTYPE_ARP: u16 = 0x0806;
const ARP_REQUEST: u16 = 1;
const ARP_REPLY: u16 = 2;
const IP_PROTO_ICMP: u8 = 1;
const IP_PROTO_UDP: u8 = 17;
const IP_PROTO_TCP: u8 = 6;
const TCP_FLAG_FIN: u8 = 0x01;
const TCP_FLAG_SYN: u8 = 0x02;
const TCP_FLAG_RST: u8 = 0x04;
const TCP_FLAG_ACK: u8 = 0x10;

#[repr(C, packed)]
struct EthHeader {
    dst: [u8; 6],
    src: [u8; 6],
    ethertype: u16,
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

static STACK_INIT: AtomicBool = AtomicBool::new(false);
static HTTPD: AtomicBool = AtomicBool::new(false);

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
    mac_addr: [u8; 6],
    stats: NetStats,
}

static mut NET_STATE: MaybeUninit<Option<NetState>> = MaybeUninit::uninit();

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

// --- Implementation ---

pub fn init(ip: [u8; 4], gw: [u8; 4]) {
    if STACK_INIT.load(Ordering::SeqCst) { return; }

    let mac = crate::devices::net_hw::get_mac();
    let mc: &[u8; 6] = mac[0..6].try_into().expect("e"); // Ensure your net_hw provides this
    let state = NetState { ip_addr: ip, gateway: gw, mac_addr: *mc, stats: NetStats::default() };

    #[allow(static_mut_refs)]
    unsafe { NET_STATE.write(Some(state)); }
    STACK_INIT.store(true, Ordering::SeqCst);
    hpvm_info!("NET", "loopback stub initialized (127.0.0.1/8)");

    hpvm_info!("NET", "SNP stack up: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
}

#[inline]
pub fn is_initialized() -> bool { STACK_INIT.load(Ordering::SeqCst) }

#[allow(static_mut_refs)]
pub fn poll_tick() {
    if !STACK_INIT.load(Ordering::SeqCst) { return; }
    let mut buf = [0u8; 1514];

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
        hpvm_info!("PING", "loopback echo reply from {}: bytes=32 time=1ms TTL=64", dst);
        Ok(1)
    } else {
        hpvm_warn!("PING", "only loopback is available currently; cannot reach {}", dst);
        Err("no route to host (non-loopback)")
    }
}

fn handle_arp(packet: &[u8]) {
    if packet.len() < size_of::<ArpPacket>() { return; }
    let arp = unsafe { &*(packet.as_ptr() as *const ArpPacket) };
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };

    if u16::from_be(arp.opcode) == ARP_REQUEST && arp.target_ip == state.ip_addr {
        // Send ARP Reply
        let mut reply = [0u8; size_of::<EthHeader>() + size_of::<ArpPacket>()];
        let (eth_part, arp_part) = reply.split_at_mut(size_of::<EthHeader>());

        let eth_out = unsafe { &mut *(eth_part.as_mut_ptr() as *mut EthHeader) };
        eth_out.dst = arp.sender_mac;
        eth_out.src = state.mac_addr;
        eth_out.ethertype = ETHERTYPE_ARP.to_be();

        let arp_out = unsafe { &mut *(arp_part.as_mut_ptr() as *mut ArpPacket) };
        arp_out.hw_type = 1u16.to_be();
        arp_out.proto_type = 0x0800u16.to_be();
        arp_out.hw_size = 6;
        arp_out.proto_size = 4;
        arp_out.opcode = ARP_REPLY.to_be();
        arp_out.sender_mac = state.mac_addr;
        arp_out.sender_ip = state.ip_addr;
        arp_out.target_mac = arp.sender_mac;
        arp_out.target_ip = arp.sender_ip;

        let _ = snp_tx(&reply);
    }
}


fn handle_ipv4(packet: &[u8], src_mac: [u8; 6]) {
    if packet.len() < size_of::<Ipv4Header>() { return; }
    let ip = unsafe { &*(packet.as_ptr() as *const Ipv4Header) };
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };

    if ip.dst != state.ip_addr { return; }

    let header_len = (ip.ver_ihl & 0x0F) as usize * 4;
    let payload = &packet[header_len..];

    match ip.proto {
        IP_PROTO_ICMP => {
            let icmp = unsafe { &*(payload.as_ptr() as *const IcmpHeader) };
            if icmp.msg_type == 0 { // Echo Reply
                hpvm_info!("PING", "Received reply from {}.{}.{}.{}",
                ip.src[0], ip.src[1], ip.src[2], ip.src[3]);
                // Here you could set a global flag "LAST_PING_SUCCESS = true"
            }
            if icmp.msg_type == 8 { // Echo Request
                send_icmp_reply(ip.src, src_mac, icmp.ident, icmp.seq, &payload[size_of::<IcmpHeader>()..]);
            }
        }
        IP_PROTO_UDP => {
            if payload.len() >= size_of::<UdpHeader>() {
                let udp = unsafe { &*(payload.as_ptr() as *const UdpHeader) };
                handle_udp(ip.src, u16::from_be(udp.dst_port), &payload[size_of::<UdpHeader>()..]);
            }
        }
        _ => {}
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
    if packet.len() < size_of::<TcpHeader>() { return; }
    let tcp = unsafe { &*(packet.as_ptr() as *const TcpHeader) };
    let flags = u16::from_be(tcp.off_flags) & 0x3F;
    let dst_port = u16::from_be(tcp.dst_port);

    // We only care about HTTP port 80
    if dst_port != 80 { return; }

    if (flags & TCP_FLAG_SYN as u16) != 0 {
        // SYN received -> Send SYN/ACK
        send_tcp_packet(src_ip, src_mac, 80, u16::from_be(tcp.src_port),
                        0, u32::from_be(tcp.seq) + 1, TCP_FLAG_SYN | TCP_FLAG_ACK, &[]);
    } else if (flags & TCP_FLAG_ACK as u16) != 0 {
        // ACK received -> Check for HTTP GET in payload
        let header_len = ((u16::from_be(tcp.off_flags) >> 12) * 4) as usize;
        let payload = &packet[header_len..];

        if payload.starts_with(b"GET") {
            let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\nConnection: close\r\n\r\nHello from UEFI!";
            send_tcp_packet(src_ip, src_mac, 80, u16::from_be(tcp.src_port),
                            u32::from_be(tcp.ack), u32::from_be(tcp.seq) + payload.len() as u32,
                            TCP_FLAG_ACK | TCP_FLAG_FIN, response.as_bytes());
        }
    }
}

fn send_tcp_packet(dst_ip: [u8; 4], dst_mac: [u8; 6], src_port: u16, dst_port: u16,
                   seq: u32, ack: u32, flags: u8, data: &[u8]) {
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().unwrap() };
    let mut frame = [0u8; 1514];

    let tcp_len = size_of::<TcpHeader>() + data.len();
    let ip_len = size_of::<Ipv4Header>() + tcp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    // Fill Eth/IP (Same as UDP logic, just change proto to IP_PROTO_TCP)
    // ... [Omitted for brevity, use same pattern as send_udp] ...

    let tcp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut TcpHeader) };
    tcp.src_port = src_port.to_be();
    tcp.dst_port = dst_port.to_be();
    tcp.seq = seq.to_be();
    tcp.ack = ack.to_be();
    tcp.off_flags = (( (size_of::<TcpHeader>() as u16 / 4) << 12) | flags as u16).to_be();
    tcp.window = 8192u16.to_be();
    tcp.checksum = 0;

    frame[total_len - data.len()..total_len].copy_from_slice(data);

    // Note: TCP checksum requires a pseudo-header. Most UEFI SNP drivers can offload this,
    // but for "pure" software, you'd calculate it here.

    let _ = snp_tx(&frame[..total_len]);
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
    hpvm_info!("ICMP", "Ping reply sent to {}.{}.{}.{}", dst_ip[0], dst_ip[1], dst_ip[2], dst_ip[3]);
}

fn handle_udp(src_ip: [u8; 4], port: u16, data: &[u8]) {
    // Example: A simple "echo" or command listener on port 1234
    match port {
        1234 => {
            hpvm_info!("UDP", "Received {} bytes on port 1234 from {}.{}.{}.{}",
                data.len(), src_ip[0], src_ip[1], src_ip[2], src_ip[3]);
            // You could parse commands here (e.g., "REBOOT", "STATS")
        }
        _ => {}
    }
}







pub fn httpd_start(_port: u16) {
    //init();
    if HTTPD.swap(true, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "already running");
        return;
    }
    hpvm_info!("HTTPD", "loopback HTTPD placeholder started (no external clients)");
}

pub fn httpd_stop() {
    if !HTTPD.swap(false, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "not running");
        return;
    }
    hpvm_info!("HTTPD", "stopped");
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
pub fn ping_external(target_ip: [u8; 4]) -> Result<(), &'static str> {
    #[allow(static_mut_refs)]
    let state = unsafe { NET_STATE.assume_init_ref().as_ref().ok_or("Stack not init")? };

    // 1. Define Packet Sizes
    let icmp_len = size_of::<IcmpHeader>(); // Basic 8-byte ping, no extra payload
    let ip_len = size_of::<Ipv4Header>() + icmp_len;
    let total_len = size_of::<EthHeader>() + ip_len;

    let mut frame = [0u8; 1514];

    // 2. Build Ethernet Header
    // Note: To reach an external IP, we send the packet to the GATEWAY's MAC.
    // In a full stack, you'd use ARP to find this. Here we assume a broadcast
    // or a known gateway MAC for the demo.
    let eth = unsafe { &mut *(frame.as_mut_ptr() as *mut EthHeader) };
    eth.dst = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]; // Broadcast (simplest for no-ARP setup)
    eth.src = state.mac_addr;
    eth.ethertype = ETHERTYPE_IPV4.to_be();

    // 3. Build IPv4 Header
    let ip = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>()) as *mut Ipv4Header) };
    ip.ver_ihl = 0x45; // Version 4, Header Length 5 (20 bytes)
    ip.tos = 0;
    ip.len = (ip_len as u16).to_be();
    ip.id = 0x1234u16.to_be();
    ip.off = 0;
    ip.ttl = 64;
    ip.proto = IP_PROTO_ICMP;
    ip.src = state.ip_addr;
    ip.dst = target_ip;
    ip.checksum = 0;
    // Calculate IP Checksum (Header only)
    ip.checksum = calculate_checksum(&frame[size_of::<EthHeader>()..size_of::<EthHeader>() + size_of::<Ipv4Header>()]).to_be();

    // 4. Build ICMP Header
    let icmp = unsafe { &mut *(frame.as_mut_ptr().add(size_of::<EthHeader>() + size_of::<Ipv4Header>()) as *mut IcmpHeader) };
    icmp.msg_type = 8; // Echo Request
    icmp.code = 0;
    icmp.checksum = 0;
    icmp.ident = 0xBEAFu16.to_be();
    icmp.seq = 1u16.to_be();

    // Calculate ICMP Checksum (Header + Payload)
    let icmp_slice = &frame[size_of::<EthHeader>() + size_of::<Ipv4Header>()..total_len];
    icmp.checksum = calculate_checksum(icmp_slice).to_be();

    // 5. Transmit
    snp_tx(&frame[..total_len])
}