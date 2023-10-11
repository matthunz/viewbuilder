use crate::geometry::{Point, Size};

#[derive(Debug)]
pub struct Layout {
    /// Global layout of the node.
    pub(super) layout: taffy::prelude::Layout,

    /// Translation size of the node.
    pub(super) translation: Size<f32>,
}

impl Layout {
    pub fn order(&self) -> u32 {
        self.layout.order
    }

    pub fn size(&self) -> Size<f32> {
        Size::from_taffy(self.layout.size)
    }

    pub fn position(&self) -> Point<f32> {
        Point::from_taffy(self.layout.location)
    }

    pub fn translation(&self) -> Size<f32> {
        self.translation
    }
}
