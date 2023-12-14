use concoct::{rt::RuntimeGuard, Runtime, SlotHandle};
use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};
use window::WindowMessage;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::WindowId,
};

pub mod window;
pub use window::Window;

thread_local! {
    static CURRENT: RefCell<Option<UserInterface>> = RefCell::default();
}

enum EventLoopTarget {
    EventLoop(EventLoop<()>),
    WindowTarget(&'static EventLoopWindowTarget<()>),
}

struct Inner {
    event_loop: Option<EventLoopTarget>,
    windows: HashMap<WindowId, SlotHandle<WindowMessage>>,
}

#[derive(Clone)]
pub struct UserInterface {
    rt: Runtime,
    inner: Rc<RefCell<Inner>>,
}

impl Default for UserInterface {
    fn default() -> Self {
        Self {
            rt: Default::default(),
            inner: Rc::new(RefCell::new(Inner {
                event_loop: Some(EventLoopTarget::EventLoop(EventLoop::new())),
                windows: HashMap::new(),
            })),
        }
    }
}

impl UserInterface {
    pub fn current() -> Self {
        Self::try_current().unwrap()
    }

    pub fn try_current() -> Option<Self> {
        CURRENT
            .try_with(|cell| cell.borrow().clone())
            .ok()
            .flatten()
    }

    pub fn enter(&self) -> UserInterfaceGuard {
        CURRENT
            .try_with(|cell| *cell.borrow_mut() = Some(self.clone()))
            .unwrap();

        let rt = self.rt.enter();

        UserInterfaceGuard { _rt: rt }
    }

    pub fn run(self) {
        let event_loop = if let EventLoopTarget::EventLoop(event_loop) =
            self.inner.borrow_mut().event_loop.take().unwrap()
        {
            event_loop
        } else {
            todo!()
        };

        event_loop.run(move |event, event_loop, _| {
            self.rt.try_run();

            let mut me = self.inner.borrow_mut();
            me.event_loop = Some(EventLoopTarget::WindowTarget(unsafe {
                mem::transmute(event_loop)
            }));

            match event {
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::Resized(size) => {
                        me.windows[&window_id].send(WindowMessage::Resized(size));
                    }
                    _ => {}
                },
                _ => {}
            };

            me.event_loop.take();
        });
    }

    pub(crate) fn create_window(&self, handle: SlotHandle<WindowMessage>) -> winit::window::Window {
        let mut me = self.inner.borrow_mut();
        let event_loop = match me.event_loop.as_ref().unwrap() {
            EventLoopTarget::EventLoop(event_loop) => &**event_loop,
            EventLoopTarget::WindowTarget(event_loop) => event_loop,
        };
        let window = winit::window::Window::new(event_loop).unwrap();
        me.windows.insert(window.id(), handle);
        window
    }
}

pub struct UserInterfaceGuard {
    _rt: RuntimeGuard,
}

impl Drop for UserInterfaceGuard {
    fn drop(&mut self) {
        CURRENT.try_with(|cell| cell.borrow_mut().take()).unwrap();
    }
}
