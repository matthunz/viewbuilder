use crate::object;
use kurbo::Point;

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_pos(&self, point: Point);
}
