//! Generic protobuf request/response dispatcher.
//!
//! Routes incoming length-prefixed protobuf messages to registered handlers
//! by message type, then frames and sends the response.

use alloc::string::String;
use alloc::vec::Vec;
use prost::Message as ProstMessage;

use crate::network::transport::{self, TransportType};

/// Type-erased handler that receives raw protobuf bytes and returns a response.
type BoxedHandler = dyn Fn(&[u8]) -> Option<Vec<u8>>;

/// A single registered handler entry.
struct HandlerEntry {
    /// Human-readable name for debugging.
    name: &'static str,
    /// First few bytes (field tag) used for dispatch identification.
    dispatch_tag: u8,
    /// The handler function.
    handler: &'static BoxedHandler,
}

/// Generic protobuf dispatcher with handler registration.
pub struct ProtobufDispatcher {
    handlers: [Option<HandlerEntry>; 16],
    next_slot: usize,
}

impl ProtobufDispatcher {
    /// Create a new empty dispatcher.
    pub fn new() -> Self {
        Self {
            handlers: Default::default(),
            next_slot: 0,
        }
    }

    /// Register a handler for a specific message type identified by a dispatch tag.
    /// The dispatch tag is typically the first protobuf field tag (wire key >> 3).
    /// Returns `true` if registration succeeded.
    pub fn register_handler(
        &mut self,
        name: &'static str,
        dispatch_tag: u8,
        handler: &'static BoxedHandler,
    ) -> bool {
        if self.next_slot >= self.handlers.len() {
            return false;
        }
        self.handlers[self.next_slot] = Some(HandlerEntry {
            name,
            dispatch_tag,
            handler,
        });
        self.next_slot += 1;
        true
    }

    /// Dispatch a raw packet: deframe, identify message type, call handler, frame response.
    pub fn dispatch(&self, raw_packet: &[u8]) -> Option<Vec<u8>> {
        let (flags, payload) = transport::deframe_message(raw_packet).ok()?;

        // Identify message type by the first wire key (field tag)
        if payload.is_empty() {
            return None;
        }
        let dispatch_tag = payload[0] >> 3;

        // Find matching handler
        for slot in &self.handlers {
            if let Some(entry) = slot {
                if entry.dispatch_tag == dispatch_tag {
                    if let Some(response_bytes) = (entry.handler)(payload) {
                        return Some(transport::frame_message(flags, &response_bytes));
                    }
                }
            }
        }
        None
    }

    /// Poll the network for incoming messages and dispatch them.
    pub fn poll_and_dispatch(&self, transport_type: TransportType) {
        if let Some(raw) = transport::recv(transport_type, 100) {
            if let Some(response) = self.dispatch(&raw) {
                // Echo back to sender — destination IP would need to be tracked
                // For now, just log that a dispatch occurred
                let _ = response;
            }
        }
    }
}

impl Default for ProtobufDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to create a boxed handler from a typed closure.
pub fn make_handler<M, R, F>(handler: F) -> &'static BoxedHandler
where
    M: ProstMessage + Default + 'static,
    R: ProstMessage + 'static,
    F: Fn(&M) -> Option<R> + 'static,
{
    // This requires a static allocation; for simplicity, we use a wrapper
    // that stores the handler in a static location.
    // For production use, consider a proper static allocator.
    unimplemented!("Use register_handler with a static boxed closure")
}

/// Simple handler for VmActionRequest that returns a default response.
pub fn handle_vm_action_request(bytes: &[u8]) -> Option<Vec<u8>> {
    use crate::network::generated::external::VmActionRequest;
    use crate::network::generated::external::VmActionResponse;
    use alloc::string::ToString;

    let request = VmActionRequest::decode(bytes).ok()?;
    let mut response = VmActionResponse {
        request_id: request.request_id.clone(),
        identity: request.identity,
        vm_id: 0,
        success: true,
        error: String::new(),
        output: Vec::new(),
    };
    let mut buf = Vec::new();
    response.encode(&mut buf).ok()?;
    Some(buf)
}

/// Simple handler for IPCRequest that returns a default response.
pub fn handle_ipc_request(bytes: &[u8]) -> Option<Vec<u8>> {
    use crate::network::generated::internal::IpcRequest as IPCRequest;
    use crate::network::generated::internal::IpcResponse as IPCResponse;
    use alloc::string::ToString;

    let request = IPCRequest::decode(bytes).ok()?;
    let mut response = IPCResponse {
        request_id: request.request_id.clone(),
        tid: request.tid.clone(),
        instance_uuid: request.instance_uuid.clone(),
        vm_id: 0,
        output: Vec::new(),
        error: String::new(),
        success: true,
    };
    let mut buf = Vec::new();
    response.encode(&mut buf).ok()?;
    Some(buf)
}

/// Simple handler for VDispatchKernelCommand that returns a default response.
pub fn handle_kernel_command(bytes: &[u8]) -> Option<Vec<u8>> {
    use crate::network::generated::internal::safe::VDispatchKernelCommand;
    use crate::network::generated::internal::safe::KernelIpc as KernelIPC;
    use alloc::string::ToString;

    let request = VDispatchKernelCommand::decode(bytes).ok()?;
    let mut response = KernelIPC {
        version: request.version,
        checksum: 0,
        metadata: String::new(),
        flags: 0,
        command: request.command.clone(),
        priority: request.priority,
        content: String::new(),
        sdata: Vec::new(),
        pdata: Vec::new(),
        request_id: request.request_id.clone(),
        success: true,
        error: String::new(),
        output: Vec::new(),
        sequence: request.sequence,
        auth_tag: Vec::new(),
    };
    let mut buf = Vec::new();
    response.encode(&mut buf).ok()?;
    Some(buf)
}
