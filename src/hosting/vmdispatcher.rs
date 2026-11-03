//! VM Dispatcher: routes VM/CVM/Container deploy and signal requests.
//!
//! In **Node** or **Standalone** mode, the dispatcher processes requests locally
//! via the RequestProcessor. In **Manager** mode, it maintains a NodeRegistry
//! and forwards requests via RPC to the appropriate node.

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::hosting::interface::{HostVmBackend, HostVmError};
use crate::hosting::rqproc::{RequestProcessor, RequestResult};
use crate::hosting::types::{
    CommandAction, ConfigPayload, EnvironmentType, IPCMessage, InstancePayload, NodeInfo,
    OperationMode,
};
use crate::network::generated::external::{
    self as proto,
    vm_action_request::ActionPayload as ProtoActionPayload,
    VmConfig, CustomOsConfig as ProtoCustomOsConfig, PortMapping as ProtoPortMapping,
    VmIdentity,
};
use crate::network::rpc_client;
use crate::vdebug_autoprefix;

/// Default heartbeat timeout for node liveness checks (ms).
const NODE_HEARTBEAT_TIMEOUT_MS: u64 = 30_000;

/// Default RPC timeout for outbound requests to nodes (ms).
const RPC_TIMEOUT_MS: u32 = 5_000;

// ---------------------------------------------------------------------------
// NodeRegistry: tracks connected nodes in Manager mode
// ---------------------------------------------------------------------------

/// Registry of managed nodes.
pub struct NodeRegistry {
    nodes: HashMap<String, NodeInfo>,
    /// Maps instance UUID → node ID for deployed instances.
    instance_to_node: HashMap<String, String>,
}

impl NodeRegistry {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            instance_to_node: HashMap::new(),
        }
    }

    /// Register a new node.
    pub fn register_node(&mut self, node: NodeInfo) {
        let id = node.node_id.clone();
        self.nodes.insert(id.clone(), node);
        vdebug_autoprefix!("NodeRegistry: registered node {} at {}", id, id);
    }

    /// Remove a node from the registry.
    pub fn unregister_node(&mut self, node_id: &str) {
        if let Some(node) = self.nodes.remove(node_id) {
            // Clear instance mappings for this node
            self.instance_to_node.retain(|_, nid| nid != node_id);
            vdebug_autoprefix!("NodeRegistry: unregistered node {}", node_id);
        }
    }

    /// Update a node's heartbeat timestamp.
    pub fn heartbeat(&mut self, node_id: &str, now_ms: u64, cpu_cores: u32, memory_mb: u64) -> bool {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.online = true;
            node.last_heartbeat_ms = now_ms;
            node.cpu_cores = cpu_cores;
            node.memory_mb = memory_mb;
            true
        } else {
            false
        }
    }

    /// Check which nodes are currently alive.
    pub fn check_liveness(&mut self, now_ms: u64) {
        for (_, node) in self.nodes.iter_mut() {
            if !node.is_alive(now_ms, NODE_HEARTBEAT_TIMEOUT_MS) {
                node.online = false;
                vdebug_autoprefix!("NodeRegistry: node {} marked offline (heartbeat timeout)", node.node_id);
            }
        }
    }

    /// Find a node that has capacity for the given resource requirements.
    pub fn find_node_with_capacity(&self, required_cpu: u32, required_mem_mb: u64) -> Option<&NodeInfo> {
        self.nodes.values()
            .filter(|n| n.online && n.has_capacity(required_cpu, required_mem_mb))
            .min_by_key(|n| n.deployed_instances.len())
    }

    /// Assign an instance to a node.
    pub fn assign_instance(&mut self, instance_uuid: &str, node_id: &str) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.deployed_instances.push(String::from(instance_uuid));
        }
        self.instance_to_node.insert(String::from(instance_uuid), String::from(node_id));
        vdebug_autoprefix!(
            "NodeRegistry: assigned instance {} to node {}",
            instance_uuid,
            node_id
        );
    }

    /// Remove an instance assignment.
    pub fn unassign_instance(&mut self, instance_uuid: &str) {
        if let Some(node_id) = self.instance_to_node.remove(instance_uuid) {
            if let Some(node) = self.nodes.get_mut(&node_id) {
                node.deployed_instances.retain(|u| u != instance_uuid);
            }
            vdebug_autoprefix!(
                "NodeRegistry: unassigned instance {} from node {}",
                instance_uuid,
                node_id
            );
        }
    }

    /// Get the node ID for a deployed instance.
    pub fn get_node_for_instance(&self, instance_uuid: &str) -> Option<&String> {
        self.instance_to_node.get(instance_uuid)
    }

    /// Get a node by ID.
    pub fn get_node(&self, node_id: &str) -> Option<&NodeInfo> {
        self.nodes.get(node_id)
    }

    /// Iterate over all registered nodes.
    pub fn iter_nodes(&self) -> hashbrown::hash_map::Values<String, NodeInfo> {
        self.nodes.values()
    }

    /// Get a mutable node by ID.
    pub fn get_node_mut(&mut self, node_id: &str) -> Option<&mut NodeInfo> {
        self.nodes.get_mut(node_id)
    }

    /// List all nodes.
    pub fn list_nodes(&self) -> Vec<&NodeInfo> {
        self.nodes.values().collect()
    }

    /// Total number of registered nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

// ---------------------------------------------------------------------------
// VmDispatcher: mode-aware request routing
// ---------------------------------------------------------------------------

/// The VM dispatcher routes IPC messages either locally or to remote nodes.
pub struct VmDispatcher<B: HostVmBackend> {
    mode: OperationMode,
    processor: RequestProcessor<B>,
    registry: NodeRegistry,
}

impl<B: HostVmBackend> VmDispatcher<B> {
    pub fn new(mode: OperationMode, backend: B) -> Self {
        vdebug_autoprefix!("VmDispatcher: initialized in {} mode", mode);
        Self {
            mode,
            processor: RequestProcessor::new(backend),
            registry: NodeRegistry::new(),
        }
    }

    /// Get the current operation mode.
    pub fn mode(&self) -> OperationMode {
        self.mode
    }

    /// Switch the operation mode (e.g., from Standalone to Manager).
    pub fn set_mode(&mut self, mode: OperationMode) {
        vdebug_autoprefix!("VmDispatcher: switching from {} to {} mode", self.mode, mode);
        self.mode = mode;
    }

    /// Get a reference to the node registry.
    pub fn registry(&self) -> &NodeRegistry {
        &self.registry
    }

    /// Get a mutable reference to the node registry.
    pub fn registry_mut(&mut self) -> &mut NodeRegistry {
        &mut self.registry
    }

    /// Get the underlying processor reference.
    pub fn processor(&self) -> &RequestProcessor<B> {
        &self.processor
    }

    /// Get the underlying processor mutable reference.
    pub fn processor_mut(&mut self) -> &mut RequestProcessor<B> {
        &mut self.processor
    }

    /// Process a heartbeat from a node.
    pub fn node_heartbeat(&mut self, node_id: &str, now_ms: u64, cpu_cores: u32, memory_mb: u64) -> bool {
        self.registry.heartbeat(node_id, now_ms, cpu_cores, memory_mb)
    }

    /// Periodic liveness check for all registered nodes.
    pub fn check_node_liveness(&mut self, now_ms: u64) {
        self.registry.check_liveness(now_ms);
    }

    /// Register a node in the registry.
    pub fn register_node(&mut self, node: NodeInfo) {
        self.registry.register_node(node);
    }

    /// Unregister a node from the registry.
    pub fn unregister_node(&mut self, node_id: &str) {
        self.registry.unregister_node(node_id);
    }

    /// Core dispatch: routes the IPC message based on operation mode.
    pub fn dispatch(&mut self, message: &IPCMessage) -> Result<RequestResult, HostVmError> {
        vdebug_autoprefix!(
            "VmDispatcher: dispatching {:?} for {} in {} mode",
            message.action,
            message.request.inst_uuid,
            self.mode
        );

        match self.mode {
            OperationMode::Standalone | OperationMode::Node => {
                // Local dispatch: process directly via RequestProcessor
                self.processor.process(message)
            }
            OperationMode::Manager => {
                // Manager mode: route to the appropriate node via RPC
                self.dispatch_to_node(message)
            }
        }
    }

    /// Route an IPC message to the appropriate node via RPC (Manager mode only).
    fn dispatch_to_node(&mut self, message: &IPCMessage) -> Result<RequestResult, HostVmError> {
        let instance_uuid = &message.request.inst_uuid;

        // For Create actions, find a node with capacity
        let node_id = if message.action == CommandAction::Create {
            // Extract resource requirements from payload
            let (cpu, mem) = if let InstancePayload::CfgPayload(cfg) = &message.payload {
                (cfg.cpu_cores, cfg.memory_size)
            } else {
                (1, 256) // Default fallback
            };

            let node = self.registry.find_node_with_capacity(cpu, mem)
                .ok_or(HostVmError::InvalidConfiguration("no available node with sufficient capacity"))?;
            
            let nid = node.node_id.clone();
            self.registry.assign_instance(instance_uuid, &nid);
            Some(nid)
        } else {
            // For non-Create actions, look up the existing assignment
            self.registry.get_node_for_instance(instance_uuid)
                .cloned()
                .or_else(|| {
                    // If not found in registry, try to process locally as fallback
                    vdebug_autoprefix!(
                        "VmDispatcher: instance {} not found in registry, attempting local dispatch",
                        instance_uuid
                    );
                    None
                })
        };

        if let Some(node_id) = node_id {
            // Forward via RPC to the target node
            let node = self.registry.get_node(&node_id)
                .ok_or(HostVmError::InvalidConfiguration("node not found in registry"))?;

            if !node.online {
                return Err(HostVmError::InvalidConfiguration("target node is offline"));
            }

            // Convert IPC message to protobuf VmActionRequest
            let request = ipc_to_proto_request(message);
            
            vdebug_autoprefix!(
                "VmDispatcher: forwarding {:?} for {} to node {} ({})",
                message.action,
                instance_uuid,
                node_id,
                node.ip
            );

            // Send RPC request
            match rpc_client::send_request_to_node(node, &request, RPC_TIMEOUT_MS) {
                Some(response) => {
                    if response.success {
                        Ok(RequestResult {
                            tid: message.request.tid.clone(),
                            instance_uuid: message.request.inst_uuid.clone(),
                            vm_id: response.vm_id,
                            output: if response.output.is_empty() { None } else { Some(response.output) },
                        })
                    } else {
                        Err(HostVmError::Backend(response.error))
                    }
                }
                None => {
                    Err(HostVmError::Backend(String::from("RPC request to node timed out")))
                }
            }
        } else {
            // No node assignment found — fall back to local dispatch
            vdebug_autoprefix!(
                "VmDispatcher: no node assignment for {}, dispatching locally",
                instance_uuid
            );
            self.processor.process(message)
        }
    }

    /// Forward a signal (Start/Stop/Restart/Terminal/LoadISO) to the node hosting an instance.
    pub fn forward_signal(&mut self, instance_uuid: &str, action: CommandAction, payload: InstancePayload) -> Result<RequestResult, HostVmError> {
        let msg = IPCMessage::new(
            crate::hosting::types::IPCHeader {
                rid: alloc::format!("signal-{}", instance_uuid),
                tid: String::new(),
                inst_uuid: String::from(instance_uuid),
                client_token: None,
                verification_id: None,
            },
            action,
            payload,
        );
        self.dispatch(&msg)
    }

    /// Deploy a VM/CVM/Container to a specific node or auto-select.
    pub fn deploy(&mut self, instance_uuid: &str, env: EnvironmentType, cfg: ConfigPayload, target_node: Option<String>) -> Result<RequestResult, HostVmError> {
        let msg = IPCMessage::new(
            crate::hosting::types::IPCHeader {
                rid: alloc::format!("deploy-{}", instance_uuid),
                tid: String::new(),
                inst_uuid: String::from(instance_uuid),
                client_token: None,
                verification_id: None,
            },
            CommandAction::Create,
            InstancePayload::CfgPayload(cfg),
        );

        // If target node is specified, pre-assign
        if let Some(node_id) = &target_node {
            if let Some(node) = self.registry.get_node(node_id) {
                vdebug_autoprefix!("VmDispatcher: deploying {} to specified node {}", instance_uuid, node_id);
                self.registry.assign_instance(instance_uuid, node_id);
            }
        }

        // Create a message with the correct environment
        let mut msg = msg;
        msg.environment = env;
        self.dispatch(&msg)
    }
}

// ---------------------------------------------------------------------------
// Conversion: IPC → Protobuf (for RPC forwarding)
// ---------------------------------------------------------------------------

/// Convert an IPCMessage to a protobuf VmActionRequest for RPC forwarding.
fn ipc_to_proto_request(msg: &IPCMessage) -> proto::VmActionRequest {
    let env = match msg.environment {
        EnvironmentType::VM => 1,
        EnvironmentType::Container => 2,
        EnvironmentType::CVM => 3,
    };

    let action = match &msg.action {
        CommandAction::Create => 1,
        CommandAction::Start => 2,
        CommandAction::Stop => 3,
        CommandAction::Restart => 4,
        CommandAction::Delete => 5,
        CommandAction::Terminal => 6,
        CommandAction::LoadISO { .. } => 7,
    };

    let identity = Some(VmIdentity {
        tid: msg.request.tid.clone(),
        instance_uuid: msg.request.inst_uuid.clone(),
    });

    let payload = match &msg.payload {
        InstancePayload::CfgPayload(cfg) => {
            let port_mappings = cfg.portmappings.iter().map(|(h, g)| ProtoPortMapping {
                host_port: *h as u32,
                guest_port: *g as u32,
            }).collect();

            let custom_os = cfg.custom_os_cfg.as_ref().map(|co| ProtoCustomOsConfig {
                os_type: co.os_type.clone(),
                os_version: co.os_version.clone(),
                bootloader: co.bootloader.clone().unwrap_or_default(),
                kernel_path: co.kernel_path.clone().unwrap_or_default(),
                initrd_path: co.initrd_path.clone().unwrap_or_default(),
                cmdline_args: co.cmdline_args.clone().unwrap_or_default(),
                virtio_enabled: co.virtio_enabled,
            });

            let metadata: alloc::collections::BTreeMap<String, String> = cfg.metadata.as_ref()
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
            Some(ProtoActionPayload::Config(VmConfig {
                cpu_cores: cfg.cpu_cores,
                memory_size_mb: cfg.memory_size,
                storage_size_mb: cfg.storage_mb,
                port_mappings,
                custom_os,
                metadata,
                assigned_ip: cfg.assigned_ip.clone().unwrap_or_default(),
            }))
        }
        InstancePayload::TerminalData(term) => {
            Some(ProtoActionPayload::Terminal(
                crate::network::generated::external::TerminalInput {
                    session_id: term.session_id.clone(),
                    data: term.data.clone(),
                }
            ))
        }
        InstancePayload::Empty => None,
    };

    // For LoadISO, set the iso_path in the payload
    let payload = if let CommandAction::LoadISO { iso_path } = &msg.action {
        if !iso_path.is_empty() {
            Some(ProtoActionPayload::IsoPath(iso_path.clone()))
        } else {
            payload
        }
    } else {
        payload
    };

    let extra_data: alloc::collections::BTreeMap<String, String> = msg.extra_data.as_ref()
        .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        .unwrap_or_default();
    proto::VmActionRequest {
        request_id: msg.request.rid.clone(),
        identity,
        client_token: msg.request.client_token.clone().unwrap_or_default(),
        verification_id: msg.request.verification_id.clone().unwrap_or_default(),
        environment: env,
        action,
        extra_data,
        action_payload: payload,
    }
}
