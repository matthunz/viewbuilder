use slotmap::DefaultKey;
use std::borrow::Cow;

pub mod tree;
pub use tree::Tree;

#[derive(Debug)]
pub enum ElementKind {
    Canvas,
    Container,
    Text,
}

pub enum ElementData {
    Canvas,
    Container,
    Text(Cow<'static, str>),
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

    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ElementData::Text(content.into()))
    }

    pub fn kind(&self) -> ElementKind {
        match self.data {
            ElementData::Canvas => ElementKind::Canvas,
            ElementData::Container => ElementKind::Container,
            ElementData::Text(_) => ElementKind::Text,
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

fn main() {
    let mut tree = Tree::default();

    let a = tree.insert(Element::text("Hello World!"));

    let mut root = Element::new(ElementData::Container);
    root.child(a);

    let root_key = tree.insert(root);

    dbg!(tree.display(root_key));
}
