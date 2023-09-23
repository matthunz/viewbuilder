use std::borrow::Cow;

use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Dimension};

use crate::{Click, Tree};

#[derive(Debug, PartialEq, Eq)]
pub enum ElementKind {
    Container,
    Text,
}

pub enum ElementData {
    Container { size: Option<Size<Dimension>> },
    Text(Cow<'static, str>),
}

#[derive(Default)]
pub struct ContainerBuilder {
    size: Option<Size<Dimension>>,
    on_click: Option<Box<dyn FnMut(Click)>>,
    pub children: Option<Vec<DefaultKey>>,
}

impl ContainerBuilder {
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
        let mut elem = Element::new(ElementData::Container {
            size: self.size.take(),
        });
        elem.children = self.children.take();

        tree.insert(elem)
    }
}

pub struct Element {
    pub data: ElementData,
    pub children: Option<Vec<DefaultKey>>,
}

impl Element {
    pub fn new(data: ElementData) -> Self {
        Self {
            data,
            children: None,
        }
    }

    pub fn builder() -> ContainerBuilder {
        ContainerBuilder::default()
    }

    pub fn text(content: impl Into<Cow<'static, str>>) -> Self {
        Self::new(ElementData::Text(content.into()))
    }

    pub fn kind(&self) -> ElementKind {
        match self.data {
            ElementData::Container { .. } => ElementKind::Container,
            ElementData::Text(_) => ElementKind::Text,
        }
    }
}
