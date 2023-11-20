use crate::{element::TextElement, View};
use kurbo::Point;
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
        TextElement::new(self.content.to_string(), self.on_click.take())
    }

    fn rebuild(&'a mut self, element: &mut Self::Element) {
        if self.content != element.content() {
            element.set_content(self.content.to_string());
        }
    }

    
}
