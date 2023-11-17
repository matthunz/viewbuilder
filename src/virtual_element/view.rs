use super::VirtualElement;
use crate::{any_element::AnyElement, element::View, virtual_tree::VirtualNode};
use std::any::Any;

pub struct VirtualView {}

impl VirtualElement for VirtualView {
    fn from_vnode(&self, node: &VirtualNode) -> Box<dyn AnyElement> {
        if let VirtualNode::Element {
            tag: _,
            attrs,
            children,
        } = node
        {
            let mut view = View::builder();

            for _child in children {
                // TODO
            }

            for _attr in attrs {}

            Box::new(view.build())
        } else {
            todo!()
        }
    }

    fn set_attribute(&self, _name: &str, _value: Box<dyn Any>, _element: &mut dyn AnyElement) {}

    fn set_handler(
        &self,
        _name: &str,
        _handler: Box<dyn FnMut() + Send>,
        _element: &mut dyn AnyElement,
    ) {
    }

    fn hydrate_text(&self, _path: usize, _value: String, _element: &mut dyn AnyElement) {}

    fn set_text(&self, _value: String, _element: &mut dyn AnyElement) {}
}
