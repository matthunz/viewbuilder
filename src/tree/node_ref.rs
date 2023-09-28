use crate::{
    element::{ElementData, Overflow},
    node::NodeData,
    Context, Node, NodeKey,
};
use std::borrow::Cow;
use taffy::{prelude::Size, style::Dimension};

macro_rules! make_methods {
    ($fn_ident:ident, $set_fn_ident:ident) => {
        pub fn $fn_ident(&mut self) -> Overflow {
            self.element().$fn_ident().unwrap_or(Overflow::Hidden)
        }

        pub fn $set_fn_ident(&mut self, overflow: Overflow) {
            self.element().$set_fn_ident(overflow);
            self.update();
        }
    };
}

/// Reference to an element in a tree.
///
/// This struct is created with [`Context::node`].
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

    pub fn update(&mut self) {
        self.tree.changes.insert(self.key);
    }

    /// Update the text of a node.
    ///
    /// ## Panics
    /// This function will panic if the current reference is to an element,
    /// not to a text node.
    pub fn set_text(&mut self, content: impl Into<Cow<'static, str>>) {
        if let NodeData::Text {
            content: ref mut dst,
            ref mut data,
            ..
        } = self.node().data
        {
            *dst = content.into();
            data.as_mut().unwrap().text_blob.take();
        } else {
            todo!()
        }
        self.update();
    }

    /// Update the size of the element.
    pub fn set_size(&mut self, size: Size<Dimension>) {
        self.as_mut().set_size(size);
        self.update();
    }

    /// Get the current scroll translation.
    pub fn translation(&mut self) -> kurbo::Size {
        self.node().translation
    }

    /// Set the current scroll translation.
    pub fn set_translation(&mut self, size: kurbo::Size) {
        self.node().translation = size;
        self.update();
    }

    make_methods!(overflow_x, set_overflow_x);
    make_methods!(overflow_y, set_overflow_y);

    /// Get the absolute layout of the node, relative to the window.
    pub fn layout(&mut self) -> Option<taffy::prelude::Layout> {
        self.node().layout
    }

    /// Scroll the node by a delta in 2D.
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
