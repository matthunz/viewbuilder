use concoct::{Handle, Object, Signal};
use std::{mem, time::Instant};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceEvent, DeviceId, Event as RawEvent, StartCause, WindowEvent as RawWindowEvent},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::WindowId,
};

#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    CursorMoved {
        device_id: DeviceId,
        position: PhysicalPosition<f64>,
    },
    CursorEntered {
        device_id: DeviceId,
    },
    CursorLeft {
        device_id: DeviceId,
    },
    Resized(PhysicalSize<u32>),
    Focused(bool),
    Occluded(bool),
    Destroyed,
}

#[derive(Clone, Debug, PartialEq)]
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

pub(crate) enum EventLoopTarget<T: 'static> {
    EventLoop(winit::event_loop::EventLoop<T>),
    WindowTarget(&'static EventLoopWindowTarget<T>),
}

/// Application event loop.
/// ```no_run
/// use concoct::{Context, Object};
/// use viewbuilder::{event_loop::Event, EventLoop, Window};
/// 
/// struct App;
/// 
/// impl Object for App {}
/// 
/// impl App {
///     pub fn event(_cx: &mut Context<Self>, event: Event<()>) {
///         dbg!(event);
///     }
/// }
/// 
/// let event_loop = EventLoop::<()>::new().start();
/// 
/// let app = App.start();
/// event_loop.bind(&app, App::event);
///
/// EventLoop::run(event_loop);
/// ```
pub struct EventLoop<E: 'static> {
    pub(crate) raw: Option<EventLoopTarget<E>>,
    control_flow: ControlFlow,
}

impl<E: 'static> Default for EventLoop<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: 'static> EventLoop<E> {
    pub fn new() -> Self {
        Self {
            raw: Some(EventLoopTarget::EventLoop(
                winit::event_loop::EventLoopBuilder::with_user_event().build(),
            )),
            control_flow: ControlFlow::Poll,
        }
    }

    pub fn wait(&mut self) {
        self.control_flow = ControlFlow::Wait;
    }

    pub fn wait_until(&mut self, instant: Instant) {
        self.control_flow = ControlFlow::WaitUntil(instant);
    }

    pub fn exit(&mut self, code: i32) {
        self.control_flow = ControlFlow::ExitWithCode(code);
    }

    pub fn run(handle: Handle<Self>) {
        let mut me = handle.borrow_mut();
        let raw = if let EventLoopTarget::EventLoop(raw) = me.raw.take().unwrap() {
            raw
        } else {
            unimplemented!()
        };
        drop(me);

        raw.run(move |event, event_loop, control_flow| {
            let mut me = handle.borrow_mut();
            me.raw = Some(EventLoopTarget::WindowTarget(unsafe {
                mem::transmute(event_loop)
            }));
            me.control_flow = ControlFlow::Poll;
            drop(me);

            let event = match event {
                RawEvent::UserEvent(custom) => Event::Custom(custom),
                RawEvent::NewEvents(start_cause) => Event::NewEvents(start_cause),
                RawEvent::WindowEvent { window_id, event } => {
                    let window_event = match event {
                        RawWindowEvent::AxisMotion {
                            device_id: _,
                            axis: _,
                            value: _,
                        } => todo!(),
                        RawWindowEvent::Resized(size) => WindowEvent::Resized(size),
                        RawWindowEvent::Moved(_) => todo!(),
                        RawWindowEvent::CloseRequested => todo!(),
                        RawWindowEvent::Destroyed => WindowEvent::Destroyed,
                        RawWindowEvent::DroppedFile(_) => todo!(),
                        RawWindowEvent::HoveredFile(_) => todo!(),
                        RawWindowEvent::HoveredFileCancelled => todo!(),
                        RawWindowEvent::ReceivedCharacter(_) => todo!(),
                        RawWindowEvent::Focused(focused) => WindowEvent::Focused(focused),

                        RawWindowEvent::KeyboardInput {
                            device_id: _,
                            input: _,
                            is_synthetic: _,
                        } => todo!(),
                        RawWindowEvent::ModifiersChanged(_) => todo!(),
                        RawWindowEvent::Ime(_) => todo!(),
                        RawWindowEvent::CursorMoved {
                            device_id,
                            position,
                            ..
                        } => WindowEvent::CursorMoved {
                            device_id,
                            position,
                        },
                        RawWindowEvent::CursorEntered { device_id } => {
                            WindowEvent::CursorEntered { device_id }
                        }
                        RawWindowEvent::CursorLeft { device_id } => {
                            WindowEvent::CursorLeft { device_id }
                        }
                        RawWindowEvent::MouseWheel {
                            device_id: _,
                            delta: _,
                            phase: _,
                            ..
                        } => todo!(),
                        RawWindowEvent::MouseInput {
                            device_id: _,
                            state: _,
                            button: _,
                            ..
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
                        RawWindowEvent::Occluded(occluded) => WindowEvent::Occluded(occluded),
                    };
                    Event::Window {
                        window_id,
                        event: window_event,
                    }
                }
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
            *control_flow = handle.borrow().control_flow;
        });
    }
}

impl<E> Object for EventLoop<E> {}

impl<E: 'static> Signal<Event<E>> for EventLoop<E> {}
