use crate::{
    element::ElementData,
    event,
    node::{NodeData, NodeKind},
    tree::{Item, ItemMut, NodeRef, Tree},
    Event, Node, NodeKey,
};
use accesskit::{NodeClassSet, NodeId, TreeUpdate};

use skia_safe::Canvas;

use std::num::NonZeroU128;
use taffy::{prelude::Size, style_helpers::TaffyMaxContent, Taffy};

pub(crate) struct Inner {
    pub(crate) changes: Vec<NodeKey>,
    next_id: NonZeroU128,
    unused_ids: Vec<NodeId>,
    taffy: Taffy,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            changes: Default::default(),
            next_id: NonZeroU128::MIN,
            unused_ids: Default::default(),
            taffy: Taffy::default(),
        }
    }
}

enum Handler {
    Click(Box<dyn FnMut(&mut Context, event::Click)>, event::Click),
    MouseIn(Box<dyn FnMut(&mut Context, event::MouseIn)>, event::MouseIn),
    MouseOut(
        Box<dyn FnMut(&mut Context, event::MouseOut)>,
        event::MouseOut,
    ),
}

enum HandlerFn {
    Click(Box<dyn FnMut(&mut Context, event::Click)>),
    MouseIn(Box<dyn FnMut(&mut Context, event::MouseIn)>),
    MouseOut(Box<dyn FnMut(&mut Context, event::MouseOut)>),
}

#[derive(Default)]
pub struct Context {
    pub nodes: Tree,
    pub(crate) inner: Inner,
}

impl Context {
    pub fn send(&mut self, key: NodeKey, event: Event) {
        let node = &mut self.nodes[key];
        let handler = if let NodeData::Element(ref mut elem) = node.data {
            match event {
                Event::Click(click) => elem.on_click.take().map(|f| Handler::Click(f, click)),
                Event::MouseIn(mouse_in) => elem
                    .on_mouse_in
                    .take()
                    .map(|f| Handler::MouseIn(f, mouse_in)),
                Event::MouseOut(mouse_out) => elem
                    .on_mouse_out
                    .take()
                    .map(|f| Handler::MouseOut(f, mouse_out)),
            }
        } else {
            None
        };

        let handler_fn = handler.map(|handler| match handler {
            Handler::Click(mut f, click) => {
                f(self, click);
                HandlerFn::Click(f)
            }
            Handler::MouseIn(mut f, hover) => {
                f(self, hover);
                HandlerFn::MouseIn(f)
            }
            Handler::MouseOut(mut f, mouse_out) => {
                f(self, mouse_out);
                HandlerFn::MouseOut(f)
            }
        });

        let node = &mut self.nodes[key];
        if let Some(handler_fn) = handler_fn {
            if let NodeData::Element(ref mut elem) = node.data {
                match handler_fn {
                    HandlerFn::Click(f) => elem.on_click = Some(f),
                    HandlerFn::MouseIn(f) => elem.on_mouse_in = Some(f),
                    HandlerFn::MouseOut(f) => elem.on_mouse_out = Some(f),
                }
            }
        }
    }

    pub fn display(&self, root: NodeKey) -> String {
        let mut s = String::new();

        for item in self.nodes.iter(root) {
            match item {
                Item::Node {
                    node: element,
                    level,
                    ..
                } => {
                    for _ in 0..level {
                        s.push_str("  ");
                    }

                    match &element.data {
                        NodeData::Text(content) => s.push_str(&format!("\"{}\",", content)),
                        NodeData::Element(ElementData { size, .. }) => {
                            s.push_str("{\n");
                            if let Some(size) = size {
                                for _ in 0..level + 1 {
                                    s.push_str("  ");
                                }

                                s.push_str(&format!(
                                    "size: ({:?}, {:?}),\n",
                                    size.width, size.height
                                ));
                            }
                        }
                    }
                }
                Item::Pop { kind, level } => {
                    if kind == NodeKind::Element {
                        s.push('\n');

                        for _ in 0..level {
                            s.push_str("  ");
                        }

                        s.push_str("},");
                    }
                }
            }
        }
        s
    }

    pub fn insert(&mut self, node: impl Into<Node>) -> NodeKey {
        let key = self.nodes.insert(node.into());
        self.inner.changes.push(key);
        key
    }

    /// Get a reference to the node stored under the given key.
    pub fn node(&mut self, key: NodeKey) -> NodeRef {
        NodeRef::new(key, self)
    }

    pub fn layout(&mut self, root: NodeKey) {
        if self.inner.changes.is_empty() {
            return;
        }

        for key in &self.inner.changes {
            let child_layout_keys: Vec<_> = self.nodes[*key]
                .children
                .iter()
                .flatten()
                .filter_map(|child| self.nodes[*child].layout_key)
                .collect();

            let node = &mut self.nodes[*key];
            node.layout(&mut self.inner.taffy);

            let layout_key = node.layout_key.unwrap();
            let layout_children = self.inner.taffy.children(layout_key).unwrap();
            for child_layout_key in child_layout_keys {
                if !layout_children.contains(&child_layout_key) {
                    self.inner
                        .taffy
                        .add_child(layout_key, child_layout_key)
                        .unwrap();
                }
            }
        }

        // Compute the layout of the taffy tree.
        let root_layout = self.nodes[root].layout_key.unwrap();
        taffy::compute_layout(&mut self.inner.taffy, root_layout, Size::MAX_CONTENT).unwrap();

        // Compute the absolute layout of each node.
        let mut stack: Vec<taffy::prelude::Layout> = Vec::new();
        for item in self.nodes.iter_mut(root) {
            match item {
                ItemMut::Node { node, level: _ } => {
                    let mut layout = self
                        .inner
                        .taffy
                        .layout(node.layout_key.unwrap())
                        .unwrap()
                        .clone();
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

    pub fn semantics(&mut self) -> TreeUpdate {
        let mut tree_update = TreeUpdate::default();
        for key in &self.inner.changes {
            let node = &mut self.nodes[*key];

            let semantics_builder = node.semantics();
            let semantics = semantics_builder.build(&mut NodeClassSet::lock_global());

            let id = if let Some(id) = self.inner.unused_ids.pop() {
                id
            } else {
                let id = self.inner.next_id;
                self.inner.next_id = self.inner.next_id.checked_add(1).unwrap();
                NodeId(id)
            };

            tree_update.nodes.push((id, semantics));
        }
        tree_update
    }

    /// Paint the tree onto a skia canvas, clearing any changes that were made.
    pub fn paint(&mut self, root: NodeKey, canvas: &mut Canvas) {
        for item in self.nodes.iter_mut(root) {
            if let ItemMut::Node { node, level: _ } = item {
                node.paint(canvas);
            }
        }
        self.inner.changes.clear();
    }
}
