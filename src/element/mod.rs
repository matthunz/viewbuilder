use std::any::Any;

use skia_safe::Canvas;

mod linear_layout;
pub use linear_layout::LinearLayoutElement;

mod text;
use taffy::geometry::Size;
pub use text::TextElement;

use crate::WindowMessage;

pub trait Element {
    fn layout(&mut self) -> Size<f64>;

    fn handle(&mut self, msg: WindowMessage, output: &mut Vec<Box<dyn Any>>);

    fn render(&mut self, canvas: &mut Canvas);
}

impl<T: Element + ?Sized> Element for &mut T {
    fn layout(&mut self) -> Size<f64> {
        (&mut **self).layout()
    }

    fn handle(&mut self, msg: WindowMessage, output: &mut Vec<Box<dyn Any>>) {
        (&mut **self).handle(msg, output)
    }

    fn render(&mut self, canvas: &mut Canvas) {
        (&mut **self).render(canvas)
    }
}
