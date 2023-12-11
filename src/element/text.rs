use crate::{Element, Handle};
use kurbo::Size;
use std::borrow::Cow;

pub enum TextMessage {
    Set,
}

pub struct Text {
    content: Cow<'static, str>,
}

impl Text {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Element for Text {
    type Message = TextMessage;

    fn update(&mut self, cx: Handle<Self>, _msg: Self::Message) {
        cx.layout();
    }

    fn layout(&mut self, _min_size: Option<Size>, _max_size: Option<Size>) -> Size {
        dbg!("layout");
        Size::default()
    }
}
