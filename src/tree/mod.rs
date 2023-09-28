use crate::{
    node::{NodeData, NodeKind},
    Node, NodeKey,
};
use kurbo::Point;
use slotmap::SlotMap;
use std::ops::{Index, IndexMut};

mod iter;
pub use self::iter::{Item, Iter};

mod iter_mut;
pub use iter_mut::{ItemMut, IterMut};

mod node_ref;
pub use self::node_ref::NodeRef;

/// Tree of user interface nodes.

pub struct Tree<T> {
    nodes: SlotMap<NodeKey, Node<T>>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<T> Tree<T> {
    pub fn iter(&self, root: NodeKey) -> Iter<T> {
        Iter::new(self, root)
    }

    pub fn iter_mut(&mut self, root: NodeKey) -> IterMut<T> {
        IterMut::new(self, root)
    }

    pub(crate) fn insert(&mut self, node: Node<T>) -> NodeKey {
        self.nodes.insert(node)
    }

    /// Get the target under the current point.
    ///
    /// This function will return `Some` with a key to the node at `point`
    /// with the highest layout order. Otherwise `None` is returned.
    pub fn target(&self, root: NodeKey, point: Point) -> Option<NodeKey> {
        self.iter(root)
            .filter_map(|item| {
                if let Item::Node {
                    key,
                    node,
                    level: _,
                } = item
                {
                    // Ignore text nodes
                    if node.kind() == NodeKind::Text {
                        return None;
                    }

                    // Check if `point` is contained inside the current element.
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

    /// Display the tree as a string starting from a root key.
    pub fn display(&self, root: NodeKey) -> String {
        let mut s = String::new();

        for item in self.iter(root) {
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
                        NodeData::Text { content, .. } => s.push_str(&format!("\"{}\",", content)),
                        NodeData::Element(data) => {
                            s.push_str("{\n");
                            if let Some(size) = data.size() {
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
}

impl<T> Index<NodeKey> for Tree<T> {
    type Output = Node<T>;

    fn index(&self, index: NodeKey) -> &Self::Output {
        &self.nodes[index]
    }
}

impl<T> IndexMut<NodeKey> for Tree<T> {
    fn index_mut(&mut self, index: NodeKey) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
