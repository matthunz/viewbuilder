use crate::{element::ElementData, node::NodeData, Context, Node, NodeKey};

use std::borrow::Cow;
use taffy::{prelude::Size, style::Dimension};

/// Reference to an element in a tree.
///
/// This struct is created with [`Tree::node`].
pub struct NodeRef<'a> {
    key: NodeKey,
    tree: &'a mut Context,
}

impl<'a> NodeRef<'a> {
    /// Create a new node reference.
    pub(crate) fn new(key: NodeKey, tree: &'a mut Context) -> Self {
        Self { key, tree }
    }

    /// Move the reference to the parent element.
    pub fn parent(&mut self) -> &mut Self {
        let parent_key = self.node().parent.unwrap();
        self.key = parent_key;
        self
    }

    /// Get a reference the current node.
    pub fn node(&mut self) -> &mut Node {
        &mut self.tree.tree.nodes[self.key]
    }

    /// Get a reference the current element.
    ///
    /// ## Panics
    /// This function will panic if the current reference is to a text node,
    /// not to an element.
    pub fn element(&mut self) -> &mut ElementData {
        if let NodeData::Element(ref mut element) = self.node().data {
            element
        } else {
            todo!()
        }
    }

    /// Update the text of a node.
    ///
    /// ## Panics
    /// This function will panic if the current reference is to an element,
    /// not to a text node.
    pub fn set_text(&mut self, content: impl Into<Cow<'static, str>>) {
        if let NodeData::Text(ref mut dst) = self.node().data {
            *dst = content.into();
        } else {
            todo!()
        }
    }

    /// Update the size of the element.
    pub fn set_size(&mut self, size: Size<Dimension>) {
        self.as_mut().set_size(size);
        self.tree.changes.push(self.key);
    }
}

impl<'a> AsMut<ElementData> for NodeRef<'a> {
    fn as_mut(&mut self) -> &mut ElementData {
        self.element()
    }
}
