use crate::virtual_tree::DynAttribute;
use dioxus_native_core::node_ref::NodeMask;
use dioxus_native_core::real_dom::NodeRef;
use skia_safe::Canvas;
use std::sync::{Arc, Mutex};
use taffy::prelude::Layout;
use taffy::Taffy;

mod text;
pub use self::text::Text;

mod view;
pub use self::view::View;

pub trait Element: Send {
    fn update(&mut self, node: NodeRef<DynAttribute>, mask: NodeMask, taffy: &Arc<Mutex<Taffy>>);

    fn render(&mut self, layout: Layout, canvas: &mut Canvas);
}
