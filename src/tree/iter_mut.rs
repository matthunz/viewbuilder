use super::Tree;
use crate::{node::NodeKind, Node, NodeKey};
use std::marker::PhantomData;

enum Operation {
    Key(NodeKey),
    Pop(NodeKind),
}

pub enum ItemMut<'a, T> {
    Node { node: &'a mut Node<T>, level: usize },
    Pop { kind: NodeKind, level: usize },
}

pub struct IterMut<'a, T> {
    tree: *mut Tree<T>,
    stack: Vec<Operation>,
    count: usize,
    _marker: PhantomData<&'a mut Tree<T>>,
}

impl<'a, T> IterMut<'a, T> {
    pub(crate) fn new(tree: &'a mut Tree<T>, root: NodeKey) -> Self {
        IterMut {
            tree,
            stack: vec![Operation::Key(root)],
            count: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = ItemMut<'a, T>;

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

                ItemMut::Node {
                    node: elem,
                    level: count,
                }
            }
            Operation::Pop(kind) => {
                self.count -= 1;
                ItemMut::Pop {
                    kind,
                    level: self.count,
                }
            }
        })
    }
}
