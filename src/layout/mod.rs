use core::fmt;
use slotmap::DefaultKey;
use std::collections::HashMap;
use taffy::{
    prelude::{Layout, Size},
    style::Style,
    style_helpers::TaffyMaxContent,
    Taffy,
};

enum Operation {
    Push(DefaultKey),
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
    global_layouts: HashMap<DefaultKey, GlobalLayout>,
}

impl LayoutTree {
    pub fn layout(&self, key: DefaultKey) -> Option<TreeLayout> {
        self.global_layouts.get(&key).map(|global| TreeLayout {
            layout: global.layout,
            translation: global.translation,
        })
    }

    pub fn insert(&mut self, node: LayoutNode) -> DefaultKey {
        let key = self.taffy.new_leaf(node.style).unwrap();
        self.global_layouts.insert(
            key,
            GlobalLayout {
                layout: Layout::new(),
                is_listening: false,
                translation: node.translation,
            },
        );
        key
    }

    pub fn insert_with_children(&mut self, style: Style, children: &[DefaultKey]) -> DefaultKey {
        self.taffy.new_with_children(style, children).unwrap()
    }

    pub fn is_listening(&self, key: DefaultKey) -> bool {
        let global_layout = self.global_layouts.get(&key).unwrap();
        global_layout.is_listening
    }

    pub fn listen(&mut self, key: DefaultKey) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = true;
    }

    pub fn unlisten(&mut self, key: DefaultKey) {
        let global_layout = self.global_layouts.get_mut(&key).unwrap();
        global_layout.is_listening = false;
    }

    /// Compute the layout of the tree.
    pub fn build_with_listener(
        &mut self,
        root: DefaultKey,
        mut listener: impl FnMut(DefaultKey, &Layout),
    ) {
        taffy::compute_layout(&mut self.taffy, root, Size::MAX_CONTENT).unwrap();

        let mut stack = vec![Operation::Push(root)];
        let mut layouts: Vec<Layout> = vec![];
        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let mut layout = self.taffy.layout(key).unwrap().clone();
                    if let Some(parent_layout) = layouts.last() {
                        layout.location.x += parent_layout.location.x;
                        layout.location.y += parent_layout.location.y;
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

                    layouts.push(layout);
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
    use taffy::{
        prelude::{Rect, Size},
        style::{LengthPercentage, Style},
    };

    #[test]
    fn it_works() {
        let mut tree = LayoutTree::default();

        let a = tree.insert(LayoutNode::default());

        let b = tree.insert_with_children(Style::default(), &[a]);

        let root = tree.insert_with_children(
            Style {
                size: Size::from_points(100., 100.),
                padding: Rect {
                    left: LengthPercentage::Points(100.),
                    right: LengthPercentage::Points(0.),
                    top: LengthPercentage::Points(0.),
                    bottom: LengthPercentage::Points(0.),
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
