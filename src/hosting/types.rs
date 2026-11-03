use alloc::collections::VecDeque;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use crate::vdebug_autoprefix;

/// HPVMx operation mode determines how VM dispatch works.
/// - Standalone: single instance, local dispatch only.
/// - Node: receives RPC commands from a Manager, dispatches locally.
/// - Manager: maintains a registry of Nodes, routes deploy/signal requests via RPC.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationMode {
    Standalone,
    Node,
    Manager,
}

impl core::fmt::Display for OperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            OperationMode::Standalone => write!(f, "Standalone"),
            OperationMode::Node => write!(f, "Node"),
            OperationMode::Manager => write!(f, "Manager"),
        }
    }
}

/// Describes a registered node in Manager mode.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Unique node identifier.
    pub node_id: String,
    /// Node IP address (e.g. "192.168.1.10").
    pub ip: String,
    /// MSMI RPC port on the node (default 5001).
    pub port: u16,
    /// Whether the node is currently reachable.
    pub online: bool,
    /// Last heartbeat timestamp in milliseconds.
    pub last_heartbeat_ms: u64,
    /// Total capacity in CPU cores available on this node.
    pub cpu_cores: u32,
    /// Total capacity in memory (MB) available on this node.
    pub memory_mb: u64,
    /// Set of instance UUIDs currently deployed on this node.
    pub deployed_instances: Vec<String>,
}

impl NodeInfo {
    pub fn new(node_id: &str, ip: &str, port: u16) -> Self {
        Self {
            node_id: String::from(node_id),
            ip: String::from(ip),
            port,
            online: false,
            last_heartbeat_ms: 0,
            cpu_cores: 0,
            memory_mb: 0,
            deployed_instances: Vec::new(),
        }
    }

    /// Check if the node is considered online within the given timeout window.
    pub fn is_alive(&self, now_ms: u64, timeout_ms: u64) -> bool {
        self.online && (now_ms - self.last_heartbeat_ms <= timeout_ms)
    }

    /// Check available capacity for a new instance.
    pub fn has_capacity(&self, required_cpu: u32, required_mem_mb: u64) -> bool {
        self.cpu_cores >= required_cpu && self.memory_mb >= required_mem_mb
    }
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentType {
    VM,
    Container,
    CVM,
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandAction {
    Start,
    Stop,
    Restart,
    Create,
    Delete,
    Terminal,
    LoadISO { iso_path: String },
}

/// Identifies an instance using the diagram's `TID.UUID` format (e.g. `0000..9999.a1b2-c3d4-e5f6`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TidUuid {
    pub tid: String,
    pub uuid: String,
}

impl TidUuid {
    pub fn new(tid: &str, uuid: &str) -> Self {
        Self {
            tid: String::from(tid),
            uuid: String::from(uuid),
        }
    }

    /// Parse a combined `TID.UUID` string.
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        if let Some((tid_part, uuid_part)) = s.split_once('.') {
            if tid_part.is_empty() || uuid_part.is_empty() {
                return Err("invalid TID.UUID components");
            }
            Ok(Self::new(tid_part, uuid_part))
        } else {
            // Default fallback: treat whole string as UUID with default tenant "0000"
            if s.is_empty() {
                return Err("empty identifier");
            }
            Ok(Self::new("0000", s))
        }
    }

    pub fn formatted(&self) -> String {
        alloc::format!("{}.{}", self.tid, self.uuid)
    }
}

/// Client verification data stored in the Client Data LUT.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientData {
    pub client_token: String,
    pub verification_id: String,
    pub tid: String,
    pub inst_uuid: String,
    pub created_at_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IPCHeader {
    pub rid: String,
    pub tid: String,
    pub inst_uuid: String,
    pub client_token: Option<String>,
    pub verification_id: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalPayload {
    pub session_id: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum InstancePayload {
    CfgPayload(ConfigPayload),
    TerminalData(TerminalPayload),
    /// Lifecycle actions normally have no configuration to carry.
    Empty,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigPayload {
    pub cpu_cores: u32,
    pub memory_size: u64,
    pub storage_mb: u64,

    pub portmappings: Vec<(u16, u16)>, // (host_port, guest_port)

    pub custom_os_cfg: Option<CustomOsConfig>,

    pub metadata: Option<HashMap<String, String>>,

    pub assigned_ip: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomOsConfig {
    pub os_type: String,
    pub os_version: String,
    pub bootloader: Option<String>,
    pub kernel_path: Option<String>,
    pub initrd_path: Option<String>,
    pub cmdline_args: Option<String>,
    pub virtio_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IPCMessage {
    pub request: IPCHeader,
    pub environment: EnvironmentType,
    pub action: CommandAction,
    pub payload: InstancePayload,
    pub extra_data: Option<HashMap<String, String>>,
}

impl IPCMessage {
    pub fn new(request: IPCHeader, action: CommandAction, payload: InstancePayload) -> Self {
        Self {
            request,
            environment: EnvironmentType::VM,
            action,
            payload,
            extra_data: None,
        }
    }

    pub fn with_environment(
        request: IPCHeader,
        environment: EnvironmentType,
        action: CommandAction,
        payload: InstancePayload,
    ) -> Self {
        Self {
            request,
            environment,
            action,
            payload,
            extra_data: None,
        }
    }
}

/// VirtCOM Ring Buffer for Guest VM <-> Action Processor <-> WebSocket Console bridging.
#[derive(Debug, Clone)]
pub struct VirtComRingBuf {
    pub capacity: usize,
    pub in_buf: VecDeque<u8>,
    pub out_buf: VecDeque<u8>,
}

impl VirtComRingBuf {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            in_buf: VecDeque::with_capacity(capacity),
            out_buf: VecDeque::with_capacity(capacity),
        }
    }

    pub fn write_in(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            if self.in_buf.len() < self.capacity {
                self.in_buf.push_back(b);
                written += 1;
            } else {
                break;
            }
        }
        vdebug_autoprefix!("VirtCOM IN_BUF wrote {} bytes (queued: {})", written, self.in_buf.len());
        written
    }

    pub fn read_in(&mut self, max: usize) -> Vec<u8> {
        let count = core::cmp::min(max, self.in_buf.len());
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(b) = self.in_buf.pop_front() {
                out.push(b);
            }
        }
        out
    }

    pub fn write_out(&mut self, data: &[u8]) -> usize {
        let mut written = 0;
        for &b in data {
            if self.out_buf.len() < self.capacity {
                self.out_buf.push_back(b);
                written += 1;
            } else {
                break;
            }
        }
        vdebug_autoprefix!("VirtCOM OUT_BUF wrote {} bytes (queued: {})", written, self.out_buf.len());
        written
    }

    pub fn read_out(&mut self, max: usize) -> Vec<u8> {
        let count = core::cmp::min(max, self.out_buf.len());
        let mut out = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(b) = self.out_buf.pop_front() {
                out.push(b);
            }
        }
        out
    }
}

pub fn unpack_message(buffer: &[u8]) -> Result<IPCMessage, String> {
    vdebug_autoprefix!("Unpacking binary IPC message ({} bytes)", buffer.len());
    let msg: IPCMessage = postcard::from_bytes(buffer).map_err(|e| e.to_string())?;
    Ok(msg)
}

pub fn pack_message(msg: &IPCMessage) -> Result<Vec<u8>, String> {
    vdebug_autoprefix!("Packing binary IPC message for {:?}", msg.action);
    let serialized: Vec<u8> = postcard::to_allocvec(msg).map_err(|e| e.to_string())?;
    Ok(serialized)
}

/// Decode a REST/WebSocket JSON request body at the host boundary.
pub fn unpack_json_message(buffer: &[u8]) -> Result<IPCMessage, String> {
    vdebug_autoprefix!("Unpacking JSON IPC message ({} bytes)", buffer.len());
    serde_json_core::from_slice(buffer)
        .map(|(message, _remainder)| message)
        .map_err(|error| error.to_string())
}
