use super::UserInterface;
use concoct::{Handle, Object, Signal, Slot};
use std::ops::Deref;
use winit::dpi::PhysicalSize;

pub struct Window {
    raw: Option<winit::window::Window>,
}

impl Window {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn raw(&self) -> &winit::window::Window {
        self.raw.as_ref().unwrap()
    }
}

impl Object for Window {
    fn started(&mut self, cx: Handle<Self>) {
        let window = UserInterface::current().create_window(cx.slot());
        self.raw = Some(window);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Resized(pub PhysicalSize<u32>);

impl Deref for Resized {
    type Target = PhysicalSize<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Signal<Resized> for Window {}

pub struct SetSize {}

impl Slot<SetSize> for Window {
    fn update(&mut self, _cx: Handle<Self>, _msg: SetSize) {}
}

pub enum WindowMessage {
    Resized(PhysicalSize<u32>),
}

impl Slot<WindowMessage> for Window {
    fn update(&mut self, cx: Handle<Self>, msg: WindowMessage) {
        match msg {
            WindowMessage::Resized(size) => cx.emit(Resized(size)),
        }
    }
}
