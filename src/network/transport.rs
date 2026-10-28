//! Transport abstraction layer for sending and receiving protobuf messages
//! over TCP or UDP using the underlying net_stack.

use alloc::vec::Vec;
use crate::devices::net_stack;

/// Supported transport protocols.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportType {
    Tcp,
    Udp,
}

/// Errors that can occur during transport operations.
#[derive(Debug, Clone, Copy)]
pub enum TransportError {
    TcpConnectFailed,
    MacResolutionFailed,
    StackNotInitialized,
}

/// Frame a protobuf message with length-prefix header.
/// Format: [1-byte flags][4-byte big-endian length][payload]
pub fn frame_message(flags: u8, payload: &[u8]) -> Vec<u8> {
    let len = payload.len() as u32;
    let mut frame = Vec::with_capacity(5 + payload.len());
    frame.push(flags);
    frame.extend_from_slice(&len.to_be_bytes());
    frame.extend_from_slice(payload);
    frame
}

/// Deframe a length-prefixed message, returning (flags, payload).
pub fn deframe_message(buffer: &[u8]) -> Result<(u8, &[u8]), TransportError> {
    if buffer.len() < 5 {
        return Err(TransportError::StackNotInitialized);
    }
    let flags = buffer[0];
    let payload_len = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]) as usize;
    if buffer.len() < 5 + payload_len {
        return Err(TransportError::StackNotInitialized);
    }
    Ok((flags, &buffer[5..5 + payload_len]))
}

/// Send a payload over the specified transport to the given destination.
pub fn send(
    dst_ip: [u8; 4],
    dst_port: u16,
    transport: TransportType,
    payload: &[u8],
) -> Result<(), TransportError> {
    if !net_stack::is_initialized() {
        return Err(TransportError::StackNotInitialized);
    }
    match transport {
        TransportType::Tcp => {
            if !net_stack::tcp_connect(dst_ip, dst_port) {
                return Err(TransportError::TcpConnectFailed);
            }
            net_stack::tcp_send(dst_ip, dst_port, payload);
            net_stack::tcp_disconnect(dst_ip, dst_port);
            Ok(())
        }
        TransportType::Udp => {
            // Get MAC address from the network state for UDP send
            let state = net_stack::get_state();
            let src_port = 49152 + (state.ip_addr[3] as u16);
            net_stack::send_udp(dst_ip, state.mac_addr, src_port, dst_port, payload);
            Ok(())
        }
    }
}

/// Receive data from the network by polling. Returns the raw payload if available.
pub fn recv(transport: TransportType, timeout_ms: u32) -> Option<Vec<u8>> {
    if !net_stack::is_initialized() {
        return None;
    }
    let mut attempts = timeout_ms / 10;
    while attempts > 0 {
        net_stack::poll_tick();
        uefi::boot::stall(core::time::Duration::from_millis(10));

        if transport == TransportType::Tcp {
            // Check TCP receive buffer
            let state = net_stack::get_state();
            if !state.tcp_rx_data.is_empty() {
                let data = state.tcp_rx_data.clone();
                return Some(data);
            }
        }
        // UDP data is handled inline by poll_tick via handle_udp;
        // for a full implementation, a shared receive buffer would be needed.

        attempts -= 1;
    }
    None
}
