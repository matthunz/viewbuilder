use super::UserInterface;
use concoct::{Handle, Object, Signal, Slot};
use std::{any::Any, ops::Deref};
use winit::{
    dpi::{PhysicalSize, Size},
    window::WindowBuilder,
};

pub struct Builder {
    raw: Option<WindowBuilder>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::from_raw(WindowBuilder::new())
    }
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_raw(raw: WindowBuilder) -> Self {
        Self { raw: Some(raw) }
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        let raw = self.raw.take().unwrap();
        self.raw = Some(raw.with_title(title));
        self
    }

    pub fn build(&mut self) -> Window {
        Window {
            raw: Some(WindowState::Builder(self.raw.take().unwrap())),
        }
    }
}

enum WindowState {
    Builder(WindowBuilder),
    Window(winit::window::Window),
}

pub struct Window {
    raw: Option<WindowState>,
}

impl Default for Window {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Window {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn raw(&self) -> &winit::window::Window {
        match self.raw.as_ref().unwrap() {
            WindowState::Builder(_) => todo!(),
            WindowState::Window(window) => &window,
        }
    }
}

impl Object for Window {
    fn started(&mut self, cx: Handle<Self>) {
        match self.raw.take().unwrap() {
            WindowState::Builder(builder) => {
                let window = UserInterface::current().create_window(builder, cx.slot());
                self.raw = Some(WindowState::Window(window));
            }
            WindowState::Window(_) => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResizedEvent(pub PhysicalSize<u32>);

impl Deref for ResizedEvent {
    type Target = PhysicalSize<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Signal<ResizedEvent> for Window {}

pub struct CustomMessage(pub Box<dyn Any>);

impl Signal<CustomMessage> for Window {}

pub struct SetSizeMessage<S>(pub S);

impl<S: Into<Size>> Slot<SetSizeMessage<S>> for Window {
    fn update(&mut self, _cx: Handle<Self>, msg: SetSizeMessage<S>) {
        self.raw().set_inner_size(msg.0.into())
    }
}

pub enum RawWindowMessage {
    UserEvent(Box<dyn Any>),
    Resized(PhysicalSize<u32>),
}

impl Slot<RawWindowMessage> for Window {
    fn update(&mut self, cx: Handle<Self>, msg: RawWindowMessage) {
        match msg {
            RawWindowMessage::UserEvent(user_event) => cx.emit(CustomMessage(user_event)),
            RawWindowMessage::Resized(size) => cx.emit(ResizedEvent(size)),
        }
    }
}
