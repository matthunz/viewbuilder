use crate::{
    element::ElementData,
    node::{NodeData, Overflow},
    Context, Node, NodeKey,
};
use std::borrow::Cow;
use taffy::{prelude::Size, style::Dimension};

/// Reference to an element in a tree.
///
/// This struct is created with [`Tree::node`].
pub struct NodeRef<'a, T> {
    key: NodeKey,
    tree: &'a mut Context<T>,
}

impl<'a, T> NodeRef<'a, T> {
    /// Create a new node reference.
    pub(crate) fn new(key: NodeKey, tree: &'a mut Context<T>) -> Self {
        Self { key, tree }
    }

    /// Move the reference to the parent element.
    pub fn parent(&mut self) -> &mut Self {
        let parent_key = self.node().parent.unwrap();
        self.key = parent_key;
        self
    }

    /// Get a reference the current node.
    pub fn node(&mut self) -> &mut Node<T> {
        &mut self.tree.tree.nodes[self.key]
    }

    /// Get a reference the current element.
    ///
    /// ## Panics
    /// This function will panic if the current reference is to a text node,
    /// not to an element.
    pub fn element(&mut self) -> &mut ElementData<T> {
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
        if let NodeData::Text {
            content: ref mut dst,
            ..
        } = self.node().data
        {
            *dst = content.into();
        } else {
            todo!()
        }
        self.tree.changes.push(self.key);
    }

    /// Update the size of the element.
    pub fn set_size(&mut self, size: Size<Dimension>) {
        self.as_mut().set_size(size);
        self.tree.changes.push(self.key);
    }

    pub fn translation(&mut self) -> kurbo::Size {
        self.node().translation
    }

    pub fn set_translation(&mut self, size: kurbo::Size) {
        self.node().translation = size;
        self.tree.changes.push(self.key);
    }

    pub fn overflow_x(&mut self) -> Overflow {
        self.node().overflow_x
    }

    pub fn overflow_y(&mut self) -> Overflow {
        self.node().overflow_y
    }

    pub fn set_overflow_x(&mut self, overflow: Overflow) {
        self.node().overflow_x = overflow;
        self.tree.changes.push(self.key);
    }

    pub fn set_overflow_y(&mut self, overflow: Overflow) {
        self.node().overflow_y = overflow;
        self.tree.changes.push(self.key);
    }

    pub fn layout(&mut self) -> Option<taffy::prelude::Layout> {
        self.node().layout
    }

    pub fn scroll(&mut self, delta: kurbo::Size) {
        match (self.overflow_x(), self.overflow_y()) {
            (Overflow::Scroll, Overflow::Scroll) => {
                let x = self.translation().height + delta.width;
                let y = self.translation().height + delta.height;
                self.set_translation(kurbo::Size::new(x, y));
            }
            (Overflow::Scroll, Overflow::Hidden) => {
                let x = self.translation().height + delta.width;
                self.set_translation(kurbo::Size::new(x, 0.));
            }
            (Overflow::Hidden, Overflow::Scroll) => {
                let y = self.translation().height + delta.height;
                self.set_translation(kurbo::Size::new(0., y));
            }
            (Overflow::Hidden, Overflow::Hidden) => {}
        }
    }
}

impl<'a, T> AsMut<ElementData<T>> for NodeRef<'a, T> {
    fn as_mut(&mut self) -> &mut ElementData<T> {
        self.element()
    }
}
