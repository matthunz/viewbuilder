use crate::{node::NodeKind, Node, NodeKey};
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
#[derive(Default)]
pub struct Tree {
    nodes: SlotMap<NodeKey, Node>,
}

impl Tree {
    pub fn iter(&self, root: NodeKey) -> Iter {
        Iter::new(self, root)
    }

    pub fn iter_mut(&mut self, root: NodeKey) -> IterMut {
        IterMut::new(self, root)
    }

    pub(crate) fn insert(&mut self, node: Node) -> NodeKey {
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
}

impl Index<NodeKey> for Tree {
    type Output = Node;

    fn index(&self, index: NodeKey) -> &Self::Output {
        &self.nodes[index]
    }
}

impl IndexMut<NodeKey> for Tree {
    fn index_mut(&mut self, index: NodeKey) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
