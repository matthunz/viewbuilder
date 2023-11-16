use crate::{any_element::AnyElement, element::View, Element, ElementRef};
use kurbo::Point;
use skia_safe::Canvas;
use slotmap::{DefaultKey, SparseSecondaryMap};
use std::{
    marker::PhantomData,
    ops::{Index, IndexMut},
};
use taffy::{
    prelude::{Layout, Size},
    style::Style,
    Taffy,
};

pub struct Node {
    pub element: Box<dyn AnyElement>,
    pub layout: Layout,
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

    /// Compute the layout of the tree.
    pub fn layout(&mut self) {
        self.taffy
            .compute_layout(self.root, Size::max_content())
            .unwrap();

        let mut parents: Vec<Layout> = Vec::new();
        let mut levels = self.levels_mut();
        while let Some(item) = levels.next() {
            match item {
                Item::Push(key) => {
                    let mut layout = levels.ui.taffy.layout(key).unwrap().clone();
                    if let Some(parent_layout) = parents.last() {
                        layout.location.x += parent_layout.location.x;
                        layout.location.x += parent_layout.location.x;
                    }
                    levels.ui.nodes[key].layout = layout;
                    parents.push(layout);
                }
                Item::Pop => {
                    parents.pop();
                }
            }
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        for node in self.nodes.values_mut() {
            if let Some(image) = node.element.as_element_mut().render(node.layout.size) {
                canvas.draw_image(
                    image,
                    (
                        node.layout.location.x.floor(),
                        node.layout.location.y.floor(),
                    ),
                    None,
                );
            }
        }
    }

    pub fn target(&self, point: Point) -> Option<DefaultKey> {
        self.nodes
            .iter()
            .filter(move |(key, node)| {
                point.x >= node.layout.location.x as _
                    && point.x <= (node.layout.location.x + node.layout.size.width) as _
                    && point.y >= node.layout.location.y as _
                    && point.y <= (node.layout.location.y + node.layout.size.height) as _
            })
            .max_by_key(|(key, node)| node.layout.order)
            .map(|(key, node)| key)
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
