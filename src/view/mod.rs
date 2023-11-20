use crate::{element::TextElement, Element};

mod linear_layout;
pub use linear_layout::LinearLayout;

mod text;
pub use text::Text;

pub trait View<'a, M> {
    type Element: Element;

    fn build(&'a mut self) -> Self::Element;

    fn rebuild(&'a mut self, element: &mut Self::Element);

    fn handle(&'a mut self, msg: M);
}

impl<'a, M> View<'a, M> for &'a str {
    type Element = TextElement;

    fn build(&'a mut self) -> Self::Element {
        TextElement::new(self.to_string())
    }

    fn rebuild(&'a mut self, _element: &mut Self::Element) {
        dbg!(self);
    }

    fn handle(&'a mut self, _msg: M) {}
}
