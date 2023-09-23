use crate::{node::NodeKind, Event, Node, NodeData};
use slotmap::{DefaultKey, SlotMap};

mod iter;
pub use iter::Iter;

use self::iter::Item;

#[derive(Default)]
pub struct Tree {
    nodes: SlotMap<DefaultKey, Node>,
}

impl Tree {
    pub fn send(&mut self, key: DefaultKey, event: Event) {
        let node = &mut self.nodes[key];
        match event {
            Event::Click(click) => {
                if let NodeData::Element { ref mut on_click, .. } = node.data {
                    on_click.as_mut().unwrap()(click)
                }
            }
        }
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
                        NodeData::Element { size, .. } => {
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

    pub fn insert(&mut self, element: Node) -> DefaultKey {
        self.nodes.insert(element)
    }
}
