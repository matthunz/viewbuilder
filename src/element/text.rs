use crate::Element;

pub struct TextElement {}

impl Element for TextElement {
    fn render(&mut self, _canvas: &mut skia_safe::Canvas) {}
}
