use kurbo::Point;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MouseEvent {
    MouseDown,
    MouseUp,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    MouseEvent {
        mouse_event: MouseEvent,
        point: Point,
    },
}
