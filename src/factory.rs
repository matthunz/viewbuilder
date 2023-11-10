use std::sync::{Arc, Mutex};

use crate::{
    element::{Element, View},
    layout::LayoutComponent,
    virtual_tree::{DynAttribute, StyleComponent},
};
use dioxus_native_core::{
    prelude::ElementNode,
    real_dom::{NodeImmutable, NodeRef},
};
use taffy::Taffy;

pub trait Factory: Send {
    fn create_element(
        &mut self,
        node: NodeRef<DynAttribute>,
        element_node: &ElementNode<DynAttribute>,
        taffy: &Arc<Mutex<Taffy>>,
    ) -> Box<dyn Element>;
}

pub struct ViewFactory {}

impl Factory for ViewFactory {
    fn create_element(
        &mut self,
        node: NodeRef<DynAttribute>,
        _element_node: &ElementNode<DynAttribute>,
        taffy: &Arc<Mutex<Taffy>>,
    ) -> Box<dyn Element> {
        let layout = node.get::<LayoutComponent>().unwrap();

        let guard = taffy.lock().unwrap();
        let layout = guard.layout(layout.key.unwrap()).unwrap();
        dbg!(layout);

        let style = node.get::<StyleComponent>().unwrap().clone();
        Box::new(View { style: style.0 })
    }
}
