use crate::{
    element::{Element, View},
    virtual_tree::{DynAttribute}, layout::FlexDirection,
};
use dioxus_native_core::{
    node::{OwnedAttributeDiscription, OwnedAttributeValue},
    prelude::ElementNode,
    real_dom::{NodeImmutable, NodeRef},
    tree::Node,
};
use std::{collections::HashMap, hash::BuildHasherDefault};

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
        element_node: &ElementNode<DynAttribute>,
    ) -> Box<dyn Element> {
        let flex_direction = *node.get::<FlexDirection>().unwrap();
        dbg!(flex_direction);

        Box::new(View { flex_direction })
    }
}
