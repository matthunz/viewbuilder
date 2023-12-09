use crate::{Element, ElementRef};
use kurbo::Size;
use slotmap::DefaultKey;
use std::borrow::Cow;

#[derive(Default)]
pub struct TextBuilder {}

impl TextBuilder {
    pub fn on_click(&mut self, f: impl FnMut(ElementRef<Text>) + 'static) -> &mut Self {
        self
    }

    pub fn build(&mut self, content: impl Into<Cow<'static, str>>) -> Text {
        Text::new(content)
    }
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

    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }

    pub fn set_content(&mut self, _content: impl Into<Cow<'static, str>>) {}
}

impl Element for Text {
    fn children(&self) -> Option<Box<[DefaultKey]>> {
        None
    }

    fn layout(&mut self, _min: Option<Size>, _max: Option<Size>) -> Size {
        Size::new(100., 100.)
    }
}
