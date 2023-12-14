use super::UserInterface;
use concoct::{Handle, Object, Signal, Slot};
use std::ops::Deref;
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
pub struct Resized(pub PhysicalSize<u32>);

impl Deref for Resized {
    type Target = PhysicalSize<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Signal<Resized> for Window {}

pub struct SetSize<S>(pub S);

impl<S: Into<Size>> Slot<SetSize<S>> for Window {
    fn update(&mut self, _cx: Handle<Self>, msg: SetSize<S>) {
        self.raw().set_inner_size(msg.0.into())
    }
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
