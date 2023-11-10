use super::Element;
use crate::virtual_tree::DynAttribute;
use dioxus_native_core::real_dom::NodeRef;
use taffy::style::Style;

pub struct View {
    pub(crate) style: Style,
}

impl Element for View {
    fn update(
        &mut self,
        _node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
    ) {
    }

    fn render(&mut self, _canvas: &mut skia_safe::Canvas) {}
}
