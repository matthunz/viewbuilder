use crate::object;
use kurbo::Point;
use winit::event::{ElementState, MouseButton};

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_moved(&self, point: Point);

    #[signal]
    fn mouse_event(&self, state: ElementState, button: MouseButton);
}
