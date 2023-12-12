use crate::UserInterface;
use concoct::{Context, Handler, Object, Signal};
use winit::dpi::PhysicalSize;

#[derive(Default)]
pub struct Window {
    cx: Option<Context<Self>>,
}

impl Object for Window {
    fn start(&mut self, cx: Context<Self>) {
        UserInterface::current()
            .inner
            .borrow_mut()
            .pending_windows
            .push(cx.clone());

        self.cx = Some(cx);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Resized(pub PhysicalSize<u32>);

impl Signal<Resized> for Window {}

pub struct SetSize {}

impl Handler<SetSize> for Window {
    fn handle(&mut self, _handle: Context<Self>, _msg: SetSize) {}
}
