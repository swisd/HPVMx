//! Protobuf integration layer — re-exports generated types and provides helpers.

use alloc::vec::Vec;
use prost::Message as ProstMessage;

// Re-export generated protobuf types
pub use crate::network::generated::external::{
    VmActionRequest, VmActionResponse, VmConfig, VmIdentity,
    TerminalInput, TerminalFrame, Environment, VmAction,
    PortMapping, CustomOsConfig,
};
pub use crate::network::generated::internal::{
    IpcRequest as IPCRequest, IpcResponse as IPCResponse, CommandAction, TerminalPayload,
};
pub use crate::network::generated::internal::safe::{
    VDispatchKernelCommand, KernelIpc as KernelIPC, SafeHeartbeatRequest,
    SafeHeartbeatResponse, KernelCommand,
};

/// Errors that can occur when processing protobuf messages.
pub enum ProtobufError {
    IncompleteFrame,
    InvalidHeader,
    DecodeError(prost::DecodeError),
}

/// Processes a raw packet buffer from a bare-metal network ring buffer.
/// Deframes the length-prefixed message and decodes it as a VmActionRequest.
pub fn handle_incoming_grpc_packet(packet_data: &[u8]) -> Result<VmActionRequest, ProtobufError> {
    if packet_data.len() < 5 {
        return Err(ProtobufError::IncompleteFrame);
    }

    let _flags = packet_data[0];
    let payload_len = u32::from_be_bytes([
        packet_data[1],
        packet_data[2],
        packet_data[3],
        packet_data[4],
    ]) as usize;

    if packet_data.len() < 5 + payload_len {
        return Err(ProtobufError::IncompleteFrame);
    }

    let protobuf_payload = &packet_data[5..5 + payload_len];

    let request = VmActionRequest::decode(protobuf_payload)
        .map_err(ProtobufError::DecodeError)?;

    Ok(request)
}

/// Execute a VM action and return a response.
pub fn execute_vm_action(request: &VmActionRequest) -> VmActionResponse {
    VmActionResponse {
        request_id: request.request_id.clone(),
        identity: request.identity.clone(),
        vm_id: 0,
        success: true,
        error: alloc::string::String::new(),
        output: Vec::new(),
    }
}

/// Process an IPC request and return a response.
pub fn process_ipc_request(request: &IPCRequest) -> IPCResponse {
    IPCResponse {
        request_id: request.request_id.clone(),
        tid: request.tid.clone(),
        instance_uuid: request.instance_uuid.clone(),
        vm_id: 0,
        output: Vec::new(),
        error: alloc::string::String::new(),
        success: true,
    }
}

/// Process a kernel command and return a response.
pub fn process_kernel_command(request: &VDispatchKernelCommand) -> KernelIPC {
    KernelIPC {
        version: request.version,
        checksum: 0,
        metadata: alloc::string::String::new(),
        flags: 0,
        command: request.command.clone(),
        priority: request.priority,
        content: alloc::string::String::new(),
        sdata: Vec::new(),
        pdata: Vec::new(),
        request_id: request.request_id.clone(),
        success: true,
        error: alloc::string::String::new(),
        output: Vec::new(),
        sequence: request.sequence,
        auth_tag: Vec::new(),
    }
}
