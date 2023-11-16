use crate::{any_element::AnyElement, element::View, Element, ElementRef};
use slotmap::{DefaultKey, SparseSecondaryMap};
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};
use taffy::{prelude::Layout, style::Style, Taffy};

pub struct Node {
    pub(crate) element: Box<dyn AnyElement>,
    pub(crate) layout: Layout,
}

/// Graphical user interface.
pub struct UserInterface {
    pub(crate) nodes: SparseSecondaryMap<DefaultKey, Node>,
    pub(crate) taffy: Taffy,
    pub(crate) root: DefaultKey,
}

impl UserInterface {
    pub(crate) fn new() -> Self {
        let mut taffy = Taffy::new();
        let root = taffy.new_leaf(Style::default()).unwrap();
        let mut nodes = SparseSecondaryMap::new();
        let node = Node {
            element: Box::new(View::default()),
            layout: Layout::new(),
        };
        nodes.insert(root, node);

        Self { nodes, taffy, root }
    }

    /// Insert an element into the user interface.
    pub fn insert<T>(&mut self, mut element: T) -> ElementRef<T>
    where
        T: Element + 'static,
    {
        let key = self.taffy.new_leaf(element.layout()).unwrap();
        if let Some(children) = element.children() {
            self.taffy.set_children(key, &children).unwrap();
        }

        let root: &mut View = self.nodes[self.root]
            .element
            .as_any_mut()
            .downcast_mut()
            .unwrap();
        root.add_child(key);
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

    pub fn levels_mut(&mut self) -> LevelsMut {
        LevelsMut {
            stack: vec![Item::Push(self.root)],
            ui: self,
        }
    }
}

impl<T: 'static> Index<ElementRef<T>> for UserInterface {
    type Output = T;

    fn index(&self, index: ElementRef<T>) -> &Self::Output {
        index.get(self).unwrap()
    }
}

impl<T: 'static> IndexMut<ElementRef<T>> for UserInterface {
    fn index_mut(&mut self, index: ElementRef<T>) -> &mut Self::Output {
        index.get_mut(self).unwrap()
    }
}

pub enum Item {
    Push(DefaultKey),
    Pop,
}

pub struct LevelsMut<'a> {
    stack: Vec<Item>,
    pub ui: &'a mut UserInterface,
}

impl<'a> Iterator for LevelsMut<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(op) = self.stack.pop() {
            if let Item::Push(key) = op {
                let node = &mut self.ui.nodes[key];
                let children = node.element.as_element_mut().children();
                self.stack.push(Item::Pop);
                self.stack
                    .extend(children.iter().flatten().copied().map(Item::Push));
            }
            Some(op)
        } else {
            None
        }
    }
}
