use concoct::{Runtime, RuntimeGuard, SlotHandle};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use window::WindowMessage;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowId,
};

pub use viewbuilder_macros::main;

pub mod element;

pub mod view;

pub mod window;
pub use window::Window;

thread_local! {
    static CURRENT: RefCell<Option<UserInterface>> = RefCell::default();
}

#[derive(Default)]
struct Inner {
    pending_windows: Vec<SlotHandle<WindowMessage>>,
    windows: HashMap<WindowId, (winit::window::Window, SlotHandle<WindowMessage>)>,
}

#[derive(Clone, Default)]
pub struct UserInterface {
    rt: Runtime,
    inner: Rc<RefCell<Inner>>,
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

        UserInterfaceGuard { rt }
    }

    pub fn run(self) {
        EventLoop::new().run(move |event, event_loop, _| {
            self.rt.try_run();

            let mut me = self.inner.borrow_mut();
            while let Some(handle) = me.pending_windows.pop() {
                let window = winit::window::Window::new(&event_loop).unwrap();
                me.windows.insert(window.id(), (window, handle));
            }

            match event {
                Event::WindowEvent { window_id, event } => match event {
                    WindowEvent::Resized(size) => {
                        me.windows[&window_id].1.send(WindowMessage::Resized(size));
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

pub struct UserInterfaceGuard {
    rt: RuntimeGuard,
}

impl Drop for UserInterfaceGuard {
    fn drop(&mut self) {
        CURRENT.try_with(|cell| cell.borrow_mut().take()).unwrap();
    }
}
