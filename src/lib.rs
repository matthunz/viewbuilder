extern crate self as viewbuilder;

use kurbo::Point;
use rt::RuntimeGuard;

use std::collections::HashMap;
use winit::event_loop::EventLoop;
use winit::window::WindowId;

pub use viewbuilder_macros::object;

mod any_object;
pub use self::any_object::AnyObject;

mod handle;
pub use self::handle::{Handle, HandleState, Ref};

mod object;
pub use self::object::Object;

mod rt;
pub(crate) use self::rt::Node;
pub use rt::Runtime;

mod signal;
pub use self::signal::Signal;

mod slot;
pub use self::slot::Slot;

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_pos(&self, point: Point);
}

pub struct App {
    event_loop: EventLoop<()>,
    rt: Runtime,
    windows: HashMap<WindowId, (winit::window::Window, Handle<Window>)>,
}

impl App {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            rt: Runtime::default(),
            windows: HashMap::new(),
        }
    }

    pub fn enter(&self) -> RuntimeGuard {
        self.rt.enter()
    }

    pub fn insert_window(&mut self, handle: Handle<Window>) {
        let window = winit::window::Window::new(&self.event_loop).unwrap();
        self.windows.insert(window.id(), (window, handle));
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, _| {
            self.rt.try_run();

            match event {
                winit::event::Event::WindowEvent { window_id, event } => match event {
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        let handle = &self.windows[&window_id].1;
                        self.rt.send(
                            handle.key(),
                            Box::new((Point::new(position.x, position.y),)),
                        );
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}
