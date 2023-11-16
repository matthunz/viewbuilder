use skia_safe::{Canvas, Image};
use slotmap::DefaultKey;
use taffy::style::Style;

mod any_element;

mod element_ref;
pub use element_ref::ElementRef;

mod transaction;
pub use transaction::Transaction;

mod user_interface;
pub use user_interface::UserInterface;

mod view;
pub use view::View;

pub trait Element {
    fn children(&self) -> Option<Vec<DefaultKey>>;

    fn layout(&mut self) -> Style;

    fn render(&mut self) -> Image;
}
