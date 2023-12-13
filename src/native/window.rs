use super::{view::IntoView, UserInterface};
use concoct::{ Handle, Object, Signal, Slot};
use std::ops::Deref;
use winit::dpi::PhysicalSize;

pub struct Window<V> {
    view: Handle<V>,
    cx: Option<Handle<Self>>,
}

impl<V> Window<V> {
    pub fn new(view: impl IntoView<View = V>) -> Self {
        Self {
            view: view.into_view(),
            cx: None,
        }
    }
}

impl<V: 'static> Object for Window<V> {
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

impl<V: 'static> Signal<Resized> for Window<V> {}

pub struct SetSize {}

impl<V: 'static> Slot<SetSize> for Window<V> {
    fn handle(&mut self, _handle: Handle<Self>, _msg: SetSize) {}
}

pub enum WindowMessage {
    Resized(PhysicalSize<u32>),
}

impl<V: 'static> Slot<WindowMessage> for Window<V> {
    fn handle(&mut self, handle: Handle<Self>, msg: WindowMessage) {
        match msg {
            WindowMessage::Resized(size) => handle.emit(Resized(size)),
        }
    }
}
