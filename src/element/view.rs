use std::fmt;

use super::Element;
use crate::{geometry::Size, layout::Layout};
use slotmap::DefaultKey;

pub struct ViewElement {
    children: Vec<DefaultKey>,
}

impl ViewElement {
    pub fn new(children: Vec<DefaultKey>) -> Self {
        Self { children }
    }
}

impl Element for ViewElement {
    fn children(&self) -> Option<Vec<slotmap::DefaultKey>> {
        Some(self.children.clone())
    }

    fn push_child(&mut self, key: DefaultKey) {
        self.children.push(key);
    }

    fn layout(&mut self) -> crate::layout::Builder {
        // TODO
        let mut b = Layout::builder();
        b.size(Size::from_points(500., 500.));
        b
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, _layout: &crate::layout::Layout, _canvas: &mut skia_safe::Canvas) {}
}

impl fmt::Display for ViewElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "view")
    }
}
