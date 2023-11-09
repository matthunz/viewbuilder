use crate::element::{Element, Text};

pub trait TextFactory {
    fn create_text(&mut self, text: &str) -> Box<dyn Element>;
}

pub struct TextElementFactory {}

impl TextFactory for TextElementFactory {
    fn create_text(&mut self, _text: &str) -> Box<dyn Element> {
        Box::new(Text {})
    }
}
