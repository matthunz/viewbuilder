use kurbo::Point;
use crate::object;

pub struct Window {}

#[object]
impl Window {
    #[signal]
    fn cursor_pos(&self, point: Point);
}
