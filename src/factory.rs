use crate::{
    element::{Element, View},
    virtual_tree::{DynAttribute, StyleComponent},
};
use dioxus_native_core::{
    prelude::ElementNode,
    real_dom::{NodeImmutable, NodeRef},
};

pub trait Factory: Send {
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
        let style = node.get::<StyleComponent>().unwrap().clone();
        Box::new(View { style: style.0 })
    }
}
