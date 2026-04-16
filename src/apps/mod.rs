//! Registry and management for parallel applications.
//!
//! This module contains the `APP_REGISTRY`, which is the central list of
//! all available applications in the system.
//!
//! Each application in the registry provides a name, a constructor,
//! an icon, and a version string.

use alloc::boxed::Box;
use crate::apps::appinstaller::AppInstallerApp;
use crate::apps::simple_app::SimpleApp;
use crate::apps::clock::ClockApp;
use crate::apps::keystepper::CH64App;
use crate::apps::manual::InstructionManualApp;
use crate::env::Runnable;
use crate::ui::pixel_graphics::icons;
use crate::ui::pixel_graphics::icons::ICON32;

pub(crate) mod simple_app;
pub mod keystepper;
mod clock;
mod cube;
mod netman;
mod manual;
mod appinstaller;

/// A type alias for a function that creates a boxed app and returns its preferred window dimensions.
type AppConstructor = fn() -> (Box<dyn Runnable>, (usize, usize));

/// The Registry: A static list of application names, their constructors, icons, and versions.
pub(crate) static APP_REGISTRY: &[(&str, AppConstructor, ICON32, &str)] = &[
    ("SimpleApp", || {
        let app = SimpleApp { color: [0x000000, 0xFFFFFF, 0xFF7700] };
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::TRAFFIC_LIGHT_32_ICON_DATA, "0.1.0"),
    ("Clock", || {
        let app = ClockApp{};
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::CLOCK_RED_32_ICON_DATA, "0.2.1"),
    ("Manual", || {
        let app = InstructionManualApp{};
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::MANUAL_BOOK_32_ICON_DATA, "0.1.1"),
    ("CH64", || {
        let app = CH64App{};
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "0.1.0"),

    //("Cube", || {
    //    let app = CubeApp{angle: 30f64 };
    //    let dims = crate::env::AppInfo::dimensions(&app);
    //    (Box::new(app), dims)
    //}),
    ("Add..", || {
        let app = AppInstallerApp{};
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ADD_PLUS_32_ICON_DATA, "0.1.0")
];