use skia_safe::Canvas;

mod linear_layout;
pub use linear_layout::LinearLayoutElement;

mod text;
pub use text::TextElement;

pub trait Element {
    fn render(&mut self, canvas: &mut Canvas);
}
