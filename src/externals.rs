//! Public host API for in-kernel applications and background work.
//!
//! Applications implement [`Runnable`] and can be launched directly into the active
//! dashboard runtime. Short-lived/cooperative work can be attached to an app with
//! [`spawn_app_task`]; `Send` futures can use the global multi-core executor.
//!
//! An in-tree app can construct and attach itself with:
//! ```ignore
//! let pid = crate::externals::launch_application(
//!     &mut global_data, "Example", "1.0", (480, 320), ExampleApp::new(),
//! );
//! ```

use alloc::boxed::Box;
use alloc::string::String;
pub use crate::env::{
    Application, Background, BackgroundSteppedApplicationContext, BackgroundTask,
    Environment, GlobalEnvironmentData, Runnable, SteppedApplicationContext,
    XSteppedApplicationContext,
};
pub use crate::ui::pixel_graphics::PixelGraphics;
pub use uefi::proto::console::text::Key;

/// File and directory operations exposed to in-kernel applications.
pub mod fs {
    use alloc::{string::String, vec::Vec};

    pub fn read(path: &str) -> Result<Vec<u8>, &'static str> { crate::filesystem::FileSystem::read_file(path) }
    pub fn read_text(path: &str) -> Result<String, &'static str> { crate::filesystem::FileSystem::read_file_to_string(path) }
    pub fn write(path: &str, bytes: &[u8]) -> Result<(), &'static str> { crate::filesystem::FileSystem::write_to_file_bytes(path, bytes, 'w') }
    pub fn append(path: &str, bytes: &[u8]) -> Result<(), &'static str> { crate::filesystem::FileSystem::write_to_file_bytes(path, bytes, 'a') }
    pub fn create(path: &str) -> Result<uefi::proto::media::file::FileHandle, &'static str> { crate::filesystem::FileSystem::create(path) }
    pub fn make_dir(path: &str) -> Result<(), &'static str> { crate::filesystem::FileSystem::mkdir(path) }
    pub fn remove(path: &str) -> Result<(), &'static str> { crate::filesystem::FileSystem::remove(path) }
    pub fn remove_dir(path: &str) -> Result<(), &'static str> { crate::filesystem::FileSystem::remove_dir(path) }
    pub fn rename(from: &str, to: &str) -> Result<(), &'static str> { crate::filesystem::FileSystem::rename(from, to) }
    pub fn copy(from: &str, to: &str) -> Result<(), &'static str> { crate::filesystem::FileSystem::copy(from, to) }
    pub fn list(path: &str) -> Result<Vec<(String, bool)>, &'static str> { crate::filesystem::FileSystem::read_dir(path) }
    pub fn current_dir() -> Result<String, ()> { crate::filesystem::FileSystem::get_cwd() }
    pub fn change_dir(path: &str) { crate::filesystem::FileSystem::cd(path) }
    pub fn statistics() -> (u64, u64, u64, u64) { crate::filesystem::disk_stats() }
}

/// CPU and PCI operations. PCI writes directly affect hardware configuration space.
pub mod hardware {
    use alloc::vec::Vec;
    pub fn cpu_info() -> crate::hardware::cpu::CpuInfo { crate::hardware::cpu::CpuInfo::get() }
    pub fn core_count() -> u32 { crate::hardware::cpu::core_count() }
    pub fn application_processor_count() -> u32 { crate::hardware::cpu::ap_count() }
    pub fn pci_devices() -> Vec<crate::hardware::pci::PciDeviceInfo> { crate::hardware::pci::scan_bus() }
    pub fn pci_read_u32(bus: u8, slot: u8, function: u8, offset: u8) -> u32 { crate::hardware::pci::pci_config_read_u32(bus, slot, function, offset) }
    pub fn pci_write_u32(bus: u8, slot: u8, function: u8, offset: u8, value: u32) { crate::hardware::pci::pci_config_write_u32(bus, slot, function, offset, value) }
}

/// Network and audio driver entry points.
pub mod devices {
    pub fn network_initialize() -> Result<(), &'static str> { crate::devices::net_hw::init() }
    pub fn network_is_initialized() -> bool { crate::devices::net_hw::is_initialized() }
    pub fn network_link_up() -> bool { crate::devices::net_hw::link_up() }
    pub fn network_transmit(frame: &[u8]) -> Result<(), &'static str> { crate::devices::net_hw::tx(frame) }
    pub fn network_receive(buffer: &mut [u8]) -> Result<usize, &'static str> { crate::devices::net_hw::rx(buffer) }
    pub fn beep(frequency_hz: u32) { crate::devices::audio::beep(frequency_hz) }
    pub fn play_tone(frequency_hz: u32, duration_ms: u64) { crate::devices::audio::play_tone_nb(frequency_hz, duration_ms) }
    pub fn mute() { crate::devices::audio::mute() }
    pub fn sleep_ms(milliseconds: u64) { crate::devices::timer::sleep_ms(milliseconds) }
}

/// Direct drawing helpers backed by PixelGraphics.
pub mod gui {
    use crate::ui::pixel_graphics::PixelGraphics;
    pub fn resolution() -> Option<(usize, usize)> { PixelGraphics::new().map(|g| g.resolution()) }
    pub fn clear(color: u32) {
        if let Some(mut g) = PixelGraphics::new() { g.clear(color); g.flip(); }
    }
    pub fn fill_rect(x: usize, y: usize, width: usize, height: usize, color: u32) {
        if let Some(mut g) = PixelGraphics::new() { g.fill_rect(x, y, width, height, color); g.flip(); }
    }
    pub fn draw_text(x: usize, y: usize, text: &str, foreground: u32, background: u32) {
        if let Some(mut g) = PixelGraphics::new() { g.draw_text_bg(x, y, text, foreground, background); g.flip(); }
    }
    pub fn draw_button(x: usize, y: usize, width: usize, height: usize, label: &str, focused: bool) {
        if let Some(mut g) = PixelGraphics::new() { g.draw_button(x, y, width, height, label, focused); g.flip(); }
    }
}

/// Kernel-wide environment values and timing utilities.
pub mod system {
    use alloc::string::String;
    pub fn set_variable(key: &str, value: &str) { crate::env::set_global_var(key, value); }
    pub fn get_variable(key: &str) -> Option<String> { crate::env::get_global_var(key) }
    pub fn variables() -> alloc::vec::Vec<crate::env::EnvironmentVariable> { crate::env::global_vars_snapshot() }
    pub fn timestamp_ms() -> u64 {
        #[cfg(target_arch = "x86_64")]
        { unsafe { core::arch::x86_64::_rdtsc() / (crate::TSC_PER_US.max(1) * 1000) } }
        #[cfg(not(target_arch = "x86_64"))]
        { 0 }
    }
}

/// Persistence helpers for HPVMx system and device settings.
pub mod registry {
    pub fn load_devices() -> Result<(), &'static str> {
        crate::registry::load_device_registry(crate::registry::DEFAULT_DEVICE_REG_PATH)
    }
    pub fn save_devices() -> Result<(), &'static str> {
        crate::registry::save_device_registry(crate::registry::DEFAULT_DEVICE_REG_PATH)
    }
    pub fn load_system(settings: &mut crate::ui::UiSettings) -> Result<(), &'static str> {
        crate::registry::load_system_registry(crate::registry::DEFAULT_SYSTEM_REG_PATH, settings)
    }
    pub fn save_system(settings: &crate::ui::UiSettings) -> Result<(), &'static str> {
        crate::registry::save_system_registry(crate::registry::DEFAULT_SYSTEM_REG_PATH, settings)
    }
}

/// Operations on the live stepped-app table. App code still advances on the OS scheduler.
pub mod processes {
    use crate::env::GlobalEnvironmentData;
    pub fn count(runtime: &GlobalEnvironmentData) -> usize { runtime.active_apps.len() }
    pub fn terminate(runtime: &mut GlobalEnvironmentData, pid: usize) -> bool {
        let Some(index) = runtime.active_apps.iter().position(|app| app.pid == pid) else { return false; };
        runtime.active_apps.remove(index);
        runtime.focused_process_idx = runtime.focused_process_idx.and_then(|focused| {
            if focused == index { None } else if focused > index { Some(focused - 1) } else { Some(focused) }
        });
        if runtime.selected_app_idx > index { runtime.selected_app_idx -= 1; }
        runtime.selected_app_idx = runtime.selected_app_idx.min(runtime.active_apps.len().saturating_sub(1));
        true
    }
}

/// Explicitly unsafe allocator calls for ABI consumers that need raw buffers.
/// Pass the same size and alignment to `deallocate` that were used to allocate.
pub mod memory {
    use core::alloc::Layout;
    pub unsafe fn allocate(size: usize, alignment: usize) -> Option<*mut u8> {
        let alignment = alignment.max(core::mem::align_of::<usize>()).checked_next_power_of_two()?;
        let layout = Layout::from_size_align(size.max(1), alignment).ok()?;
        let ptr = unsafe { alloc::alloc::alloc(layout) };
        (!ptr.is_null()).then_some(ptr)
    }
    pub unsafe fn deallocate(ptr: *mut u8, size: usize, alignment: usize) -> bool {
        if ptr.is_null() { return false; }
        let Some(alignment) = alignment.max(core::mem::align_of::<usize>()).checked_next_power_of_two() else { return false; };
        let Ok(layout) = Layout::from_size_align(size.max(1), alignment) else { return false; };
        unsafe { alloc::alloc::dealloc(ptr, layout); }
        true
    }
}

/// Package a Rust application implementation with its executable metadata.
pub fn application<T>(name: &str, version: &str, dimensions: (usize, usize), app: T) -> Application
where
    T: Runnable + Clone + 'static,
{
    let mut application = Application::new(Box::new(app));
    application.name = String::from(name);
    application.version = String::from(version);
    application.dimensions = dimensions;
    application
}

/// Construct a runnable window context without registering it in the built-in app menu.
pub fn create_application<T>(name: &str, version: &str, dimensions: (usize, usize), app: T) -> SteppedApplicationContext
where
    T: Runnable + Clone + 'static,
{
    SteppedApplicationContext::new(application(name, version, dimensions, app), None)
}

/// Start an application immediately by inserting it into the live dashboard process list.
/// Returns the assigned process ID.
pub fn launch_application<T>(
    runtime: &mut GlobalEnvironmentData,
    name: &str,
    version: &str,
    dimensions: (usize, usize),
    app: T,
) -> usize
where
    T: Runnable + Clone + 'static,
{
    let context = create_application(name, version, dimensions, app);
    let pid = context.pid;
    runtime.active_apps.push(context);
    pid
}

/// Attach a cooperative background task to an app. The app scheduler ticks it
/// alongside the app's logic and removes it when `tick` returns `true`.
pub fn spawn_app_task<T>(context: &mut SteppedApplicationContext, task: T)
where
    T: BackgroundTask + 'static,
{
    context.spawn_task(task);
}

/// Attach a cooperative task to the newer app-context variant.
pub fn spawn_x_app_task<T>(context: &mut XSteppedApplicationContext, task: T)
where
    T: BackgroundTask + 'static,
{
    context.spawn_task(task);
}

/// Attach a `Send` async future to an app so its lifecycle follows that app.
pub fn spawn_app_future<F>(context: &mut SteppedApplicationContext, future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    context.spawn_async(future);
}

/// Attach a `Send` async future to the newer app-context variant.
pub fn spawn_x_app_future<F>(context: &mut XSteppedApplicationContext, future: F)
where
    F: Future<Output = ()> + Send + 'static,
{
    context.spawn_async(future);
}

/// Spawn a `Send` future on the initialized global multi-core executor.
/// Returns `None` until that executor has been initialized.
pub fn spawn_background_future<F>(future: F) -> Option<crate::multipar::task::TaskHandle>
where
    F: Future<Output = ()> + Send + 'static,
{
    crate::env::spawn_multicore(future)
}


/// Four-byte marker identifying an HPVMx statically linked executable descriptor.
pub const EXECUTABLE_MAGIC: [u8; 4] = *b"HPVX";
pub const EXECUTABLE_ABI_VERSION: u16 = 1;

/// Runtime mode selected by a compiled executable's header.
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutableKind {
    SteppedApplication = 1,
    SteppedBackgroundTask = 2,
}

/// Small ABI header placed in every statically linked executable descriptor.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExecutableHeader {
    pub magic: [u8; 4],
    pub abi_version: u16,
    pub kind: ExecutableKind,
}

impl ExecutableHeader {
    pub const fn new(kind: ExecutableKind) -> Self {
        Self { magic: EXECUTABLE_MAGIC, abi_version: EXECUTABLE_ABI_VERSION, kind }
    }

    pub const fn is_valid(&self) -> bool {
        self.magic[0] == EXECUTABLE_MAGIC[0]
            && self.magic[1] == EXECUTABLE_MAGIC[1]
            && self.magic[2] == EXECUTABLE_MAGIC[2]
            && self.magic[3] == EXECUTABLE_MAGIC[3]
            && self.abi_version == EXECUTABLE_ABI_VERSION
    }
}

pub type ApplicationEntry = fn() -> (Box<dyn Runnable>, (usize, usize));
pub type BackgroundTaskEntry = fn() -> Box<dyn BackgroundTask>;

/// Entry point payload. Both constructors are plain Rust function pointers, so
/// the app/task code is linked into the kernel image at build time.
#[derive(Clone, Copy)]
pub enum ExecutableEntry {
    Application(ApplicationEntry),
    BackgroundTask(BackgroundTaskEntry),
}

/// One statically linked executable that can be installed in an executable table.
#[derive(Clone, Copy)]
pub struct CompiledExecutable {
    pub header: ExecutableHeader,
    pub name: &'static str,
    pub version: &'static str,
    pub entry: ExecutableEntry,
}

impl CompiledExecutable {
    pub const fn application(name: &'static str, version: &'static str, entry: ApplicationEntry) -> Self {
        Self {
            header: ExecutableHeader::new(ExecutableKind::SteppedApplication),
            name,
            version,
            entry: ExecutableEntry::Application(entry),
        }
    }

    pub const fn background_task(name: &'static str, version: &'static str, entry: BackgroundTaskEntry) -> Self {
        Self {
            header: ExecutableHeader::new(ExecutableKind::SteppedBackgroundTask),
            name,
            version,
            entry: ExecutableEntry::BackgroundTask(entry),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutableError {
    InvalidHeader,
    EntryKindMismatch,
    NotFound,
}

/// Owns cooperative background executables. Call `step_background_tasks` once
/// per scheduler/frame iteration to advance them and reap completed entries.
pub struct ExecutableManager {
    background_tasks: alloc::vec::Vec<BackgroundSteppedApplicationContext>,
}

impl Default for ExecutableManager {
    fn default() -> Self { Self::new() }
}

impl ExecutableManager {
    pub const fn new() -> Self {
        Self { background_tasks: alloc::vec::Vec::new() }
    }

    /// Validate the descriptor header and dispatch its entry by the header tag.
    /// Application contexts enter the dashboard process list; background
    /// contexts are owned and stepped by this manager.
    pub fn run(
        &mut self,
        executable: &CompiledExecutable,
        runtime: &mut GlobalEnvironmentData,
    ) -> Result<usize, ExecutableError> {
        if !executable.header.is_valid() { return Err(ExecutableError::InvalidHeader); }
        match (executable.header.kind, executable.entry) {
            (ExecutableKind::SteppedApplication, ExecutableEntry::Application(entry)) => {
                let (logic, dimensions) = entry();
                let mut app = Application::new(logic);
                app.name = String::from(executable.name);
                app.version = String::from(executable.version);
                app.dimensions = dimensions;
                let context = SteppedApplicationContext::new(app, None);
                let pid = context.pid;
                runtime.active_apps.push(context);
                Ok(pid)
            }
            (ExecutableKind::SteppedBackgroundTask, ExecutableEntry::BackgroundTask(entry)) => {
                let mut background = Background::new(entry());
                background.name = String::from(executable.name);
                background.version = String::from(executable.version);
                let context = BackgroundSteppedApplicationContext::new(background);
                let pid = context.pid;
                self.background_tasks.push(context);
                Ok(pid)
            }
            _ => Err(ExecutableError::EntryKindMismatch),
        }
    }

    /// Find and run a linked executable by its descriptor name.
    pub fn run_named(
        &mut self,
        name: &str,
        executables: &[CompiledExecutable],
        runtime: &mut GlobalEnvironmentData,
    ) -> Result<usize, ExecutableError> {
        let executable = executables.iter().find(|item| item.name == name)
            .ok_or(ExecutableError::NotFound)?;
        self.run(executable, runtime)
    }

    /// Advance each background entry once, removing those whose step reports completion.
    pub fn step_background_tasks(&mut self) -> usize {
        let before = self.background_tasks.len();
        self.background_tasks.retain_mut(|task| task.step());
        before - self.background_tasks.len()
    }

    pub fn background_task_count(&self) -> usize { self.background_tasks.len() }
}


/// Disk executable loader and its stable plugin ABI.
pub use crate::disk_executable::{
    DiskExecutableManager, HpxCpuInfo, HpxHeader, HpxPciDeviceInfo, PluginHostApi,
    HPX_HOST_API_VERSION,
};
