use crate::{
    event_loop::{Event, EventLoopTarget, WindowEvent},
    EventLoop,
};
use concoct::{Context, Handle, Object, Signal};
use winit::window::{Window as RawWindow, WindowBuilder};

enum WindowState<E: 'static> {
    Builder(WindowBuilder),
    Window {
        window: RawWindow,
        event_loop: Handle<EventLoop<E>>,
    },
}

pub struct Window<E: 'static> {
    state: Option<WindowState<E>>,
}

impl<E> Window<E> {
    pub fn new() -> Self {
        Self {
            state: Some(WindowState::Builder(WindowBuilder::new())),
        }
    }

    pub fn insert(cx: &mut Context<Self>, event_loop: &Handle<EventLoop<E>>)
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
                cx.state = Some(WindowState::Window {
                    window,
                    event_loop: event_loop.clone(),
                });
            }
            WindowState::Window { .. } => todo!(),
        }
    }

    pub fn handle(cx: &mut Context<Self>, event: Event<E>) {
        let window = match cx.state.as_ref().unwrap() {
            WindowState::Window {
                window,
                event_loop: _,
            } => window,
            _ => todo!(),
        };
        match event {
            Event::Window { window_id, event } => {
                if window_id == window.id() {
                    cx.emit(event);
                }
            }
            _ => {}
        }
    }
}

impl<E> Object for Window<E> {}

impl<E> Signal<WindowEvent> for Window<E> {}
