use super::{GlobalLayout, LayoutTree};
use crate::geometry::Size;
use taffy::{
    prelude::Node,
    style::{Dimension, Style},
};

#[derive(Debug, Default)]
pub(super) struct Inner {
    pub(super) style: Style,
    pub(super) translation: Size<f32>,
    pub(super) is_listening: bool,
}

/// Builder for a layout node.
#[derive(Debug)]
pub struct Builder {
    pub(super) inner: Option<Inner>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            inner: Some(Inner::default()),
        }
    }
}

impl Builder {
    /// Set the size of the node.
    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.inner.as_mut().unwrap().style.size = size.into_taffy();
        self
    }

    pub fn is_listening(&mut self, is_listening: bool) -> &mut Self {
        self.inner.as_mut().unwrap().is_listening = is_listening;
        self
    }

    /// Build a new node with children and return its key.
    pub fn build(&mut self, tree: &mut LayoutTree) -> Node {
        let inner = self.inner.take().unwrap();

        let key = tree.taffy.new_leaf(inner.style).unwrap();
        tree.global_layouts.insert(
            key,
            GlobalLayout {
                layout: taffy::prelude::Layout::new(),
                is_listening: inner.is_listening,
                translation: inner.translation,
            },
        );
        key
    }

    /// Build a new node with children and return its key.
    pub fn build_with_children(&mut self, tree: &mut LayoutTree, children: &[Node]) -> Node {
        let inner = self.inner.take().unwrap();

        let key = tree.taffy.new_with_children(inner.style, children).unwrap();
        tree.global_layouts.insert(
            key,
            GlobalLayout {
                layout: taffy::prelude::Layout::new(),
                is_listening: inner.is_listening,
                translation: inner.translation,
            },
        );
        key
    }
}
