//! Startup and Boot OS Test Suite
//!
//! Provides validation and testing routines for OS bootloader sequence,
//! kernel loading, environment initialization, VMM/CVM initialization,
//! hosting gateway bootstrap, and VM Guard heartbeat state machines.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use crate::vdebug_autoprefix;
use crate::env::{get_global_var, set_global_var};
use crate::hosting::lut::{CidLut, ClientDataLut, InstanceLut, PortLut};
use crate::hosting::rqproc::{RequestProcessor, RequestResult};
use crate::hosting::types::{
    ClientData, CommandAction, ConfigPayload, EnvironmentType, IPCHeader, IPCMessage, InstancePayload,
};
use crate::hosting::vmdispatch::{GuardError, HeartbeatSource, SmRingBuf, VmGuard, VmPort};
use crate::hosting::interface::{HostVmBackend, HostVmError};
use crate::vmm::container::{ContainerResources, ContainerSpec, ContainerState};
use crate::vmm::cvm::{ContainerVirtualMachine, CvmState};
use crate::vmm::cvmbus::CvmBusMessage;
use crate::vmm::interface::{PortIoInterface, PortOwner};
use crate::ui::{DashboardTab, DashboardUI, VirtSubTab, ContainerDisplayInfo, CvmDisplayInfo};
use crate::ui::tabui::virtualization::X_Virtualization;

/// Mock HostVmBackend for testing startup/boot hosting components.
struct MockBootVmBackend {
    created_vms: BTreeMap<String, u32>,
    next_id: u32,
}

impl MockBootVmBackend {
    fn new() -> Self {
        Self {
            created_vms: BTreeMap::new(),
            next_id: 1,
        }
    }
}

impl HostVmBackend for MockBootVmBackend {
    fn create(&mut self, instance_uuid: &str, config: &ConfigPayload) -> Result<u32, HostVmError> {
        if config.cpu_cores == 0 || config.memory_size == 0 {
            return Err(HostVmError::InvalidConfiguration("CPU and memory must be non-zero"));
        }
        let id = self.next_id;
        self.next_id += 1;
        self.created_vms.insert(instance_uuid.to_string(), id);
        vdebug_autoprefix!("MockBootVmBackend: Created VM {} for UUID {}", id, instance_uuid);
        Ok(id)
    }

    fn start(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("MockBootVmBackend: Started VM {}", vm_id);
        Ok(())
    }

    fn stop(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("MockBootVmBackend: Stopped VM {}", vm_id);
        Ok(())
    }

    fn delete(&mut self, vm_id: u32) -> Result<(), HostVmError> {
        vdebug_autoprefix!("MockBootVmBackend: Deleted VM {}", vm_id);
        Ok(())
    }

    fn load_iso(&mut self, vm_id: u32, iso_path: &str) -> Result<(), HostVmError> {
        vdebug_autoprefix!("MockBootVmBackend: Loaded ISO '{}' into VM {}", iso_path, vm_id);
        Ok(())
    }

    fn terminal_write(&mut self, _vm_id: u32, data: &[u8]) -> Result<usize, HostVmError> {
        vdebug_autoprefix!("MockBootVmBackend: VirtCOM write {} bytes", data.len());
        Ok(data.len())
    }

    fn terminal_read(&mut self, _vm_id: u32, _max_len: usize) -> Result<Vec<u8>, HostVmError> {
        Ok(b"BootOS v1.15.1 Ready\n".to_vec())
    }
}

/// Test 1: OS Environment and Global State Initialization
pub fn test_boot_environment() -> bool {
    vdebug_autoprefix!("Running test_boot_environment...");
    set_global_var("BOOT_STAGE", "INIT");
    set_global_var("SYSTEM_ARCH", "x86_64");
    set_global_var("UEFI_TARGET", "true");

    let stage = get_global_var("BOOT_STAGE");
    let arch = get_global_var("SYSTEM_ARCH");
    let uefi = get_global_var("UEFI_TARGET");

    let pass = stage.as_deref() == Some("INIT")
        && arch.as_deref() == Some("x86_64")
        && uefi.as_deref() == Some("true");

    if pass {
        vdebug_autoprefix!(10, "test_boot_environment: PASSED");
    } else {
        vdebug_autoprefix!(12, "test_boot_environment: FAILED");
    }
    pass
}

/// Test 2: CVM, CVMBus and Container Engine Boot Sequence
pub fn test_boot_cvm_and_containers() -> bool {
    vdebug_autoprefix!("Running test_boot_cvm_and_containers...");

    let mut cvm = ContainerVirtualMachine::new(100);
    if cvm.state != CvmState::Created {
        vdebug_autoprefix!("test_boot_cvm_and_containers: CVM state mismatch");
        return false;
    }

    if cvm.start().is_err() || cvm.state != CvmState::Running {
        vdebug_autoprefix!("test_boot_cvm_and_containers: CVM failed to enter Running state");
        return false;
    }

    // Initialize container inside CVM kernel via CVMBus
    let spec = ContainerSpec {
        name: "boot-init-container".to_string(),
        image: "core-system:v1".to_string(),
        resources: ContainerResources {
            memory_mb: 64,
            cpu_shares: 100,
        },
    };

    // Queue creation via CVMBus
    cvm.kernel.bus.send(CvmBusMessage::CreateContainer(spec));
    let service_res = cvm.kernel.service_one();
    let container_id = match service_res {
        Ok(Some(id)) => id,
        _ => {
            vdebug_autoprefix!("test_boot_cvm_and_containers: Failed to create container via CVMBus");
            return false;
        }
    };

    // Start container via CVMBus
    cvm.kernel.bus.send(CvmBusMessage::StartContainer(container_id));
    if let Err(e) = cvm.kernel.service_one() {
        vdebug_autoprefix!("test_boot_cvm_and_containers: Failed to start container via CVMBus: {:?}", e);
        return false;
    }

    let running = cvm.kernel.containers.get(container_id).map(|c| c.state) == Some(ContainerState::Running);
    if !running {
        vdebug_autoprefix!("test_boot_cvm_and_containers: Container not running");
        return false;
    }

    if cvm.stop().is_err() || cvm.state != CvmState::Stopped {
        vdebug_autoprefix!(12, "test_boot_cvm_and_containers: CVM stop failed");
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_cvm_and_containers: PASSED");
    true
}

/// Test 3: Hosting Subsystem LUT Resolution & Port Mapping
pub fn test_boot_hosting_luts() -> bool {
    vdebug_autoprefix!("Running test_boot_hosting_luts...");

    let mut instance_lut = InstanceLut::new();
    let mut port_lut = PortLut::new();
    let mut cid_lut = CidLut::new();
    let mut client_data_lut = ClientDataLut::new();

    let test_uuid = "a1b2-c3d4-e5f6";
    // Register instance
    if instance_lut.bind(test_uuid, 1).is_err() {
        vdebug_autoprefix!("test_boot_hosting_luts: InstanceLut bind failed");
        return false;
    }
    if instance_lut.resolve(test_uuid) != Some(1) {
        vdebug_autoprefix!("test_boot_hosting_luts: InstanceLut resolve failed");
        return false;
    }

    // Register Port mapping
    if port_lut.map_port(1, 8080, 80).is_err() {
        vdebug_autoprefix!("test_boot_hosting_luts: PortLut map_port failed");
        return false;
    }
    if port_lut.resolve_guest_port(8080) != Some((1, 80)) {
        vdebug_autoprefix!("test_boot_hosting_luts: PortLut lookup failed");
        return false;
    }
    if port_lut.resolve_host_port(1, 80) != Some(8080) {
        vdebug_autoprefix!("test_boot_hosting_luts: PortLut reverse lookup failed");
        return false;
    }

    // Register CID mapping
    cid_lut.bind_container(42, 1, test_uuid);
    if cid_lut.resolve_vm_id(42) != Some(1) || cid_lut.resolve_uuid(42) != Some(test_uuid) {
        vdebug_autoprefix!("test_boot_hosting_luts: CidLut lookup failed");
        return false;
    }

    // Register Client Data verification
    let client_data = ClientData {
        client_token: "token-secret".to_string(),
        verification_id: "verif-key-777".to_string(),
        tid: "0001".to_string(),
        inst_uuid: test_uuid.to_string(),
        created_at_ms: 1000,
    };
    client_data_lut.register(client_data);
    if client_data_lut.get_by_token("token-secret").is_none() {
        vdebug_autoprefix!("test_boot_hosting_luts: ClientDataLut token verification failed");
        return false;
    }
    if client_data_lut.get_by_verification_id("verif-key-777").is_none() {
        vdebug_autoprefix!("test_boot_hosting_luts: ClientDataLut verif_id verification failed");
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_hosting_luts: PASSED");
    true
}

/// Test 4: VM Guard Heartbeat and Gateway Authorization Lifecycle
pub fn test_boot_vmguard_lifecycle() -> bool {
    vdebug_autoprefix!("Running test_boot_vmguard_lifecycle...");

    let mut guard = VmGuard::new(5000, "vs-token-1", "is-token-1");

    // Initial state: No heartbeats received, must be ReadOnly
    if guard.authorize_mutation(100).is_ok() {
        vdebug_autoprefix!("test_boot_vmguard_lifecycle: Guard should be ReadOnly initially");
        return false;
    }

    // Provide only VS heartbeat
    let _ = guard.heartbeat(HeartbeatSource::VisualizationServer, "vs-token-1", 200);
    if guard.authorize_mutation(200).is_ok() {
        vdebug_autoprefix!("test_boot_vmguard_lifecycle: Guard should be ReadOnly with only VS heartbeat");
        return false;
    }

    // Provide IS heartbeat -> should become writable
    let _ = guard.heartbeat(HeartbeatSource::IdentityServer, "is-token-1", 250);
    if guard.authorize_mutation(300).is_err() {
        vdebug_autoprefix!("test_boot_vmguard_lifecycle: Guard should be Writable after VS + IS heartbeats");
        return false;
    }

    // Fast-forward beyond timeout (e.g. at 6000ms, elapsed > 5000ms)
    if guard.authorize_mutation(6000) != Err(GuardError::ExpiredHeartbeat) {
        vdebug_autoprefix!("test_boot_vmguard_lifecycle: Guard should expire after timeout");
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_vmguard_lifecycle: PASSED");
    true
}

/// Test 5: End-to-End VmPort Gateway & SmRingBuf Boot Dispatching
pub fn test_boot_vmport_gateway() -> bool {
    vdebug_autoprefix!("Running test_boot_vmport_gateway...");

    let backend = MockBootVmBackend::new();
    let mut port = VmPort::new(backend, 10000, "token-vs", "token-is");
    let _ = port.heartbeat(HeartbeatSource::VisualizationServer, "token-vs", 1000);
    let _ = port.heartbeat(HeartbeatSource::IdentityServer, "token-is", 1000);

    let config = ConfigPayload {
        cpu_cores: 2,
        memory_size: 1024,
        storage_mb: 2048,
        portmappings: Vec::new(),
        custom_os_cfg: None,
        metadata: None,
        assigned_ip: None,
    };

    let msg = IPCMessage {
        request: IPCHeader {
            rid: "req-boot-1".to_string(),
            tid: "0001".to_string(),
            inst_uuid: "a1b2-c3d4-e5f6".to_string(),
            client_token: None,
            verification_id: None,
        },
        environment: EnvironmentType::VM,
        action: CommandAction::Create,
        payload: InstancePayload::CfgPayload(config),
        extra_data: None,
    };

    let resp = port.dispatch(&msg, 1500);
    match resp {
        crate::hosting::vmdispatch::VmPortResponse::Accepted(res) => {
            if res.instance_uuid != "0001.a1b2-c3d4-e5f6" || res.vm_id != 1 {
                vdebug_autoprefix!(12, "test_boot_vmport_gateway: Unexpected RequestResult {:?}", res);
                return false;
            }
        }
        crate::hosting::vmdispatch::VmPortResponse::Rejected(err) => {
            vdebug_autoprefix!(12, "test_boot_vmport_gateway: Request was unexpectedly rejected: {:?}", err);
            return false;
        }
    }

    // Also test IPC queue enqueue and queued processing
    let queued_msg = IPCMessage {
        request: IPCHeader {
            rid: "req-boot-2".to_string(),
            tid: "0001".to_string(),
            inst_uuid: "a1b2-c3d4-e5f6".to_string(),
            client_token: None,
            verification_id: None,
        },
        environment: EnvironmentType::VM,
        action: CommandAction::Start,
        payload: InstancePayload::Empty,
        extra_data: None,
    };

    if port.enqueue_ipc(queued_msg).is_err() {
        vdebug_autoprefix!(12, "test_boot_vmport_gateway: Enqueue IPC failed");
        return false;
    }

    let queued_resp = port.process_queued_ipc(1600);
    if !matches!(queued_resp, Some(crate::hosting::vmdispatch::VmPortResponse::Accepted(_))) {
        vdebug_autoprefix!(12, "test_boot_vmport_gateway: Queued IPC processing failed: {:?}", queued_resp);
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_vmport_gateway: PASSED");
    true
}

/// Test 6: Port I/O Interface and Device Register Mapping
pub fn test_boot_port_io_interface() -> bool {
    vdebug_autoprefix!("Running test_boot_port_io_interface...");

    let mut port_io = PortIoInterface::new(PortOwner::Vm(1));
    if port_io.register_port(0x3F8, 1, true).is_err() {
        vdebug_autoprefix!(12, "test_boot_port_io_interface: Failed to register COM1 port");
        return false;
    }
    if port_io.register_port(0x60, 1, false).is_err() {
        vdebug_autoprefix!(12, "test_boot_port_io_interface: Failed to register read-only keyboard port");
        return false;
    }

    if port_io.write(0x3F8, &[0x41]).is_err() {
        vdebug_autoprefix!(12, "test_boot_port_io_interface: Failed to write to COM1 port");
        return false;
    }

    let com1_val = port_io.read(0x3F8, 1);
    if com1_val != Ok(vec![0x41]) {
        vdebug_autoprefix!(12, "test_boot_port_io_interface: COM1 read mismatch: {:?}", com1_val);
        return false;
    }

    // Ensure write to read-only port is rejected
    if port_io.write(0x60, &[0xFF]).is_ok() {
        vdebug_autoprefix!(12, "test_boot_port_io_interface: Read-only port accepted write unexpectedly");
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_port_io_interface: PASSED");
    true
}

/// Test 7: Virtualization Management Tab UI, Subtabs, and Architecture Views
pub fn test_boot_virtualization_ui() -> bool {
    vdebug_autoprefix!("Running test_boot_virtualization_ui...");

    // Test subtab cycling
    let mut tab = VirtSubTab::VMs;
    tab = tab.next();
    if tab != VirtSubTab::Containers {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Next after VMs should be Containers");
        return false;
    }
    tab = tab.next();
    if tab != VirtSubTab::CVMs {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Next after Containers should be CVMs");
        return false;
    }
    tab = tab.next();
    if tab != VirtSubTab::Architecture {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Next after CVMs should be Architecture");
        return false;
    }
    tab = tab.next();
    if tab != VirtSubTab::VMs {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Next after Architecture should cycle back to VMs");
        return false;
    }

    // Test DashboardUI virtualization state and methods
    let pm = crate::pm::PackageManager::new();
    let mut ui = DashboardUI::new(pm);
    if ui.virt_subtab != VirtSubTab::VMs {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Default subtab should be VMs");
        return false;
    }
    if ui.containers.is_empty() || ui.cvms.is_empty() {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: Initial container and CVM catalogs should not be empty");
        return false;
    }

    let initial_containers = ui.containers.len();
    ui.add_container(ContainerDisplayInfo {
        id: 99,
        name: String::from("test-container"),
        image: String::from("test:v1"),
        state: String::from("Running"),
        cpu_shares: 512,
        memory_mb: 256,
        port_count: 1,
    });
    if ui.containers.len() != initial_containers + 1 {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: add_container failed");
        return false;
    }

    let initial_cvms = ui.cvms.len();
    ui.add_cvm(CvmDisplayInfo {
        id: 99,
        name: String::from("test-cvm"),
        kernel: String::from("MicroKernel-v1.0"),
        state: String::from("Running"),
        memory_mb: 512,
        vcpus: 2,
        container_count: 1,
        bus_channels: 2,
    });
    if ui.cvms.len() != initial_cvms + 1 {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: add_cvm failed");
        return false;
    }

    // Test X_Virtualization App instantiation
    let x_virt = X_Virtualization::new();
    if x_virt.virt_subtab != VirtSubTab::VMs {
        vdebug_autoprefix!(12, "test_boot_virtualization_ui: X_Virtualization default subtab mismatch");
        return false;
    }

    vdebug_autoprefix!(10, "test_boot_virtualization_ui: PASSED");
    true
}

/// Master Runner for all Startup/Boot OS tests.
pub fn run_all_boot_tests() -> bool {
    vdebug_autoprefix!(11, "========================================");
    vdebug_autoprefix!(11, "STARTING HPVMx STARTUP & BOOT OS TESTS");
    vdebug_autoprefix!(11, "========================================");

    let mut all_passed = true;

    if !test_boot_environment() {
        all_passed = false;
    }
    if !test_boot_cvm_and_containers() {
        all_passed = false;
    }
    if !test_boot_hosting_luts() {
        all_passed = false;
    }
    if !test_boot_vmguard_lifecycle() {
        all_passed = false;
    }
    if !test_boot_vmport_gateway() {
        all_passed = false;
    }
    if !test_boot_port_io_interface() {
        all_passed = false;
    }
    if !test_boot_virtualization_ui() {
        all_passed = false;
    }

    if all_passed {
        vdebug_autoprefix!(10, ">>> ALL STARTUP & BOOT OS TESTS PASSED <<<");
    } else {
        vdebug_autoprefix!(12, ">>> SOME STARTUP & BOOT OS TESTS FAILED <<<");
    }

    all_passed
}
