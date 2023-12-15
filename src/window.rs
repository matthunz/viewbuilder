use concoct::{Handle, Object, Signal};
use winit::event::{
    DeviceEvent, DeviceId, Event as RawEvent, StartCause, WindowEvent as RawWindowEvent,
};

pub enum WindowEvent {}

pub enum Event<E> {
    Custom(E),
    NewEvents(StartCause),
    Resumed,
    DeviceEvent {
        device_id: DeviceId,
        event: DeviceEvent,
    },
    MainEventsCleared,
    RedrawEventsCleared,
}

pub struct EventLoop<E: 'static> {
    raw: Option<winit::event_loop::EventLoop<E>>,
}

impl<E: 'static> EventLoop<E> {
    pub fn new() -> Self {
        Self {
            raw: Some(winit::event_loop::EventLoopBuilder::with_user_event().build()),
        }
    }

    pub fn run(handle: Handle<Self>) {
        let mut me = handle.borrow_mut();
        let raw = me.raw.take().unwrap();
        drop(me);

        raw.run(move |event, _, _| {
            let event = match event {
                RawEvent::UserEvent(custom) => Event::Custom(custom),
                RawEvent::NewEvents(start_cause) => Event::NewEvents(start_cause),
                RawEvent::WindowEvent {
                    window_id: _,
                    event,
                } => match event {
                    RawWindowEvent::AxisMotion {
                        device_id: _,
                        axis: _,
                        value: _,
                    } => todo!(),
                    RawWindowEvent::Resized(_) => todo!(),
                    RawWindowEvent::Moved(_) => todo!(),
                    RawWindowEvent::CloseRequested => todo!(),
                    RawWindowEvent::Destroyed => todo!(),
                    RawWindowEvent::DroppedFile(_) => todo!(),
                    RawWindowEvent::HoveredFile(_) => todo!(),
                    RawWindowEvent::HoveredFileCancelled => todo!(),
                    RawWindowEvent::ReceivedCharacter(_) => todo!(),
                    RawWindowEvent::Focused(_) => todo!(),
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
                    RawWindowEvent::Occluded(_) => todo!(),
                },
                RawEvent::DeviceEvent { device_id, event } => {
                    Event::DeviceEvent { device_id, event }
                }
                RawEvent::Suspended => todo!(),
                RawEvent::Resumed => Event::Resumed,
                RawEvent::MainEventsCleared => Event::MainEventsCleared,
                RawEvent::RedrawRequested(_) => todo!(),
                RawEvent::RedrawEventsCleared => Event::RedrawEventsCleared,
                RawEvent::LoopDestroyed => todo!(),
            };

            handle.cx().emit(event)
        });
    }
}

impl<E> Object for EventLoop<E> {}

impl<E: 'static> Signal<Event<E>> for EventLoop<E> {}
