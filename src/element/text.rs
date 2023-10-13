use std::{borrow::Cow, fmt};

use super::Element;
use crate::layout::{self, Layout};
use skia_safe::{Canvas, Color4f, Font, Paint, TextBlob};
use slotmap::DefaultKey;
use taffy::node::MeasureFunc;

pub struct TextElement {
    value: Cow<'static, str>,
    text_blob: TextBlob,
}

impl TextElement {
    pub fn new(content: impl Into<Cow<'static, str>>, font: &Font) -> Self {
        let value = content.into();
        let text_blob = TextBlob::new(&value, font).unwrap();
        Self { text_blob, value }
    }
}

impl Element for TextElement {
    fn children(&self) -> Option<Vec<DefaultKey>> {
        None
    }

    fn push_child(&mut self, _key: DefaultKey) {
        todo!()
    }

    fn set_attr(&mut self, name: &str, value: &dyn std::any::Any) {
        
    }

    fn layout(&mut self) -> layout::Builder {
        let mut builder = Layout::builder();
        let blob = self.text_blob.clone();
        builder.on_measure(MeasureFunc::Boxed(Box::new(move |_a, _size| {
            taffy::prelude::Size {
                width: blob.bounds().width() / 2.,
                height: blob.bounds().height(),
            }
        })));
        builder
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, layout: &Layout, canvas: &mut Canvas) {
        let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
        canvas.draw_text_blob(
            &self.text_blob,
            (
                layout.position().x,
                layout.position().y + self.text_blob.bounds().height(),
            ),
            &paint,
        );
    }
}

impl fmt::Display for TextElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}
