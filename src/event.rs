use crate::NodeKey;
use kurbo::Point;

pub enum EventKind {
    Click,
    MouseIn,
    MouseOut,
}

pub enum Event {
    Click(MouseEvent),
    MouseIn(MouseEvent),
    MouseOut(MouseEvent),
}

impl Event {
    pub fn kind(&self) -> EventKind {
        match self {
            Event::Click(_) => EventKind::Click,
            Event::MouseIn(_) => EventKind::MouseIn,
            Event::MouseOut(_) => EventKind::MouseOut,
        }
    }
}

pub struct MouseEvent {
    pub target: NodeKey,
    pub location: Point,
}
