use super::View;
use concoct::{Object, Slot};
use std::rc::Rc;

pub struct Text {
    content: Rc<str>,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            content: Rc::from(String::new()),
        }
    }
}

impl Object for Text {}

impl View for Text {}

impl Slot<Rc<str>> for Text {
    fn handle(&mut self, _handle: concoct::Handle<Self>, msg: Rc<str>) {
        dbg!(msg);
    }
}
