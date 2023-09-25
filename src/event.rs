use kurbo::Point;

use crate::NodeKey;

pub enum Event {
    Click(MouseEvent),
    MouseIn(MouseEvent),
    MouseOut(MouseEvent),
}


pub struct MouseEvent {
    pub target: NodeKey,
    pub location: Point,
}
