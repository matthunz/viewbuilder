use crate::{
    event_loop::{Event, EventLoopTarget, WindowEvent},
    EventLoop,
};
use concoct::{Context, Handle, Object, Signal};
use winit::window::{Window as RawWindow, WindowBuilder};

mod builder;
pub use self::builder::Builder;

enum WindowState {
    Builder(WindowBuilder),
    Window { window: RawWindow },
}

/// Application window.
/// ```no_run
/// use concoct::{Context, Object};
/// use viewbuilder::{event_loop::WindowEvent, EventLoop, Window};
/// 
/// struct App;
/// 
/// impl App {
///     pub fn event(_cx: &mut Context<Self>, event: WindowEvent) {
///         dbg!(event);
///     }
/// }
/// 
/// impl Object for App {}
/// 
/// let event_loop = EventLoop::<()>::new().start();
/// 
/// let window = Window::new().start();
/// Window::insert(&mut window.cx(), &event_loop);
/// 
/// let app = App.start();
/// window.bind(&app, App::event);
/// 
/// EventLoop::run(event_loop);
/// ```
pub struct Window {
    state: Option<WindowState>,
}

impl Window {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub(crate) fn from_builder(builder: WindowBuilder) -> Self {
        Self {
            state: Some(WindowState::Builder(builder)),
        }
    }

    pub fn builder() -> Builder {
        Builder::default()
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
