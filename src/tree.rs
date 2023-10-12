use crate::{element::Element, layout::LayoutTree, Operation};
use skia_safe::Canvas;
use slotmap::{DefaultKey, SlotMap};

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

                    if let Some(parent) = parents.last() {
                        self.layout_tree.add_child(*parent, layout_key);
                    }
                    parents.push(layout_key);

                    stack.push(Operation::Pop);
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

        self.layout_tree.build_with_listener(root, |_, _| {});
    }

    pub fn paint(&mut self, root: DefaultKey, canvas: &mut Canvas) {
        let mut stack = vec![Operation::Push(root)];
        let mut parents = Vec::new();

        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let node = self.nodes.get_mut(key).unwrap();
                    let layout = self.layout_tree.layout(node.layout_key.unwrap()).unwrap();
                    node.element.paint(&layout, canvas);

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
