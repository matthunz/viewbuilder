use crate::{Element, Handle};
use kurbo::Size;

pub struct Window {}

impl Window {
    pub fn new<T>(_content: T) -> Self {
        Self {}
    }
}

impl Element for Window {
    type Message = ();

    fn update(&mut self, _cx: Handle<Self>, _msg: Self::Message) {
        todo!()
    }

    fn layout(&mut self, _min_size: Option<Size>, _max_size: Option<Size>) -> Size {
        todo!()
    }
}
