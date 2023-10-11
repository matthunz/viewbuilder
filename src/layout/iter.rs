use super::{LayoutTree, Node, TreeLayout};

pub struct Iter<'a> {
    tree: &'a LayoutTree,
    stack: Vec<Node>,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(tree: &'a LayoutTree, root: Node) -> Self {
        Self {
            tree,
            stack: vec![root],
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = TreeLayout;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop().map(|key| self.tree.layout(key).unwrap())
    }
}
