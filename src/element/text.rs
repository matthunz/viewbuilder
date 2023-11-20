use std::borrow::Cow;

use crate::Element;
use skia_safe::{
    wrapper::NativeTransmutableWrapper, Color4f, Font, FontStyle, Paint, TextBlob, Typeface,
};
use taffy::geometry::Size;

pub struct TextElement {
    content: Cow<'static, str>,
}

impl TextElement {
    pub fn new(content: impl Into<Cow<'static, str>>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Element for TextElement {
    fn layout(&mut self) -> taffy::prelude::Size<f64> {
        Size {
            width: 50.,
            height: 50.,
        }
    }

    fn render(&mut self, canvas: &mut skia_safe::Canvas) {
        let typeface = Typeface::new("monospace", FontStyle::default()).unwrap();
        let font = Font::new(typeface, 100.);
        let text_blob = TextBlob::new(&self.content, &font).unwrap();
        let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
        canvas.draw_text_blob(&text_blob, (50., 50.), &paint);
    }
}
