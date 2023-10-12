use crate::{element::Element, layout::LayoutTree, Operation};
use skia_safe::Canvas;
use slotmap::{DefaultKey, SlotMap};
use std::mem;

struct Node {
    element: Box<dyn Element>,
    layout_key: Option<DefaultKey>,
    parent: Option<DefaultKey>,
}

#[derive(Default)]
pub struct Tree {
    layout_tree: LayoutTree,
    nodes: SlotMap<DefaultKey, Node>,
    dirty: Vec<DefaultKey>,
}

impl Tree {
    pub fn get_mut(&mut self, key: DefaultKey) -> &mut dyn Element {
        &mut *self.nodes[key].element
    }

    pub fn insert(&mut self, element: Box<dyn Element>) -> DefaultKey {
        let key = self.nodes.insert(Node {
            element,
            layout_key: None,
            parent: None
        });

        for child_key in self.nodes[key].element.children().iter().flatten() {
            self.nodes[*child_key].parent = Some(key);
        }

        self.dirty.push(key);
        key
    }

    pub fn layout(&mut self, root: DefaultKey) {
        for key in mem::take(&mut self.dirty) {
            let node = &mut self.nodes[key];
            let layout_key = node.element.layout().build(&mut self.layout_tree);
            node.layout_key = Some(layout_key);

            if let Some(parent_key) = node.parent {
                if let Some(parent_layout_key) = self.nodes[parent_key].layout_key {
                    self.layout_tree.add_child(parent_layout_key, layout_key);
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

                    parents.push(key);

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
    }
}
