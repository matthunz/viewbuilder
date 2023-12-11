extern crate self as viewbuilder;

use kurbo::Point;
use slotmap::DefaultKey;
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

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_pos(&self, point: Point);
}

pub struct App {
    event_loop: EventLoop<()>,
    rt: Runtime,
    windows: HashMap<WindowId, (winit::window::Window, DefaultKey)>,
}

impl App {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
            rt: Runtime::current(),
            windows: HashMap::new(),
        }
    }

    pub fn insert_window(&mut self, handle: Handle<Window>) {
        let window = winit::window::Window::new(&self.event_loop).unwrap();
        self.windows
            .insert(window.id(), (window, handle.state.key()));
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, _| {
            self.rt.run();

            match event {
                winit::event::Event::WindowEvent { window_id, event } => match event {
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        let key = self.windows[&window_id].1;
                        self.rt
                            .send(key, Box::new(Point::new(position.x, position.y)));
                    }
                    _ => {}
                },
                _ => {}
            }
        });
    }
}

pub trait Slot<O, D> {
    fn handle(&mut self, object: &mut O, data: D);
}

macro_rules! impl_slot {
    ($($t:tt),*) => {
        impl<F: Fn(&mut O, $($t),*,), O, $($t),*> Slot<O, ($($t),*,)> for F {
            fn handle(&mut self, object: &mut O, data: ($($t),*,)) {
                #[allow(non_snake_case)]
                let ($($t),*,) = data;
                self(object, $($t),*)
            }
        }
    };
}

impl_slot!(T1);
impl_slot!(T1, T2);
impl_slot!(T1, T2, T3);
impl_slot!(T1, T2, T3, T4);
impl_slot!(T1, T2, T3, T4, T5);
impl_slot!(T1, T2, T3, T4, T5, T6);
impl_slot!(T1, T2, T3, T4, T5, T6, T7);
impl_slot!(T1, T2, T3, T4, T5, T6, T7, T8);
