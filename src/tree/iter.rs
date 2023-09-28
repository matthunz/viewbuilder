use super::Tree;
use crate::{node::NodeKind, Node, NodeKey};

enum Operation {
    Key(NodeKey),
    Pop(NodeKind),
}

pub enum Item<'a, T> {
    Node {
        key: NodeKey,
        node: &'a Node<T>,
        level: usize,
    },
    Pop {
        kind: NodeKind,
        level: usize,
    },
}

pub struct Iter<'a, T> {
    tree: &'a Tree<T>,
    stack: Vec<Operation>,
    count: usize,
}

impl<'a, T> Iter<'a, T> {
    pub(crate) fn new(tree: &'a Tree<T>, root: NodeKey) -> Self {
        Iter {
            tree,
            stack: vec![Operation::Key(root)],
            count: 0,
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = Item<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|item| match item {
            Operation::Key(key) => {
                let elem = &self.tree.nodes[key];

                self.stack.push(Operation::Pop(elem.kind()));
                for child in elem.children.iter().flatten().copied().map(Operation::Key) {
                    self.stack.push(child);
                }

                let count = self.count;
                self.count += 1;

                Item::Node {
                    key,
                    node: elem,
                    level: count,
                }
            }
            Operation::Pop(kind) => {
                self.count -= 1;
                Item::Pop {
                    kind,
                    level: self.count,
                }
            }
        })
    }
}
