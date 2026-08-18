//! Environment and application life-cycle management.
//!
//! This module provides the infrastructure for running applications,
//! managing their environments, and handling background tasks.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::cell::UnsafeCell;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use uefi::fs::Path;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use uefi::proto::console::text::Key;
use uefi_raw::protocol::hii::config::HiiTime;
use crate::{apps, GLOBALENV};
use crate::apps::AppConstructor;
use crate::hpvmlog::LOGGING_SILENCED;
use crate::ui::pixel_graphics::icons::ICON32;
use crate::ui::pixel_graphics::PixelGraphics;
use crate::apps::x_overview::X_Overview;
use crate::apps::x_vms::X_VMs;
use crate::apps::x_storage::X_Storage;
use crate::apps::x_resources::X_Resources;
use crate::apps::x_apps::X_Apps;
use crate::apps::x_console::X_Console;
use crate::apps::x_network::X_Network;
use crate::apps::x_devices::X_Devices;
use crate::apps::x_settings::X_Settings;
use crate::apps::x_packages::X_Packages;
use crate::apps::x_test::X_Test;
use crate::apps::x_createvm::X_CreateVM;
use crate::apps::x_editor::X_Editor;
use crate::pm::PackageManager;
use crate::rng::XorShiftRng;
use crate::ui::{DashboardTab, DashboardUI, DeviceCategory, FileEntry, FilePendingAction, ResourceMonitorTab, SystemResources, TextEditor, UiSettings, VmDisplayInfo};

pub type EnvironmentVariable = (String, String);

static GLOBAL_ENV_READY: AtomicBool = AtomicBool::new(false);
static mut GLOBAL_ENV_VARS: MaybeUninit<BTreeMap<String, String>> = MaybeUninit::uninit();

pub fn global_data_ref() -> Option<&'static GlobalEnvironmentData> {
    unsafe {
        GLOBALENV.as_ref().map(|env| &env.data)
    }
}

pub(crate) fn global_env_vars_mut() -> &'static mut BTreeMap<String, String> {
    unsafe {
        if !GLOBAL_ENV_READY.load(Ordering::SeqCst) {
            GLOBAL_ENV_VARS.write(BTreeMap::new());
            GLOBAL_ENV_READY.store(true, Ordering::SeqCst);
        }
        GLOBAL_ENV_VARS.assume_init_mut()
    }
}

fn global_env_vars_ref() -> Option<&'static BTreeMap<String, String>> {
    if !GLOBAL_ENV_READY.load(Ordering::SeqCst) {
        return None;
    }
    unsafe { Some(GLOBAL_ENV_VARS.assume_init_ref()) }
}

pub fn set_global_var(key: &str, value: &str) {
    global_env_vars_mut().insert(key.to_string(), value.to_string());
}

pub fn get_global_var(key: &str) -> Option<String> {
    global_env_vars_ref().and_then(|vars| vars.get(key).cloned())
}

pub fn global_vars_snapshot() -> Vec<EnvironmentVariable> {
    match global_env_vars_ref() {
        Some(vars) => vars.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        None => Vec::new(),
    }
}

/// Local environment, app-specific
/// Local environment for an application.
///
/// Contains path variables and other settings that are specific to a single
/// application instance.
#[derive(Clone)]
pub struct Environment {
    pub cd: EnvironmentVariable,
    pub xd: EnvironmentVariable,
    pub tmp: EnvironmentVariable,
    pub user: EnvironmentVariable,
    pub devname: EnvironmentVariable,
    pub global_data: Option<&'static GlobalEnvironmentData>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            cd: ("".to_string(), "".to_string()),
            xd: ("".to_string(), "".to_string()),
            tmp: ("".to_string(), "".to_string()),
            user: ("".to_string(), "".to_string()),
            devname: ("".to_string(), "".to_string()),
            global_data: None,
        }
    }
}

/// Global environment, not app-specific
/// Global system environment.
///
/// Contains system-wide variables like the number of processors and OS version.
pub struct GlobalEnvironment {
    // pub cd: EnvironmentVariable,
    // pub xd: EnvironmentVariable,
    // pub tmp: EnvironmentVariable,
    // pub user: EnvironmentVariable,
    // pub devname: EnvironmentVariable,
    // pub processor_count: EnvironmentVariable,
    // pub os_version: EnvironmentVariable,
    pub data: GlobalEnvironmentData,
}

impl GlobalEnvironment {
    pub fn new() -> GlobalEnvironment {
        GlobalEnvironment {
            data: GlobalEnvironmentData::new()
        }
    }
}


/// Represents a runnable application.
///
/// This structure holds the application's metadata and its core logic
/// represented by the `Runnable` trait.
#[derive(Clone)]
pub struct Application {
    /// The name of the application.
    pub name: String,
    /// The version string.
    pub version: String,
    /// Entry point for JIT-compiled code, if applicable.
    pub jit_entry: Unknown<(String, u64)>,
    /// The local environment variables for this application.
    pub local_env: Environment,
    /// The actual application logic.
    pub inner: Box<dyn Runnable>,
    /// The preferred window dimensions (width, height).
    pub dimensions: (usize, usize),
}
#[derive(Clone)]
pub struct Background {
    /// The name of the application.
    pub name: String,
    /// The version string.
    pub version: String,
    /// Entry point for JIT-compiled code, if applicable.
    pub jit_entry: Unknown<(String, u64)>,
    /// The local environment variables for this application.
    pub local_env: Unknown<Environment>,
    /// The actual application logic.
    pub inner: Box<dyn BackgroundTask>,
}

pub trait RunnableClone {
    fn clone_box(&self) -> Box<dyn Runnable>;
}


impl<T> RunnableClone for T
where
    T: 'static + Runnable + Clone,
{
    fn clone_box(&self) -> Box<dyn Runnable> {
        Box::new(self.clone())
    }
}

/// Core trait for application logic and rendering.
///
/// Any application that wants to be managed by the system must implement this trait.
pub trait Runnable: RunnableClone {
    /// Renders the application to the provided `PixelGraphics` context.
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize); // Adjust types as needed
    /// Updates the application's internal state.
    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment);
    /// Handles a single keyboard input event.
    fn input(&mut self, key: Key);
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}

impl Clone for Box<dyn Runnable> {
    fn clone(&self) -> Box<dyn Runnable> {
        self.clone_box()
    }
}


pub trait BackgroundTaskClone {
    fn clone_box(&self) -> Box<dyn BackgroundTask>;
}


impl<T> BackgroundTaskClone for T
where
    T: 'static + BackgroundTask + Clone,
{
    fn clone_box(&self) -> Box<dyn BackgroundTask> {
        Box::new(self.clone())
    }
}


/// Represents a task that runs in the background.
pub trait BackgroundTask: BackgroundTaskClone {
    /// Performs a single tick of work.
    /// Drawing and input are not required
    /// Returns `true` if the task is finished and can be removed, 
    /// or `false` if it needs more processing time.
    fn tick(&mut self, vars: &mut Vec<String>, env: &mut Environment) -> bool;
}

impl Clone for Box<dyn BackgroundTask> {
    fn clone(&self) -> Box<dyn BackgroundTask> {
        self.clone_box()
    }
}

/// A lightweight spinlock implementation suitable for `#![no_std]` bare-metal environments.
#[derive(Debug)]
pub struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

impl<T> SpinLock<T> {
    /// Creates a new `SpinLock` protecting the given data.
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the spinlock, spinning until the lock is acquired.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            core::hint::spin_loop();
        }
        SpinLockGuard { lock: self }
    }

    /// Tries to acquire the spinlock without blocking.
    pub fn try_lock(&self) -> Option<SpinLockGuard<'_, T>> {
        if !self.locked.swap(true, Ordering::Acquire) {
            Some(SpinLockGuard { lock: self })
        } else {
            None
        }
    }
}

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> core::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> core::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(core::ptr::null(), &VTABLE)
}

/// Creates a no-op dummy waker for polling futures in cooperative single-threaded runtimes.
pub fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

/// A background task backed by an asynchronous Rust `Future`.
///
/// Wraps any `Future<Output = ()>` into a `BackgroundTask` compatible with HPVMx's
/// cooperative stepping loop in `SteppedApplicationContext` and `XSteppedApplicationContext`.
#[derive(Clone)]
pub struct AsyncBackgroundTask {
    future: Arc<SpinLock<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
}

impl AsyncBackgroundTask {
    /// Creates a new `AsyncBackgroundTask` from any static `Future`.
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Self {
            future: Arc::new(SpinLock::new(Some(Box::pin(future)))),
        }
    }

    /// Creates a new `AsyncBackgroundTask` from a pinned boxed future.
    pub fn from_pin_box(future: Pin<Box<dyn Future<Output = ()> + Send>>) -> Self {
        Self {
            future: Arc::new(SpinLock::new(Some(future))),
        }
    }
}

impl BackgroundTask for AsyncBackgroundTask {
    fn tick(&mut self, _vars: &mut Vec<String>, _env: &mut Environment) -> bool {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);

        if let Some(mut guard) = self.future.try_lock() {
            if let Some(fut) = guard.as_mut() {
                match fut.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        *guard = None;
                        true
                    }
                    Poll::Pending => false,
                }
            } else {
                true
            }
        } else {
            false
        }
    }
}

/// A `Future` adapter that drives a `BackgroundTask` by invoking `.tick()` on each poll.
pub struct BackgroundTaskFuture<T: BackgroundTask> {
    pub task: T,
    pub vars: Vec<String>,
    pub env: Environment,
}

impl<T: BackgroundTask> BackgroundTaskFuture<T> {
    pub fn new(task: T, vars: Vec<String>, env: Environment) -> Self {
        Self { task, vars, env }
    }
}

impl<T: BackgroundTask + Unpin> Future for BackgroundTaskFuture<T> {
    type Output = (Vec<String>, Environment);

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.task.tick(&mut this.vars, &mut this.env) {
            Poll::Ready((core::mem::take(&mut this.vars), this.env.clone()))
        } else {
            Poll::Pending
        }
    }
}

/// A `Future` adapter that drives a boxed `BackgroundTask`.
pub struct BoxedBackgroundTaskFuture {
    pub task: Box<dyn BackgroundTask>,
    pub vars: Vec<String>,
    pub env: Environment,
}

impl BoxedBackgroundTaskFuture {
    pub fn new(task: Box<dyn BackgroundTask>, vars: Vec<String>, env: Environment) -> Self {
        Self { task, vars, env }
    }
}

impl Future for BoxedBackgroundTaskFuture {
    type Output = (Vec<String>, Environment);

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.task.tick(&mut this.vars, &mut this.env) {
            Poll::Ready((core::mem::take(&mut this.vars), this.env.clone()))
        } else {
            Poll::Pending
        }
    }
}

/// A future that yields execution back to the executor / stepping loop once.
pub struct YieldFuture {
    yielded: bool,
}

impl Future for YieldFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

/// Yield execution back to the host/executor once.
pub fn yield_now() -> YieldFuture {
    YieldFuture { yielded: false }
}

/// A future that waits for a certain number of step ticks before completing.
pub struct TickDelayFuture {
    remaining_ticks: usize,
}

impl Future for TickDelayFuture {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.remaining_ticks == 0 {
            Poll::Ready(())
        } else {
            self.remaining_ticks -= 1;
            Poll::Pending
        }
    }
}

/// Creates a future that resolves after `ticks` polling iterations.
pub fn sleep_ticks(ticks: usize) -> TickDelayFuture {
    TickDelayFuture { remaining_ticks: ticks }
}

/// Metadata and capability information about an application.
pub trait AppInfo {
    /// Returns the display name of the application.
    fn name(&self) -> &str;
    /// Returns the version string.
    fn version(&self) -> &str;
    /// Returns the author's name. Defaults to "Unknown".
    fn author(&self) -> &str { "Unknown" }
    /// Returns the application's 32x32 icon data (1024 pixels).
    fn icon(&self) -> [u32; 1024];
    /// Returns the preferred window dimensions (width, height).
    fn dimensions(&self) -> (usize, usize);
}

impl Application {
    pub fn new(inner: Box<dyn Runnable>) -> Self
        {
            Application {
                name: "application".to_string(),
                version: "0.0.1".to_string(),
                jit_entry: None,
                local_env: Environment::new(),
                inner,
                dimensions: (400, 300),
            }
        }
    pub fn dimensions(&self) -> [usize; 2] { [self.dimensions.0, self.dimensions.1] }
    pub fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) { self.inner.draw(graphics_entity, vars, x, y); }
    pub fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) { self.inner.logic(vars, env); }
    pub fn input(&mut self, key: Key) { self.inner.input(key); }
}

impl Background {
    pub fn new(inner: Box<dyn BackgroundTask>) -> Self {
        Background {
            name: "background_task".to_string(),
            version: "0.0.1".to_string(),
            jit_entry: None,
            local_env: None,
            inner,
        }
    }
    pub fn tick(&mut self, vars: &mut Vec<String>, env: &mut Environment) -> bool {
        self.inner.tick(vars, env)
    }
}

/// Local context for an app

/// Context for running an application in a blocking loop.
#[deprecated(since = "1.5.4", note = "use SteppedApplicationContext instead")]
pub struct ApplicationContext {
    pub parent: Unknown<Application>,
    pub application: Application,
    pub background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: BTreeMap<String, String>,
    pub environment: Environment,
    pub exit_requested: bool,
}

/// Execution context for an application that can be stepped manually.
///
/// This is used by the windowing system to update multiple applications
///
#[derive(Clone)]
pub struct SteppedApplicationContext {
    pub parent: Unknown<Application>,
    pub application: Application,
    pub background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: BTreeMap<String, String>,
    pub environment: Environment,
    pub local_vars: Vec<String>,
    pub window: WindowState,
    pub exit_requested: bool,
    pub pid: usize,
    pub ui_time: usize,
    pub cpu_time: usize,
}

pub enum AppOrBackground {
    App(Application),
    Background(Background),
}

pub struct BackgroundSteppedApplicationContext {
    pub parent: Unknown<AppOrBackground>,
    pub background: Background,
    pub metadata: BTreeMap<String, String>,
    pub environment: Environment,
    pub local_vars: Vec<String>,
    pub exit_requested: bool,
    pub pid: usize,
    pub cpu_time: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct WindowState {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub is_minimized: bool,
    pub is_maximized: bool,
    pub restore_x: usize,
    pub restore_y: usize,
    pub restore_width: usize,
    pub restore_height: usize,
}

impl WindowState {
    pub const TITLE_BAR_HEIGHT: usize = 20;
    pub const BORDER: usize = 2;
    pub const MIN_WIDTH: usize = 120;
    pub const MIN_HEIGHT: usize = 80;

    pub fn new(x: usize, y: usize, content_width: usize, content_height: usize) -> Self {
        let width = core::cmp::max(content_width + Self::BORDER, Self::MIN_WIDTH);
        let height = core::cmp::max(content_height + Self::TITLE_BAR_HEIGHT, Self::MIN_HEIGHT);
        Self {
            x,
            y,
            width,
            height,
            is_minimized: false,
            is_maximized: false,
            restore_x: x,
            restore_y: y,
            restore_width: width,
            restore_height: height,
        }
    }

    pub fn content_origin(&self) -> (usize, usize) {
        (self.x + Self::BORDER, self.y + Self::TITLE_BAR_HEIGHT)
    }

    pub fn minimize(&mut self) {
        self.is_minimized = true;
    }

    pub fn unminimize(&mut self) {
        self.is_minimized = false;
    }

    pub fn toggle_minimize(&mut self) {
        self.is_minimized = !self.is_minimized;
    }

    pub fn maximize(&mut self, screen_width: usize, screen_height: usize) {
        if !self.is_maximized {
            self.restore_x = self.x;
            self.restore_y = self.y;
            self.restore_width = self.width;
            self.restore_height = self.height;
        }
        self.x = 0;
        self.y = 32;
        self.width = screen_width;
        self.height = screen_height.saturating_sub(60);
        self.is_maximized = true;
        self.is_minimized = false;
    }

    pub fn restore(&mut self) {
        if self.is_maximized {
            self.x = self.restore_x;
            self.y = self.restore_y;
            self.width = self.restore_width;
            self.height = self.restore_height;
            self.is_maximized = false;
        }
    }

    pub fn toggle_maximize(&mut self, screen_width: usize, screen_height: usize) {
        if self.is_maximized {
            self.restore();
        } else {
            self.maximize(screen_width, screen_height);
        }
    }

    pub fn move_by(&mut self, dx: isize, dy: isize, bounds: (usize, usize)) {
        if self.is_maximized {
            self.restore();
        }
        self.x = offset_clamped(self.x, dx, bounds.0.saturating_sub(self.width));
        self.y = offset_clamped(self.y, dy, bounds.1.saturating_sub(self.height));
    }

    pub fn resize_by(&mut self, dw: isize, dh: isize, bounds: (usize, usize)) {
        if self.is_maximized {
            self.restore();
        }
        let max_width = bounds.0.saturating_sub(self.x).max(Self::MIN_WIDTH);
        let max_height = bounds.1.saturating_sub(self.y).max(Self::MIN_HEIGHT);

        self.width = offset_clamped(self.width, dw, max_width).max(Self::MIN_WIDTH);
        self.height = offset_clamped(self.height, dh, max_height).max(Self::MIN_HEIGHT);
    }
}

fn offset_clamped(value: usize, delta: isize, max: usize) -> usize {
    if delta.is_negative() {
        value.saturating_sub(delta.unsigned_abs()).min(max)
    } else {
        value.saturating_add(delta as usize).min(max)
    }
}



#[derive(Clone)]
pub struct GlobalEnvironmentData {
    pub vms: Vec<VmDisplayInfo>,
    pub resources: SystemResources,
    pub scroll_offset: usize,
    pub console_scroll_offset: usize,
    pub console_h_scroll_offset: usize,
    pub current_path: String,
    pub files: Vec<FileEntry>,
    pub selected_file_idx: usize,
    pub categories: Vec<DeviceCategory>,
    pub selected_device_idx: usize,
    pub device_action_idx: usize,

    // Fields for Create VM UI
    pub new_vm_name: String,
    pub new_vm_memory_mb: u32,
    pub new_vm_vcpus: u32,
    pub create_vm_focus_idx: usize,
    pub vm_action_idx: usize, // For VM actions (0: Start, 1: Stop, 2: Reset, 3: Zero, 4: Delete)
    pub selected_vm_idx: usize,
    pub filesys_action_idx: usize,
    pub filesys_pending_action: Option<FilePendingAction>,
    pub filesys_new_counter: usize,
    pub term_selected: bool,
    pub term_buf: String,
    pub editor: Option<TextEditor>,
    pub package_manager: PackageManager,
    pub iter: u64,
    pub active_apps: Vec<SteppedApplicationContext>,
    pub focused_process_idx: Option<usize>, // Which app gets the keyboard?
    pub selected_app_idx: usize,
    pub app_window_position: (usize, usize),
    pub ctrl_mode: bool,
    pub alt_mode: bool,
    pub fn_mode: bool,
    pub selected_package_idx: usize,
    pub package_action_idx: usize,
    pub selected_network_action_idx: usize,
    pub network_target: String,
    pub selected_settings_category_idx: usize,
    pub selected_settings_idx: usize,
    pub settings: UiSettings,
    pub status_line: String,
    pub command_history: Vec<String>,
    pub history_idx: Option<usize>,

    // New functional UI features
    pub notifications: Vec<(String, usize)>, // (message, duration_frames)
    pub command_palette_active: bool,
    pub command_palette_query: String,
    pub command_palette_selected: usize,
    pub command_palette_scroll_offset: usize,
    pub startup_menu_active: bool,
    pub selected_startup_app: usize,

    pub glitch_y: usize,
    pub pci_devices: Vec<crate::hardware::pci::PciDeviceInfo>,
    pub tab_apps: BTreeMap<DashboardTab, XSteppedApplicationContext>,
    pub resmon_tab: ResourceMonitorTab,
    pub cycles: usize,
    pub selected_process_idx: usize,
}

impl GlobalEnvironmentData {
    pub fn new() -> Self {
        GlobalEnvironmentData {
        vms: Vec::new(),
        resources: SystemResources {
            total_memory_mb: 0,
            used_memory_mb: 0,
            cpu_count: 0,
            cpu_usage: 0,
            cpu_core_usage: Vec::new(),
            disk_read_kbps: 0,
            disk_write_kbps: 0,
            net_rx_kbps: 0,
            net_tx_kbps: 0,
            gpu_usage: 0,
            cpu_history: Vec::with_capacity(100),
            mem_history: Vec::with_capacity(100),
            disk_read_history: Vec::with_capacity(100),
            disk_write_history: Vec::with_capacity(100),
            net_rx_history: Vec::with_capacity(100),
            net_tx_history: Vec::with_capacity(100),
            gpu_history: Vec::with_capacity(100),
            fps_history: Vec::with_capacity(100),
            ft_ms_history: Vec::with_capacity(100),
            fps: 0,
            frame_ms: 0,
        },
        scroll_offset: 0,
        console_scroll_offset: 0,
        console_h_scroll_offset: 0,
        current_path: String::from("\\"),
        files: Vec::new(),
        selected_file_idx: 0,
        categories: Vec::new(),
        selected_device_idx: 0,
        device_action_idx: 0,
        new_vm_name: String::from("NewVM"),
        new_vm_memory_mb: 256,
        new_vm_vcpus: 1,
        create_vm_focus_idx: 0,
        vm_action_idx: 0,
        selected_vm_idx: 0,
        filesys_action_idx: 0,
        filesys_pending_action: None,
        filesys_new_counter: 1,
        term_selected: false,
        term_buf: "".to_string(),
        editor: None,
        package_manager: PackageManager::new(),
        iter: 0,
        active_apps: Vec::new(),
        focused_process_idx: None,
        selected_app_idx: 0,

        app_window_position: (100, 100),
        ctrl_mode: false,
        alt_mode: false,
        fn_mode: false,
        selected_package_idx: 0,
        package_action_idx: 0,
        selected_network_action_idx: 0,
        network_target: String::from("127.0.0.1"),
        selected_settings_category_idx: 0,
        selected_settings_idx: 0,
        settings: UiSettings {
            extra_debug_info: false,
            folder_absolute_sizes: false,
            state_save_restore: true,
            extended_symbol_library: true,
            ring0_udmi_udxi: false,
            controllang_support: false,
            pg_vshaders: true,
            experimental_mem_comp: false,
            auto_refresh_storage: true,
            show_hidden_files: false,
            general_profile: 0,
            boot_target: 0,
            interface_density: 0,
            vm_safety_policy: 0,
            network_profile: 0,
            storage_policy: 0,
            package_policy: 0,
            developer_level: 0,
            security_policy: 0,
            ui_scaling: 1, // 100%
            terminal_font: 0,
            pg_scanlines: false,
            pg_dither: false,
            pg_glitch: false,
            pg_aberration: 0,
        },
        status_line: String::from("Ready"),
        command_history: Vec::new(),
        history_idx: None,
        notifications: Vec::new(),
        command_palette_active: false,
        command_palette_query: String::new(),
        command_palette_selected: 0,
        command_palette_scroll_offset: 0,
        startup_menu_active: false,
        selected_startup_app: 0,
        glitch_y: 0,
        pci_devices: Vec::new(),
        tab_apps: BTreeMap::new(),
        resmon_tab: ResourceMonitorTab::Resources,
        cycles: 0,
        selected_process_idx: 0,
        }
    }

    pub fn pull_from_ui(&mut self, ui: DashboardUI) {
        self.vms = ui.vms;
        self.resources = ui.resources;
        self.scroll_offset = ui.scroll_offset;
        self.console_scroll_offset = ui.scroll_offset;
        self.console_h_scroll_offset = ui.scroll_offset;
        self.current_path = ui.current_path;
        self.files = ui.files;
        self.selected_file_idx = ui.selected_file_idx;
        self.categories = ui.categories;
        self.selected_device_idx = ui.selected_device_idx;
        self.device_action_idx = ui.device_action_idx;
        self.new_vm_name = ui.new_vm_name;
        self.new_vm_memory_mb = ui.new_vm_memory_mb;
        self.new_vm_vcpus = ui.new_vm_vcpus;
        self.create_vm_focus_idx = ui.create_vm_focus_idx;
        self.vm_action_idx = ui.vm_action_idx;
        self.selected_vm_idx = ui.selected_vm_idx;
        self.filesys_action_idx = ui.filesys_action_idx;
        self.filesys_pending_action = ui.filesys_pending_action;
        self.filesys_new_counter = ui.filesys_new_counter;
        self.term_selected = ui.term_selected;
        self.term_buf = ui.term_buf;
        self.editor = ui.editor;
        self.package_manager = ui.package_manager;
        self.iter = ui.iter;
        self.active_apps = ui.active_apps;
        self.focused_process_idx = ui.focused_process_idx;
        self.app_window_position = ui.app_window_position;
        self.ctrl_mode = ui.ctrl_mode;
        self.alt_mode = ui.alt_mode;
        self.fn_mode = ui.fn_mode;
        self.selected_package_idx = ui.selected_package_idx;
        self.package_action_idx = ui.package_action_idx;
        self.selected_network_action_idx = ui.selected_network_action_idx;
        self.network_target = ui.network_target;
        self.selected_settings_idx = ui.selected_settings_idx;
        self.settings = ui.settings;
        self.status_line = ui.status_line;
        self.command_history = ui.command_history;
        self.history_idx = ui.history_idx;
        self.notifications = ui.notifications;
        self.command_palette_active = ui.command_palette_active;
        self.command_palette_selected = ui.command_palette_selected;
        self.command_palette_scroll_offset = ui.command_palette_scroll_offset;
        self.startup_menu_active = ui.startup_menu_active;
        self.selected_startup_app = ui.selected_startup_app;
        self.glitch_y = ui.glitch_y;
        self.pci_devices = ui.pci_devices;
        self.tab_apps = ui.tab_apps;
        self.resmon_tab = ui.resmon_tab;
        self.cycles = ui.cycles;
        self.selected_process_idx = ui.selected_process_idx;

    }

    pub fn pull_from_ui_thru(&mut self, ui: DashboardUI) -> DashboardUI {
        self.vms = ui.vms.clone();
        self.resources = ui.resources.clone();
        self.scroll_offset = ui.scroll_offset.clone();
        self.console_scroll_offset = ui.scroll_offset.clone();
        self.console_h_scroll_offset = ui.scroll_offset.clone();
        self.current_path = ui.current_path.clone();
        self.files = ui.files.clone();
        self.selected_file_idx = ui.selected_file_idx.clone();
        self.categories = ui.categories.clone();
        self.selected_device_idx = ui.selected_device_idx.clone();
        self.device_action_idx = ui.device_action_idx.clone();
        self.new_vm_name = ui.new_vm_name.clone();
        self.new_vm_memory_mb = ui.new_vm_memory_mb.clone();
        self.new_vm_vcpus = ui.new_vm_vcpus.clone();
        self.create_vm_focus_idx = ui.create_vm_focus_idx.clone();
        self.vm_action_idx = ui.vm_action_idx.clone();
        self.selected_vm_idx = ui.selected_vm_idx.clone();
        self.filesys_action_idx = ui.filesys_action_idx.clone();
        self.filesys_pending_action = ui.filesys_pending_action.clone();
        self.filesys_new_counter = ui.filesys_new_counter.clone();
        self.term_selected = ui.term_selected.clone();
        self.term_buf = ui.term_buf.clone();
        self.editor = ui.editor.clone();
        self.package_manager = ui.package_manager.clone();
        self.iter = ui.iter.clone();
        self.active_apps = ui.active_apps.clone();
        self.focused_process_idx = ui.focused_process_idx.clone();
        self.app_window_position = ui.app_window_position.clone();
        self.ctrl_mode = ui.ctrl_mode.clone();
        self.alt_mode = ui.alt_mode.clone();
        self.fn_mode = ui.fn_mode.clone();
        self.selected_package_idx = ui.selected_package_idx.clone();
        self.package_action_idx = ui.package_action_idx.clone();
        self.selected_network_action_idx = ui.selected_network_action_idx.clone();
        self.network_target = ui.network_target.clone();
        self.selected_settings_idx = ui.selected_settings_idx.clone();
        self.settings = ui.settings.clone();
        self.status_line = ui.status_line.clone();
        self.command_history = ui.command_history.clone();
        self.history_idx = ui.history_idx.clone();
        self.notifications = ui.notifications.clone();
        self.command_palette_active = ui.command_palette_active.clone();
        self.command_palette_selected = ui.command_palette_selected.clone();
        self.command_palette_scroll_offset = ui.command_palette_scroll_offset.clone();
        self.startup_menu_active = ui.startup_menu_active.clone();
        self.selected_startup_app = ui.selected_startup_app.clone();
        self.glitch_y = ui.glitch_y.clone();
        self.pci_devices = ui.pci_devices.clone();
        self.tab_apps = ui.tab_apps.clone();
        self.resmon_tab = ui.resmon_tab.clone();
        self.cycles = ui.cycles.clone();
        self.selected_process_idx = ui.selected_process_idx.clone();
        ui

    }
}












#[allow(deprecated)]
impl ApplicationContext {
    pub fn new(app: Application, background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>) -> ApplicationContext {
        ApplicationContext {
            parent: None,
            application: app,
            background_tasks,
            global: false,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            exit_requested: false,
        }
    }

    // pub unsafe fn run(&mut self){
    //     self.application.local_env = Some(self.environment.clone());
    //     // more stuff
    //     LOGGING_SILENCED = true;
    //
    //     let mut app_local_vars = Vec::new();
    //     loop {
    //         if self.exit_requested {
    //             if let Some(mut pg) = PixelGraphics::new() {
    //                 let mut pg = pg.with_backbuffer();
    //                 let (width, height) = pg.resolution();
    //                 pg.exit()
    //             }
    //             break
    //         }
    //
    //         if let Some(tasks) = self.background_tasks.as_mut() {
    //             tasks.retain_mut(|task| !task.tick());
    //         }
    //         /// Process chain: draw, logic, input
    //         if let Some(mut pg) = PixelGraphics::new() {
    //             let mut pg = pg.with_backbuffer();
    //             let (width, height) = pg.resolution();
    //
    //             // Draw background
    //             pg.clear(0x222222);
    //             pg.app_context_border(&self.application.name);
    //             self.application.draw(&mut pg, &app_local_vars, 200, 200); // UI only has read access to local vars
    //
    //             pg.flip();
    //         }
    //
    //         self.application.logic(&mut app_local_vars); // App logic has RW access to local vars
    //         let key = system::with_stdin(|i| {
    //             match i.read_key() {
    //                 Ok(Some(key)) => Some(key),
    //                 _ => None,
    //             }
    //         });
    //
    //         if let Some(key) = key {
    //             self.handle_input(key)
    //         }
    //
    //     }
    //     uefi::system::with_stdout(|stdout| {
    //         // Reset() is the most effective way to tell UEFI "ignore previous pixels, start over"
    //         let _ = stdout.reset(true);
    //         let _ = stdout.clear();
    //     });
    //     LOGGING_SILENCED = false;
    // }
    // pub fn handle_input(&mut self, key: Key) {
    //     use uefi::proto::console::text::ScanCode;
    //     match key {
    //         Key::Special(ScanCode::ESCAPE) => {
    //             self.exit_requested = true;
    //         }
    //         _ => {
    //             self.application.input(key);
    //         }
    //     }
    // }

    pub fn from_name(name: &str) -> Option<ApplicationContext> {
        let registry_entry = crate::apps::APP_REGISTRY.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        Some(ApplicationContext::new(app, None))
    }
}


impl SteppedApplicationContext {
    pub fn new(app: Application, background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>) -> SteppedApplicationContext {
        let dimensions = app.dimensions;
        let mut rng = XorShiftRng::new(12);
        let mut id0 = rng.rand_range(2000, 9000) as usize;
        SteppedApplicationContext {
            parent: None,
            application: app,
            background_tasks,
            global: false,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            local_vars: Vec::new(),
            window: WindowState::new(100, 100, dimensions.0, dimensions.1),
            exit_requested: false,
            pid: id0,
            ui_time: 0,
            cpu_time: 0,
        }
    }

    pub fn with_window_position(mut self, x: usize, y: usize) -> Self {
        self.window.x = x;
        self.window.y = y;
        self.window.restore_x = x;
        self.window.restore_y = y;
        self
    }

    /// Performs one 'tick' of the application.
    /// Returns true if the app is still running, false if it wants to exit.
    pub fn step(&mut self, key: Option<Key>) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

        // Pull data from GLOBALENV if available
        self.environment.global_data = global_data_ref();

        // 1. Update Environment
        self.application.local_env = self.environment.clone();

        // 2. Run background work and app logic.
        if let Some(tasks) = self.background_tasks.as_mut() {
            tasks.retain_mut(|task| !task.tick(&mut self.local_vars, &mut self.environment));
        }
        self.application.logic(&mut self.local_vars, &mut self.environment);


        // 3. Handle forwarded input
        if let Some(k) = key {
            self.handle_input(k);
        }

        let end_busy = unsafe { core::arch::x86_64::_rdtsc() };
        unsafe {
            crate::hpvmlog::BUSY_TSC = crate::hpvmlog::BUSY_TSC.saturating_add(end_busy.saturating_sub(start_busy));
        }

        !self.exit_requested
    }

    pub fn draw(&self, graphics_entity: &mut PixelGraphics) {
        let (x, y) = self.window.content_origin();
        self.application.draw(graphics_entity, &self.local_vars, x, y);
    }

    pub fn move_window_by(&mut self, dx: isize, dy: isize, bounds: (usize, usize)) {
        self.window.move_by(dx, dy, bounds);
    }

    pub fn resize_window_by(&mut self, dw: isize, dh: isize, bounds: (usize, usize)) {
        self.window.resize_by(dw, dh, bounds);
    }
    pub fn handle_input(&mut self, key: Key) {
        use uefi::proto::console::text::ScanCode;
        if matches!(key, Key::Special(ScanCode::ESCAPE)) {
            self.exit_requested = true;
        } else {
            self.application.input(key);
        }
    }
    pub fn from_name(name: &str) -> Option<SteppedApplicationContext> {
        let registry_entry = crate::apps::APP_REGISTRY.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        let mut ctx = SteppedApplicationContext::new(app, None);
        ctx.window = WindowState::new(100, 100, dims.0, dims.1);

        Some(ctx)
    }
    pub fn from_name_custom_registry(name: &str, registry: &[(&str, AppConstructor, ICON32, &str)]) -> Option<SteppedApplicationContext> {
        let registry_entry = registry.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        let mut ctx = SteppedApplicationContext::new(app, None);
        ctx.window = WindowState::new(100, 100, dims.0, dims.1);

        Some(ctx)
    }

    /// Spawns an asynchronous future as a background task.
    pub fn spawn_async<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = AsyncBackgroundTask::new(future);
        match self.background_tasks.as_mut() {
            Some(tasks) => tasks.push(Box::new(task)),
            None => self.background_tasks = Some(alloc::vec![Box::new(task)]),
        }
    }

    /// Spawns a background task.
    pub fn spawn_task<T: BackgroundTask + 'static>(&mut self, task: T) {
        match self.background_tasks.as_mut() {
            Some(tasks) => tasks.push(Box::new(task)),
            None => self.background_tasks = Some(alloc::vec![Box::new(task)]),
        }
    }

    /// Advances the application by one step in a polling context.
    /// Returns `Poll::Pending` if running, or `Poll::Ready(())` if exit was requested.
    pub fn poll_step(&mut self, key: Option<Key>) -> Poll<()> {
        if self.step(key) {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl Future for SteppedApplicationContext {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_step(None)
    }
}

impl BackgroundSteppedApplicationContext {
    pub fn new(background: Background) -> Self {
        let mut rng = XorShiftRng::new(12);
        let mut id0 = rng.rand_range(9001, 15000) as usize;
        Self {
            parent: None,
            background,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            local_vars: Vec::new(),
            exit_requested: false,
            pid: id0,
            cpu_time: 0,
        }
    }

    pub fn step(&mut self) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

        self.background.local_env = Some(self.environment.clone());
        if self.background.tick(&mut self.local_vars, &mut self.environment) {
            self.exit_requested = true;
        }

        let end_busy = unsafe { core::arch::x86_64::_rdtsc() };
        unsafe {
            crate::hpvmlog::BUSY_TSC = crate::hpvmlog::BUSY_TSC.saturating_add(end_busy.saturating_sub(start_busy));
        }

        !self.exit_requested
    }

    /// Advances the background task by one step in a polling context.
    /// Returns `Poll::Pending` if running, or `Poll::Ready(())` if exit was requested.
    pub fn poll_step(&mut self) -> Poll<()> {
        if self.step() {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl Future for BackgroundSteppedApplicationContext {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_step()
    }
}
/// Alias of Option\<T\>
pub type Unknown<T> = Option<T>;



#[derive(Clone)]
pub struct XSteppedApplicationContext {
    pub parent: Unknown<Application>,
    pub application: Application,
    pub background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: BTreeMap<String, String>,
    pub environment: Environment,
    pub local_vars: Vec<String>,
    pub window: WindowState,
    pub exit_requested: bool,

    // UI-like environment recreation
    pub selected_tab: u8, // Using u8 to avoid circular dependency with DashboardTab if possible, or we can use an int
    pub status_line: String,
    pub current_path: String,
    pub cursor: (usize, usize),
    pub scroll_offset: (usize, usize),
    pub focused_idx: usize,
    pub action_idx: usize,
    pub selection_idx: usize,
    pub edit_buffer: String,
    pub search_query: String,
    pub from_ui: bool,
    pub pid: usize,
    pub ui_time: usize,
    pub cpu_time: usize,
}

impl XSteppedApplicationContext {
    pub fn new(app: Application, background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>) -> Self {
        let dims = app.dimensions;
        let mut rng = XorShiftRng::new(12);
        let mut id0 = rng.rand_range(1000, 1999) as usize;
        Self {
            parent: None,
            application: app,
            background_tasks,
            global: false,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            local_vars: Vec::new(),
            window: WindowState::new(100, 100, dims.0, dims.1),
            exit_requested: false,

            selected_tab: 0,
            status_line: String::new(),
            current_path: String::from("\\"),
            cursor: (0, 0),
            scroll_offset: (0, 0),
            focused_idx: 0,
            action_idx: 0,
            selection_idx: 0,
            edit_buffer: String::new(),
            search_query: String::new(),
            from_ui: false,
            pid: id0,
            ui_time: 0,
            cpu_time: 0,
        }
    }

    pub fn with_window_position(mut self, x: usize, y: usize) -> Self {
        self.window.x = x;
        self.window.y = y;
        self.window.restore_x = x;
        self.window.restore_y = y;
        self
    }

    pub fn step(&mut self, key: Option<Key>) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

        // Pull data from GLOBALENV if available
        self.environment.global_data = global_data_ref();

        // 0. Handle forwarded input BEFORE logic so logic can see updated state from input if needed
        // (though logic usually pulls from global_data which is updated above)
        if let Some(k) = key {
            self.handle_input(k);
        }

        // 1. Run application logic
        self.application.logic(&mut self.local_vars, &mut self.environment);

        // 2. Run background tasks
        if let Some(tasks) = self.background_tasks.as_mut() {
            tasks.retain_mut(|task| !task.tick(&mut self.local_vars, &mut self.environment));
        }

        let end_busy = unsafe { core::arch::x86_64::_rdtsc() };
        unsafe {
            crate::hpvmlog::BUSY_TSC = crate::hpvmlog::BUSY_TSC.saturating_add(end_busy.saturating_sub(start_busy));
        }

        !self.exit_requested
    }

    pub fn draw(&self, graphics_entity: &mut PixelGraphics) {
        let (x, y) = self.window.content_origin();
        self.application.draw(graphics_entity, &self.local_vars, x, y);
    }

    pub fn move_window_by(&mut self, dx: isize, dy: isize, bounds: (usize, usize)) {
        self.window.move_by(dx, dy, bounds);
    }

    pub fn resize_window_by(&mut self, dw: isize, dh: isize, bounds: (usize, usize)) {
        self.window.resize_by(dw, dh, bounds);
    }

    pub fn handle_input(&mut self, key: Key) {
        use uefi::proto::console::text::ScanCode;
        match key {
            Key::Special(ScanCode::ESCAPE) => {
                self.exit_requested = true;
            }
            _ => {
                self.application.input(key);
            }
        }
    }

    pub fn from_name(name: &str) -> Option<XSteppedApplicationContext> {
        let registry_entry = crate::apps::APP_REGISTRY.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        let mut ctx = XSteppedApplicationContext::new(app, None);
        ctx.window = WindowState::new(100, 100, dims.0, dims.1);

        Some(ctx)
    }

    pub fn from_name_custom_registry(name: &str, registry: &[(&str, AppConstructor, ICON32, &str)]) -> Option<XSteppedApplicationContext> {
        let registry_entry = registry.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        let mut ctx = XSteppedApplicationContext::new(app, None);
        ctx.window = WindowState::new(100, 100, dims.0, dims.1);

        Some(ctx)
    }

    /// Avoid using this function as it may cause issues (dashboard should pull from GLOBALENV)
    pub fn sync_back_to_dashboard(&self, ui: &mut crate::ui::DashboardUI) {
        if !self.from_ui { return; }
        
        let tab = crate::ui::DashboardTab::from_u8(self.selected_tab);
        
        match tab {
            crate::ui::DashboardTab::Overview => {
                // Overview usually doesn't push state back yet
            }
            crate::ui::DashboardTab::VirtualMachines => {
                if let Some(vms_app) = self.application.inner.as_any().downcast_ref::<X_VMs>() {
                    ui.selected_vm_idx = vms_app.selected_vm_idx;
                    ui.vm_action_idx = vms_app.vm_action_idx;
                }
            }
            crate::ui::DashboardTab::Storage => {
                if let Some(storage) = self.application.inner.as_any().downcast_ref::<X_Storage>() {
                    ui.current_path = storage.current_path.clone();
                    ui.selected_file_idx = storage.selected_file_idx;
                    ui.filesys_action_idx = storage.filesys_action_idx;
                    ui.filesys_pending_action = storage.filesys_pending_action;
                    ui.status_line = storage.status_line.clone();
                    ui.filesys_new_counter = storage.filesys_new_counter;
                }
            }
            crate::ui::DashboardTab::Apps => {
                if let Some(apps) = self.application.inner.as_any().downcast_ref::<X_Apps>() {
                    ui.selected_app_idx = apps.selected_app_idx;
                }
            }
            crate::ui::DashboardTab::Resources => {
                if let Some(resources) = self.application.inner.as_any().downcast_ref::<X_Resources>() {
                    ui.resources = resources.resources.clone();
                }
            }
            crate::ui::DashboardTab::Network => {
                if let Some(network) = self.application.inner.as_any().downcast_ref::<X_Network>() {
                    ui.selected_network_action_idx = network.selected_network_action_idx;
                    ui.network_target = network.network_target.clone();
                }
            }
            crate::ui::DashboardTab::Console => {
                if let Some(console) = self.application.inner.as_any().downcast_ref::<X_Console>() {
                    ui.term_buf = console.term_buf.clone();
                }
            }
            crate::ui::DashboardTab::Devices => {
                if let Some(devices) = self.application.inner.as_any().downcast_ref::<X_Devices>() {
                    ui.categories = devices.categories.clone();
                    ui.selected_device_idx = devices.selected_device_idx;
                }
            }
            crate::ui::DashboardTab::Packages => {
                if let Some(pkg) = self.application.inner.as_any().downcast_ref::<X_Packages>() {
                    ui.selected_package_idx = pkg.selected_package_idx;
                    ui.package_action_idx = pkg.package_action_idx;
                    ui.status_line = pkg.status_line.clone();
                }
            }
            crate::ui::DashboardTab::Settings => {
                if let Some(settings) = self.application.inner.as_any().downcast_ref::<X_Settings>() {
                    ui.settings = settings.settings.clone();
                    ui.selected_settings_idx = settings.selected_settings_idx;
                    ui.selected_settings_category_idx = settings.selected_settings_category_idx;
                }
            }
            crate::ui::DashboardTab::CreateVM => {
                if let Some(cvm) = self.application.inner.as_any().downcast_ref::<X_CreateVM>() {
                    ui.new_vm_name = cvm.new_vm_name.clone();
                    ui.new_vm_memory_mb = cvm.new_vm_memory_mb;
                    ui.new_vm_vcpus = cvm.new_vm_vcpus;
                    ui.create_vm_focus_idx = cvm.create_vm_focus_idx;
                }
            }
            DashboardTab::Editor => {
                if let Some(editor) = self.application.inner.as_any().downcast_ref::<X_Editor>() {
                     // ui.editor = editor.editor.clone(); // Not implemented in ui yet?
                }
            }
            _ => {}
        }
    }

    pub fn from_dashboard(ui: &DashboardUI, tab: DashboardTab) -> Self {
        let name = match tab {
            crate::ui::DashboardTab::Overview => "X_Overview",
            crate::ui::DashboardTab::VirtualMachines => "X_VMs",
            crate::ui::DashboardTab::Storage => "X_FileManager",
            crate::ui::DashboardTab::Resources => "X_Resources",
            crate::ui::DashboardTab::Apps => "X_Apps",
            crate::ui::DashboardTab::Network => "X_Network",
            crate::ui::DashboardTab::Console => "X_Console",
            crate::ui::DashboardTab::Devices => "X_Devices",
            crate::ui::DashboardTab::Settings => "X_Settings",
            crate::ui::DashboardTab::Packages => "X_Packages",
            crate::ui::DashboardTab::Test => "X_Test",
            crate::ui::DashboardTab::CreateVM => "X_CreateVM",
            crate::ui::DashboardTab::Editor => "X_Editor",
            _ => "X_Overview",
        };

        let mut ctx = Self::from_name(name).unwrap_or_else(|| {
            let (app_logic, dims) = crate::apps::error::ErrorApp::new("App not found");
            let mut app = Application::new(app_logic);
            app.name = name.to_string();
            app.dimensions = dims;
            XSteppedApplicationContext::new(app, None)
        });

        ctx.from_ui = true;
        ctx.selected_tab = tab as u8;
        // Tab applications render inside the dashboard content area rather
        // than as independent movable windows.
        ctx.window = WindowState::new(0, 80, ctx.application.dimensions.0, ctx.application.dimensions.1);
        ctx.current_path = ui.current_path.clone();
        ctx.selection_idx = ui.selected_vm_idx;

        match tab {
            crate::ui::DashboardTab::Overview => {
                if let Some(overview) = ctx.application.inner.as_any_mut().downcast_mut::<X_Overview>() {
                    overview.cpu_count = ui.resources.cpu_count;
                    overview.cpu_usage = ui.resources.cpu_usage;
                    overview.used_memory_mb = ui.resources.used_memory_mb;
                    overview.total_memory_mb = ui.resources.total_memory_mb;
                    overview.disk_read_kbps = ui.resources.disk_read_kbps as u32;
                    overview.disk_write_kbps = ui.resources.disk_write_kbps as u32;
                    overview.net_rx_kbps = ui.resources.net_rx_kbps as u32;
                    overview.net_tx_kbps = ui.resources.net_tx_kbps as u32;
                    overview.running_vms = ui.vms.iter().filter(|v| v.state.contains("Running")).count();
                    overview.total_vms = ui.vms.len();
                    overview.files_count = ui.files.len();
                    overview.categories_count = ui.categories.len();
                }
            }
            crate::ui::DashboardTab::VirtualMachines => {
                if let Some(vms_app) = ctx.application.inner.as_any_mut().downcast_mut::<X_VMs>() {
                    vms_app.vms = ui.vms.clone();
                    vms_app.selected_vm_idx = ui.selected_vm_idx;
                    vms_app.vm_action_idx = ui.vm_action_idx;
                }
            }
            crate::ui::DashboardTab::Storage => {
                if let Some(storage) = ctx.application.inner.as_any_mut().downcast_mut::<X_Storage>() {
                    storage.current_path = ui.current_path.clone();
                    storage.files = ui.files.clone();
                    storage.selected_file_idx = ui.selected_file_idx;
                    storage.filesys_action_idx = ui.filesys_action_idx;
                    storage.filesys_pending_action = ui.filesys_pending_action;
                    storage.status_line = ui.status_line.clone();
                    storage.filesys_new_counter = ui.filesys_new_counter;
                }
            }
            crate::ui::DashboardTab::Apps => {
                if let Some(apps) = ctx.application.inner.as_any_mut().downcast_mut::<X_Apps>() {
                    apps.selected_app_idx = ui.selected_app_idx;
                }
            }
            crate::ui::DashboardTab::Console => {
                if let Some(console) = ctx.application.inner.as_any_mut().downcast_mut::<X_Console>() {
                    console.term_buf = ui.term_buf.clone();
                }
            }
            crate::ui::DashboardTab::Resources => {
                if let Some(res) = ctx.application.inner.as_any_mut().downcast_mut::<X_Resources>() {
                    res.resources = ui.resources.clone();
                }
            }
            crate::ui::DashboardTab::Network => {
                if let Some(net) = ctx.application.inner.as_any_mut().downcast_mut::<X_Network>() {
                    net.selected_network_action_idx = ui.selected_network_action_idx;
                    net.network_target = ui.network_target.clone();
                }
            }
            crate::ui::DashboardTab::Devices => {
                if let Some(dev) = ctx.application.inner.as_any_mut().downcast_mut::<X_Devices>() {
                    dev.categories = ui.categories.clone();
                    dev.selected_device_idx = ui.selected_device_idx;
                }
            }
            crate::ui::DashboardTab::Settings => {
                if let Some(settings) = ctx.application.inner.as_any_mut().downcast_mut::<X_Settings>() {
                    settings.settings = ui.settings.clone();
                    settings.selected_settings_idx = ui.selected_settings_idx;
                    settings.selected_settings_category_idx = ui.selected_settings_category_idx;
                }
            }
            crate::ui::DashboardTab::Packages => {
                if let Some(pkg) = ctx.application.inner.as_any_mut().downcast_mut::<X_Packages>() {
                    pkg.selected_package_idx = ui.selected_package_idx;
                    pkg.package_action_idx = ui.package_action_idx;
                    pkg.registry = ui.package_manager.registry.clone();
                    pkg.status_line = ui.status_line.clone();
                }
            }
            crate::ui::DashboardTab::CreateVM => {
                if let Some(cvm) = ctx.application.inner.as_any_mut().downcast_mut::<X_CreateVM>() {
                    cvm.new_vm_name = ui.new_vm_name.clone();
                    cvm.new_vm_memory_mb = ui.new_vm_memory_mb;
                    cvm.new_vm_vcpus = ui.new_vm_vcpus;
                    cvm.create_vm_focus_idx = ui.create_vm_focus_idx;
                }
            }
            crate::ui::DashboardTab::Editor => {
                if let Some(editor_app) = ctx.application.inner.as_any_mut().downcast_mut::<X_Editor>() {
                    // Sync editor state if needed
                }
            }
            crate::ui::DashboardTab::Test => {
                if let Some(test) = ctx.application.inner.as_any_mut().downcast_mut::<X_Test>() {
                    // X_Test doesn't have an iter field in its struct definition
                }
            }
            _ => {}
        }

        ctx
    }

    /// Spawns an asynchronous future as a background task.
    pub fn spawn_async<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = AsyncBackgroundTask::new(future);
        match self.background_tasks.as_mut() {
            Some(tasks) => tasks.push(Box::new(task)),
            None => self.background_tasks = Some(alloc::vec![Box::new(task)]),
        }
    }

    /// Spawns a background task.
    pub fn spawn_task<T: BackgroundTask + 'static>(&mut self, task: T) {
        match self.background_tasks.as_mut() {
            Some(tasks) => tasks.push(Box::new(task)),
            None => self.background_tasks = Some(alloc::vec![Box::new(task)]),
        }
    }

    /// Advances the application by one step in a polling context.
    /// Returns `Poll::Pending` if running, or `Poll::Ready(())` if exit was requested.
    pub fn poll_step(&mut self, key: Option<Key>) -> Poll<()> {
        if self.step(key) {
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

impl Future for XSteppedApplicationContext {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.poll_step(None)
    }
}

/// Runs self-tests for the async/await multitasking subsystem.
///
/// Verifies:
/// 1. `AsyncBackgroundTask` step progression and termination across multiple yields.
/// 2. `SteppedApplicationContext::spawn_async` execution and lifecycle.
/// 3. `XSteppedApplicationContext::spawn_async` execution and lifecycle.
/// 4. Context `.await` / polling as a `Future`.
/// 5. `BackgroundTaskFuture` adapter for synchronous background tasks.
/// 6. `sleep_ticks` delay future.
pub fn run_async_tests() -> bool {
    use core::sync::atomic::AtomicUsize;

    // 1. Test AsyncBackgroundTask with yield_now and state update
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.store(0, Ordering::SeqCst);

    let task_fut = async {
        COUNTER.fetch_add(1, Ordering::SeqCst);
        yield_now().await;
        COUNTER.fetch_add(10, Ordering::SeqCst);
        yield_now().await;
        COUNTER.fetch_add(100, Ordering::SeqCst);
    };

    let mut async_task = AsyncBackgroundTask::new(task_fut);
    let mut vars = Vec::new();
    let mut env = Environment::new();

    // Tick 1: runs to first yield (counter = 1), returns false (pending)
    let done1 = async_task.tick(&mut vars, &mut env);
    if done1 || COUNTER.load(Ordering::SeqCst) != 1 {
        return false;
    }

    // Tick 2: runs to second yield (counter = 11), returns false (pending)
    let done2 = async_task.tick(&mut vars, &mut env);
    if done2 || COUNTER.load(Ordering::SeqCst) != 11 {
        return false;
    }

    // Tick 3: runs to end (counter = 111), returns true (ready)
    let done3 = async_task.tick(&mut vars, &mut env);
    if !done3 || COUNTER.load(Ordering::SeqCst) != 111 {
        return false;
    }

    // 2. Test SteppedApplicationContext spawn_async and stepping
    static APP_ASYNC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    APP_ASYNC_COUNTER.store(0, Ordering::SeqCst);

    #[derive(Clone)]
    struct DummyAppCloneable;
    impl Runnable for DummyAppCloneable {
        fn draw(&self, _: &mut PixelGraphics, _: &Vec<String>, _: usize, _: usize) {}
        fn logic(&mut self, _: &mut Vec<String>, _: &mut Environment) {}
        fn input(&mut self, _: Key) {}
        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
    }

    let dummy_app = Application::new(Box::new(DummyAppCloneable));
    let mut ctx = SteppedApplicationContext::new(dummy_app, None);
    ctx.spawn_async(async {
        APP_ASYNC_COUNTER.fetch_add(5, Ordering::SeqCst);
        yield_now().await;
        APP_ASYNC_COUNTER.fetch_add(50, Ordering::SeqCst);
    });

    // Step 1: counter becomes 5, task pending
    let _ = ctx.step(None);
    if APP_ASYNC_COUNTER.load(Ordering::SeqCst) != 5 {
        return false;
    }
    if ctx.background_tasks.as_ref().map(|t| t.len()).unwrap_or(0) != 1 {
        return false;
    }

    // Step 2: counter becomes 55, task completes and is removed
    let _ = ctx.step(None);
    if APP_ASYNC_COUNTER.load(Ordering::SeqCst) != 55 {
        return false;
    }
    if ctx.background_tasks.as_ref().map(|t| t.len()).unwrap_or(0) != 0 {
        return false;
    }

    // 3. Test XSteppedApplicationContext spawn_async and Future polling
    static X_ASYNC_COUNTER: AtomicUsize = AtomicUsize::new(0);
    X_ASYNC_COUNTER.store(0, Ordering::SeqCst);

    let x_dummy_app = Application::new(Box::new(DummyAppCloneable));
    let mut x_ctx = XSteppedApplicationContext::new(x_dummy_app, None);
    x_ctx.spawn_async(async {
        X_ASYNC_COUNTER.fetch_add(7, Ordering::SeqCst);
        sleep_ticks(2).await;
        X_ASYNC_COUNTER.fetch_add(70, Ordering::SeqCst);
    });

    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // Poll 1: runs to sleep_ticks(2), returns Pending
    let poll1 = Pin::new(&mut x_ctx).poll(&mut cx);
    if !poll1.is_pending() || X_ASYNC_COUNTER.load(Ordering::SeqCst) != 7 {
        return false;
    }

    // Poll 2: tick 1 of sleep_ticks, returns Pending
    let poll2 = Pin::new(&mut x_ctx).poll(&mut cx);
    if !poll2.is_pending() || X_ASYNC_COUNTER.load(Ordering::SeqCst) != 7 {
        return false;
    }

    // Poll 3: sleep_ticks completes, counter += 70, task finishes
    let poll3 = Pin::new(&mut x_ctx).poll(&mut cx);
    if !poll3.is_pending() || X_ASYNC_COUNTER.load(Ordering::SeqCst) != 77 {
        return false;
    }

    // Request exit and poll: should return Ready(())
    x_ctx.exit_requested = true;
    let poll_exit = Pin::new(&mut x_ctx).poll(&mut cx);
    if !poll_exit.is_ready() {
        return false;
    }

    true
}
