use std::{collections::VecDeque, fmt};

use slotmap::{DefaultKey, SlotMap};

#[derive(Debug)]
pub enum ElementKind {
    Canvas,
    Container,
    Text,
}

pub enum ElementData {
    Canvas,
    Container,
    Text,
}

pub struct Element {
    data: ElementData,
    children: Option<Vec<DefaultKey>>,
}

impl Element {
    pub fn new(data: ElementData) -> Self {
        Self {
            data,
            children: None,
        }
    }

    pub fn kind(&self) -> ElementKind {
        match self.data {
            ElementData::Canvas => ElementKind::Canvas,
            ElementData::Container => ElementKind::Container,
            ElementData::Text => ElementKind::Text,
        }
    }

    pub fn child(&mut self, child: DefaultKey) {
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child])
        }
    }
}

#[derive(Default)]
pub struct Tree {
    elements: SlotMap<DefaultKey, Element>,
}

enum Item {
    Key(DefaultKey),
    Pop,
}

impl Tree {
    pub fn display(&self, root: DefaultKey) {
        let mut stack = vec![Item::Key(root)];
        let mut count = 0;

        while let Some(item) = stack.pop() {
            match item {
                Item::Key(key) => {
                    let elem = &self.elements[key];

                    let mut indent = String::new();
                    for _ in 0..count {
                        indent.push_str("  ")
                    }
                    println!("{indent}{:?}", elem.kind());

                    stack.push(Item::Pop);
                    for child in elem.children.iter().flatten().copied().map(Item::Key) {
                        stack.push(child);
                    }

                    count += 1;
                }

                Item::Pop => count -= 1,
            }
        }
    }
}

fn main() {
    let mut tree = Tree::default();

    let a = tree.elements.insert(Element::new(ElementData::Text));
    let b = tree.elements.insert(Element::new(ElementData::Text));
    let c = tree.elements.insert(Element::new(ElementData::Text));

    let mut d = Element::new(ElementData::Container);
    d.child(b);
    d.child(c);
    let d_key = tree.elements.insert(d);

    let mut root = Element::new(ElementData::Container);
    root.child(a);
    root.child(d_key);
    let root_key = tree.elements.insert(root);

    dbg!(tree.display(root_key));
}
