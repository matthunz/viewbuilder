use std::borrow::Cow;

use slotmap::DefaultKey;

use crate::{Click, Tree};

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

pub struct Builder {
    element: Option<Element>,
}

impl Builder {
    pub fn child(&mut self, key: DefaultKey) -> &mut Self {
        let elem = self.element.as_mut().unwrap();
        if let Some(ref mut children) = elem.children {
            children.push(key);
        } else {
            elem.children = Some(vec![key])
        }
        self
    }

    pub fn on_click(&mut self, handler: Box<dyn FnMut(Click)>) -> &mut Self {
        self.element.as_mut().unwrap().on_click = Some(handler);
        self
    }

    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        tree.insert(self.element.take().unwrap())
    }
}

pub struct Element {
    pub data: ElementData,
    pub children: Option<Vec<DefaultKey>>,
    on_click: Option<Box<dyn FnMut(Click)>>,
}

impl Element {
    pub fn new(data: ElementData) -> Self {
        Self {
            data,
            children: None,
            on_click: None,
        }
    }

    pub fn builder() -> Builder {
        Builder {
            element: Some(Self::new(ElementData::Container)),
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
