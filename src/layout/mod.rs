//! Layout

use crate::{geometry::Size, Operation};
use core::fmt;
use dioxus::{
    core::{
        exports::bumpalo::{boxed::Box as BumpBox, Bump},
        AnyValue, AttributeValue,
    },
    prelude::IntoAttributeValue,
};
use slotmap::SparseSecondaryMap;
use std::cell::RefCell;
use taffy::{style_helpers::TaffyMaxContent, Taffy};

pub use taffy::node::Node;

mod iter;
pub use self::iter::Iter;

mod layout;
pub use layout::Layout;

mod builder;
pub use self::builder::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlexDirection {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

impl FlexDirection {
    pub(crate) fn into_taffy(self) -> taffy::prelude::FlexDirection {
        match self {
            FlexDirection::Row => taffy::prelude::FlexDirection::Row,
            FlexDirection::RowReverse => taffy::prelude::FlexDirection::RowReverse,
            FlexDirection::Column => taffy::prelude::FlexDirection::Column,
            FlexDirection::ColumnReverse => taffy::prelude::FlexDirection::ColumnReverse,
        }
    }
}

impl<'a> IntoAttributeValue<'a> for FlexDirection {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let boxed: BumpBox<'a, dyn AnyValue> = unsafe { BumpBox::from_raw(bump.alloc(self)) };
        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}

#[derive(Debug)]
struct GlobalLayout {
    layout: taffy::prelude::Layout,
    is_listening: bool,
    translation: Size<f32>,
}

/// Layout tree built with Taffy.
#[derive(Default)]
pub struct LayoutTree {
    /// The taffy layout tree.
    taffy: Taffy,

    /// Global layout mappings to the taffy tree.
    global_layouts: SparseSecondaryMap<Node, GlobalLayout>,
}

impl LayoutTree {
    /// Return the global layout of a node by its key.
    pub fn layout(&self, key: Node) -> Option<Layout> {
        self.global_layouts.get(key).map(|global| Layout {
            layout: global.layout,
            translation: global.translation,
        })
    }

    /// Create an iterator over the layouts in the tree.
    pub fn iter(&self, root: Node) -> Iter {
        Iter::new(self, root)
    }

    pub fn children(&self, parent: Node) -> Vec<Node> {
        self.taffy.children(parent).unwrap()
    }

    pub fn add_child(&mut self, parent: Node, child: Node) {
        self.taffy.add_child(parent, child).unwrap();
    }

    /// Check the listening flag for a node in the tree.
    pub fn is_listening(&self, key: Node) -> bool {
        let global_layout = self.global_layouts.get(key).unwrap();
        global_layout.is_listening
    }

    /// Set the listening flag for a node in the tree.
    pub fn listen(&mut self, key: Node) {
        let global_layout = self.global_layouts.get_mut(key).unwrap();
        global_layout.is_listening = true;
    }

    /// Remove the listening flag for a node in the tree.
    pub fn unlisten(&mut self, key: Node) {
        let global_layout = self.global_layouts.get_mut(key).unwrap();
        global_layout.is_listening = false;
    }

    /// Get the current translation of a node in the tree.
    pub fn translation(&self, key: Node) -> Size<f32> {
        let global_layout = self.global_layouts.get(key).unwrap();
        global_layout.translation
    }

    /// Get a mutable reference to the current translation of a node in the tree.
    pub fn translation_mut(&mut self, key: Node) -> &mut Size<f32> {
        let global_layout = self.global_layouts.get_mut(key).unwrap();
        &mut global_layout.translation
    }

    /// Compute the layout of the tree.
    pub fn build_with_listener(&mut self, root: Node, mut listener: impl FnMut(Node, &Layout)) {
        taffy::compute_layout(&mut self.taffy, root, taffy::prelude::Size::MAX_CONTENT).unwrap();

        let mut stack = vec![Operation::Push(root)];
        let mut layouts: Vec<Layout> = vec![];
        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let mut layout = *self.taffy.layout(key).unwrap();
                    if let Some(parent) = layouts.last() {
                        layout.location.x += parent.layout.location.x + parent.translation.width;
                        layout.location.y += parent.layout.location.y + parent.translation.height;
                    }

                    let dst = self.global_layouts.get_mut(key).unwrap();
                    if dst.layout.location != layout.location
                        || dst.layout.order != layout.order
                        || dst.layout.size != layout.size
                    {
                        if dst.is_listening {
                            listener(
                                key,
                                &Layout {
                                    layout,
                                    translation: dst.translation,
                                },
                            )
                        }
                        dst.layout = layout;
                    }

                    layouts.push(Layout {
                        layout,
                        translation: dst.translation,
                    });
                    stack.push(Operation::Pop);

                    let children = self.taffy.children(key).unwrap();
                    stack.extend(children.into_iter().map(Operation::Push));
                }
                Operation::Pop => {
                    layouts.pop();
                }
            }
        }
    }
}

impl fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayoutTree")
            .field("global_layouts", &self.global_layouts)
            .finish()
    }
}
