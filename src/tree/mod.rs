use std::borrow::Cow;

use crate::{
    node::{Element, NodeKind},
    Event, Node, NodeData,
};
use slotmap::{DefaultKey, SlotMap};

mod iter;
pub use iter::Iter;
use taffy::{prelude::Size, style::Dimension};

use self::iter::Item;

#[derive(Default)]
struct Inner {
    pub(crate) changes: Vec<DefaultKey>,
}

#[derive(Default)]
pub struct Tree {
    nodes: SlotMap<DefaultKey, Node>,
    inner: Inner,
}

impl Tree {
    pub fn send(&mut self, key: DefaultKey, event: Event) {
        let node = &mut self.nodes[key];
        let (mut handler, click) = match event {
            Event::Click(click) => {
                if let NodeData::Element(Element {
                    ref mut on_click, ..
                }) = node.data
                {
                    (on_click.take().unwrap(), click)
                } else {
                    todo!()
                }
            }
        };

        handler(self, click);

        let node = &mut self.nodes[key];
        if let NodeData::Element(Element {
            ref mut on_click, ..
        }) = node.data
        {
            *on_click = Some(handler);
        } else {
            todo!()
        };
    }

    pub fn iter(&self, root: DefaultKey) -> Iter {
        Iter::new(self, root)
    }

    pub fn display(&self, root: DefaultKey) -> String {
        let mut s = String::new();

        for item in self.iter(root) {
            match item {
                Item::Element { element, level } => {
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
                    if kind == NodeKind::Container {
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
        self.nodes.insert(node.into())
    }

    pub fn element(&mut self, key: DefaultKey) -> ElementRef {
        if let NodeData::Element(ref mut element) = self.nodes[key].data {
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
        if let NodeData::Text(ref mut dst) = self.nodes[key].data {
            *dst = content.into();
        } else {
            todo!()
        }
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
}
