use crate::WindowMessage;
use skia_safe::Canvas;
use std::any::Any;
use taffy::geometry::Size;

mod linear_layout;
pub use linear_layout::LinearLayoutElement;

mod text;
pub use text::TextElement;

pub trait Element {
    fn layout(&mut self) -> Size<f64>;

    fn handle(&mut self, msg: WindowMessage, output: &mut Vec<Box<dyn Any>>);

    fn render(&mut self, canvas: &mut Canvas);
}

impl<T: Element + ?Sized> Element for &mut T {
    fn layout(&mut self) -> Size<f64> {
        (**self).layout()
    }

    fn handle(&mut self, msg: WindowMessage, output: &mut Vec<Box<dyn Any>>) {
        (**self).handle(msg, output)
    }

    fn render(&mut self, canvas: &mut Canvas) {
        (**self).render(canvas)
    }
}
