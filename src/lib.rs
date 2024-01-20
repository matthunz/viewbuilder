use std::{any, fmt, ops::Index};
use slotmap::{DefaultKey, SlotMap};

pub trait Element {}

pub trait AnyElement {
    fn name(&self) -> &'static str;
}

impl<E: Element> AnyElement for E {
    fn name(&self) -> &'static str {
        any::type_name::<E>()
    }
}

struct Node {
    element: Box<dyn AnyElement>,
    children: Vec<DefaultKey>,
}

#[derive(Default)]
pub struct Tree {
    nodes: SlotMap<DefaultKey, Node>,
}

impl Tree {
    pub fn insert(&mut self, element: impl Element + 'static)  -> DefaultKey{
        let node = Node {
            element: Box::new(element),
            children: Vec::new(),
        };
        self.nodes.insert(node)
    }

    pub fn slice(&self, root: DefaultKey) -> Slice {
        let node = &self.nodes[root];
        Slice { tree: self, node }
    }
}

pub struct Slice<'a> {
    tree: &'a Tree,
    node: &'a Node,
}

impl fmt::Debug for Slice<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tuple = f.debug_tuple(self.node.element.name());

        for child_key in &self.node.children {
            let child = &self.tree.nodes[*child_key];
            tuple.field(&Slice {
                tree: self.tree,
                node: child,
            });
        }

        tuple.finish()
    }
}
