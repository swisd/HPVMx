//! Minimal networking scaffolding for HPVMx
//! Provides placeholder implementations for ping, lanscan, and an HTTP management server
//! so shell commands are available without requiring a fully wired NIC.

#![allow(dead_code)]

use alloc::format;
use crate::{hpvm_info, message, Color};
use crate::hpvm_log;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use core::time::Duration;
use uefi::boot;
use crate::{hpvm_error, hpvm_warn};
use crate::devices::net_stack::{EthHeader, ETHERTYPE_IPV4};
use crate::rng::XorShiftRng;
use super::net_hw;
use super::net_stack;

static HTTPD_RUNNING: AtomicBool = AtomicBool::new(false);

const UDP_PORT_DHCP_CLIENT: u16 = 68;
const UDP_PORT_DHCP_SERVER: u16 = 67;
const DHCP_XID: u32 = 371836547u32;
const DHCP_MIN_SIZE: usize = 240; // Fixed header (236) + magic cookie (4) = minimum for options
const DHCP_OPTIONS_OFFSET: usize = 240;

#[repr(C, packed)]
#[derive(Debug)]
struct DhcpPacket {
    op: u8,        // 1 = Request, 2 = Reply
    htype: u8,     // 1 = Ethernet
    hlen: u8,      // 6
    hops: u8,      // 0
    xid: u32,      // Transaction ID
    secs: u16,
    flags: u16,    // 0x8000 for broadcast
    ciaddr: [u8; 4],
    yiaddr: [u8; 4], // "Your" (Client) IP
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 16], // Client MAC
    sname: [u8; 64],
    file: [u8; 128],
    magic: [u8; 4],  // 0x63, 0x82, 0x53, 0x63
    options: [u8; 308], // DHCP packet is typically 576 bytes total, minus headers
}

#[repr(C, packed)]
struct DhcpReply {
    op: u8,
    htype: u8,
    hlen: u8,
    hops: u8,
    xid: u32,
    secs: u16,
    flags: u16,
    ciaddr: [u8; 4],
    yiaddr: [u8; 4],     // "Your" (client) IP address
    siaddr: [u8; 4],
    giaddr: [u8; 4],
    chaddr: [u8; 16],
    sname: [u8; 64],
    file: [u8; 128],
    magic: [u8; 4],      // Magic Cookie (0x63, 0x82, 0x53, 0x63)
    options: [u8; 308],
}

fn parse_dhcp_offer(data: &[u8]) -> Option<([u8; 4], [u8; 4], [u8; 4], [u8; 4])> {
    // Accept packets as small as 240 bytes (fixed header + magic cookie).
    // Many DHCP servers send shorter packets than the full 576 bytes.
    if data.len() < DHCP_MIN_SIZE {
        crate::vdebug!("dhcp", "Parse failed: size {} < {}", data.len(), DHCP_MIN_SIZE);
        return None;
    }

    // Read the fixed header fields directly from the byte slice
    let op = data[0];
    // Verify this is a reply (op=2)
    if op != 2 {
        crate::vdebug!("dhcp", "Parse failed: op={} (expected 2)", op);
        return None;
    }

    // Verify XID matches our request
    let xid = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    if xid != DHCP_XID {
        crate::vdebug!("dhcp", "XID mismatch: got 0x{:08x} (bytes {:02x}{:02x}{:02x}{:02x}), expected 0x{:08x}",
            xid, data[4], data[5], data[6], data[7], DHCP_XID);
        return None;
    }

    // Verify Magic Cookie at offset 236-239
    if data[236] != 0x63 || data[237] != 0x82 || data[238] != 0x53 || data[239] != 0x63 {
        crate::vdebug!("dhcp", "Invalid magic cookie: {:02x} {:02x} {:02x} {:02x}",
            data[236], data[237], data[238], data[239]);
        return None;
    }

    // Extract offered IP from yiaddr (offset 10-13)
    let offered_ip: [u8; 4] = [data[10], data[11], data[12], data[13]];

    // Parse options starting at offset 240
    let options = &data[DHCP_OPTIONS_OFFSET..];
    let mut server_ip = [0u8; 4];
    let mut subnet_mask = [0u8; 4];
    let mut router = [0u8; 4];
    let mut is_offer_or_ack = false;

    let mut cursor = 0;
    let mut option_count = 0;
    while cursor < options.len() {
        let opt_type = options[cursor];
        if opt_type == 255 {
            crate::vdebug!("dhcp", "Options end after {} options, cursor={}", option_count, cursor);
            break;
        }
        if opt_type == 0 { // Padding
            cursor += 1;
            continue;
        }
        if cursor + 1 >= options.len() { break; } // Safety: need at least length byte

        let opt_len = options[cursor + 1] as usize;
        if cursor + 2 + opt_len > options.len() { break; } // Safety: value fits

        let opt_value = &options[cursor + 2..cursor + 2 + opt_len];

        // Log each option type for diagnosis
        crate::vdebug!("dhcp", "  Option {}: type={}, len={}", option_count, opt_type, opt_len);
        option_count += 1;

        match opt_type {
            53 => { // DHCP Message Type
                // Value 2 = Offer, Value 5 = Ack
                let msg_type_val = if !opt_value.is_empty() { opt_value[0] } else { 0 };
                crate::vdebug!("dhcp", "  Message Type option found: value={}", msg_type_val);
                if msg_type_val == 2 || msg_type_val == 5 {
                    is_offer_or_ack = true;
                }
            }
            54 => { // Server Identifier
                if opt_len == 4 { server_ip.copy_from_slice(opt_value); }
            }
            1 => { // Subnet Mask
                if opt_len == 4 { subnet_mask.copy_from_slice(opt_value); }
            }
            3 => { // Router (Gateway)
                if opt_len >= 4 { router.copy_from_slice(&opt_value[0..4]); }
            }
            _ => {}
        }
        cursor += 2 + opt_len;
    }

    if is_offer_or_ack {
        crate::vdebug!("dhcp", "Parsed DHCP Offer/ACK: ip={:?}, server={:?}, router={:?}, mask={:?}",
            offered_ip, server_ip, router, subnet_mask);
        Some((offered_ip, server_ip, subnet_mask, router))
    } else {
        crate::vdebug!("dhcp", "DHCP message type not Offer/Ack");
        None
    }
}

pub fn discover_config() -> Option<([u8; 4], [u8; 4], [u8; 4])> {
    let mac = net_hw::get_mac();

    // Verify MAC isn't all zeros here
    if mac.iter().all(|&x| x == 0) {
        hpvm_error!("NET", "MAC address is all zeros! Check NIC initialization.");
    }

    crate::vdebug!("dhcp", "mac: {:?}", mac);

    // Enable promiscuous mode during DHCP discovery to ensure we receive
    // broadcast DHCP responses on bridged LANs where the NIC may filter them.
    let _ = net_hw::set_receive_filters(true);

    //let mut rng = XorShiftRng::new(20);
    //let xid = rng.rand(4) as u32;

    let mut packet = DhcpPacket {
        op: 1, htype: 1, hlen: 6, hops: 0,
        xid: DHCP_XID.to_be(),
        secs: 0,
        flags: 0x8000u16.to_be(),
        ciaddr: [0; 4], yiaddr: [0; 4], siaddr: [0; 4], giaddr: [0; 4],
        chaddr: [0; 16],
        sname: [0; 64], file: [0; 128],
        magic: [0x63, 0x82, 0x53, 0x63],
        options: [0; 308],
    };

    packet.chaddr[..6].copy_from_slice(&mac[..6]);

    let mut cursor = 0;
    // DHCP Discover
    packet.options[cursor..cursor+3].copy_from_slice(&[53, 1, 1]);
    cursor += 3;

    // Parameter Request List
    packet.options[cursor] = 55;
    packet.options[cursor + 1] = 3;
    packet.options[cursor + 2] = 1; // Subnet Mask
    packet.options[cursor + 3] = 3; // Router
    packet.options[cursor + 4] = 6; // DNS
    cursor += 5;

    // Client Identifier
    packet.options[cursor] = 61;
    packet.options[cursor + 1] = 7;
    packet.options[cursor + 2] = 1;
    packet.options[cursor + 3..cursor + 9].copy_from_slice(&mac[..6]);
    cursor += 9;

    packet.options[cursor] = 255;


    // Log the XID bytes being sent
    let xid_stored = packet.xid;
    let xid_bytes = xid_stored.to_be_bytes();
    crate::vdebug!("dhcp", "Sending DHCP Discover with XID=0x{:08x} (bytes {:02x}{:02x}{:02x}{:02x}), expected parse as 0x{:08x}",
        DHCP_XID, xid_bytes[0], xid_bytes[1], xid_bytes[2], xid_bytes[3], DHCP_XID);

    match net_stack::send_raw_udp([0,0,0,0], [255,255,255,255], [0xFF; 6], 68, 67, unsafe {
        core::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<DhcpPacket>())
    }) {
        Ok(_) => {}
        Err(e) => {hpvm_warn!("NET", "Could not send RAWUDP Discover: {}", e)}
    }

    crate::vdebug!("NET", "DHCP Discover sent, waiting for Offer...");

    let mut cfg: Option<([u8; 4], [u8; 4], [u8; 4], [u8; 4])> = None;

    // Retry DHCP Discover up to 3 times on bridged LAN — servers can be slow or drop the first request
    for attempt in 1..=3 {
        if cfg.is_some() { break; }
        crate::vdebug!("dhcp", "DHCP Offer attempt {}/3", attempt);

        let mut timeout = 500; // 5 second timeout per attempt (bridged LAN can be slow)
        while timeout > 0 {
            if let Some(config) = poll_for_dhcp_response() {
                crate::vdebug!("dhcp", "Received DHCP Offer: {:?}", config);
                cfg = Some(config);
                break;
            }
            boot::stall(Duration::from_micros(10_000)); // 10ms wait between polls
            timeout -= 1;
        }

        // Resend DHCP Discover if no offer received yet
        if cfg.is_none() && attempt < 3 {
            crate::vdebug!("dhcp", "No offer received, resending DHCP Discover...");
            match net_stack::send_raw_udp([0,0,0,0], [255,255,255,255], [0xFF; 6], 68, 67, unsafe {
                core::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<DhcpPacket>())
            }) {
                Ok(_) => {}
                Err(e) => {hpvm_warn!("NET", "Could not resend RAWUDP Discover: {}", e)}
            }
        }
    }

    if let Some((offered_ip, server_ip, _subnet_mask, router)) = cfg {
        send_dhcp_request(offered_ip, server_ip, DHCP_XID);

        crate::vdebug!("dhcp", "Waiting for DHCP ACK...");
        let mut timeout = 500; // 5 second timeout for DHCP ACK (bridged LAN can be slow)
        let mut poll_count = 0u32;
        while timeout > 0 {
            if let Some((ip, _srv, subnet_mask, ack_router)) = poll_for_dhcp_response() {
                crate::vdebug!("dhcp", "Received DHCP ACK: ip={:?}, router={:?}", ip, ack_router);
                // Use router (option 3) as gateway
                let mut gateway = if ack_router != [0u8; 4] { ack_router } else { router };
                if gateway == [0u8; 4] {
                    // No router option provided — apply VirtualBox NAT heuristic.
                    // In VirtualBox NAT, the DHCP server is 10.0.2.1 but the gateway is 10.0.2.2.
                    // If the DHCP server is 10.0.2.1, assume the gateway is the next IP.
                    if server_ip == [10, 0, 2, 1] {
                        gateway = [10, 0, 2, 2];
                        crate::vdebug!("dhcp", "VirtualBox NAT detected, setting gateway to 10.0.2.2");
                    } else {
                        // Generic fallback: use server_ip as gateway
                        gateway = server_ip;
                    }
                }
                // Disable promiscuous mode now that DHCP is complete
                let _ = net_hw::set_receive_filters(false);
                return Some((ip, gateway, subnet_mask));
            }
            boot::stall(Duration::from_micros(10_000));
            timeout -= 1;
            poll_count += 1;
            if poll_count % 100 == 0 {
                crate::vdebug!("dhcp", "ACK polling: {} iterations, no ACK yet", poll_count);
            }
        }
        crate::vdebug!("NET", "no dhcp response after {} polls, using fallback IP. configure static ip in network tab", poll_count);
    } else {
        crate::vdebug!("NET", "no dhcp offer received, using fallback IP. configure static ip in network tab");
    }
    // Disable promiscuous mode after DHCP discovery
    let _ = net_hw::set_receive_filters(false);

    // Try to load saved static IP from XDB before falling back to link-local
    if let Some((ip_str, gw_str, mask_str)) = load_network_config() {
        if let Some(ip) = parse_ip(&ip_str) {
            if let Some(gw) = parse_ip(&gw_str) {
                if let Some(mask) = parse_ip(&mask_str) {
                    crate::vdebug!("NET", "DHCP failed, using saved static IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
                    return Some((ip, gw, mask));
                }
            }
        }
    }

    crate::vdebug!("NET", "DHCP failed and no saved config, using link-local fallback 169.254.1.1");
    Some(([169, 254, 1, 1], [169, 254, 1, 254], [255, 255, 0, 0]))
}

pub fn send_dhcp_request(offered_ip: [u8; 4], server_ip: [u8; 4], xid: u32) {
    let mac = net_hw::get_mac();
    let mut packet = DhcpPacket {
        op: 1, htype: 1, hlen: 6, hops: 0,
        xid: xid.to_be(), // Use the same XID as the Discover
        secs: 0, flags: 0x8000u16.to_be(),
        ciaddr: [0; 4], yiaddr: [0; 4], siaddr: [0; 4], giaddr: [0; 4],
        chaddr: [0; 16], sname: [0; 64], file: [0; 128],
        magic: [0x63, 0x82, 0x53, 0x63],
        options: [0; 308],
    };
    packet.chaddr[..6].copy_from_slice(&mac[..6]);

    let mut cursor = 0;

    // 1. DHCP Message Type: Request (Value 3)
    packet.options[cursor..cursor+3].copy_from_slice(&[53, 1, 3]);
    cursor += 3;

    // 2. Client Identifier (Option 61) - Must match the Discover's Client Identifier
    packet.options[cursor] = 61;
    packet.options[cursor + 1] = 7;
    packet.options[cursor + 2] = 1; // Ethernet hardware type
    packet.options[cursor + 3..cursor + 9].copy_from_slice(&mac[..6]);
    cursor += 9;

    // 3. Requested IP Address (Option 50)
    packet.options[cursor] = 50;
    packet.options[cursor + 1] = 4;
    packet.options[cursor + 2..cursor + 6].copy_from_slice(&offered_ip);
    cursor += 6;

    // 4. Server Identifier (Option 54) - The IP of the DHCP Server
    packet.options[cursor] = 54;
    packet.options[cursor + 1] = 4;
    packet.options[cursor + 2..cursor + 6].copy_from_slice(&server_ip);
    cursor += 6;

    // 5. End Option
    packet.options[cursor] = 255;

    crate::vdebug!("NET", "Sending DHCP Request...");
    // Broadcast the request
    if let Ok(_result) = net_stack::send_raw_udp(
        [0, 0, 0, 0],
        [255, 255, 255, 255],
        [0xFF; 6],
        68, 67,
        unsafe {
            core::slice::from_raw_parts(&packet as *const _ as *const u8, size_of::<DhcpPacket>())
        }
    ) {} else {
        hpvm_warn!("NET", "Could not send RAWUDP")
    }
}

fn poll_for_dhcp_response() -> Option<([u8; 4], [u8; 4], [u8; 4], [u8; 4])> {
    let mut buf = [0u8; 1514];

    // Drain the RX buffer specifically looking for DHCP
    while let Ok(sz) = net_hw::rx(&mut buf) {
        if sz == 0 { break; }
        let frame = &buf[..sz];
        if frame.len() < size_of::<EthHeader>() + 20 + 8 { continue; } // Eth + IP + UDP min

        let eth = unsafe { &*(frame.as_ptr() as *const EthHeader) };
        if u16::from_be(eth.ethertype) != ETHERTYPE_IPV4 { continue; }

        let ip_payload = &frame[size_of::<EthHeader>()..];
        // Basic check for UDP (Protocol 17)
        if ip_payload[9] == 17 {
            let ihl = (ip_payload[0] & 0x0F) as usize * 4;
            if ip_payload.len() < ihl + 8 { continue; }
            let udp_payload = &ip_payload[ihl..];
            let dst_port = u16::from_be_bytes([udp_payload[2], udp_payload[3]]);
            let src_port = u16::from_be_bytes([udp_payload[0], udp_payload[1]]);

            // Log all UDP packets to see what's arriving
            if dst_port == 68 || src_port == 68 {
                crate::vdebug!("dhcp", "RX UDP port 68/67: dst_port={}, src_port={}, size={}", dst_port, src_port, sz);
            }

            if dst_port == 68 {
                let dhcp_data = &udp_payload[8..];
                let dhcp_data_len = dhcp_data.len();
                crate::vdebug!("dhcp", "RX UDP port 68 (src_port={}), total_size={}, dhcp_data={}", src_port, sz, dhcp_data_len);
                // Hex dump first 20 bytes for diagnosis
                if dhcp_data_len >= 20 {
                    crate::vdebug!("dhcp", "  First 20 bytes: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
                        dhcp_data[0], dhcp_data[1], dhcp_data[2], dhcp_data[3],
                        dhcp_data[4], dhcp_data[5], dhcp_data[6], dhcp_data[7],
                        dhcp_data[8], dhcp_data[9], dhcp_data[10], dhcp_data[11],
                        dhcp_data[12], dhcp_data[13], dhcp_data[14], dhcp_data[15],
                        dhcp_data[16], dhcp_data[17], dhcp_data[18], dhcp_data[19]);
                }
                if let Some(res) = parse_dhcp_offer(dhcp_data) {
                    crate::vdebug!("dhcp", "DHCP Offer/ACK parsed: ip={:?}", res.0);
                    return Some(res);
                } else {
                    crate::vdebug!("dhcp", "DHCP parse failed — data_len={}", dhcp_data_len);
                }
            }
        }
    }
    None
}

/// Helper to parse "1.2.3.4" into [u8; 4]
pub fn parse_ip(ip: &str) -> Option<[u8; 4]> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 { return None; }
    let mut octets = [0u8; 4];
    for i in 0..4 {
        octets[i] = parts[i].parse().ok()?;
    }
    Some(octets)
}

/// Helper to parse "1.2.3.4:80" or "1.2.3.4" into ([u8; 4], u16)
pub fn parse_endpoint(endpoint: &str) -> Option<([u8; 4], u16)> {
    let parts: Vec<&str> = endpoint.split(':').collect();
    if parts.len() == 1 {
        let ip = parse_ip(parts[0])?;
        Some((ip, 80))
    } else if parts.len() == 2 {
        let ip = parse_ip(parts[0])?;
        let port = parts[1].parse().ok()?;
        Some((ip, port))
    } else {
        None
    }
}

/// Ensure hardware is initialized (SNP). Best-effort.
fn ensure_hw() {
    if !net_hw::is_initialized() {
        let _ = net_hw::init();
    }

    if !net_stack::is_initialized() {
        crate::vdebug!("NET", "Discovering network configuration via DHCP...");

        // Attempt to get config from the wire
        if let Some((my_ip, my_gw, my_mask)) = discover_config() {
            net_stack::init(my_ip, my_gw, my_mask);
            crate::vdebug!("NET", "DHCP Success: {}.{}.{}.{}", my_ip[0], my_ip[1], my_ip[2], my_ip[3]);
        } else {
            hpvm_warn!("NET", "DHCP failed. Falling back to static recovery IP.");
            let fallback_ip = [169, 254, 1, 1];
            let fallback_gw = [169, 254, 1, 254];
            let fallback_mask = [255, 255, 0, 0];
            net_stack::init(fallback_ip, fallback_gw, fallback_mask);
        }
    }
}


pub fn ensure_net() {
    if !net_hw::is_initialized() {
        let _ = net_hw::init();
    }

    // First, try to load saved network config from file
    if !net_stack::is_static_ip_configured() {
        if let Some((ip_str, gw_str, mask_str)) = load_network_config() {
            if let Some(ip) = parse_ip(&ip_str) {
                if let Some(gw) = parse_ip(&gw_str) {
                    if let Some(mask) = parse_ip(&mask_str) {
                        net_stack::set_static_ip(ip, gw, mask);
                        crate::vdebug!("NET", "Loaded saved network config: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
                    }
                }
            }
        }
    }

    if !net_stack::is_initialized() {
        // Check if static IP is configured — bypass DHCP if so
        if net_stack::is_static_ip_configured() {
            let (ip, gw, mask) = net_stack::get_static_ip_config();
            net_stack::init(ip, gw, mask);
            crate::vdebug!("NET", "Using static IP: {}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
        } else {
            crate::vdebug!("NET", "Discovering network configuration via DHCP...");

            // Attempt to get config from the wire
            if let Some((my_ip, my_gw, my_mask)) = discover_config() {
                net_stack::init(my_ip, my_gw, my_mask);
                crate::vdebug!("NET", "DHCP Success: {}.{}.{}.{}", my_ip[0], my_ip[1], my_ip[2], my_ip[3]);
            } else {
                hpvm_warn!("NET", "DHCP failed. Falling back to static recovery IP.");
                let fallback_ip = [169, 254, 1, 1];
                let fallback_gw = [169, 254, 1, 254];
                let fallback_mask = [255, 255, 0, 0];
                net_stack::init(fallback_ip, fallback_gw, fallback_mask);
            }
        }
    }
}

/// Return true if the network stack has been initialized.
pub fn is_initialized() -> bool {
    net_stack::is_initialized()
}

/// Print simple NIC status to the console (MAC/MTU/link).
pub fn status() {
    ensure_net();
    let backend = net_stack::backend_name();
    let stats = net_stack::stats();
    match net_hw::get_info() {
        Some(info) => {
            let mut out = String::new();
            out.push_str("\nNIC Status:\n");
            out.push_str(&format!("  MAC:      {:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}\n",
                                  info.mac[0], info.mac[1], info.mac[2], info.mac[3], info.mac[4], info.mac[5]));
            out.push_str(&format!("  MTU:      {}\n", info.mtu));
            // Check current link status from SNP, not just the cached value at init time
            let link_up = net_hw::link_up();
            out.push_str(&format!("  Link:     {}\n", if link_up { "UP" } else { "DOWN" }));
            out.push_str(&format!("  Backend:  {}\n", backend));
            out.push_str(&format!("  Traffic:  RX {} pkts ({} bytes) / TX {} pkts ({} bytes)\n",
                                  stats.rx_pkts, stats.rx_bytes, stats.tx_pkts, stats.tx_bytes));
            message!("\n", "{}", out)
        }
        None => hpvm_error!("NET", "no NIC detected"),
    }
}

pub fn ping(ip_str: &str, _count: usize, _timeout_ms: u64) -> Result<u32, &'static str> {
    ensure_net();

    // Handle Loopback
    if ip_str == "127.0.0.1" || ip_str.eq_ignore_ascii_case("localhost") {
        return net_stack::ping_loopback(ip_str);
    }

    // Handle External
    if let Some(target_ip) = parse_ip(ip_str) {
        // Our ping_external now actually sends a packet!
        // Note: For now, it returns success if the packet was sent.
        // Actual RTT calculation requires the main loop calling net_stack::poll_tick().
        let _ = net_stack::ping_external(target_ip, 64, true);
        Ok((0))
    } else {
        Err("invalid ip format")
    }
}

/// Scan a /24 network by trying TCP port 80 (HTTP) like the provided batch example.
/// Example prefix: "192.168.1."
pub fn lanscan(prefix: &str) {
    ensure_net();
    if !prefix.ends_with('.') {
        hpvm_error!("NET", "prefix must end with '.' e.g. 192.168.1.");
        return;
    }
    let octets: Vec<&str> = prefix.trim_end_matches('.').split('.').collect();
    if octets.len() != 3 {
        hpvm_error!("NET", "prefix must have 3 octets, e.g. 10.0.0.");
        return;
    }

    let found: Vec<String> = Vec::new();
    let mut lines: [String; 11] = [
        String::from("1   "), String::from("26  "), String::from("51  "), String::from("76  "),
        String::from("101 "), String::from("126 "), String::from("151 "), String::from("176 "),
        String::from("201 "), String::from("226 "), String::from("251 "),
    ];

    crate::vdebug!("NET", "Starting LAN scan via ARP probes...");

    for host in 1..=255u16 {
        let ip_str = alloc::format!("{}{}", prefix, host);
        let _target_ip = parse_ip(&ip_str).unwrap();

        // In this manual stack, "lanscan" can work by broadcasting
        // a small UDP probe or an ARP request.
        // For now, we simulate the "send" part:
        let reachable = net_stack::ping_external(_target_ip, 64, false); // Real-time feedback requires an async listener

        // Append visualization
        let idx = (host.saturating_sub(1) / 25) as usize;
        if idx < 11 {
            lines[idx].push_str(if reachable { "[■]" } else { "[x]" });
        }

        // Periodic redraw
        if host % 10 == 0 || host == 255 {
            uefi::system::with_stdout(|s| {
                let _ = s.clear();
                let _ = core::fmt::Write::write_str(s, "\nLAN Scan (TCP:80)\n");
                let _ = core::fmt::Write::write_str(s, "────────────────────────────\n");
                for l in &lines { let _ = core::fmt::Write::write_str(s, l); let _ = core::fmt::Write::write_str(s, "\n"); }
                let _ = core::fmt::Write::write_str(s, "\nIPs Found:\n");
                for ipf in &found { let _ = core::fmt::Write::write_str(s, ipf); let _ = core::fmt::Write::write_str(s, "\n"); }
                let _ = core::fmt::Write::write_str(s, "\nTip: actual probing requires a NIC driver.\n");
            });
        }












//    let mut found: Vec<String> = Vec::new();

    // Color blocks placeholders (no ANSI in UEFI console, so use symbols)
//    let good = "[■]"; // reachable
//    let bad = "[ ]";  // unreachable

//    for host in 1..=255u16 {
        // Compose IP
//        let ip = alloc::format!("{}{}", prefix, host);

        // Try to detect host by TCP:80 (placeholder always fails for now)
//        let reachable = false; // until NIC is wired, we assume unreachable

//        if reachable {
//            found.push(ip.clone());
//        }

        // Append block to the corresponding line
//        let idx = match host {
//            1..=25 => 0,
//            26..=50 => 1,
//            51..=75 => 2,
//            76..=100 => 3,
//            101..=125 => 4,
//            126..=150 => 5,
//            151..=175 => 6,
//            176..=200 => 7,
//            201..=225 => 8,
//            226..=250 => 9,
//            _ => 10,
//        };
//        lines[idx].push_str(if reachable { good } else { bad });

        // Periodic redraw to give user feedback
//        if host % 5 == 0 || host == 255 {
//            uefi::system::with_stdout(|s| {
//                let _ = s.clear();
 //               let _ = core::fmt::Write::write_str(s, "\nLAN Scan (TCP:80)\n");
 //               let _ = core::fmt::Write::write_str(s, "────────────────────────────\n");
//                for l in &lines { let _ = core::fmt::Write::write_str(s, l); let _ = core::fmt::Write::write_str(s, "\n"); }
//                let _ = core::fmt::Write::write_str(s, "\nIPs Found:\n");
//                for ipf in &found { let _ = core::fmt::Write::write_str(s, ipf); let _ = core::fmt::Write::write_str(s, "\n"); }
//                let _ = core::fmt::Write::write_str(s, "\nTip: actual probing requires a NIC driver.\n");
//            });
//        }
    }
    crate::vdebug!("NET", "scan complete. hosts detected: {}", found.len());
}

/// Start the HTTP management server on the specified port.
/// Ensures the network stack is initialized first, then enables TCP/HTTP handling
/// and spawns a background polling task so incoming requests are processed in real-time.
pub fn httpd_start(port: u16) {
    crate::vdebug!("HTTPD", "httpd_start called with port {}", port);
    ensure_net();

    // Verify the stack is actually initialized before enabling the server
    if !net_stack::is_initialized() {
        hpvm_error!("HTTPD", "cannot start: network stack not initialized");
        crate::vdebug!("HTTPD", "httpd_start: aborting due to uninitialized stack");
        return;
    }

    if HTTPD_RUNNING.swap(true, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "already running");
        crate::vdebug!("HTTPD", "httpd_start: aborting, already running");
        return;
    }

    crate::vdebug!("HTTPD", "httpd_start: initializing net_stack HTTPD");
    net_stack::httpd_start(port);

    // The background async task approach doesn't work reliably because the dummy waker
    // doesn't re-add the task to the ready queue after yield_now().
    // Instead, we rely on the main loop's poll_tick() call and poll_global_ready() call.
    // The main loop already calls both, so the network stack is polled every iteration.
    // However, the main loop blocks on read_line() between iterations, so we need to
    // ensure poll_global_ready() is called frequently enough.
    crate::vdebug!("HTTPD", "httpd_start: network polling handled by main loop");

    // Print the server's IP address so the user knows where to connect
    let state = net_stack::get_state();
    hpvm_info!("HTTPD", "Server started on {}.{}.{}.{}:{}",
        state.ip_addr[0], state.ip_addr[1], state.ip_addr[2], state.ip_addr[3], port);
    hpvm_info!("HTTPD", "Gateway: {}.{}.{}.{}  Mask: {}.{}.{}.{}",
        state.gateway[0], state.gateway[1], state.gateway[2], state.gateway[3],
        state.subnet_mask[0], state.subnet_mask[1], state.subnet_mask[2], state.subnet_mask[3]);
}

pub fn httpd_stop() {
    if !HTTPD_RUNNING.swap(false, Ordering::SeqCst) {
        hpvm_warn!("HTTPD", "not running");
        return;
    }
    net_stack::httpd_stop();
}

/// Save network configuration to XDB hive0 for persistence across reboots
pub fn save_network_config(ip: &str, gw: &str, mask: &str) -> Result<(), &'static str> {
    let db = crate::xdb::get_global_db_mut().ok_or("XDB not initialized")?;
    
    // Store IP
    let _ = db.parse_command(
        &format!("INSERT INTO 1 ROW 10 VALUES Int32(10) Int32(10) Text(\"network_ip\") Text(\"String\") Text(\"{}\")", ip).as_bytes().to_vec(),
        &mut Vec::new()
    );
    
    // Store Gateway
    let _ = db.parse_command(
        &format!("INSERT INTO 1 ROW 11 VALUES Int32(11) Int32(11) Text(\"network_gw\") Text(\"String\") Text(\"{}\")", gw).as_bytes().to_vec(),
        &mut Vec::new()
    );
    
    // Store Mask
    let _ = db.parse_command(
        &format!("INSERT INTO 1 ROW 12 VALUES Int32(12) Int32(12) Text(\"network_mask\") Text(\"String\") Text(\"{}\")", mask).as_bytes().to_vec(),
        &mut Vec::new()
    );
    
    // Commit changes
    let _ = db.parse_command(&"COMMIT".as_bytes().to_vec(), &mut Vec::new());
    
    crate::vdebug!("NET", "Network config saved to XDB: IP={}, GW={}, Mask={}", ip, gw, mask);
    Ok(())
}

/// Load network configuration from XDB hive0, returns (ip, gw, mask) if found
pub fn load_network_config() -> Option<(String, String, String)> {
    let db = crate::xdb::get_global_db_mut()?;
    
    // Use GET commands to directly fetch rows by ID (10=IP, 11=GW, 12=Mask)
    // This avoids the SELECT WHERE limitation which only supports 'id' and 'val' columns
    let result_ip = db.parse_command(&"GET 1 10".as_bytes().to_vec(), &mut Vec::new());
    let result_gw = db.parse_command(&"GET 1 11".as_bytes().to_vec(), &mut Vec::new());
    let result_mask = db.parse_command(&"GET 1 12".as_bytes().to_vec(), &mut Vec::new());
    
    let ip = extract_data_text(&result_ip);
    let gw = extract_data_text(&result_gw);
    let mask = extract_data_text(&result_mask);
    
    match (ip, gw, mask) {
        (Some(ip), Some(gw), Some(mask)) => {
            crate::vdebug!("NET", "Loaded network config from XDB: IP={}, GW={}, Mask={}", ip, gw, mask);
            Some((ip, gw, mask))
        },
        _ => {
            crate::vdebug!("NET", "No saved network config found in XDB");
            None
        }
    }
}

/// Extract the last Text value from XDB GET result (the 'data' field)
/// GET result format: "FOUND: [Entry { data: Int32(10) }, Entry { data: Int32(10) }, Entry { data: Text(\"network_ip\") }, Entry { data: Text(\"String\") }, Entry { data: Text(\"192.168.1.100\") }]\r\n"
/// The schema is: id Int32, key Int32, name Text, type Text, data Text
/// We need the last Text value which is the 'data' field containing the actual config value
fn extract_data_text(result: &str) -> Option<String> {
    if !result.contains("FOUND:") {
        return None;
    }
    // Find all Text("...") occurrences and return the last one (the 'data' field)
    let mut last_text = None;
    let mut search_from = 0;
    let full = result;
    loop {
        let slice = &full[search_from..];
        if let Some(start) = slice.find("Text(\"") {
            let abs_start = search_from + start + 6; // Skip "Text(\""
            let after = &full[abs_start..];
            if let Some(end) = after.find("\")") {
                last_text = Some(String::from(&after[..end]));
            }
            search_from = abs_start + after.find("\")").map(|e| e + 2).unwrap_or(1);
        } else {
            break;
        }
    }
    last_text
}

