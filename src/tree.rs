use crate::{
    layout::{self, LayoutTree},
    Operation,
};
use slotmap::{DefaultKey, SlotMap};

pub trait Element {
    fn children(&mut self) -> Option<Vec<DefaultKey>>;

    fn layout(&mut self) -> layout::Builder;
}

struct Node {
    element: Box<dyn Element>,
    layout_key: Option<DefaultKey>,
}

#[derive(Default)]
pub struct Tree {
    layout_tree: LayoutTree,
    nodes: SlotMap<DefaultKey, Node>,
}

impl Tree {
    pub fn insert(&mut self, element: Box<dyn Element>) -> DefaultKey {
        self.nodes.insert(Node {
            element,
            layout_key: None,
        })
    }

    pub fn layout(&mut self, root: DefaultKey) {
        let mut stack = vec![Operation::Push(root)];
        let mut parents = Vec::new();

        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let node = self.nodes.get_mut(key).unwrap();
                    let layout_key = node.element.layout().build(&mut self.layout_tree);
                    node.layout_key = Some(layout_key);

                    stack.push(Operation::Pop);
                    parents.push(key);

                    stack.extend(
                        node.element
                            .children()
                            .iter()
                            .flatten()
                            .map(|child_key| Operation::Push(*child_key)),
                    )
                }
                Operation::Pop => {
                    parents.pop();
                }
            }
        }
    }
}
