//! Multiprocessor (MP) services, Application Processor (AP) discovery, and AP worker lifecycle.
//!
//! Provides hardware-level multi-core discovery, UEFI `MpServices` protocol acquisition,
//! processor topology inspection, and AP background async worker lifecycle management for HPVMx.

use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
#[allow(unused_imports)]
use crate::hpvm_log;
#[allow(unused_imports)]
use uefi::proto::console::text::Color;
use uefi::boot::{self, EventType, ScopedProtocol, Tpl};
pub use uefi::proto::pi::mp::{CpuPhysicalLocation, MpServices, Procedure, ProcessorCount, ProcessorInformation};
use crate::env::SpinLock;

/// Status of an individual Application Processor (AP) worker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApWorkerStatus {
    Uninitialized,
    Starting,
    Running,
    Stopping,
    Stopped,
    Error,
}

/// Statistics and runtime metrics for an individual AP worker.
#[derive(Debug, Clone)]
pub struct ApWorkerStat {
    pub core_index: usize,
    pub processor_id: u64,
    pub is_active: bool,
    pub tasks_executed: usize,
}

/// Per-AP worker execution context passed across UEFI MP Services boundaries.
pub struct ApWorkerContext {
    /// Zero-based AP logical index.
    pub core_index: usize,
    /// Hardware processor ID (e.g., Local APIC ID).
    pub processor_id: u64,
    /// Number of async tasks successfully executed on this AP.
    pub tasks_executed: AtomicUsize,
    /// Whether the worker loop is currently running on this core.
    pub is_active: AtomicBool,
    /// Flag signalling the worker loop on this AP to stop execution.
    pub stop_requested: AtomicBool,
}

impl ApWorkerContext {
    /// Creates a new `ApWorkerContext`.
    pub fn new(core_index: usize, processor_id: u64) -> Self {
        Self {
            core_index,
            processor_id,
            tasks_executed: AtomicUsize::new(0),
            is_active: AtomicBool::new(false),
            stop_requested: AtomicBool::new(false),
        }
    }

    /// Signals the AP worker to stop.
    pub fn request_stop(&self) {
        self.stop_requested.store(true, Ordering::SeqCst);
    }

    /// Returns `true` if the worker is currently active.
    pub fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }

    /// Returns the number of tasks executed by this worker.
    pub fn tasks_executed(&self) -> usize {
        self.tasks_executed.load(Ordering::SeqCst)
    }
}

/// AP worker entry procedure invoked by UEFI MP Services on each AP core.
pub extern "efiapi" fn ap_worker_entry(context: *mut core::ffi::c_void) {
    if context.is_null() {
        return;
    }
    let ctx = unsafe { &*(context as *const ApWorkerContext) };
    ctx.is_active.store(true, Ordering::SeqCst);

    while !ctx.stop_requested.load(Ordering::SeqCst) {
        let did_work = crate::multipar::task::poll_global_one();
        if did_work {
            ctx.tasks_executed.fetch_add(1, Ordering::SeqCst);
        } else {
            core::hint::spin_loop();
        }
    }

    ctx.is_active.store(false, Ordering::SeqCst);
}

/// Handle managing a running AP worker core and its UEFI event pointer.
pub struct ApWorkerHandle {
    pub core_index: usize,
    pub processor_id: u64,
    pub context: Arc<ApWorkerContext>,
    pub event_ptr: Option<*mut core::ffi::c_void>,
}

unsafe impl Send for ApWorkerHandle {}
unsafe impl Sync for ApWorkerHandle {}

/// Manager pool for AP worker threads and multi-core async execution.
pub struct ApWorkerPool {
    workers: Vec<ApWorkerHandle>,
}

unsafe impl Send for ApWorkerPool {}
unsafe impl Sync for ApWorkerPool {}

impl ApWorkerPool {
    /// Creates a new empty `ApWorkerPool`.
    pub const fn new() -> Self {
        Self {
            workers: Vec::new(),
        }
    }

    /// Returns `true` if any AP worker is currently running.
    pub fn is_running(&self) -> bool {
        self.workers.iter().any(|w| w.context.is_active())
    }

    /// Returns the count of currently active AP workers.
    pub fn active_count(&self) -> usize {
        self.workers.iter().filter(|w| w.context.is_active()).count()
    }

    /// Returns the total number of tasks executed across all AP workers.
    pub fn total_tasks_executed(&self) -> usize {
        self.workers
            .iter()
            .map(|w| w.context.tasks_executed())
            .sum()
    }

    /// Returns statistics for all registered AP workers.
    pub fn worker_stats(&self) -> Vec<ApWorkerStat> {
        self.workers
            .iter()
            .map(|w| ApWorkerStat {
                core_index: w.core_index,
                processor_id: w.processor_id,
                is_active: w.context.is_active(),
                tasks_executed: w.context.tasks_executed(),
            })
            .collect()
    }

    /// Launches worker loops on all available enabled APs using UEFI MP Services.
    ///
    /// Returns the number of AP workers successfully started.
    pub fn start_all_aps(&mut self) -> Result<usize, &'static str> {
        // Stop any previously running workers first
        self.stop_all();

        let topology = match MpManager::detect_topology() {
            Some(t) => t,
            None => return Err("UEFI MP Services protocol unavailable"),
        };

        let ap_cores: Vec<ProcessorCoreInfo> = topology.enabled_aps().cloned().collect();
        if ap_cores.is_empty() {
            return Ok(0);
        }

        let mp = match MpManager::get_mp_services() {
            Some(mp) => mp,
            None => return Err("Failed to acquire MP Services protocol handle"),
        };

        let mut started_count = 0;
        for ap in ap_cores {
            let ctx = Arc::new(ApWorkerContext::new(ap.index, ap.processor_id));
            let raw_ctx = Arc::into_raw(Arc::clone(&ctx)) as *mut core::ffi::c_void;

            // Create non-blocking UEFI event for AP completion
            let event = unsafe {
                boot::create_event(EventType::empty(), Tpl::APPLICATION, None, None).ok()
            };
            let event_ptr = event.as_ref().map(|e| e.as_ptr());

            let res = mp.startup_this_ap(
                ap.index,
                ap_worker_entry,
                raw_ctx,
                event,
                None, // Run until signaled to stop
            );

            match res {
                Ok(()) => {
                    self.workers.push(ApWorkerHandle {
                        core_index: ap.index,
                        processor_id: ap.processor_id,
                        context: ctx,
                        event_ptr,
                    });
                    started_count += 1;
                }
                Err(status) => {
                    // Reclaim context Arc on failure
                    unsafe {
                        let _ = Arc::from_raw(raw_ctx as *const ApWorkerContext);
                    }
                    if let Some(ptr) = event_ptr {
                        if let Some(ev) = unsafe { uefi::Event::from_ptr(ptr) } {
                            let _ = boot::close_event(ev);
                        }
                    }
                    crate::hpvm_error!(
                        "cpu:mp",
                        "Failed to start AP core #{} (Status: {:?})",
                        ap.index,
                        status
                    );
                }
            }
        }

        Ok(started_count)
    }

    /// Signals all running AP workers to stop execution and cleans up UEFI events.
    pub fn stop_all(&mut self) {
        for worker in &self.workers {
            worker.context.request_stop();
        }

        // Give AP loops a brief moment to observe stop signal
        for _ in 0..100 {
            if self.active_count() == 0 {
                break;
            }
            core::hint::spin_loop();
        }

        for worker in self.workers.drain(..) {
            if let Some(ptr) = worker.event_ptr {
                if let Some(event) = unsafe { uefi::Event::from_ptr(ptr) } {
                    let _ = boot::close_event(event);
                }
            }
        }
    }
}

impl Drop for ApWorkerPool {
    fn drop(&mut self) {
        self.stop_all();
    }
}

/// Global shared AP worker pool.
pub static GLOBAL_AP_POOL: SpinLock<ApWorkerPool> = SpinLock::new(ApWorkerPool::new());

/// Starts AP workers on all available enabled AP cores for the global executor.
pub fn start_global_ap_workers() -> Result<usize, &'static str> {
    let mut pool = GLOBAL_AP_POOL.lock();
    pool.start_all_aps()
}

/// Stops all active AP workers.
pub fn stop_global_ap_workers() {
    let mut pool = GLOBAL_AP_POOL.lock();
    pool.stop_all();
}

/// Returns the number of currently active AP workers.
pub fn active_global_ap_workers() -> usize {
    let pool = GLOBAL_AP_POOL.lock();
    pool.active_count()
}

/// Returns the total tasks completed by AP workers.
pub fn total_ap_tasks_executed() -> usize {
    let pool = GLOBAL_AP_POOL.lock();
    pool.total_tasks_executed()
}

/// Returns statistics for all global AP workers.
pub fn global_ap_worker_stats() -> Vec<ApWorkerStat> {
    let pool = GLOBAL_AP_POOL.lock();
    pool.worker_stats()
}

/// Detailed information regarding an individual logical processor core.
#[derive(Debug, Clone)]
pub struct ProcessorCoreInfo {
    /// Zero-based index used by UEFI MP Services (0..total).
    pub index: usize,
    /// Unique hardware processor ID (e.g., Local APIC ID).
    pub processor_id: u64,
    /// Whether this core is currently acting as the Bootstrap Processor (BSP).
    pub is_bsp: bool,
    /// Whether this core is enabled in firmware/hardware.
    pub is_enabled: bool,
    /// Whether this core is healthy.
    pub is_healthy: bool,
    /// Physical socket / package identifier.
    pub package: u32,
    /// Physical core identifier within the package.
    pub core: u32,
    /// Logical SMT / hyperthread identifier within the core.
    pub thread: u32,
}

impl ProcessorCoreInfo {
    /// Creates `ProcessorCoreInfo` from UEFI `ProcessorInformation`.
    pub fn from_uefi(index: usize, info: &ProcessorInformation) -> Self {
        Self {
            index,
            processor_id: info.processor_id,
            is_bsp: info.is_bsp(),
            is_enabled: info.is_enabled(),
            is_healthy: info.is_healthy(),
            package: info.location.package,
            core: info.location.core,
            thread: info.location.thread,
        }
    }
}

/// Discovered CPU topology containing all detected processors and APs.
#[derive(Debug, Clone, Default)]
pub struct MpTopology {
    /// Total number of logical processors detected by firmware.
    pub total_processors: usize,
    /// Number of enabled processors.
    pub enabled_processors: usize,
    /// Index of the BSP core.
    pub bsp_index: Option<usize>,
    /// List of all detected logical cores.
    pub processors: Vec<ProcessorCoreInfo>,
}

impl MpTopology {
    /// Returns the number of enabled Application Processors (APs).
    pub fn enabled_ap_count(&self) -> usize {
        self.processors
            .iter()
            .filter(|p| !p.is_bsp && p.is_enabled)
            .count()
    }

    /// Returns `true` if there is at least one usable AP core available.
    pub fn has_usable_aps(&self) -> bool {
        self.enabled_ap_count() > 0
    }

    /// Returns an iterator over all enabled AP cores.
    pub fn enabled_aps(&self) -> impl Iterator<Item = &ProcessorCoreInfo> {
        self.processors.iter().filter(|p| !p.is_bsp && p.is_enabled)
    }

    /// Returns core info for the BSP, if detected.
    pub fn bsp_info(&self) -> Option<&ProcessorCoreInfo> {
        self.processors.iter().find(|p| p.is_bsp)
    }
}

/// Manager for UEFI MP Services and AP core discovery.
pub struct MpManager;

impl MpManager {
    /// Checks if the UEFI MP Services protocol is supported and available on this platform.
    pub fn is_supported() -> bool {
        boot::get_handle_for_protocol::<MpServices>().is_ok()
    }

    /// Attempts to acquire an exclusive handle to the UEFI MP Services protocol.
    pub fn get_mp_services() -> Option<ScopedProtocol<MpServices>> {
        let handle = boot::get_handle_for_protocol::<MpServices>().ok()?;
        boot::open_protocol_exclusive::<MpServices>(handle).ok()
    }

    /// Returns the caller processor's logical index via MP Services.
    pub fn who_am_i() -> Option<usize> {
        let mp = Self::get_mp_services()?;
        mp.who_am_i().ok()
    }

    /// Detects and enumerates all logical processors, BSP, and APs.
    pub fn detect_topology() -> Option<MpTopology> {
        let mp = Self::get_mp_services()?;
        let counts = mp.get_number_of_processors().ok()?;

        let mut topology = MpTopology {
            total_processors: counts.total,
            enabled_processors: counts.enabled,
            bsp_index: None,
            processors: Vec::with_capacity(counts.total),
        };

        for i in 0..counts.total {
            if let Ok(info) = mp.get_processor_info(i) {
                let core_info = ProcessorCoreInfo::from_uefi(i, &info);
                if core_info.is_bsp {
                    topology.bsp_index = Some(i);
                }
                topology.processors.push(core_info);
            }
        }

        Some(topology)
    }

    /// Logs discovered multi-core topology to HPVMx debug output.
    pub fn log_topology(topology: &MpTopology) {
        crate::vdebug!(
            "cpu:mp",
            "MP Services detected: {} total, {} enabled, {} APs",
            topology.total_processors,
            topology.enabled_processors,
            topology.enabled_ap_count()
        );

        for p in &topology.processors {
            crate::vdebug!(
                "cpu:mp",
                " Core #{}: ID={:#x}, Role={}, Enabled={}, Healthy={}, Loc=Pkg{}/Core{}/Th{}",
                p.index,
                p.processor_id,
                if p.is_bsp { "BSP" } else { "AP" },
                p.is_enabled,
                p.is_healthy,
                p.package,
                p.core,
                p.thread
            );
        }
    }
}

/// Runs self-tests for MP topology parsing, AP worker contexts, and data structures.
pub fn run_mp_tests() -> bool {
    let topo = MpTopology {
        total_processors: 4,
        enabled_processors: 4,
        bsp_index: Some(0),
        processors: alloc::vec![
            ProcessorCoreInfo {
                index: 0,
                processor_id: 0,
                is_bsp: true,
                is_enabled: true,
                is_healthy: true,
                package: 0,
                core: 0,
                thread: 0,
            },
            ProcessorCoreInfo {
                index: 1,
                processor_id: 1,
                is_bsp: false,
                is_enabled: true,
                is_healthy: true,
                package: 0,
                core: 1,
                thread: 0,
            },
            ProcessorCoreInfo {
                index: 2,
                processor_id: 2,
                is_bsp: false,
                is_enabled: false,
                is_healthy: true,
                package: 0,
                core: 2,
                thread: 0,
            },
            ProcessorCoreInfo {
                index: 3,
                processor_id: 3,
                is_bsp: false,
                is_enabled: true,
                is_healthy: true,
                package: 0,
                core: 3,
                thread: 0,
            },
        ],
    };

    if topo.enabled_ap_count() != 2 {
        return false;
    }
    if !topo.has_usable_aps() {
        return false;
    }
    if topo.bsp_info().map(|b| b.index) != Some(0) {
        return false;
    }

    let ap_indices: Vec<usize> = topo.enabled_aps().map(|p| p.index).collect();
    if ap_indices != alloc::vec![1, 3] {
        return false;
    }

    // Verify ApWorkerContext lifecycle and atomic synchronization
    let ctx = ApWorkerContext::new(1, 0x10);
    if ctx.is_active() || ctx.tasks_executed() != 0 {
        return false;
    }
    ctx.is_active.store(true, Ordering::SeqCst);
    ctx.tasks_executed.fetch_add(5, Ordering::SeqCst);
    if !ctx.is_active() || ctx.tasks_executed() != 5 {
        return false;
    }
    ctx.request_stop();
    if !ctx.stop_requested.load(Ordering::SeqCst) {
        return false;
    }

    true
}
