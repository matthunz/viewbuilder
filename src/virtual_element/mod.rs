mod text;
use std::any::Any;

use crate::{any_element::AnyElement, virtual_tree::VirtualNode};

pub use self::text::VirtualText;

pub trait VirtualElement: Send {
    fn from_vnode(&self, node: &VirtualNode) -> Box<dyn AnyElement>;

    fn set_attribute(&self, name: &str, value: Box<dyn Any>, element: &mut dyn AnyElement);

    fn set_handler(
        &self,
        name: &str,
        handler: Box<dyn FnMut() + Send>,
        element: &mut dyn AnyElement,
    );
}
