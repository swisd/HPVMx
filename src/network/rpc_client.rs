//! RPC client for sending VmActionRequest messages to remote nodes.
//!
//! Used by the Manager mode VmDispatcher to forward deploy/signal requests
//! to registered nodes via the MSMI protocol on port 5001.
//!
//! Wire format (per message):
//!   [1-byte flags][4-byte big-endian length][protobuf payload]

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use prost::Message as ProstMessage;

use crate::devices::net_stack;
use crate::hosting::types::NodeInfo;
use crate::network::generated::external::{
    VmActionRequest, VmActionResponse, VmIdentity,
    ServerStatsRequest, ServerStatsResponse, NodeStatsRequest, NodeStatsResponse,
};
use crate::network::transport::frame_message;
use crate::vdebug_autoprefix;

/// Default flags byte for RPC frames.
const RPC_FLAGS: u8 = 0x01;

/// Global request ID counter for outbound RPC calls.
static mut OUTBOUND_REQ_COUNTER: u32 = 1;

/// Generate a unique request ID for outbound RPC calls.
fn next_request_id() -> String {
    unsafe {
        let id = OUTBOUND_REQ_COUNTER;
        OUTBOUND_REQ_COUNTER = OUTBOUND_REQ_COUNTER.wrapping_add(1);
        format!("rpc-{:08x}", id)
    }
}

/// Parse an IP string like "192.168.1.10" into [u8; 4].
fn parse_ip(ip_str: &str) -> Option<[u8; 4]> {
    let parts: alloc::vec::Vec<&str> = ip_str.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let mut ip = [0u8; 4];
    for (i, part) in parts.iter().enumerate() {
        match part.parse::<u8>() {
            Ok(val) => ip[i] = val,
            Err(_) => return None,
        }
    }
    Some(ip)
}

/// Send a VmActionRequest to a remote node and wait for the response.
///
/// This function performs a full TCP connect → send → receive → disconnect cycle.
/// It blocks polling for the response until timeout_ms elapses.
pub fn send_request_to_node(
    node: &NodeInfo,
    request: &VmActionRequest,
    timeout_ms: u32,
) -> Option<VmActionResponse> {
    if !net_stack::is_initialized() {
        vdebug_autoprefix!("RPC client: network stack not initialized");
        return None;
    }

    let dst_ip = parse_ip(&node.ip)?;
    let dst_port = node.port;

    vdebug_autoprefix!(
        "RPC client: sending {:?} request to node {} at {}:{})",
        request.action,
        node.node_id,
        node.ip,
        dst_port
    );

    // Encode the request
    let mut payload_buf = Vec::new();
    if let Err(e) = request.encode(&mut payload_buf) {
        vdebug_autoprefix!("RPC client: failed to encode request: {:?}", e);
        return None;
    }

    // Frame the message
    let framed = frame_message(RPC_FLAGS, &payload_buf);

    // Establish TCP connection
    if !net_stack::tcp_connect(dst_ip, dst_port) {
        vdebug_autoprefix!(
            "RPC client: failed to connect to node {} at {}:{}",
            node.node_id,
            node.ip,
            dst_port
        );
        return None;
    }

    // Send the framed request
    net_stack::tcp_send(dst_ip, dst_port, &framed);

    // Poll for response
    let mut attempts = timeout_ms / 10;
    let mut response_data = Vec::new();
    while attempts > 0 {
        net_stack::poll_tick();
        uefi::boot::stall(core::time::Duration::from_millis(10));

        // Check TCP receive buffer
        let state = net_stack::get_state();
        if !state.tcp_rx_data.is_empty() {
            response_data = state.tcp_rx_data.clone();
            break;
        }
        attempts -= 1;
    }

    // Disconnect
    net_stack::tcp_disconnect(dst_ip, dst_port);

    if response_data.is_empty() {
        vdebug_autoprefix!(
            "RPC client: no response from node {} within {}ms",
            node.node_id,
            timeout_ms
        );
        return None;
    }

    // Deframe the response
    let (_flags, proto_bytes) = match crate::network::transport::deframe_message(&response_data) {
        Ok(result) => result,
        Err(e) => {
            vdebug_autoprefix!("RPC client: failed to deframe response from node {}: {:?}", node.node_id, e);
            return None;
        }
    };

    // Decode the VmActionResponse
    match VmActionResponse::decode(proto_bytes) {
        Ok(response) => {
            vdebug_autoprefix!(
                "RPC client: received response from node {} (success={}, vm_id={})",
                node.node_id,
                response.success,
                response.vm_id
            );
            Some(response)
        }
        Err(e) => {
            vdebug_autoprefix!(
                "RPC client: failed to decode response from node {}: {:?}",
                node.node_id,
                e
            );
            None
        }
    }
}

/// Send a heartbeat ping to a node to check if it's alive.
pub fn ping_node(node: &NodeInfo) -> bool {
    let request = VmActionRequest {
        request_id: next_request_id(),
        identity: None,
        client_token: String::new(),
        verification_id: String::new(),
        environment: 0, // Unspecified
        action: 0,       // Unspecified — just a connectivity check
        extra_data: alloc::collections::BTreeMap::new(),
        action_payload: None,
    };

    send_request_to_node(node, &request, 1000).is_some()
}

/// Build a VmActionRequest from IPC-compatible parameters.
pub fn build_vm_action_request(
    identity: Option<VmIdentity>,
    environment: i32,
    action: i32,
    payload: Option<crate::network::generated::external::vm_action_request::ActionPayload>,
) -> VmActionRequest {
    VmActionRequest {
        request_id: next_request_id(),
        identity,
        client_token: String::new(),
        verification_id: String::new(),
        environment,
        action,
        extra_data: alloc::collections::BTreeMap::new(),
        action_payload: payload,
    }
}

// ---------------------------------------------------------------------------
// Stats polling functions — Intermediate Server ↔ Hypervisor RPC
// ---------------------------------------------------------------------------

/// Request server-wide stats from a remote node via RPC.
///
/// Sends a ServerStatsRequest and returns the decoded ServerStatsResponse.
pub fn request_server_stats(node: &NodeInfo, timeout_ms: u32) -> Option<ServerStatsResponse> {
    if !net_stack::is_initialized() {
        vdebug_autoprefix!("RPC client: network stack not initialized");
        return None;
    }

    let dst_ip = parse_ip(&node.ip)?;
    let dst_port = node.port;

    vdebug_autoprefix!(
        "RPC client: requesting server stats from node {} at {}:{}",
        node.node_id, node.ip, dst_port
    );

    let request = ServerStatsRequest {
        request_id: next_request_id(),
    };

    let mut payload_buf = Vec::new();
    if let Err(e) = request.encode(&mut payload_buf) {
        vdebug_autoprefix!("RPC client: failed to encode ServerStatsRequest: {:?}", e);
        return None;
    }

    let framed = frame_message(RPC_FLAGS, &payload_buf);

    if !net_stack::tcp_connect(dst_ip, dst_port) {
        vdebug_autoprefix!(
            "RPC client: failed to connect to node {} for stats",
            node.node_id
        );
        return None;
    }

    net_stack::tcp_send(dst_ip, dst_port, &framed);

    let mut attempts = timeout_ms / 10;
    let mut response_data = Vec::new();
    while attempts > 0 {
        net_stack::poll_tick();
        uefi::boot::stall(core::time::Duration::from_millis(10));
        let state = net_stack::get_state();
        if !state.tcp_rx_data.is_empty() {
            response_data = state.tcp_rx_data.clone();
            break;
        }
        attempts -= 1;
    }

    net_stack::tcp_disconnect(dst_ip, dst_port);

    if response_data.is_empty() {
        vdebug_autoprefix!(
            "RPC client: no stats response from node {} within {}ms",
            node.node_id, timeout_ms
        );
        return None;
    }

    let (_flags, proto_bytes) = match crate::network::transport::deframe_message(&response_data) {
        Ok(result) => result,
        Err(e) => {
            vdebug_autoprefix!("RPC client: failed to deframe stats response from node {}: {:?}", node.node_id, e);
            return None;
        }
    };

    // Try ServerStatsResponse first, then fall back to VmActionResponse with embedded stats
    if let Ok(response) = ServerStatsResponse::decode(proto_bytes) {
        vdebug_autoprefix!(
            "RPC client: received ServerStatsResponse from node {} (success={})",
            node.node_id, response.success
        );
        Some(response)
    } else {
        // Try VmActionResponse where output contains ServerStatsResponse
        if let Ok(vm_response) = VmActionResponse::decode(proto_bytes) {
            if vm_response.success && !vm_response.output.is_empty() {
                if let Ok(server_stats) = ServerStatsResponse::decode(&*vm_response.output) {
                    return Some(server_stats);
                }
            }
        }
        None
    }
}

/// Request stats for a specific node from a remote server via RPC.
pub fn request_node_stats(node: &NodeInfo, target_node_id: &str, timeout_ms: u32) -> Option<NodeStatsResponse> {
    if !net_stack::is_initialized() {
        return None;
    }

    let dst_ip = parse_ip(&node.ip)?;
    let dst_port = node.port;

    vdebug_autoprefix!(
        "RPC client: requesting node stats for {} from node {} at {}:{}",
        target_node_id, node.node_id, node.ip, dst_port
    );

    let request = NodeStatsRequest {
        request_id: next_request_id(),
        node_id: String::from(target_node_id),
    };

    let mut payload_buf = Vec::new();
    if let Err(e) = request.encode(&mut payload_buf) {
        vdebug_autoprefix!("RPC client: failed to encode NodeStatsRequest: {:?}", e);
        return None;
    }

    let framed = frame_message(RPC_FLAGS, &payload_buf);

    if !net_stack::tcp_connect(dst_ip, dst_port) {
        return None;
    }

    net_stack::tcp_send(dst_ip, dst_port, &framed);

    let mut attempts = timeout_ms / 10;
    let mut response_data = Vec::new();
    while attempts > 0 {
        net_stack::poll_tick();
        uefi::boot::stall(core::time::Duration::from_millis(10));
        let state = net_stack::get_state();
        if !state.tcp_rx_data.is_empty() {
            response_data = state.tcp_rx_data.clone();
            break;
        }
        attempts -= 1;
    }

    net_stack::tcp_disconnect(dst_ip, dst_port);

    if response_data.is_empty() {
        return None;
    }

    let (_flags, proto_bytes) = match crate::network::transport::deframe_message(&response_data) {
        Ok(result) => result,
        Err(_) => return None,
    };

    match NodeStatsResponse::decode(proto_bytes) {
        Ok(response) => {
            vdebug_autoprefix!(
                "RPC client: received NodeStatsResponse for {} (success={})",
                target_node_id, response.success
            );
            Some(response)
        }
        Err(e) => {
            vdebug_autoprefix!("RPC client: failed to decode NodeStatsResponse: {:?}", e);
            None
        }
    }
}

/// Request server stats using the legacy VmActionRequest protocol (action=GET_SERVER_STATS).
/// This is compatible with nodes that only support VmActionRequest/Response.
pub fn request_server_stats_legacy(node: &NodeInfo, timeout_ms: u32) -> Option<ServerStatsResponse> {
    let request = VmActionRequest {
        request_id: next_request_id(),
        identity: None,
        client_token: String::new(),
        verification_id: String::new(),
        environment: 0,
        action: 10, // GET_SERVER_STATS
        extra_data: alloc::collections::BTreeMap::new(),
        action_payload: None,
    };

    let vm_response = send_request_to_node(node, &request, timeout_ms)?;
    
    if !vm_response.success {
        vdebug_autoprefix!("RPC client: legacy stats request failed: {}", vm_response.error);
        return None;
    }

    // The output field contains the ServerStatsResponse protobuf
    if vm_response.output.is_empty() {
        return None;
    }

    match ServerStatsResponse::decode(&*vm_response.output) {
        Ok(response) => {
            vdebug_autoprefix!("RPC client: received legacy ServerStatsResponse from node {}", node.node_id);
            Some(response)
        }
        Err(e) => {
            vdebug_autoprefix!("RPC client: failed to decode legacy stats: {:?}", e);
            None
        }
    }
}
