use crate::element::{Element, Text};

pub trait TextFactory: Send {
    fn create_text(&mut self, text: &str) -> Box<dyn Element>;
}

pub struct TextElementFactory {}

impl TextFactory for TextElementFactory {
    fn create_text(&mut self, text: &str) -> Box<dyn Element> {
        Box::new(Text {
            content: text.to_owned(),
        })
    }
}
