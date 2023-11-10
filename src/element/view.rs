use std::sync::{Arc, Mutex};

use super::Element;
use crate::{layout::LayoutComponent, virtual_tree::DynAttribute};
use dioxus_native_core::real_dom::{NodeImmutable, NodeRef};
use taffy::{prelude::Layout, style::Style, Taffy};

pub struct View {
    pub(crate) style: Style,
}

impl Element for View {
    fn update(
        &mut self,
        node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
        taffy: &Arc<Mutex<Taffy>>,
    ) {
        let layout = node.get::<LayoutComponent>().unwrap();

        let guard = taffy.lock().unwrap();
        let layout = guard.layout(layout.key.unwrap()).unwrap();
    }

    fn render(&mut self, layout: Layout, _canvas: &mut skia_safe::Canvas) {}
}
