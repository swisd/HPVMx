//! RPC server for MSMI (Hypervisor Management Interface) protocol.
//!
//! Provides protobuf message processing for VmActionRequest/VmActionResponse.
//! TCP state machine lives in `net_stack::handle_rpcd`; this module handles
//! the application-layer protobuf decode → hosting subsystem → encode flow.
//!
//! Wire format (per message):
//!   [1-byte flags][4-byte big-endian length][protobuf payload]

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use prost::Message as ProstMessage;

use crate::hosting::interface::HostVmError;
use crate::hosting::types::{
    CommandAction, ConfigPayload, CustomOsConfig, EnvironmentType, IPCHeader, IPCMessage,
    InstancePayload, TerminalPayload,
};
use crate::network::generated::external::{
    self as proto, VmActionRequest, VmActionResponse, VmIdentity,
    ServerStatsRequest, ServerStatsResponse, NodeStatsRequest, NodeStatsResponse,
    CpuStats, MemoryStats, NetworkStats, DiskStats, VirtualizationStats, NodeStats, GpuStats,
};
use crate::vdebug_autoprefix;

// ---------------------------------------------------------------------------
// Conversion: protobuf → IPC
// ---------------------------------------------------------------------------

/// Convert a protobuf VmActionRequest into an hosting IPCMessage.
fn proto_request_to_ipc(request: &VmActionRequest) -> Result<IPCMessage, String> {
    let env = match request.environment {
        1 => EnvironmentType::VM,
        2 => EnvironmentType::Container,
        3 => EnvironmentType::CVM,
        _ => EnvironmentType::VM,
    };

    let action = match request.action {
        1 => CommandAction::Create,
        2 => CommandAction::Start,
        3 => CommandAction::Stop,
        4 => CommandAction::Restart,
        5 => CommandAction::Delete,
        6 => CommandAction::Terminal,
        7 => CommandAction::LoadISO {
            iso_path: String::new(),
        },
        _ => {
            return Err(format!("unknown action: {}", request.action));
        }
    };

    let identity = request.identity.as_ref();
    let tid = identity.map(|i| i.tid.clone()).unwrap_or_default();
    let instance_uuid = identity.map(|i| i.instance_uuid.clone()).unwrap_or_default();

    let header = IPCHeader {
        rid: request.request_id.clone(),
        tid,
        inst_uuid: instance_uuid,
        client_token: if request.client_token.is_empty() {
            None
        } else {
            Some(request.client_token.clone())
        },
        verification_id: if request.verification_id.is_empty() {
            None
        } else {
            Some(request.verification_id.clone())
        },
    };

    let payload = match &request.action_payload {
        Some(proto::vm_action_request::ActionPayload::Config(cfg)) => {
            let portmappings = cfg
                .port_mappings
                .iter()
                .map(|pm| (pm.host_port as u16, pm.guest_port as u16))
                .collect();

            let custom_os = cfg.custom_os.as_ref().map(|co| CustomOsConfig {
                os_type: co.os_type.clone(),
                os_version: co.os_version.clone(),
                bootloader: if co.bootloader.is_empty() { None } else { Some(co.bootloader.clone()) },
                kernel_path: if co.kernel_path.is_empty() { None } else { Some(co.kernel_path.clone()) },
                initrd_path: if co.initrd_path.is_empty() { None } else { Some(co.initrd_path.clone()) },
                cmdline_args: if co.cmdline_args.is_empty() { None } else { Some(co.cmdline_args.clone()) },
                virtio_enabled: co.virtio_enabled,
            });

            let metadata = if cfg.metadata.is_empty() {
                None
            } else {
                Some(
                    cfg.metadata
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                )
            };

            let assigned_ip = if cfg.assigned_ip.is_empty() {
                None
            } else {
                Some(cfg.assigned_ip.clone())
            };

            InstancePayload::CfgPayload(ConfigPayload {
                cpu_cores: cfg.cpu_cores,
                memory_size: cfg.memory_size_mb as u64,
                storage_mb: cfg.storage_size_mb,
                portmappings,
                custom_os_cfg: custom_os,
                metadata,
                assigned_ip,
            })
        }
        Some(proto::vm_action_request::ActionPayload::Terminal(term)) => {
            InstancePayload::TerminalData(TerminalPayload {
                session_id: term.session_id.clone(),
                data: term.data.clone(),
            })
        }
        Some(proto::vm_action_request::ActionPayload::IsoPath(path)) => {
            // For LoadISO, we need to set the iso_path in the action
            // The payload is Empty, but the action carries the path
            InstancePayload::Empty
        }
        None => InstancePayload::Empty,
    };

    // If LoadISO with a path, update the action
    let action = if matches!(action, CommandAction::LoadISO { .. }) {
        if let Some(proto::vm_action_request::ActionPayload::IsoPath(path)) = &request.action_payload {
            CommandAction::LoadISO {
                iso_path: path.clone(),
            }
        } else {
            action
        }
    } else {
        action
    };

    let mut msg = IPCMessage::new(header, action, payload);
    msg.environment = env;

    if !request.extra_data.is_empty() {
        msg.extra_data = Some(
            request
                .extra_data
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        );
    }

    Ok(msg)
}

// ---------------------------------------------------------------------------
// Public API: process_proto_payload
// ---------------------------------------------------------------------------

/// Process a single protobuf payload and return the encoded response bytes.
/// This function is called from `net_stack::handle_rpcd` after the TCP layer
/// has reassembled a complete length-prefixed frame.
///
/// Supports multiple message types dispatched by the first byte of the payload:
/// - VmActionRequest (tag 11 = 0x0B): standard VM lifecycle actions
/// - ServerStatsRequest (tag 1 = 0x08): request server-wide stats
/// - NodeStatsRequest (tag 1 = 0x08 with node_id): request specific node stats
///
/// The processor is accessed via a callback registered by `register_processor_callback`.
pub fn process_proto_payload(payload: &[u8]) -> Option<Vec<u8>> {
    if payload.is_empty() {
        return None;
    }

    // Check the first wire tag to determine message type
    // VmActionRequest: field 1 (request_id) has wire tag 0x0A (1 << 3 | 2)
    // ServerStatsRequest: field 1 (request_id) has wire tag 0x0A
    // NodeStatsRequest: field 1 (request_id) has wire tag 0x0A
    // Since all start with the same tag, we try decoding in order of specificity

    // Try ServerStatsRequest first (has only request_id field)
    if let Ok(stats_req) = ServerStatsRequest::decode(payload) {
        vdebug_autoprefix!(
            "RPC: decoded ServerStatsRequest (id={})",
            stats_req.request_id
        );
        let response = collect_server_stats(&stats_req.request_id);
        let mut buf = Vec::new();
        if response.encode(&mut buf).is_ok() {
            return Some(buf);
        }
    }

    // Try NodeStatsRequest (has request_id + node_id)
    if let Ok(node_req) = NodeStatsRequest::decode(payload) {
        if !node_req.node_id.is_empty() {
            vdebug_autoprefix!(
                "RPC: decoded NodeStatsRequest (id={}, node={})",
                node_req.request_id,
                node_req.node_id
            );
            let response = collect_node_stats(&node_req.request_id, &node_req.node_id);
            let mut buf = Vec::new();
            if response.encode(&mut buf).is_ok() {
                return Some(buf);
            }
        }
    }

    // Default: VmActionRequest
    let request = VmActionRequest::decode(payload).ok()?;
    vdebug_autoprefix!(
        "RPC: decoded VmActionRequest (id={}, action={}, env={})",
        request.request_id,
        request.action,
        request.environment
    );

    // Handle stats actions via VmActionRequest
    if request.action == 8 || request.action == 10 {
        // GET_STATS or GET_SERVER_STATS
        vdebug_autoprefix!("RPC: handling GET_STATS action");
        let response = collect_server_stats(&request.request_id);
        let mut buf = Vec::new();
        response.encode(&mut buf).ok()?;
        return Some(buf);
    }
    if request.action == 9 {
        // GET_NODE_STATS
        let node_id = request.identity.as_ref()
            .map(|i| i.tid.clone())
            .unwrap_or_default();
        vdebug_autoprefix!("RPC: handling GET_NODE_STATS for node {}", node_id);
        let response = collect_node_stats(&request.request_id, &node_id);
        let mut buf = Vec::new();
        response.encode(&mut buf).ok()?;
        return Some(buf);
    }

    let identity = request.identity.clone();

    // Convert to IPC message
    let ipc_msg = match proto_request_to_ipc(&request) {
        Ok(msg) => msg,
        Err(e) => {
            vdebug_autoprefix!("RPC: failed to convert to IPC: {}", e);
            let response = VmActionResponse {
                request_id: request.request_id.clone(),
                identity,
                vm_id: 0,
                success: false,
                error: e,
                output: Vec::new(),
            };
            let mut buf = Vec::new();
            response.encode(&mut buf).ok()?;
            return Some(buf);
        }
    };

    // Call the registered processor callback
    let callback = unsafe { PROCESSOR_CALLBACK };
    if let Some(cb) = callback {
        let result = cb(&ipc_msg);
        match result {
            Ok(req_result) => {
                vdebug_autoprefix!(
                    "RPC: processed successfully (vm_id={})",
                    req_result.vm_id
                );
                let response = VmActionResponse {
                    request_id: request.request_id,
                    identity,
                    vm_id: req_result.vm_id,
                    success: true,
                    error: String::new(),
                    output: req_result.output.unwrap_or_default(),
                };
                let mut buf = Vec::new();
                response.encode(&mut buf).ok()?;
                Some(buf)
            }
            Err(e) => {
                let error_str = match e {
                    HostVmError::InvalidConfiguration(s) => s.to_string(),
                    HostVmError::Backend(s) => s,
                };
                vdebug_autoprefix!("RPC: processing failed: {}", error_str);
                let response = VmActionResponse {
                    request_id: request.request_id,
                    identity,
                    vm_id: 0,
                    success: false,
                    error: error_str,
                    output: Vec::new(),
                };
                let mut buf = Vec::new();
                response.encode(&mut buf).ok()?;
                Some(buf)
            }
        }
    } else {
        vdebug_autoprefix!("RPC: no processor registered, returning error");
        let response = VmActionResponse {
            request_id: request.request_id,
            identity,
            vm_id: 0,
            success: false,
            error: String::from("RPC processor not initialized"),
            output: Vec::new(),
        };
        let mut buf = Vec::new();
        response.encode(&mut buf).ok()?;
        Some(buf)
    }
}

// ---------------------------------------------------------------------------
// Stats collection helpers
// ---------------------------------------------------------------------------

/// Collect server-wide stats and return a ServerStatsResponse.
fn collect_server_stats(request_id: &str) -> ServerStatsResponse {
    let hostname = crate::SYSTEMNAME;
    let version = env!("CARGO_PKG_VERSION");

    // Always read CPU info from CpuInfo::get() — it's fast and always available
    let cpu_info = crate::hardware::cpu::CpuInfo::get();
    let cpu_cores = cpu_info.cores.max(1);
    let cpu_threads = cpu_info.threads.max(1);
    let cpu_brand = cpu_info.brand;
    let cpu_clock_mhz = unsafe { crate::TSC_PER_US as u32 };

    // Read real-time stats from GLOBAL_SYSTEM_TRACKER (same source as HTTP /usage endpoint)
    let (cpu_usage, total_mem_mb, used_mem_mb, mem_usage_pct,
         gpu_device_name, gpu_usage, gpu_fps,
         net_rx_kbps, net_tx_kbps, disk_read_kbps, disk_write_pct) = unsafe {
        if let Some(tracker) = crate::GLOBAL_SYSTEM_TRACKER.as_ref() {
            (
                tracker.cpu_usage,
                tracker.total_memory_mb as u64,
                tracker.used_memory_mb as u64,
                if tracker.total_memory_mb > 0 {
                    (tracker.used_memory_mb * 100 / tracker.total_memory_mb)
                } else { 0 },
                alloc::format!("UEFI GOP Display Adapter"),
                tracker.gpu_usage,
                0u32,
                tracker.net_rx_kbps,
                tracker.net_tx_kbps,
                tracker.disk_read_kbps,
                tracker.disk_usage_percent,
            )
        } else {
            (
                0u32,
                crate::get_total_physical_memory_mb() as u64,
                0u64,
                0u32,
                alloc::format!("UEFI GOP Display Adapter"),
                0u32,
                0u32,
                0u64,
                0u64,
                0u64,
                0u32,
            )
        }
    };

    // Network stats
    let net_stats = crate::devices::net_stack::stats();
    let net_state = crate::devices::net_stack::get_state();
    let ip_str = alloc::format!("{}.{}.{}.{}",
        net_state.ip_addr[0], net_state.ip_addr[1], net_state.ip_addr[2], net_state.ip_addr[3]);
    let mac_str = alloc::format!("{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        net_state.mac_addr[0], net_state.mac_addr[1], net_state.mac_addr[2],
        net_state.mac_addr[3], net_state.mac_addr[4], net_state.mac_addr[5]);
    let gw_str = alloc::format!("{}.{}.{}.{}",
        net_state.gateway[0], net_state.gateway[1], net_state.gateway[2], net_state.gateway[3]);
    let mask_str = alloc::format!("{}.{}.{}.{}",
        net_state.subnet_mask[0], net_state.subnet_mask[1], net_state.subnet_mask[2], net_state.subnet_mask[3]);

    // Disk stats
    let (_disk_r_bytes, _disk_w_bytes, _disk_r_ops, _disk_w_ops) = crate::filesystem::disk_stats();
    let disk_usage_pct = disk_write_pct; // Already set from tracker above

    // VM stats
    let (vm_count, vm_running) = unsafe {
        if let Some(hv) = crate::HYPERVISOR.as_ref() {
            let vms = hv.list_vms();
            let total = vms.len();
            let running = vms.iter().filter(|(_, _, s)| s.to_string().contains("running")).count();
            (total as u32, running as u32)
        } else {
            (0, 0)
        }
    };

    let backend = crate::devices::net_stack::backend_name();

    // Collect node stats if in Manager mode
    let nodes = collect_node_stats_list();

    ServerStatsResponse {
        request_id: String::from(request_id),
        success: true,
        error: String::new(),
        hostname: String::from(hostname),
        version: String::from(version),
        time: String::from("unknown"),
        uptime_seconds: 0,
        cpu: Some(CpuStats {
            cores: cpu_cores as u32,
            usage_percent: cpu_usage,
            threads: cpu_threads,
            brand: cpu_brand,
            clock_mhz: cpu_clock_mhz,
        }),
        memory: Some(MemoryStats {
            total_mb: total_mem_mb,
            used_mb: used_mem_mb,
            usage_percent: mem_usage_pct,
        }),
        network: Some(NetworkStats {
            ip: ip_str,
            mac: mac_str,
            gateway: gw_str,
            subnet_mask: mask_str,
            rx_kbps: net_rx_kbps,
            tx_kbps: net_tx_kbps,
            rx_packets: net_stats.rx_pkts,
            tx_packets: net_stats.tx_pkts,
            rx_bytes: net_stats.rx_bytes,
            tx_bytes: net_stats.tx_bytes,
            usage_percent: 0,
        }),
        disk: Some(DiskStats {
            read_kbps: disk_read_kbps,
            write_kbps: 0,
            read_bytes: _disk_r_bytes as u64,
            write_bytes: _disk_w_bytes as u64,
            read_ops: _disk_r_ops as u64,
            write_ops: _disk_w_ops as u64,
            usage_percent: disk_usage_pct,
        }),
        virtualization: Some(VirtualizationStats {
            vms_running: vm_running,
            vms_total: vm_count,
            hypervisor: String::from(backend),
        }),
        nodes,
        gpu: Some(GpuStats {
            device_name: gpu_device_name,
            usage_percent: gpu_usage,
            fps: gpu_fps,
        }),
    }
}

/// Collect stats for a specific node by querying it via RPC.
fn collect_node_stats(request_id: &str, node_id: &str) -> NodeStatsResponse {
    // Try to find the node in the registry
    let dispatcher = unsafe { crate::VM_DISPATCHER.as_ref() };
    if let Some(dispatcher) = dispatcher {
        let registry = dispatcher.registry();
        if let Some(node) = registry.get_node(node_id) {
            // If the node is online, try to query it via RPC
            let stats = if node.online {
                query_node_stats_via_rpc(node).unwrap_or_else(|| build_local_node_stats(node))
            } else {
                build_local_node_stats(node)
            };
            return NodeStatsResponse {
                request_id: String::from(request_id),
                success: true,
                error: String::new(),
                stats: Some(stats),
            };
        }
    }

    NodeStatsResponse {
        request_id: String::from(request_id),
        success: false,
        error: alloc::format!("node '{}' not found", node_id),
        stats: None,
    }
}

/// Build NodeStats from local registry data (no RPC query).
fn build_local_node_stats(node: &crate::hosting::types::NodeInfo) -> NodeStats {
    NodeStats {
        node_id: node.node_id.clone(),
        online: node.online,
        last_heartbeat_ms: node.last_heartbeat_ms,
        cpu: Some(CpuStats {
            cores: node.cpu_cores,
            usage_percent: 0,
            threads: 0,
            brand: String::new(),
            clock_mhz: 0,
        }),
        memory: Some(MemoryStats {
            total_mb: node.memory_mb,
            used_mb: 0,
            usage_percent: 0,
        }),
        network: None,
        disk: None,
        virtualization: None,
        deployed_instances: node.deployed_instances.len() as u32,
    }
}

/// Query a remote node for its stats via RPC.
fn query_node_stats_via_rpc(node: &crate::hosting::types::NodeInfo) -> Option<NodeStats> {
    use crate::network::rpc_client;

    let request = VmActionRequest {
        request_id: String::from("stats-query"),
        identity: None,
        client_token: String::new(),
        verification_id: String::new(),
        environment: 0,
        action: 10, // GET_SERVER_STATS
        extra_data: alloc::collections::BTreeMap::new(),
        action_payload: None,
    };

    // Send RPC request to the node
    let response = rpc_client::send_request_to_node(node, &request, 2000)?;

    if !response.success {
        vdebug_autoprefix!("RPC: node {} stats query failed: {}", node.node_id, response.error);
        return None;
    }

    // The response output contains the ServerStatsResponse protobuf
    if let Ok(server_stats) = ServerStatsResponse::decode(&*response.output) {
        let cpu = server_stats.cpu.unwrap_or(CpuStats { cores: 0, usage_percent: 0, threads: 0, brand: String::new(), clock_mhz: 0 });
        let memory = server_stats.memory.unwrap_or(MemoryStats { total_mb: 0, used_mb: 0, usage_percent: 0 });
        let network = server_stats.network.unwrap_or(NetworkStats {
            ip: String::new(), mac: String::new(), gateway: String::new(),
            subnet_mask: String::new(), rx_kbps: 0, tx_kbps: 0,
            rx_packets: 0, tx_packets: 0, rx_bytes: 0, tx_bytes: 0, usage_percent: 0,
        });
        let disk = server_stats.disk.unwrap_or(DiskStats {
            read_kbps: 0, write_kbps: 0, read_bytes: 0, write_bytes: 0,
            read_ops: 0, write_ops: 0, usage_percent: 0,
        });
        let virtualization = server_stats.virtualization.unwrap_or(VirtualizationStats {
            vms_running: 0, vms_total: 0, hypervisor: String::new(),
        });
        let vms_total = virtualization.vms_total;

        Some(NodeStats {
            node_id: node.node_id.clone(),
            online: true,
            last_heartbeat_ms: node.last_heartbeat_ms,
            cpu: Some(cpu),
            memory: Some(memory),
            network: Some(network),
            disk: Some(disk),
            virtualization: Some(virtualization),
            deployed_instances: vms_total,
        })
    } else {
        None
    }
}

/// Collect stats for all registered nodes.
fn collect_node_stats_list() -> Vec<NodeStats> {
    let dispatcher = unsafe { crate::VM_DISPATCHER.as_ref() };
    let mut nodes = Vec::new();

    if let Some(dispatcher) = dispatcher {
        let registry = dispatcher.registry();
        for node in registry.iter_nodes() {
            let stats = if node.online {
                query_node_stats_via_rpc(node).unwrap_or_else(|| build_local_node_stats(node))
            } else {
                build_local_node_stats(node)
            };
            nodes.push(stats);
        }
    }

    nodes
}

// ---------------------------------------------------------------------------
// Processor callback registration
// ---------------------------------------------------------------------------

/// Callback type for processing an IPCMessage and returning a RequestResult.
type ProcessorCallback = fn(&IPCMessage) -> Result<crate::hosting::rqproc::RequestResult, HostVmError>;

static mut PROCESSOR_CALLBACK: Option<ProcessorCallback> = None;

/// Register a callback that processes IPCMessages.
///
/// This is called from `main.rs` after the RequestProcessor is created.
/// The callback captures the processor by closing over it via a static
/// function that delegates to the global processor.
pub fn register_processor_callback(cb: ProcessorCallback) {
    unsafe {
        PROCESSOR_CALLBACK = Some(cb);
    }
    vdebug_autoprefix!("RPC: processor callback registered");
}

/// Check if the RPC processor callback is registered.
pub fn is_processor_ready() -> bool {
    unsafe { PROCESSOR_CALLBACK.is_some() }
}
