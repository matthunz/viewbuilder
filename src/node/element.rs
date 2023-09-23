use super::NodeData;
use crate::{Click, Node, Tree};
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Dimension};

#[derive(Default)]
pub struct Builder {
    size: Option<Size<Dimension>>,
    on_click: Option<Box<dyn FnMut(&mut Tree, Click)>>,
    pub children: Option<Vec<DefaultKey>>,
}

impl Builder {
    pub fn child(&mut self, key: DefaultKey) -> &mut Self {
        if let Some(ref mut children) = self.children {
            children.push(key);
        } else {
            self.children = Some(vec![key])
        }
        self
    }

    pub fn on_click(&mut self, handler: Box<dyn FnMut(&mut Tree, Click)>) -> &mut Self {
        self.on_click = Some(handler);
        self
    }

    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.size = Some(size);
        self
    }

    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        let mut elem = Node::new(NodeData::Element(Element {
            size: self.size.take(),
            on_click: self.on_click.take(),
        }));
        elem.children = self.children.take();

        tree.insert(elem)
    }
}

pub struct Element {
    pub size: Option<Size<Dimension>>,
    pub on_click: Option<Box<dyn FnMut(&mut Tree, Click)>>,
}

impl Element {
    pub fn builder() -> Builder {
        Builder::default()
    }
}
