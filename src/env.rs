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
use uefi::proto::console::text::{Key, OutputMode};
use uefi::system;
use crate::apps;
use crate::hpvmlog::LOGGING_SILENCED;
use crate::ui::pixel_graphics::PixelGraphics;

pub type EnvironmentVariable = (String, String);

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
    fn new() -> Environment {
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
pub struct Application {
    /// The name of the application.
    pub name: String,
    /// The version string.
    pub version: String,
    /// Entry point for JIT-compiled code, if applicable.
    pub jit_entry: Unknown<(String, u64)>,
    /// The local environment variables for this application.
    pub local_env: Unknown<Environment>,
    /// The actual application logic.
    pub inner: Box<dyn Runnable>,
    /// The preferred window dimensions (width, height).
    pub dimensions: (usize, usize),
}


/// Core trait for application logic and rendering.
///
/// Any application that wants to be managed by the system must implement this trait.
pub trait Runnable {
    /// Renders the application to the provided `PixelGraphics` context.
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize); // Adjust types as needed
    /// Updates the application's internal state.
    fn logic(&mut self, vars: &mut Vec<String>);
    /// Handles a single keyboard input event.
    fn input(&mut self, key: Key);
}

/// Represents a task that runs in the background.
pub trait BackgroundTask {
    /// Performs a single tick of work.
    /// 
    /// Returns `true` if the task is finished and can be removed, 
    /// or `false` if it needs more processing time.
    fn tick(&mut self) -> bool;
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
                local_env: None,
                inner,
                dimensions: (400, 300),
            }
        }
    pub fn dimensions(&self) -> [usize; 2] { [self.dimensions.0, self.dimensions.1] }
    pub fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) { self.inner.draw(graphics_entity, vars, x, y); }
    pub fn logic(&mut self, vars: &mut Vec<String>) { self.inner.logic(vars); }
    pub fn input(&mut self, key: Key) { self.inner.input(key); }
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
    pub exit_requested: bool,
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

    pub unsafe fn run(&mut self){
        self.application.local_env = Some(self.environment.clone());
        // more stuff
        LOGGING_SILENCED = true;

        let mut app_local_vars = Vec::new();
        loop {
            if self.exit_requested {
                if let Some(mut pg) = PixelGraphics::new() {
                    let mut pg = pg.with_backbuffer();
                    let (width, height) = pg.resolution();
                    pg.exit()
                }
                break
            }

            if let Some(tasks) = self.background_tasks.as_mut() {
                tasks.retain_mut(|task| !task.tick());
            }
            /// Process chain: draw, logic, input
            if let Some(mut pg) = PixelGraphics::new() {
                let mut pg = pg.with_backbuffer();
                let (width, height) = pg.resolution();

                // Draw background
                pg.clear(0x222222);
                pg.app_context_border(&self.application.name);
                self.application.draw(&mut pg, &app_local_vars, 200, 200); // UI only has read access to local vars

                pg.flip();
            }

            self.application.logic(&mut app_local_vars); // App logic has RW access to local vars
            let key = system::with_stdin(|i| {
                match i.read_key() {
                    Ok(Some(key)) => Some(key),
                    _ => None,
                }
            });

            if let Some(key) = key {
                self.handle_input(key)
            }

        }
        uefi::system::with_stdout(|stdout| {
            // Reset() is the most effective way to tell UEFI "ignore previous pixels, start over"
            let _ = stdout.reset(true);
            let _ = stdout.clear();
        });
        LOGGING_SILENCED = false;
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
        SteppedApplicationContext {
            parent: None,
            application: app,
            background_tasks,
            global: false,
            metadata: BTreeMap::new(),
            environment: Environment::new(),
            exit_requested: false,
        }
    }

    /// Performs one 'tick' of the application.
    /// Returns true if the app is still running, false if it wants to exit.
    pub fn step(&mut self, key: Option<Key>) -> bool {
        let start_busy = unsafe { core::arch::x86_64::_rdtsc() };
        if self.exit_requested {
            return false;
        }

        // 1. Update Environment
        self.application.local_env = Some(self.environment.clone());

        // 2. Run logic and draw
        // Note: You may want to pass a sub-region/viewport here later
        let mut app_local_vars = Vec::new();
        self.application.logic(&mut app_local_vars);
        if let Some(pg) = PixelGraphics::new() {
            let mut pg = pg.with_backbuffer();
            self.application.draw(&mut pg, &mut app_local_vars, 200, 200);
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
    pub fn from_name(name: &str) -> Option<SteppedApplicationContext> {
        let registry_entry = crate::apps::APP_REGISTRY.iter()
            .find(|(app_id, _, _, _)| *app_id == name)?;

        let constructor = registry_entry.1;
        let (app_logic, dims) = constructor();

        let mut app = Application::new(app_logic);
        app.name = name.to_string();
        app.dimensions = dims;

        Some(SteppedApplicationContext::new(app, None))
    }
}


/// Alias of Option\<T\>
pub type Unknown<T> = Option<T>;