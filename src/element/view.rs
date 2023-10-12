use slotmap::DefaultKey;
use crate::{geometry::Size, layout::Layout};
use super::Element;

pub struct ViewElement {
    children: Vec<DefaultKey>,
}

impl ViewElement {
    pub fn new(children: Vec<DefaultKey>) -> Self {
        Self { children }
    }
}

impl Element for ViewElement {
    fn children(&mut self) -> Option<Vec<slotmap::DefaultKey>> {
        Some(self.children.clone())
    }

    fn layout(&mut self) -> crate::layout::Builder {
        let mut builder = Layout::builder();
        builder.size(Size::from_points(1000., 1000.));
        builder
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, layout: &crate::layout::Layout, canvas: &mut skia_safe::Canvas) {

    }
}
