use super::Nodes;
use crate::{node::NodeKind, Node};
use slotmap::DefaultKey;
use std::marker::PhantomData;

enum Operation {
    Key(DefaultKey),
    Pop(NodeKind),
}

pub enum Item<'a> {
    Node { node: &'a mut Node, level: usize },
    Pop { kind: NodeKind, level: usize },
}

pub struct IterMut<'a> {
    tree: *mut Nodes,
    stack: Vec<Operation>,
    count: usize,
    _marker: PhantomData<&'a mut Nodes>,
}

impl<'a> IterMut<'a> {
    pub(crate) fn new(tree: &'a mut Nodes, root: DefaultKey) -> Self {
        IterMut {
            tree,
            stack: vec![Operation::Key(root)],
            count: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item = Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|item| match item {
            Operation::Key(key) => {
                let tree = unsafe { &mut *self.tree };
                let elem = &mut tree.nodes[key];

                self.stack.push(Operation::Pop(elem.kind()));
                for child in elem.children.iter().flatten().copied().map(Operation::Key) {
                    self.stack.push(child);
                }

                let count = self.count;
                self.count += 1;

                Item::Node {
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
