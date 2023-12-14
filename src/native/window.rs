use super::UserInterface;
use concoct::{Handle, Object, Signal, Slot};
use std::{any::Any, ops::Deref};
use winit::{
    dpi::{PhysicalSize, Size},
    window::WindowBuilder,
};

/// Builder for a window.
pub struct Builder {
    raw: Option<WindowBuilder>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::from_raw(WindowBuilder::new())
    }
}

impl Builder {
    /// Create a new window builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new window builder from its raw `winit` window builder.
    pub fn from_raw(raw: WindowBuilder) -> Self {
        Self { raw: Some(raw) }
    }

    /// Set the title for this window.
    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        let raw = self.raw.take().unwrap();
        self.raw = Some(raw.with_title(title));
        self
    }

    /// Build a new window, consuming this builder.
    /// 
    /// ## Panics
    /// 
    /// This consumes the current builder, any further calls to `build` will panic.
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

/// Native application window.
pub struct Window {
    raw: Option<WindowState>,
}

impl Default for Window {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Window {
    /// Create a default window.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new window builder.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Get a reference to the raw window.
    /// 
    /// ## Panics
    /// This function will panic if the window hasn't been started with [`Object::start`].
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

/// Signal for the window resize event.
#[derive(Clone, Copy, Debug)]
pub struct ResizedEvent(pub PhysicalSize<u32>);

impl Deref for ResizedEvent {
    type Target = PhysicalSize<u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Signal<ResizedEvent> for Window {}

/// Signal for a custom message.
pub struct CustomMessage(pub Box<dyn Any>);

impl Signal<CustomMessage> for Window {}

/// Slot to set the size of a window.
pub struct SetSizeMessage<S>(pub S);

impl<S: Into<Size>> Slot<SetSizeMessage<S>> for Window {
    fn update(&mut self, _cx: Handle<Self>, msg: SetSizeMessage<S>) {
        self.raw().set_inner_size(msg.0.into())
    }
}

/// Slot to emit an event from the raw window.
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
