use crate::{any_element::AnyElement, Element, ElementRef};
use slotmap::{DefaultKey, SparseSecondaryMap};
use std::marker::PhantomData;
use taffy::{prelude::Layout, style::Style, Taffy};

pub struct Node {
    pub(crate) element: Box<dyn AnyElement>,
    pub(crate) layout: Layout,
}

pub struct Transaction {
    pub(crate) nodes: SparseSecondaryMap<DefaultKey, Node>,
    pub(crate) taffy: Taffy,
    pub(crate) root: DefaultKey,
}

impl Transaction {
    pub(crate) fn new() -> Self {
        let mut taffy = Taffy::new();
        let root = taffy.new_leaf(Style::default()).unwrap();

        Self {
            nodes: SparseSecondaryMap::new(),
            taffy,
            root,
        }
    }

    pub fn insert<T>(&mut self, mut element: T) -> ElementRef<T>
    where
        T: Element + 'static,
    {
        let key = self.taffy.new_leaf(element.layout()).unwrap();
        if let Some(children) = element.children() {
            self.taffy.set_children(key, &children).unwrap();
        }

        self.taffy.add_child(self.root, key).unwrap();

        let node = Node {
            element: Box::new(element),
            layout: Layout::new(),
        };
        self.nodes.insert(key, node);

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}
