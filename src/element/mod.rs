use crate::layout::{self, Layout};
use accesskit::NodeBuilder;
use slotmap::DefaultKey;
use std::any::Any;
use std::fmt::Display;

mod text;
pub use self::text::TextElement;

mod view;
pub use self::view::ViewElement;

pub trait Element: Display {
    fn children(&self) -> Option<Vec<DefaultKey>>;

    fn push_child(&mut self, key: DefaultKey);

    fn set_attr(&mut self, name: &str, value: &dyn Any);

    fn layout(&mut self) -> layout::Builder;

    fn semantics(&mut self) -> NodeBuilder;

    fn paint(&mut self, layout: &Layout, canvas: &mut skia_safe::Canvas);
}
