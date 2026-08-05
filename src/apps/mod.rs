//! Registry and management for parallel applications.
//!
//! This module contains the `APP_REGISTRY`, which is the central list of
//! all available applications in the system.
//!
//! Each application in the registry provides a name, a constructor,
//! an icon, and a version string.

use alloc::boxed::Box;
use alloc::string::ToString;
use uefi::fs::IoErrorContext::CantDeleteFile;
use crate::apps::appinstaller::AppInstallerApp;
use crate::apps::simple_app::SimpleApp;
use crate::apps::clock::ClockApp;
use crate::apps::cube::CubeApp;
use crate::apps::keystepper::CH64App;
use crate::apps::manual::InstructionManualApp;
use crate::apps::snake::SnakeApp;
use crate::apps::doom::DoomApp;
use crate::apps::error::ErrorApp;
use crate::apps::mc_app::MinecraftApp;
use crate::apps::browser::BrowserApp;
use crate::apps::micro_ide::MicroIdeApp;
use crate::apps::resource_tester::SysTestApp;
use crate::apps::x_storage::X_Storage;
use crate::apps::x_overview::X_Overview;
use crate::apps::x_vms::X_VMs;
use crate::apps::x_resources::X_Resources;
use crate::apps::x_apps::X_Apps;
use crate::apps::x_network::X_Network;
use crate::apps::x_console::X_Console;
use crate::apps::x_devices::X_Devices;
use crate::apps::x_settings::X_Settings;
use crate::apps::x_packages::X_Packages;
use crate::apps::x_test::X_Test;
use crate::apps::x_createvm::X_CreateVM;
use crate::apps::x_editor::X_Editor;
use crate::env::Runnable;
use crate::filesystem::FileSystem;
use crate::ui::pixel_graphics::icons;
use crate::ui::pixel_graphics::icons::ICON32;

pub(crate) mod simple_app;
pub mod keystepper;
mod clock;
mod cube;
mod netman;
mod manual;
mod appinstaller;
mod snake;
mod browser;
mod doom;
pub mod doom_glue;
mod resource_tester;
mod mc_app;
mod micro_ide;
pub mod vm_console;

#[path = "./core/error.rs"]
pub(crate) mod error;
pub mod x_overview;
pub mod x_apps;
pub mod x_vms;
pub mod x_createvm;
pub mod x_network;
pub mod x_resources;
pub mod x_console;
pub mod x_test;
pub mod x_devices;
pub mod x_storage;
pub mod x_editor;
pub mod x_settings;
pub mod x_packages;

/// A type alias for a function that creates a boxed app and returns its preferred window dimensions.
pub type AppConstructor = fn() -> (Box<dyn Runnable>, (usize, usize));

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
        let book = FileSystem::read_file_to_string("/docs/man/manual.md")
            .unwrap_or_else(|_| include_str!("../../doc/manual.md").to_string());
        let app = InstructionManualApp::new(&*book, 1100usize);
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::MANUAL_BOOK_32_ICON_DATA, "0.1.1"),
    ("MicroIDE", || {
        let app = MicroIdeApp::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::SCRIPT_YELLOW_32_ICON_DATA, "0.1.0"),
    // ("CH64", || {
    //     let app = CH64App{};
    //     let dims = crate::env::AppInfo::dimensions(&app);
    //     (Box::new(app), dims)
    // }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "0.1.0"),
    ("Snake", || {
        let app = SnakeApp::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::SNAKE_32_ICON_DATA, "0.1.0"),
    ("Browser", || {
        let app = BrowserApp::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA, "0.1.0"),

    ("Cube", || {
       let app = CubeApp::new();
       let dims = crate::env::AppInfo::dimensions(&app);
       (Box::new(app), dims)
    }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "0.2.3"),
    // ("DOOM", || {
    //     let app = DoomApp::new();
    //     let dims = crate::env::AppInfo::dimensions(&app);
    //     (Box::new(app), dims)
    // }, icons::DOOM_32_ICON_DATA, "1.1.0"),
    ("ResTest", || {
        let app = SysTestApp::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::INTEGRATED_CIRCUIT_32_ICON_DATA, "0.1.0"),
    ("MineCrap", || {
        let app = MinecraftApp::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::FLOPPY_SAVE_32_ICON_DATA, "0.1.0"),
    ("Add..", || {
        let app = AppInstallerApp{};
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ADD_PLUS_32_ICON_DATA, "0.1.0"),
    ("ERROR", || {
        let app = ErrorApp{error: "BaseError".parse().unwrap() };
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ERROR_32_ICON_DATA, "0.1.0"),
    ("WARNING", || {
        let app = ErrorApp{error: "BaseWarning".parse().unwrap() };
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::WARNING_32_ICON_DATA, "0.1.0"),
    ("X_FileManager", || {
        let app = X_Storage::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "same"),
    ("X_Overview", || {
        let app = X_Overview::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "1.0.0"),
    ("X_VMs", || {
        let app = X_VMs::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::CUBE_WINDOW_RED_32_ICON_DATA, "1.0.0"),
    ("X_Resources", || {
        let app = X_Resources::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::INTEGRATED_CIRCUIT_32_ICON_DATA, "1.0.0"),
    ("X_Apps", || {
        let app = X_Apps::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ADD_PLUS_32_ICON_DATA, "1.0.0"),
    ("X_Network", || {
        let app = X_Network::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::COMPUTE_UNIT_V_GLOBE_32_ICON_DATA, "1.0.0"),
    ("X_Console", || {
        let app = X_Console::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::COM_PORT_32_ICON_DATA, "1.0.0"),
    ("X_Devices", || {
        let app = X_Devices::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::INTEGRATED_CIRCUIT_32_ICON_DATA, "1.0.0"),
    ("X_Settings", || {
        let app = X_Settings::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::GEAR_YB_32_ICON_DATA, "1.0.0"),
    ("X_Packages", || {
        let app = X_Packages::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ADD_PLUS_32_ICON_DATA, "1.0.0"),
    ("X_Test", || {
        let app = X_Test::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::INTEGRATED_CIRCUIT_32_ICON_DATA, "1.0.0"),
    ("X_CreateVM", || {
        let app = X_CreateVM::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::ADD_PLUS_32_ICON_DATA, "1.0.0"),
    ("X_Editor", || {
        let app = X_Editor::new();
        let dims = crate::env::AppInfo::dimensions(&app);
        (Box::new(app), dims)
    }, icons::SCRIPT_YELLOW_32_ICON_DATA, "1.0.0"),
];

// pub(crate) static HIDDEN_APP_REGISTRY: &[(&str, AppConstructor, ICON32, &str)] = &[
//     ("ERROR", || {
//         let app = ErrorApp{};
//         let dims = crate::env::AppInfo::dimensions(&app);
//         (Box::new(app), dims)
//     }, icons::ERROR_32_ICON_DATA, "0.1.0"),
// ];
