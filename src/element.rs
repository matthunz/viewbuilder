use skia_safe::Image;
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Style};

/// An element of a user interface.
pub trait Element: Send {
    /// Optional keys of this element's children.
    fn children(&self) -> Option<Vec<DefaultKey>>;

    /// Get the layout style of this element.
    fn layout(&mut self) -> Style;

    /// Render the element to an image with the given size.
    fn render(&mut self, size: Size<f32>) -> Image;
}
