use super::Builder;
use crate::geometry::{Point, Size};

#[derive(Debug)]
pub struct Layout {
    /// Global layout of the node.
    pub(super) layout: taffy::prelude::Layout,

    /// Translation size of the node.
    pub(super) translation: Size<f32>,
}

impl Layout {
    /// Create a new builder for a layout node.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Get the z-order of the layout node.
    pub fn order(&self) -> u32 {
        self.layout.order
    }

    /// Get the size of the layout node.
    pub fn size(&self) -> Size<f32> {
        Size::from_taffy(self.layout.size)
    }

    /// Get the position of the layout node.
    pub fn position(&self) -> Point<f32> {
        Point::from_taffy(self.layout.location)
    }

    /// Get the translation size of the layout node.
    pub fn translation(&self) -> Size<f32> {
        self.translation
    }
}
