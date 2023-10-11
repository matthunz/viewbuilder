use core::fmt;
use std::collections::HashMap;
use taffy::{
    prelude::{Layout, Size},
    style::Style,
    style_helpers::TaffyMaxContent,
    Taffy,
};

pub use taffy::node::Node;

mod iter;
pub use self::iter::Iter;

enum Operation {
    Push(Node),
    Pop,
}

#[derive(Debug, Default)]
pub struct LayoutNode {
    pub style: Style,
    pub translation: Size<f32>,
    pub is_listening: bool,
}

pub struct TreeLayout {
    pub layout: Layout,
    pub translation: Size<f32>,
}

#[derive(Debug)]
struct GlobalLayout {
    layout: Layout,
    is_listening: bool,
    translation: Size<f32>,
}

#[derive(Default)]
pub struct LayoutTree {
    taffy: Taffy,
    global_layouts: HashMap<Node, GlobalLayout>,
}

impl LayoutTree {
    /// Return the global layout of a node by its key.
    pub fn layout(&self, key: Node) -> Option<TreeLayout> {
        self.global_layouts.get(&key).map(|global| TreeLayout {
            layout: global.layout,
            translation: global.translation,
        })
    }

    pub fn iter(&self, root: Node) -> Iter {
        Iter::new(self, root)
    }

    /// Insert a new node and return its key.
    pub fn insert(&mut self, node: LayoutNode) -> Node {
        let key = self.taffy.new_leaf(node.style).unwrap();
        self.global_layouts.insert(
            key,
            GlobalLayout {
                layout: Layout::new(),
                is_listening: node.is_listening,
                translation: node.translation,
            },
        );
        key
    }

    /// Insert a new node with children and return its key.
    pub fn insert_with_children(
        &mut self,
        node: LayoutNode,
        children: &[Node],
    ) -> Node {
        let key = self.taffy.new_with_children(node.style, children).unwrap();
        self.global_layouts.insert(
            key,
            GlobalLayout {
                layout: Layout::new(),
                is_listening: node.is_listening,
                translation: node.translation,
            },
        );
        key
    }

    pub fn is_listening(&self, key: Node) -> bool {
        let global_layout = self.global_layouts.get(&key).unwrap();
        global_layout.is_listening
    }

    pub fn listen(&mut self, key: Node) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = true;
    }

    pub fn unlisten(&mut self, key: Node) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = false;
    }

    /// Compute the layout of the tree.
    pub fn build_with_listener(
        &mut self,
        root: Node,
        mut listener: impl FnMut(Node, &Layout),
    ) {
        taffy::compute_layout(&mut self.taffy, root, Size::MAX_CONTENT).unwrap();

        let mut stack = vec![Operation::Push(root)];
        let mut layouts: Vec<TreeLayout> = vec![];
        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let mut layout = self.taffy.layout(key).unwrap().clone();
                    if let Some(parent) = layouts.last() {
                        layout.location.x += parent.layout.location.x + parent.translation.width;
                        layout.location.y += parent.layout.location.y + parent.translation.height;
                    }

                    let dst = self.global_layouts.get_mut(&key).unwrap();
                    if dst.layout.location != layout.location
                        || dst.layout.order != layout.order
                        || dst.layout.size != layout.size
                    {
                        if dst.is_listening {
                            listener(key, &layout)
                        }
                        dst.layout = layout;
                    }

                    layouts.push(TreeLayout {
                        layout,
                        translation: dst.translation,
                    });
                    stack.push(Operation::Pop);

                    let children = self.taffy.children(key).unwrap();
                    stack.extend(children.into_iter().map(|child| Operation::Push(child)));
                }
                Operation::Pop => {
                    layouts.pop();
                }
            }
        }
    }
}

impl fmt::Debug for LayoutTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LayoutTree")
            .field("global_layouts", &self.global_layouts)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::LayoutNode;
    use crate::LayoutTree;
    use taffy::prelude::Size;

    #[test]
    fn it_works() {
        let mut tree = LayoutTree::default();
        let a = tree.insert(LayoutNode::default());
        let b = tree.insert_with_children(
            LayoutNode {
                is_listening: true,
                ..Default::default()
            },
            &[a],
        );
        let root = tree.insert_with_children(
            LayoutNode {
                translation: Size {
                    width: 0.,
                    height: 50.,
                },
                ..Default::default()
            },
            &[b],
        );

        tree.build_with_listener(root, |key, layout| {
            dbg!(key, layout);
        });
    }
}
