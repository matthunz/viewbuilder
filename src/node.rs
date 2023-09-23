use std::borrow::Cow;
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Dimension};
use crate::{Click, Tree};

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Container,
    Text,
}

pub enum NodeData {
    Element { size: Option<Size<Dimension>> },
    Text(Cow<'static, str>),
}

#[derive(Default)]
pub struct ElementBuilder {
    size: Option<Size<Dimension>>,
    on_click: Option<Box<dyn FnMut(Click)>>,
    pub children: Option<Vec<DefaultKey>>,
}

impl ElementBuilder {
    pub fn child(&mut self, key: DefaultKey) -> &mut Self {
        if let Some(ref mut children) = self.children {
            children.push(key);
        } else {
            self.children = Some(vec![key])
        }
        self
    }

    pub fn on_click(&mut self, handler: Box<dyn FnMut(Click)>) -> &mut Self {
        self.on_click = Some(handler);
        self
    }

    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        let mut elem = Node::new(NodeData::Element {
            size: self.size.take(),
        });
        elem.children = self.children.take();

        tree.insert(elem)
    }
}

pub struct Node {
    pub data: NodeData,
    pub children: Option<Vec<DefaultKey>>,
}

impl Node {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            children: None,
        }
    }

    pub fn builder() -> ElementBuilder {
        ElementBuilder::default()
    }

    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(NodeData::Text(content.into()))
    }

    pub fn kind(&self) -> NodeKind {
        match self.data {
            NodeData::Element { .. } => NodeKind::Container,
            NodeData::Text(_) => NodeKind::Text,
        }
    }
}
