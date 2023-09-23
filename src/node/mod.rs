use slotmap::DefaultKey;
use std::borrow::Cow;

pub mod element;
pub use element::Element;

#[derive(Debug, PartialEq, Eq)]
pub enum NodeKind {
    Container,
    Text,
}

pub enum NodeData {
    Element(Element),
    Text(Cow<'static, str>),
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

impl From<&'static str> for Node {
    fn from(value: &'static str) -> Self {
        Self::text(value)
    }
}
