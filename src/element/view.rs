use super::Element;
use crate::{
    geometry::Size,
    layout::{FlexDirection, Layout},
};
use slotmap::DefaultKey;
use std::fmt;

pub struct ViewElement {
    children: Vec<DefaultKey>,
    flex_direction: FlexDirection,
}

impl ViewElement {
    pub fn new(children: Vec<DefaultKey>) -> Self {
        Self {
            children,
            flex_direction: FlexDirection::Row,
        }
    }
}

impl Element for ViewElement {
    fn children(&self) -> Option<Vec<slotmap::DefaultKey>> {
        Some(self.children.clone())
    }

    fn set_attr(&mut self, name: &str, value: &dyn std::any::Any) {
        match name {
            "flex_direction" => {
                let flex_direction: &FlexDirection = value.downcast_ref().unwrap();
                self.flex_direction = *flex_direction;
            }
            _ => todo!(),
        }
    }

    fn push_child(&mut self, key: DefaultKey) {
        self.children.push(key);
    }

    fn layout(&mut self) -> crate::layout::Builder {
        // TODO
        let mut b = Layout::builder();
        b.flex_direction(self.flex_direction)
            .size(Size::from_points(500., 500.));
        b
    }

    fn semantics(&mut self) -> accesskit::NodeBuilder {
        todo!()
    }

    fn paint(&mut self, _layout: &crate::layout::Layout, _canvas: &mut skia_safe::Canvas) {}
}

impl fmt::Display for ViewElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "view {{")?;
        writeln!(f, "{:?}", self.flex_direction)
    }
}
