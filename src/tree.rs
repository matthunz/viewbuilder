use crate::{
    element::Element,
    layout::{Layout, LayoutTree},
};
use skia_safe::Canvas;
use slotmap::{DefaultKey, SparseSecondaryMap};
use std::{collections::HashSet, mem};

struct Node {
    element: Box<dyn Element>,
    parent: Option<DefaultKey>,
}

#[derive(Default)]
pub struct Tree {
    layout_tree: LayoutTree,
    nodes: SparseSecondaryMap<DefaultKey, Node>,
    pending_layouts: HashSet<DefaultKey>,
    pending_paints: HashSet<DefaultKey>,
}

impl Tree {
    pub fn get(&self, key: DefaultKey) -> &dyn Element {
        &*self.nodes[key].element
    }

    pub fn get_mut(&mut self, key: DefaultKey) -> &mut dyn Element {
        &mut *self.nodes[key].element
    }

    pub fn insert(&mut self, element: Box<dyn Element>) -> DefaultKey {
        let children: Vec<_> = element.children().into_iter().flatten().collect();
        dbg!(&children);
        let key = Layout::builder().build_with_children(&mut self.layout_tree, &children);
        self.nodes.insert(
            key,
            Node {
                element,
                parent: None,
            },
        );

        for child_key in children.iter() {
            self.nodes[*child_key].parent = Some(key);
        }

        self.pending_layouts.insert(key);
        self.pending_paints.insert(key);

        key
    }

    pub fn layout(&mut self, root: DefaultKey) {
        for key in mem::take(&mut self.pending_layouts) {
            let node = &mut self.nodes[key];
            node.element.layout().update(key, &mut self.layout_tree);
        }

        self.layout_tree.build_with_listener(root, |_, _| {});
    }

    pub fn paint(&mut self, canvas: &mut Canvas) {
        // TODO clear
        for key in &self.pending_paints {
            let node = self.nodes.get_mut(*key).unwrap();
            let layout = self.layout_tree.layout(*key).unwrap();
            node.element.paint(&layout, canvas);
        }
    }
}
