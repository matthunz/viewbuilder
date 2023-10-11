use crate::geometry::Size;
use taffy::style::{Dimension, Style};

#[derive(Debug, Default)]
pub struct LayoutNode {
    pub(super) style: Style,
    pub(super) translation: Size<f32>,
    pub(super) is_listening: bool,
}

impl LayoutNode {
    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.style.size = size.into_taffy();
        self
    }
}
