use super::NodeData;
use crate::{event, Node, Tree};
use skia_safe::Color4f;
use slotmap::DefaultKey;
use taffy::{
    prelude::Size,
    style::{Dimension, FlexDirection, AlignItems, JustifyContent},
};

#[derive(Default)]
pub struct Builder {
    size: Option<Size<Dimension>>,
    flex_direction: Option<FlexDirection>,
    on_click: Option<Box<dyn FnMut(&mut Tree, event::Click)>>,
    pub on_mouse_in: Option<Box<dyn FnMut(&mut Tree, event::MouseIn)>>,
    pub on_mouse_out: Option<Box<dyn FnMut(&mut Tree, event::MouseOut)>>,
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

    pub fn on_click(&mut self, handler: Box<dyn FnMut(&mut Tree, event::Click)>) -> &mut Self {
        self.on_click = Some(handler);
        self
    }

    pub fn on_mouse_in(&mut self, handler: Box<dyn FnMut(&mut Tree, event::MouseIn)>) -> &mut Self {
        self.on_mouse_in = Some(handler);
        self
    }

    pub fn on_mouse_out(
        &mut self,
        handler: Box<dyn FnMut(&mut Tree, event::MouseOut)>,
    ) -> &mut Self {
        self.on_mouse_out = Some(handler);
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

    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self
    }

    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
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
            on_mouse_in: self.on_mouse_in.take(),
            on_mouse_out: self.on_mouse_out.take(),
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
    pub on_click: Option<Box<dyn FnMut(&mut Tree, event::Click)>>,
    pub on_mouse_in: Option<Box<dyn FnMut(&mut Tree, event::MouseIn)>>,
    pub on_mouse_out: Option<Box<dyn FnMut(&mut Tree, event::MouseOut)>>,
    pub background_color: Option<Color4f>,
    pub flex_direction: Option<FlexDirection>,
}

impl Element {
    pub fn builder() -> Builder {
        Builder::default()
    }
}
