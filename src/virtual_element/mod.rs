mod text;
use dioxus::core::BorrowedAttributeValue;

use crate::{any_element::AnyElement, virtual_tree::VirtualNode, Element};

pub use self::text::VirtualText;

pub trait VirtualElement {
    fn from_vnode(&self, node: &VirtualNode) -> Box<dyn Element>;

    fn set_attribute(
        &self,
        name: &str,
        value: BorrowedAttributeValue,
        element: &mut dyn AnyElement,
    );
}
