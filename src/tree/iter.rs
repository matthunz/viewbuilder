use super::Tree;
use crate::{node::NodeKind, Node};
use slotmap::DefaultKey;

enum Operation {
    Key(DefaultKey),
    Pop(NodeKind),
}

pub enum Item<'a> {
    Element { element: &'a Node, level: usize },
    Pop { kind: NodeKind, level: usize },
}

pub struct Iter<'a> {
    tree: &'a Tree,
    stack: Vec<Operation>,
    count: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(tree: &'a Tree, root: DefaultKey) -> Self {
        Iter {
            tree,
            stack: vec![Operation::Key(root)],
            count: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Item<'a>;

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

                Item::Element {
                    element: elem,
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
