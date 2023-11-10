use super::Element;
use crate::virtual_tree::DynAttribute;
use dioxus_native_core::real_dom::NodeRef;
use std::sync::{Arc, Mutex};
use taffy::{prelude::Layout, Taffy};

pub struct View {}

impl Element for View {
    fn update(
        &mut self,
        _node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
        _taffy: &Arc<Mutex<Taffy>>,
    ) {
    }

    fn render(&mut self, _layout: Layout, _canvas: &mut skia_safe::Canvas) {}
}
