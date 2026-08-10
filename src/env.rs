//! Environment and application life-cycle management.
//!
//! This module provides the infrastructure for running applications,
//! managing their environments, and handling background tasks.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use uefi::fs::Path;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicBool, Ordering};
use uefi::proto::console::text::Key;
use crate::apps;
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
use crate::rng::XorShiftRng;

pub type EnvironmentVariable = (String, String);

static GLOBAL_ENV_READY: AtomicBool = AtomicBool::new(false);
static mut GLOBAL_ENV_VARS: MaybeUninit<BTreeMap<String, String>> = MaybeUninit::uninit();

fn global_env_vars_mut() -> &'static mut BTreeMap<String, String> {
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

}


impl Environment {
    pub fn new() -> Environment {
        Environment {
            cd: ("".to_string(), "".to_string()),
            xd: ("".to_string(), "".to_string()),
            tmp: ("".to_string(), "".to_string()),
            user: ("".to_string(), "".to_string()),
            devname: ("".to_string(), "".to_string()),
        }
    }
}

/// Global environment, not app-specific
/// Global system environment.
///
/// Contains system-wide variables like the number of processors and OS version.
pub struct GlobalEnvironment {
    pub cd: EnvironmentVariable,
    pub xd: EnvironmentVariable,
    pub tmp: EnvironmentVariable,
    pub user: EnvironmentVariable,
    pub devname: EnvironmentVariable,
    pub processor_count: EnvironmentVariable,
    pub os_version: EnvironmentVariable,
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
/// concurrently in the same main loop.
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
}

#[derive(Clone, Copy, Debug)]
pub struct WindowState {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl WindowState {
    pub const TITLE_BAR_HEIGHT: usize = 20;
    pub const BORDER: usize = 2;
    pub const MIN_WIDTH: usize = 120;
    pub const MIN_HEIGHT: usize = 80;

    pub fn new(x: usize, y: usize, content_width: usize, content_height: usize) -> Self {
        Self {
            x,
            y,
            width: core::cmp::max(content_width + Self::BORDER, Self::MIN_WIDTH),
            height: core::cmp::max(content_height + Self::TITLE_BAR_HEIGHT, Self::MIN_HEIGHT),
        }
    }

    pub fn content_origin(&self) -> (usize, usize) {
        (self.x + Self::BORDER, self.y + Self::TITLE_BAR_HEIGHT)
    }

    pub fn move_by(&mut self, dx: isize, dy: isize, bounds: (usize, usize)) {
        self.x = offset_clamped(self.x, dx, bounds.0.saturating_sub(self.width));
        self.y = offset_clamped(self.y, dy, bounds.1.saturating_sub(self.height));
    }

    pub fn resize_by(&mut self, dw: isize, dh: isize, bounds: (usize, usize)) {
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
        let mut id0 = rng.rand(5) as usize;
        if id0 < 1000 {
            id0 += 1000
        }
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
            pid: id0
        }
    }

    pub fn with_window_position(mut self, x: usize, y: usize) -> Self {
        self.window.x = x;
        self.window.y = y;
        self
    }

    /// Performs one 'tick' of the application.
    /// Returns true if the app is still running, false if it wants to exit.
    pub fn step(&mut self, key: Option<Key>) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

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
        match key {
            Key::Special(ScanCode::ESCAPE) => {
                self.exit_requested = true;
            }
            Key::Special(ScanCode::FUNCTION_2) => {
                //
            }
            _ => {
                self.application.input(key);
            }
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
}

impl BackgroundSteppedApplicationContext {
    pub fn new(background: Background) -> Self {
        Self {
            parent: None,
            background,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            local_vars: Vec::new(),
            exit_requested: false,
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
}

impl XSteppedApplicationContext {
    pub fn new(app: Application, background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>) -> Self {
        let dims = app.dimensions;
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
        }
    }

    pub fn with_window_position(mut self, x: usize, y: usize) -> Self {
        self.window.x = x;
        self.window.y = y;
        self
    }

    pub fn step(&mut self, key: Option<Key>) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

        // 1. Run application logic
        self.application.logic(&mut self.local_vars, &mut self.environment);

        // 2. Run background tasks
        if let Some(tasks) = self.background_tasks.as_mut() {
            tasks.retain_mut(|task| !task.tick(&mut self.local_vars, &mut self.environment));
        }

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
            _ => {}
        }
    }

    pub fn from_dashboard(ui: &crate::ui::DashboardUI, tab: crate::ui::DashboardTab) -> Self {
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
            _ => {}
        }

        ctx
    }
}
