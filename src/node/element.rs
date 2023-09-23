use super::NodeData;
use crate::{Click, Node, Tree};
use skia_safe::Color4f;
use slotmap::DefaultKey;
use taffy::{
    prelude::Size,
    style::{Dimension, FlexDirection},
};

#[derive(Default)]
pub struct Builder {
    size: Option<Size<Dimension>>,
    flex_direction: Option<FlexDirection>,
    on_click: Option<Box<dyn FnMut(&mut Tree, Click)>>,
    background_color: Option<Color4f>,
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

    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Self {
        self.flex_direction = Some(flex_direction);
        self
    }

    pub fn background_color(&mut self, color: Color4f) -> &mut Self {
        self.background_color = Some(color);
        self
    }

    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        let mut elem = Node::new(NodeData::Element(Element {
            size: self.size.take(),
            flex_direction: self.flex_direction.take(),
            on_click: self.on_click.take(),
            background_color: self.background_color.take(),
        }));
        elem.children = self.children.take();

        let key = tree.insert(elem);
        for child in self.children.iter().flatten() {
            let node = &mut tree.nodes.nodes[*child];
            node.parent = Some(key);
        }

        key
    }
}

pub struct Element {
    pub size: Option<Size<Dimension>>,
    pub on_click: Option<Box<dyn FnMut(&mut Tree, Click)>>,
    pub background_color: Option<Color4f>,
    pub flex_direction: Option<FlexDirection>,
}

impl Element {
    pub fn builder() -> Builder {
        Builder::default()
    }
}
