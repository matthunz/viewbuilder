use skia_safe::Image;
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Style};

mod text;
pub use self::text::Text;

mod view;
pub use self::view::View;

/// An element of a user interface.
pub trait Element: Send {
    /// Optional keys of this element's children.
    fn children(&self) -> Option<Vec<DefaultKey>>;

    /// Get the layout style of this element.
    fn layout(&mut self) -> Style;

    /// Render the element to an image with the given size.
    fn render(&mut self, size: Size<f32>) -> Option<Image>;
}

impl Element for () {
    fn children(&self) -> Option<Vec<DefaultKey>> {
        None
    }

    fn layout(&mut self) -> Style {
        Style::default()
    }

    fn render(&mut self, size: Size<f32>) -> Option<Image> {
        None
    }
}
