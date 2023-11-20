use crate::{element::TextElement, View};
use std::borrow::Cow;

pub struct Text<'a, M> {
    content: Cow<'a, str>,
    on_click: Option<M>,
}

impl<'a, M> Text<'a, M> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, msg: M) -> Self {
        self.on_click = Some(msg);
        self
    }
}

impl<'a, M> View<'a, M> for Text<'a, M> {
    type Element = TextElement;

    fn build(&'a mut self) -> Self::Element {
        todo!()
    }

    fn rebuild(&'a mut self, _element: &mut Self::Element) {
        todo!()
    }

    fn handle(&'a mut self, _msg: M) {
        todo!()
    }
}
