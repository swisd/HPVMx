//! VM port: guard host requests before they reach the VMM dispatcher,
//! and Shared Memory (SM) RingBuf IPC between Hypervisor Kernel / Host Server and VM Dispatcher / Guard.

use alloc::collections::VecDeque;
use alloc::string::String;
use crate::hosting::interface::{HostVmBackend, HostVmError};
use crate::hosting::rqproc::{RequestProcessor, RequestResult};
use crate::hosting::types::IPCMessage;
use crate::vdebug_autoprefix;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeartbeatSource {
    VisualizationServer,
    IdentityServer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GuardError {
    ReadOnly,
    MissingHeartbeatToken,
    InvalidHeartbeatToken,
    ExpiredHeartbeat,
}

pub struct VmGuard {
    timeout_ms: u64,
    visualization_token: String,
    identity_token: String,
    visualization: Option<u64>,
    identity: Option<u64>,
}

impl VmGuard {
    /// Tokens are provisioned by the host, not supplied by a guest VM.
    pub fn new(timeout_ms: u64, visualization_token: &str, identity_token: &str) -> Self {
        Self {
            timeout_ms,
            visualization_token: String::from(visualization_token),
            identity_token: String::from(identity_token),
            visualization: None,
            identity: None,
        }
    }

    pub fn heartbeat(&mut self, source: HeartbeatSource, token: &str, now_ms: u64) -> Result<(), GuardError> {
        vdebug_autoprefix!("VmGuard: heartbeat received from {:?} at {} ms", source, now_ms);
        if token.is_empty() {
            return Err(GuardError::MissingHeartbeatToken);
        }
        let expected = match source {
            HeartbeatSource::VisualizationServer => &self.visualization_token,
            HeartbeatSource::IdentityServer => &self.identity_token,
        };
        if expected.is_empty() || token != expected {
            vdebug_autoprefix!("VmGuard: invalid heartbeat token from {:?}", source);
            return Err(GuardError::InvalidHeartbeatToken);
        }
        match source {
            HeartbeatSource::VisualizationServer => self.visualization = Some(now_ms),
            HeartbeatSource::IdentityServer => self.identity = Some(now_ms),
        }
        Ok(())
    }

    pub fn is_writable(&self, now_ms: u64) -> bool {
        self.visualization.is_some_and(|seen| now_ms >= seen && now_ms - seen <= self.timeout_ms)
            && self.identity.is_some_and(|seen| now_ms >= seen && now_ms - seen <= self.timeout_ms)
    }

    pub fn authorize_mutation(&self, now_ms: u64) -> Result<(), GuardError> {
        if self.visualization.is_none() || self.identity.is_none() {
            vdebug_autoprefix!("VmGuard: entering READONLY mode (missing heartbeat from VS or IS)");
            return Err(GuardError::ReadOnly);
        }
        if self.is_writable(now_ms) {
            Ok(())
        } else {
            vdebug_autoprefix!("VmGuard: entering READONLY mode (heartbeat expired at {} ms)", now_ms);
            Err(GuardError::ExpiredHeartbeat)
        }
    }
}

/// Shared Memory (SM) RingBuf IPC queue connecting Host Server / Kernel and VM Dispatcher / Guard.
#[derive(Debug, Clone)]
pub struct SmRingBuf {
    capacity: usize,
    queue: VecDeque<IPCMessage>,
}

impl SmRingBuf {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            queue: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, message: IPCMessage) -> Result<(), &'static str> {
        if self.queue.len() >= self.capacity {
            vdebug_autoprefix!("SmRingBuf: queue full (capacity {})", self.capacity);
            return Err("SM RingBuf is full");
        }
        vdebug_autoprefix!("SmRingBuf: pushed IPC message {:?} for UUID {}", message.action, message.request.inst_uuid);
        self.queue.push_back(message);
        Ok(())
    }

    pub fn pop(&mut self) -> Option<IPCMessage> {
        let msg = self.queue.pop_front();
        if let Some(ref m) = msg {
            vdebug_autoprefix!("SmRingBuf: popped IPC message {:?}", m.action);
        }
        msg
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VmPortResponse {
    Accepted(RequestResult),
    Rejected(PortError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortError {
    Guard(GuardError),
    Backend(HostVmError),
}

pub struct VmPort<B> {
    guard: VmGuard,
    processor: RequestProcessor<B>,
    sm_ringbuf: SmRingBuf,
}

impl<B: HostVmBackend> VmPort<B> {
    pub fn new(backend: B, heartbeat_timeout_ms: u64, visualization_token: &str, identity_token: &str) -> Self {
        Self {
            guard: VmGuard::new(heartbeat_timeout_ms, visualization_token, identity_token),
            processor: RequestProcessor::new(backend),
            sm_ringbuf: SmRingBuf::new(128),
        }
    }

    pub fn heartbeat(&mut self, source: HeartbeatSource, token: &str, now_ms: u64) -> Result<(), GuardError> {
        self.guard.heartbeat(source, token, now_ms)
    }

    pub fn is_read_only(&self, now_ms: u64) -> bool {
        !self.guard.is_writable(now_ms)
    }

    pub fn dispatch(&mut self, message: &IPCMessage, now_ms: u64) -> VmPortResponse {
        vdebug_autoprefix!("VmPort: dispatching message {:?} (action: {:?})", message.request.rid, message.action);
        if let Err(error) = self.guard.authorize_mutation(now_ms) {
            return VmPortResponse::Rejected(PortError::Guard(error));
        }
        self.processor
            .process(message)
            .map(VmPortResponse::Accepted)
            .unwrap_or_else(|error| VmPortResponse::Rejected(PortError::Backend(error)))
    }

    pub fn enqueue_ipc(&mut self, message: IPCMessage) -> Result<(), &'static str> {
        self.sm_ringbuf.push(message)
    }

    pub fn process_queued_ipc(&mut self, now_ms: u64) -> Option<VmPortResponse> {
        let msg = self.sm_ringbuf.pop()?;
        Some(self.dispatch(&msg, now_ms))
    }

    pub fn processor(&self) -> &RequestProcessor<B> { &self.processor }
    pub fn processor_mut(&mut self) -> &mut RequestProcessor<B> { &mut self.processor }
    pub fn into_processor(self) -> RequestProcessor<B> { self.processor }
}

#[cfg(test)]
mod tests {
    use alloc::string::String;
    use alloc::vec;
    use super::*;
    use crate::hosting::interface::HostVmError;
    use crate::hosting::types::{CommandAction, ConfigPayload, IPCHeader, InstancePayload};

    #[derive(Default)]
    struct Backend {
        next: u32,
        calls: alloc::vec::Vec<&'static str>,
    }

    impl HostVmBackend for Backend {
        fn create(&mut self, _: &str, _: &ConfigPayload) -> Result<u32, HostVmError> {
            self.calls.push("create");
            self.next += 1;
            Ok(self.next)
        }
        fn start(&mut self, _: u32) -> Result<(), HostVmError> { self.calls.push("start"); Ok(()) }
        fn stop(&mut self, _: u32) -> Result<(), HostVmError> { self.calls.push("stop"); Ok(()) }
        fn delete(&mut self, _: u32) -> Result<(), HostVmError> { self.calls.push("delete"); Ok(()) }
        fn load_iso(&mut self, _: u32, _: &str) -> Result<(), HostVmError> { self.calls.push("iso"); Ok(()) }
    }

    fn request(action: CommandAction) -> IPCMessage {
        IPCMessage::new(
            IPCHeader {
                rid: String::from("r"),
                tid: String::from("0001"),
                inst_uuid: String::from("a12bc3d4-e5f6-7890-abcd-ef1234567890"),
                client_token: None,
                verification_id: None,
            },
            action,
            InstancePayload::CfgPayload(ConfigPayload {
                cpu_cores: 1,
                memory_size: 64,
                storage_mb: 0,
                portmappings: vec![],
                custom_os_cfg: None,
                metadata: None,
                assigned_ip: None,
            }),
        )
    }

    #[test]
    fn guard_requires_both_host_heartbeats_and_reverts_to_read_only() {
        let mut port = VmPort::new(Backend::default(), 10, "vs", "is");
        assert!(matches!(port.dispatch(&request(CommandAction::Create), 0), VmPortResponse::Rejected(PortError::Guard(GuardError::ReadOnly))));
        port.heartbeat(HeartbeatSource::VisualizationServer, "vs", 1).unwrap();
        port.heartbeat(HeartbeatSource::IdentityServer, "is", 1).unwrap();
        assert!(matches!(port.dispatch(&request(CommandAction::Create), 2), VmPortResponse::Accepted(_)));
        assert!(port.is_read_only(12));
    }
}
