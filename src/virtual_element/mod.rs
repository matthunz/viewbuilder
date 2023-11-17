use crate::{any_element::AnyElement, virtual_tree::VirtualNode};
use std::any::Any;

mod text;
pub use self::text::VirtualText;

mod view;
pub use self::view::VirtualView;

pub trait VirtualElement: Send + Sync {
    fn from_vnode(&self, node: &VirtualNode) -> Box<dyn AnyElement>;

    fn set_attribute(&self, name: &str, value: Box<dyn Any>, element: &mut dyn AnyElement);

    fn set_handler(
        &self,
        name: &str,
        handler: Box<dyn FnMut() + Send>,
        element: &mut dyn AnyElement,
    );

    fn hydrate_text(&self, path: usize, value: String, element: &mut dyn AnyElement);

    fn set_text(&self, value: String, element: &mut dyn AnyElement);
}
