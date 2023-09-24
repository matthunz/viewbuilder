use crate::{event, node::NodeData, Node, Tree};
use skia_safe::Color4f;
use slotmap::DefaultKey;
use taffy::{
    prelude::Size,
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};

/// Element of a user interface.
#[derive(Default)]
pub struct Element {
    size: Option<Size<Dimension>>,
    flex_direction: Option<FlexDirection>,
    align_items: Option<AlignItems>,
    justify_content: Option<JustifyContent>,
    on_click: Option<Box<dyn FnMut(&mut Tree, event::Click)>>,
    on_mouse_in: Option<Box<dyn FnMut(&mut Tree, event::MouseIn)>>,
    on_mouse_out: Option<Box<dyn FnMut(&mut Tree, event::MouseOut)>>,
    background_color: Option<Color4f>,
    children: Option<Vec<DefaultKey>>,
}

impl Element {
    /// Create a new element.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a child to the element.
    pub fn child(&mut self, key: DefaultKey) -> &mut Self {
        if let Some(ref mut children) = self.children {
            children.push(key);
        } else {
            self.children = Some(vec![key])
        }
        self
    }

    /// Set the click handler for this element.
    pub fn on_click(&mut self, handler: Box<dyn FnMut(&mut Tree, event::Click)>) -> &mut Self {
        self.on_click = Some(handler);
        self
    }

    /// Set the mouse-in handler for this element.
    pub fn on_mouse_in(&mut self, handler: Box<dyn FnMut(&mut Tree, event::MouseIn)>) -> &mut Self {
        self.on_mouse_in = Some(handler);
        self
    }

    /// Set the mouse-out handler for this element.
    pub fn on_mouse_out(
        &mut self,
        handler: Box<dyn FnMut(&mut Tree, event::MouseOut)>,
    ) -> &mut Self {
        self.on_mouse_out = Some(handler);
        self
    }

    /// Set the size of this element.
    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.size = Some(size);
        self
    }

    /// Set the flex direction of this element.
    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Self {
        self.flex_direction = Some(flex_direction);
        self
    }

    /// Set the item alignment of this element.
    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self.align_items = Some(align_items);
        self
    }

    /// Set the content justification of this element.
    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
        self.justify_content = Some(justify_content);
        self
    }

    /// Set the background color of this element.
    pub fn background_color(&mut self, color: Color4f) -> &mut Self {
        self.background_color = Some(color);
        self
    }

    /// Build the element and insert it into the tree.
    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        let mut elem = Node::new(NodeData::Element(ElementData {
            size: self.size.take(),
            flex_direction: self.flex_direction.take(),
            on_click: self.on_click.take(),
            on_mouse_in: self.on_mouse_in.take(),
            on_mouse_out: self.on_mouse_out.take(),
            background_color: self.background_color.take(),
            align_items: self.align_items.take(),
            justify_content: self.justify_content.take(),
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

/// Data of an element.
#[derive(Default)]
pub struct ElementData {
    pub(crate) size: Option<Size<Dimension>>,
    pub(crate) on_click: Option<Box<dyn FnMut(&mut Tree, event::Click)>>,
    pub(crate) on_mouse_in: Option<Box<dyn FnMut(&mut Tree, event::MouseIn)>>,
    pub(crate) on_mouse_out: Option<Box<dyn FnMut(&mut Tree, event::MouseOut)>>,
    pub(crate) background_color: Option<Color4f>,
    pub(crate) flex_direction: Option<FlexDirection>,
    pub(crate) align_items: Option<AlignItems>,
    pub(crate) justify_content: Option<JustifyContent>,
}
