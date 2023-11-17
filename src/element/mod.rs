use skia_safe::Image;
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Style};
use winit::event::WindowEvent;

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

    fn handle(&mut self, key: DefaultKey, event: WindowEvent);

    /// Render the element to an image with the given size.
    fn render(&mut self, size: Size<f32>) -> Option<Image>;
}

impl Element for Box<dyn Element> {
    fn children(&self) -> Option<Vec<DefaultKey>> {
        (&**self).children()
    }

    fn layout(&mut self) -> Style {
        (&mut **self).layout()
    }

    fn handle(&mut self, key: DefaultKey, event: WindowEvent) {
        (&mut **self).handle(key, event)
    }

    fn render(&mut self, size: Size<f32>) -> Option<Image> {
        (&mut **self).render(size)
    }
}
