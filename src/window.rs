use concoct::{Context, Handle, Object, Signal};
use std::mem;
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, Event as RawEvent, StartCause, WindowEvent as RawWindowEvent},
    event_loop::EventLoopWindowTarget,
    window::{Window as RawWindow, WindowBuilder, WindowId},
};

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
            WindowState::Window { window, event_loop } => window,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WindowEvent {
    Resized(PhysicalSize<u32>),
    Focused(bool),
    Occluded(bool),
    Destroyed,
}

#[derive(Clone)]
pub enum Event<E> {
    Custom(E),
    Window {
        window_id: WindowId,
        event: WindowEvent,
    },
    NewEvents(StartCause),
    Resumed,
    DeviceEvent {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    RedrawRequested(WindowId),
    MainEventsCleared,
    RedrawEventsCleared,
}

enum EventLoopTarget<T: 'static> {
    EventLoop(winit::event_loop::EventLoop<T>),
    WindowTarget(&'static EventLoopWindowTarget<T>),
}

pub struct EventLoop<E: 'static> {
    raw: Option<EventLoopTarget<E>>,
}

impl<E: 'static> EventLoop<E> {
    pub fn new() -> Self {
        Self {
            raw: Some(EventLoopTarget::EventLoop(
                winit::event_loop::EventLoopBuilder::with_user_event().build(),
            )),
        }
    }

    pub fn run(handle: Handle<Self>) {
        let mut me = handle.borrow_mut();
        let raw = if let EventLoopTarget::EventLoop(raw) = me.raw.take().unwrap() {
            raw
        } else {
            unimplemented!()
        };
        drop(me);

        raw.run(move |event, event_loop, _| {
            handle.borrow_mut().raw = Some(EventLoopTarget::WindowTarget(unsafe {
                mem::transmute(event_loop.clone())
            }));

            let event = match event {
                RawEvent::UserEvent(custom) => Event::Custom(custom),
                RawEvent::NewEvents(start_cause) => Event::NewEvents(start_cause),
                RawEvent::WindowEvent { window_id, event } => match event {
                    RawWindowEvent::AxisMotion {
                        device_id: _,
                        axis: _,
                        value: _,
                    } => todo!(),
                    RawWindowEvent::Resized(size) => Event::Window {
                        window_id,
                        event: WindowEvent::Resized(size),
                    },
                    RawWindowEvent::Moved(_) => todo!(),
                    RawWindowEvent::CloseRequested => todo!(),
                    RawWindowEvent::Destroyed => Event::Window {
                        window_id,
                        event: WindowEvent::Destroyed,
                    },
                    RawWindowEvent::DroppedFile(_) => todo!(),
                    RawWindowEvent::HoveredFile(_) => todo!(),
                    RawWindowEvent::HoveredFileCancelled => todo!(),
                    RawWindowEvent::ReceivedCharacter(_) => todo!(),
                    RawWindowEvent::Focused(focused) => Event::Window {
                        window_id,
                        event: WindowEvent::Focused(focused),
                    },
                    RawWindowEvent::KeyboardInput {
                        device_id: _,
                        input: _,
                        is_synthetic: _,
                    } => todo!(),
                    RawWindowEvent::ModifiersChanged(_) => todo!(),
                    RawWindowEvent::Ime(_) => todo!(),
                    RawWindowEvent::CursorMoved {
                        device_id: _,
                        position: _,
                        modifiers: _,
                    } => todo!(),
                    RawWindowEvent::CursorEntered { device_id: _ } => todo!(),
                    RawWindowEvent::CursorLeft { device_id: _ } => todo!(),
                    RawWindowEvent::MouseWheel {
                        device_id: _,
                        delta: _,
                        phase: _,
                        modifiers: _,
                    } => todo!(),
                    RawWindowEvent::MouseInput {
                        device_id: _,
                        state: _,
                        button: _,
                        modifiers: _,
                    } => todo!(),
                    RawWindowEvent::TouchpadMagnify {
                        device_id: _,
                        delta: _,
                        phase: _,
                    } => todo!(),
                    RawWindowEvent::SmartMagnify { device_id: _ } => todo!(),
                    RawWindowEvent::TouchpadRotate {
                        device_id: _,
                        delta: _,
                        phase: _,
                    } => todo!(),
                    RawWindowEvent::TouchpadPressure {
                        device_id: _,
                        pressure: _,
                        stage: _,
                    } => todo!(),
                    RawWindowEvent::Touch(_) => todo!(),
                    RawWindowEvent::ScaleFactorChanged {
                        scale_factor: _,
                        new_inner_size: _,
                    } => todo!(),
                    RawWindowEvent::ThemeChanged(_) => todo!(),
                    RawWindowEvent::Occluded(occluded) => Event::Window {
                        window_id,
                        event: WindowEvent::Occluded(occluded),
                    },
                },
                RawEvent::DeviceEvent { device_id, event } => {
                    Event::DeviceEvent { device_id, event }
                }
                RawEvent::Suspended => todo!(),
                RawEvent::Resumed => Event::Resumed,
                RawEvent::MainEventsCleared => Event::MainEventsCleared,
                RawEvent::RedrawRequested(window_id) => Event::RedrawRequested(window_id),
                RawEvent::RedrawEventsCleared => Event::RedrawEventsCleared,
                RawEvent::LoopDestroyed => todo!(),
            };

            handle.cx().emit(event);
            handle.borrow_mut().raw = None;
        });
    }
}

impl<E> Object for EventLoop<E> {}

impl<E: 'static> Signal<Event<E>> for EventLoop<E> {}