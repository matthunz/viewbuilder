use crate::Element;
use slotmap::DefaultKey;
use std::borrow::Cow;

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
    fn children(&self) -> Option<Box<[DefaultKey]>> {
        None
    }
}
