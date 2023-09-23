use crate::{
    event,
    node::{Element, NodeData, NodeKind},
    Event, Node,
};
use accesskit::{NodeClassSet, NodeId, TreeUpdate};
use kurbo::Point;
use skia_safe::{Canvas, Color4f};
use slotmap::{DefaultKey, SlotMap};
use std::{borrow::Cow, num::NonZeroU128};
use taffy::{prelude::Size, style::Dimension, style_helpers::TaffyMaxContent, Taffy};

mod iter;
use self::iter::Item;
pub use self::iter::Iter;

mod iter_mut;
pub use iter_mut::IterMut;

struct Inner {
    pub(crate) changes: Vec<DefaultKey>,
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
    Click(Box<dyn FnMut(&mut Tree, event::Click)>, event::Click),
    MouseIn(Box<dyn FnMut(&mut Tree, event::MouseIn)>, event::MouseIn),
    MouseOut(Box<dyn FnMut(&mut Tree, event::MouseOut)>, event::MouseOut),
}

enum HandlerFn {
    Click(Box<dyn FnMut(&mut Tree, event::Click)>),
    MouseIn(Box<dyn FnMut(&mut Tree, event::MouseIn)>),
    MouseOut(Box<dyn FnMut(&mut Tree, event::MouseOut)>),
}

#[derive(Default)]
pub struct Tree {
    pub nodes: Nodes,
    inner: Inner,
}

impl Tree {
    pub fn send(&mut self, key: DefaultKey, event: Event) {
        let node = &mut self.nodes.nodes[key];
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

        let node = &mut self.nodes.nodes[key];
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

    pub fn display(&self, root: DefaultKey) -> String {
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
                        NodeData::Element(Element { size, .. }) => {
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

    pub fn insert(&mut self, node: impl Into<Node>) -> DefaultKey {
        let key = self.nodes.nodes.insert(node.into());
        self.inner.changes.push(key);
        key
    }

    pub fn element(&mut self, key: DefaultKey) -> ElementRef {
        if let NodeData::Element(ref mut element) = self.nodes.nodes[key].data {
            ElementRef {
                key,
                element,
                inner: &mut self.inner,
            }
        } else {
            todo!()
        }
    }

    pub fn set_text(&mut self, key: DefaultKey, content: impl Into<Cow<'static, str>>) {
        if let NodeData::Text(ref mut dst) = self.nodes.nodes[key].data {
            *dst = content.into();
        } else {
            todo!()
        }
    }

    pub fn layout(&mut self, root: DefaultKey) {
        if self.inner.changes.is_empty() {
            return;
        }

        for key in &self.inner.changes {
            let child_layout_keys: Vec<_> = self.nodes.nodes[*key]
                .children
                .iter()
                .flatten()
                .filter_map(|child| self.nodes.nodes[*child].layout_key)
                .collect();

            let node = &mut self.nodes.nodes[*key];
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
        let root_layout = self.nodes.nodes[root].layout_key.unwrap();
        taffy::compute_layout(&mut self.inner.taffy, root_layout, Size::MAX_CONTENT).unwrap();

        // Compute the absolute layout of each node.
        let mut stack: Vec<taffy::prelude::Layout> = Vec::new();
        for item in self.nodes.iter_mut(root) {
            match item {
                iter_mut::Item::Node { node, level: _ } => {
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
                iter_mut::Item::Pop { kind: _, level: _ } => {
                    stack.pop();
                }
            }
        }
    }

    pub fn semantics(&mut self) -> TreeUpdate {
        let mut tree_update = TreeUpdate::default();
        for key in &self.inner.changes {
            let node = &mut self.nodes.nodes[*key];

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

    pub fn paint(&mut self, root: DefaultKey, canvas: &mut Canvas) {
        for item in self.nodes.iter_mut(root) {
            if let iter_mut::Item::Node { node, level: _ } = item {
                node.paint(canvas);
            }
        }
        self.inner.changes.clear();
    }

    pub fn target(&self, root: DefaultKey, point: Point) -> Option<DefaultKey> {
        self.nodes
            .iter(root)
            .filter_map(|item| {
                if let Item::Node {
                    key,
                    node,
                    level: _,
                } = item
                {
                    if node.kind() == NodeKind::Text {
                        return None;
                    }

                    let layout = node.layout.unwrap();
                    if point.x >= layout.location.x as _
                        && point.y >= layout.location.y as _
                        && point.x <= (layout.location.x + layout.size.width) as _
                        && point.y <= (layout.location.y + layout.size.height) as _
                    {
                        Some((key, layout))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .max_by_key(|(_, layout)| layout.order)
            .map(|(key, _layout)| key)
    }
}

#[derive(Default)]
pub struct Nodes {
    pub nodes: SlotMap<DefaultKey, Node>,
}

impl Nodes {
    pub fn iter(&self, root: DefaultKey) -> Iter {
        Iter::new(self, root)
    }

    pub fn iter_mut(&mut self, root: DefaultKey) -> IterMut {
        IterMut::new(self, root)
    }
}

pub struct ElementRef<'a> {
    key: DefaultKey,
    element: &'a mut Element,
    inner: &'a mut Inner,
}

impl<'a> ElementRef<'a> {
    pub fn set_size(&mut self, size: Size<Dimension>) {
        self.element.size = Some(size);
        self.inner.changes.push(self.key);
    }

    pub fn set_background_color(&mut self, color: Color4f) {
        self.element.background_color = Some(color);
        self.inner.changes.push(self.key);
    }
}
