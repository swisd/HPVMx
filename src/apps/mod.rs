use alloc::boxed::Box;
use crate::apps::simple_app::SimpleApp;
use crate::apps::clock::ClockApp;
use crate::env::Runnable;
use crate::ui::pixel_graphics::icons;
use crate::ui::pixel_graphics::icons::ICON32;

pub(crate) mod simple_app;
pub mod keystepper;
mod clock;
mod cube;

// A type alias for a function that creates a boxed app
type AppConstructor = fn() -> Box<dyn Runnable>;

// The Registry: A static list of names and their "Constructors"
pub(crate) static APP_REGISTRY: &[(&str, AppConstructor, ICON32)] = &[
    ("SimpleApp", || Box::new(SimpleApp { color: [0x000000, 0xFFFFFF, 0xFF7700] }), icons::TRAFFIC_LIGHT_32_ICON_DATA),
    ("Clock", || Box::new(ClockApp{}), icons::CLOCK_RED_32_ICON_DATA),
    //("Cube", || Box::new(CubeApp{angle: 30f64 })),
];