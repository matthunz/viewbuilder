use crate::{view::IntoView, UserInterface};
use concoct::{Context, Handle, Handler, Object, Signal};
use winit::dpi::PhysicalSize;

pub struct Window<V> {
    view: Handle<V>,
    cx: Option<Context<Self>>,
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
    fn start(&mut self, cx: Context<Self>) {
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

impl<V: 'static> Signal<Resized> for Window<V> {}

pub struct SetSize {}

impl<V: 'static> Handler<SetSize> for Window<V> {
    fn handle(&mut self, _handle: Context<Self>, _msg: SetSize) {}
}

pub enum WindowMessage {
    Resized(PhysicalSize<u32>),
}

impl<V: 'static> Handler<WindowMessage> for Window<V> {
    fn handle(&mut self, handle: Context<Self>, msg: WindowMessage) {
        match msg {
            WindowMessage::Resized(size) => handle.emit(Resized(size)),
        }
    }
}
