use kurbo::Point;

use crate::{element::TextElement, View};
use std::borrow::Cow;

pub struct Text<'a, M> {
    content: Cow<'a, str>,
    on_click: Option<Box<dyn FnMut(Point) -> M>>,
}

impl<'a, M> Text<'a, M> {
    pub fn new(content: impl Into<Cow<'a, str>>) -> Self {
        Self {
            content: content.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl FnMut(Point) -> M + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl<'a, M> View<'a, M> for Text<'a, M>
where
    M: 'static,
{
    type Element = TextElement<M>;

    fn build(&'a mut self) -> Self::Element {
        TextElement::new(&self.content, self.on_click.take())
    }

    fn rebuild(&'a mut self, _element: &mut Self::Element) {}

    fn handle(&'a mut self, _msg: M) {}
}
