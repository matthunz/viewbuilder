use crate::object;
use kurbo::Point;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
};

/// Window on the native platform.
pub struct Window {}

#[object]
impl Window {
    /// Signal for the cursor movement event.
    fn cursor_moved(&self, point: Point);

    /// Signal for the mouse event.
    fn mouse_event(&self, state: ElementState, button: MouseButton);

    /// Signal for the mouse wheel event.
    fn mouse_wheel(&self, delta: MouseScrollDelta, phase: TouchPhase);

    /// Signal for the window resize event.
    fn resized(&self, size: PhysicalSize<u32>);

    /// Signal for the window focus event.
    fn focused(&self, is_focused: bool);

    /// Signal for the cursor enter event.
    fn cursor_entered(&self);

    /// Signal for the cursor leave event.
    fn cursor_left(&self);
}
