use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use uefi::proto::console::text::Key;
use crate::env::{AppInfo, Environment, Runnable, RunnableClone};
use crate::ui::pixel_graphics::PixelGraphics;

pub struct RegistryEditorApp {

}

impl RegistryEditorApp {

}

impl AppInfo for RegistryEditorApp {
    fn name(&self) -> &str {
        todo!()
    }

    fn version(&self) -> &str {
        todo!()
    }

    fn icon(&self) -> [u32; 1024] {
        todo!()
    }

    fn dimensions(&self) -> (usize, usize) {
        todo!()
    }
}

impl RunnableClone for RegistryEditorApp {
    fn clone_box(&self) -> Box<dyn Runnable> {
        todo!()
    }
}

impl Runnable for RegistryEditorApp {
    fn draw(&self, graphics_entity: &mut PixelGraphics, vars: &Vec<String>, x: usize, y: usize) {
        todo!()
    }

    fn logic(&mut self, vars: &mut Vec<String>, env: &mut Environment) {
        todo!()
    }

    fn input(&mut self, key: Key) {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }
}