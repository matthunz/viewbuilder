use crate::{
    event::{self, EventKind},
    node::NodeData,
    tree::{ItemMut, NodeRef, Tree},
    Event, Node, NodeKey,
};
use accesskit::{NodeClassSet, NodeId, TreeUpdate};
use skia_safe::Canvas;
use std::num::NonZeroU128;
use taffy::{prelude::Size, style_helpers::TaffyMaxContent, Taffy};

/// Render context for a UI tree.
///
/// This struct acts a state machine where changes to the tree are stored between paint cycles.
///
/// 1. [`Context::layout`] computes the layout of the tree, based on changes made.
///
/// 2. [`Context::semantics`] computes the semantics tree update, also based on changes made.
///
/// 2. [`Context::paint`] will paint the tree top-down and clear any changes.
pub struct Context {
    /// Node tree.
    pub tree: Tree,

    /// Changes to be rendered in the next paint cycle.
    pub(crate) changes: Vec<NodeKey>,

    /// Next semantics node ID.
    next_id: NonZeroU128,

    /// Unused semantics node IDs.
    unused_ids: Vec<NodeId>,

    /// Taffy layout tree.
    taffy: Taffy,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            tree: Tree::default(),
            changes: Default::default(),
            next_id: NonZeroU128::MIN,
            unused_ids: Default::default(),
            taffy: Taffy::default(),
        }
    }
}

impl Context {
    /// Send an event to an element in the tree.
    pub fn send(&mut self, key: NodeKey, event: Event) {
        let node = &mut self.tree[key];
        let kind = event.kind();
        let cell = if let NodeData::Element(ref mut elem) = node.data {
            let (handler_cell, mouse_event) = match event {
                Event::Click(click) => (elem.on_click().take(), click),
                Event::MouseIn(mouse_in) => (elem.on_mouse_in().take(), mouse_in),
                Event::MouseOut(mouse_out) => (elem.on_mouse_out().take(), mouse_out),
            };
            handler_cell
                .map(|handler| (handler, mouse_event))
        } else {
            None
        };

        if let Some((mut handler, mouse_event)) = cell {
            handler(self, mouse_event);

            let node = &mut self.tree[key];
            if let NodeData::Element(ref mut elem) = node.data {
                match kind {
                    EventKind::Click => elem.set_on_click(handler),
                    EventKind::MouseIn => elem.set_on_mouse_in(handler),
                    EventKind::MouseOut => elem.set_on_mouse_out(handler),
                }
            }
        }
    }

    /// Insert a node into the tree, returning its key.
    pub fn insert(&mut self, node: impl Into<Node>) -> NodeKey {
        let key = self.tree.insert(node.into());
        self.changes.push(key);
        key
    }

    /// Get a reference to the node stored under the given key.
    pub fn node(&mut self, key: NodeKey) -> NodeRef {
        NodeRef::new(key, self)
    }

    /// Compute the layout of the tree, starting at a root node key.
    pub fn layout(&mut self, root: NodeKey) {
        if self.changes.is_empty() {
            return;
        }

        for key in &self.changes {
            let child_layout_keys: Vec<_> = self.tree[*key]
                .children
                .iter()
                .flatten()
                .filter_map(|child| self.tree[*child].layout_key)
                .collect();

            let node = &mut self.tree[*key];
            node.build_layout(&mut self.taffy);

            let layout_key = node.layout_key.unwrap();
            let layout_children = self.taffy.children(layout_key).unwrap();
            for child_layout_key in child_layout_keys {
                if !layout_children.contains(&child_layout_key) {
                    self.taffy.add_child(layout_key, child_layout_key).unwrap();
                }
            }
        }

        // Compute the layout of the taffy tree.
        let root_layout = self.tree[root].layout_key.unwrap();
        taffy::compute_layout(&mut self.taffy, root_layout, Size::MAX_CONTENT).unwrap();

        // Compute the absolute layout of each node.
        let mut stack: Vec<taffy::prelude::Layout> = Vec::new();
        for item in self.tree.iter_mut(root) {
            match item {
                ItemMut::Node { node, level: _ } => {
                    let mut layout = self.taffy.layout(node.layout_key.unwrap()).unwrap().clone();
                    if let Some(parent_layout) = stack.last() {
                        layout.location.x += parent_layout.location.x;
                        layout.location.y += parent_layout.location.y;
                    }
                    node.layout = Some(layout);
                    stack.push(layout);
                }
                ItemMut::Pop { kind: _, level: _ } => {
                    stack.pop();
                }
            }
        }
    }

    /// Compute the semantics tree update for the tree.
    pub fn semantics(&mut self) -> TreeUpdate {
        let mut tree_update = TreeUpdate::default();
        for key in &self.changes {
            let node = &mut self.tree[*key];

            let semantics_builder = node.build_semantics();
            let semantics = semantics_builder.build(&mut NodeClassSet::lock_global());

            let id = if let Some(id) = self.unused_ids.pop() {
                id
            } else {
                let id = self.next_id;
                self.next_id = self.next_id.checked_add(1).unwrap();
                NodeId(id)
            };

            tree_update.nodes.push((id, semantics));
        }
        tree_update
    }

    /// Paint the tree onto a skia canvas, clearing any changes that were made.
    pub fn paint(&mut self, root: NodeKey, canvas: &mut Canvas) {
        for item in self.tree.iter_mut(root) {
            if let ItemMut::Node { node, level: _ } = item {
                node.paint(canvas);
            }
        }
        self.changes.clear();
    }
}
