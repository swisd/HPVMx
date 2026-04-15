use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use uefi::fs::Path;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::proto::console::text::{Key, OutputMode};
use uefi::system;
use crate::hpvmlog::LOGGING_SILENCED;
use crate::ui::pixel_graphics::PixelGraphics;

pub type EnvironmentVariable = (String, String);

/// Local environment, app-specific
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
pub struct GlobalEnvironment {
    pub cd: EnvironmentVariable,
    pub xd: EnvironmentVariable,
    pub tmp: EnvironmentVariable,
    pub user: EnvironmentVariable,
    pub devname: EnvironmentVariable,
    pub processor_count: EnvironmentVariable,
    pub os_version: EnvironmentVariable,
}


pub struct Application {
    pub name: String,
    pub version: String,
    pub jit_entry: Unknown<(String, u64)>,
    pub local_env: Unknown<Environment>,
    pub inner: Box<dyn Runnable>,
}


pub trait Runnable {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>); // Adjust types as needed
    fn logic(&mut self, vars: &mut Vec<String>);
    fn input(&mut self, key: Key);
}

pub trait BackgroundTask {
    // Returns true if finished, false if it needs more time
    fn tick(&mut self) -> bool;
}



impl Application {
    pub fn new<T>(inner: T) -> Self
        where
            T: Runnable + 'static
        {
            Application {
                name: "application".to_string(),
                version: "0.0.1".to_string(),
                jit_entry: None,
                local_env: None,
                inner: Box::new(inner),
            }
        }
    pub fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>) { self.inner.draw(graphics_entity, vars); }
    pub fn logic(&mut self, vars: &mut Vec<String>) { self.inner.logic(vars); }
    pub fn input(&mut self, key: Key) { self.inner.input(key); }
}

/// Local context for an app

pub struct ApplicationContext {
    pub parent: Unknown<Application>,
    pub application: Application,
    pub background_tasks: Unknown<Vec<Box<dyn BackgroundTask>>>,
    pub global: bool,
    pub metadata: BTreeMap<String, String>,
    pub environment: Environment,
    pub exit_requested: bool,
}

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
                self.application.draw(&mut pg, &app_local_vars); // UI only has read access to local vars

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
    fn handle_input(&mut self, key: Key) {
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
}


/// Alias of Option\<T\>
pub type Unknown<T> = Option<T>;