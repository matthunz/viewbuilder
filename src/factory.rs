use crate::{
    element::{Element, View},
    layout::FlexDirection,
    virtual_tree::DynAttribute,
};
use dioxus_native_core::{
    prelude::ElementNode,
    real_dom::{NodeImmutable, NodeRef},
};

pub trait Factory {
    fn create_element(
        &mut self,
        node: NodeRef<DynAttribute>,
        element_node: &ElementNode<DynAttribute>,
    ) -> Box<dyn Element>;
}

pub struct ViewFactory {}

impl Factory for ViewFactory {
    fn create_element(
        &mut self,
        node: NodeRef<DynAttribute>,
        _element_node: &ElementNode<DynAttribute>,
    ) -> Box<dyn Element> {
        let flex_direction = *node.get::<FlexDirection>().unwrap();
        dbg!(flex_direction);

        Box::new(View { flex_direction })
    }
}
