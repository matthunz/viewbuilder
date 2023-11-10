use crate::virtual_tree::DynAttribute;
use dioxus_native_core::{
    prelude::NodeType,
    real_dom::{NodeImmutable, NodeRef},
};

use super::Element;

pub struct Text {}

impl Element for Text {
    fn update(
        &mut self,
        node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
    ) {
        let node_type = node.node_type();
        if let NodeType::Text(text_node) = &*node_type {
            dbg!(&text_node.text);
        } else {
            todo!()
        }
    }
}
