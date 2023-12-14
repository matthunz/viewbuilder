use super::UserInterface;
use concoct::{Handle, Object, Signal, Slot};
use std::ops::Deref;
use winit::dpi::PhysicalSize;

pub struct Window {
    cx: Option<Handle<Self>>,
}

impl Window {
    pub fn new() -> Self {
        Self { cx: None }
    }
}

impl Object for Window {
    fn started(&mut self, cx: Handle<Self>) {
        UserInterface::current()
            .inner
            .borrow_mut()
            .pending_windows
            .push(cx.slot());

        self.cx = Some(cx);
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
    fn handle(&mut self, _handle: Handle<Self>, _msg: SetSize) {}
}

pub enum WindowMessage {
    Resized(PhysicalSize<u32>),
}

impl Slot<WindowMessage> for Window {
    fn handle(&mut self, handle: Handle<Self>, msg: WindowMessage) {
        match msg {
            WindowMessage::Resized(size) => handle.emit(Resized(size)),
        }
    }
}
