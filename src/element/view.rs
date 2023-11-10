use super::Element;
use crate::{layout::FlexDirection, virtual_tree::DynAttribute};
use dioxus_native_core::real_dom::NodeRef;

pub struct View {
    pub(crate) flex_direction: FlexDirection,
}

impl Element for View {
    fn update(
        &mut self,
        _node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
    ) {
    }
}
