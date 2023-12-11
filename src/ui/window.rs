use crate::object;
use kurbo::Point;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
};

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_moved(&self, point: Point);

    #[signal]
    fn mouse_event(&self, state: ElementState, button: MouseButton);

    #[signal]
    fn mouse_wheel(&self, delta: MouseScrollDelta, phase: TouchPhase);

    #[signal]
    fn resized(&self, size: PhysicalSize<u32>);

    #[signal]
    fn focused(&self, is_focused: bool);

    fn cursor_entered(&self);

    fn cursor_left(&self);
}
