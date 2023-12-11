use crate::{rt::RuntimeGuard, Handle, Runtime};
use kurbo::Point;
use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::WindowId;

mod window;
pub use window::{Window, WindowHandle};

struct Inner {
    pending_windows: Vec<Handle<Window>>,
    windows: HashMap<WindowId, (winit::window::Window, Handle<Window>)>,
}

thread_local! {
    static CURRENT: RefCell<Option<Context>> = RefCell::default();
}

#[derive(Clone)]
pub struct Context {
    inner: Rc<RefCell<Inner>>,
}

impl Context {
    pub fn enter(&self) {
        CURRENT
            .try_with(|cell| {
                let mut current = cell.borrow_mut();
                if current.is_some() {
                    panic!("A Viewbuilder runtime is already running in this thread.");
                }
                *current = Some(self.clone());
            })
            .unwrap();
    }

    pub fn current() -> Self {
        Self::try_current().expect("There is no Viewbuilder runtime running on this thread.")
    }

    pub fn try_current() -> Option<Self> {
        CURRENT.try_with(|cell| cell.borrow().clone()).unwrap()
    }
}

pub struct UserInterface {
    event_loop: EventLoop<()>,
    rt: Runtime,
    context: Context,
}

impl UserInterface {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            rt: Runtime::default(),
            context: Context {
                inner: Rc::new(RefCell::new(Inner {
                    pending_windows: Vec::new(),
                    windows: HashMap::new(),
                })),
            },
        }
    }

    pub fn enter(&self) -> RuntimeGuard {
        self.context.enter();
        self.rt.enter()
    }

    pub fn insert_window(&mut self, handle: Handle<Window>) {
        let window = winit::window::Window::new(&self.event_loop).unwrap();
        self.context
            .inner
            .borrow_mut()
            .windows
            .insert(window.id(), (window, handle));
    }

    pub fn run(self) {
        self.event_loop.run(move |event, event_loop, _| {
            self.rt.try_run();

            let mut cx = self.context.inner.borrow_mut();
            while let Some(handle) = cx.pending_windows.pop() {
                let window = winit::window::Window::new(&event_loop).unwrap();
                cx.windows.insert(window.id(), (window, handle));
            }
            drop(cx);

            match event {
                winit::event::Event::WindowEvent { window_id, event } => {
                    let handle = &self.context.inner.borrow().windows[&window_id].1;
                    match event {
                        WindowEvent::CursorMoved { position, .. } => handle
                            .cursor_moved()
                            .emit((Point::new(position.x, position.y),)),
                        WindowEvent::MouseInput { state, button, .. } => {
                            handle.mouse_event().emit((state, button))
                        }
                        WindowEvent::MouseWheel { delta, phase, .. } => {
                            handle.mouse_wheel().emit((delta, phase))
                        }
                        WindowEvent::Resized(size) => handle.resized().emit((size,)),
                        WindowEvent::Focused(is_focused) => handle.focused().emit((is_focused,)),
                        WindowEvent::CursorEntered { .. } => handle.cursor_entered().emit(()),
                        WindowEvent::CursorLeft { .. } => handle.cursor_left().emit(()),
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
