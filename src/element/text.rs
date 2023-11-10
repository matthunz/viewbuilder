use super::Element;
use crate::virtual_tree::DynAttribute;
use dioxus_native_core::{
    prelude::NodeType,
    real_dom::{NodeImmutable, NodeRef},
};
use skia_safe::{Color4f, Font, Paint, TextBlob, Typeface};
use std::sync::{Arc, Mutex};
use taffy::{prelude::Layout, Taffy};

pub struct Text {
    pub(crate) content: String,
}

impl Element for Text {
    fn update(
        &mut self,
        node: NodeRef<DynAttribute>,
        _mask: dioxus_native_core::node_ref::NodeMask,
        _taffy: &Arc<Mutex<Taffy>>,
    ) {
        let node_type = node.node_type();
        if let NodeType::Text(text_node) = &*node_type {
            self.content = text_node.text.clone();
        } else {
            todo!()
        }
    }

    fn render(&mut self, layout: Layout, canvas: &mut skia_safe::Canvas) {
        dbg!(layout);
        let typeface = Typeface::new("monospace", Default::default()).unwrap();
        let font = Font::new(typeface, 24.);
        if let Some(blob) = TextBlob::new(&self.content, &font) {
            let paint = Paint::new(Color4f::new(1., 0., 0., 1.), None);
            let height = blob.bounds().height();
            canvas.draw_text_blob(
                blob,
                (layout.location.x, layout.location.y + height),
                &paint,
            );
        }
    }
}
