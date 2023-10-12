use crate::layout::{self, Layout};
use accesskit::NodeBuilder;
use slotmap::DefaultKey;

mod text;
pub use self::text::TextElement;

mod view;
pub use self::view::ViewElement;

pub trait Element {
    fn children(&mut self) -> Option<Vec<DefaultKey>>;

    fn push_child(&mut self, key: DefaultKey);

    fn layout(&mut self) -> layout::Builder;

    fn semantics(&mut self) -> NodeBuilder;

    fn paint(&mut self, layout: &Layout, canvas: &mut skia_safe::Canvas);
}
