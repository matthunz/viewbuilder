use crate::{
    event_loop::{Event, EventLoopTarget, WindowEvent},
    EventLoop,
};
use concoct::{Context, Handle, Object, Signal};
use winit::window::{Window as RawWindow, WindowBuilder};

enum WindowState {
    Builder(WindowBuilder),
    Window { window: RawWindow },
}

pub struct Window {
    state: Option<WindowState>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            state: Some(WindowState::Builder(WindowBuilder::new())),
        }
    }

    pub fn insert<E>(cx: &mut Context<Self>, event_loop: &Handle<EventLoop<E>>)
    where
        E: Clone,
    {
        event_loop.bind(&cx.handle(), Self::handle);

        match cx.state.take().unwrap() {
            WindowState::Builder(builder) => {
                let window = match event_loop.borrow_mut().raw.as_mut().unwrap() {
                    EventLoopTarget::EventLoop(event_loop) => builder.build(event_loop).unwrap(),
                    EventLoopTarget::WindowTarget(event_loop) => builder.build(event_loop).unwrap(),
                };
                cx.state = Some(WindowState::Window { window });
            }
            WindowState::Window { .. } => todo!(),
        }
    }

    /// Get a reference to the raw `winit` window.
    pub fn raw(&self) -> &RawWindow {
        match self.state.as_ref().unwrap() {
            WindowState::Window { window } => window,
            _ => todo!(),
        }
    }

    /// Gets the window's current visibility state.
    ///
    /// `None` means it couldn't be determined, so it is not recommended to use this to drive your rendering backend.
    ///
    /// ## Platform-specific
    ///
    /// - **X11:** Not implemented.
    /// - **Wayland / iOS / Android / Web:** Unsupported.
    pub fn is_visible(&self) -> Option<bool> {
        self.raw().is_visible()
    }

    /// Modifies the window's visibility.
    ///
    /// If `false`, this will hide the window. If `true`, this will show the window.
    ///
    /// ## Platform-specific
    ///
    /// - **Android / Wayland / Web:** Unsupported.
    /// - **iOS:** Can only be called on the main thread.
    pub fn set_visible(&self, visible: bool) {
        self.raw().set_visible(visible)
    }

    pub fn handle<E>(cx: &mut Context<Self>, event: Event<E>) {
        match event {
            Event::Window { window_id, event } => {
                if window_id == cx.raw().id() {
                    cx.emit(event);
                }
            }
            _ => {}
        }
    }
}

impl Object for Window {}

impl Signal<WindowEvent> for Window {}
