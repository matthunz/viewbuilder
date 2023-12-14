use concoct::{rt::RuntimeGuard, Runtime, SlotHandle};
use std::{cell::RefCell, collections::HashMap, mem, rc::Rc};
use window::RawWindowMessage;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::{WindowBuilder, WindowId},
};

#[cfg(feature = "window")]
#[cfg_attr(docsrs, doc(cfg(feature = "window")))]
pub mod window;
#[cfg(feature = "window")]
#[cfg_attr(docsrs, doc(cfg(feature = "window")))]
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
    windows: HashMap<WindowId, SlotHandle<RawWindowMessage>>,
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
                        me.windows[&window_id].send(RawWindowMessage::Resized(size));
                    }
                    _ => {}
                },
                _ => {}
            };

            me.event_loop.take();
        });
    }

    pub fn create_window(
        &self,
        builder: WindowBuilder,
        handle: SlotHandle<RawWindowMessage>,
    ) -> winit::window::Window {
        let mut me = self.inner.borrow_mut();
        let window_target = match me.event_loop.as_ref().unwrap() {
            EventLoopTarget::EventLoop(event_loop) => &**event_loop,
            EventLoopTarget::WindowTarget(event_loop) => event_loop,
        };
        let window = builder.build(window_target).unwrap();
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
