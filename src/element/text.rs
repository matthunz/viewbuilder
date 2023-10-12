use super::Element;
use crate::layout::{self, Layout};
use skia_safe::{Canvas, Color4f, Font, Paint, TextBlob};
use slotmap::DefaultKey;

pub struct TextElement {
    text_blob: TextBlob,
}

impl TextElement {
    pub fn new(content: &str, font: &Font) -> Self {
        let text_blob = TextBlob::new(content, font).unwrap();
        Self { text_blob }
    }
}

impl Element for TextElement {
    fn children(&mut self) -> Option<Vec<DefaultKey>> {
        None
    }

    fn layout(&mut self) -> layout::Builder {
        Layout::builder()
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, _layout: &Layout, canvas: &mut Canvas) {
        let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
        canvas.draw_text_blob(&self.text_blob, (0., 0.), &paint);
    }
}
