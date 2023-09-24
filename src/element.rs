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
    data: Option<ElementData>,
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
        self.data_mut().on_click = Some(handler);
        self
    }

    /// Set the mouse-in handler for this element.
    pub fn on_mouse_in(&mut self, handler: Box<dyn FnMut(&mut Tree, event::MouseIn)>) -> &mut Self {
        self.data_mut().on_mouse_in = Some(handler);
        self
    }

    /// Set the mouse-out handler for this element.
    pub fn on_mouse_out(
        &mut self,
        handler: Box<dyn FnMut(&mut Tree, event::MouseOut)>,
    ) -> &mut Self {
        self.data_mut().on_mouse_out = Some(handler);
        self
    }

    /// Set the size of this element.
    pub fn size(&mut self, size: Size<Dimension>) -> &mut Self {
        self.data_mut().size = Some(size);
        self
    }

    /// Set the flex direction of this element.
    pub fn flex_direction(&mut self, flex_direction: FlexDirection) -> &mut Self {
        self.data_mut().flex_direction = Some(flex_direction);
        self
    }

    /// Set the item alignment of this element.
    pub fn align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self.data_mut().align_items = Some(align_items);
        self
    }

    /// Set the content justification of this element.
    pub fn justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
        self.data_mut().justify_content = Some(justify_content);
        self
    }

    /// Set the background color of this element.
    pub fn background_color(&mut self, color: Color4f) -> &mut Self {
        self.data_mut().background_color = Some(color);
        self
    }

    pub fn data_mut(&mut self) -> &mut ElementData {
        self.data.as_mut().unwrap()
    }

    /// Build the element and insert it into the tree.
    pub fn build(&mut self, tree: &mut Tree) -> DefaultKey {
        let mut elem = Node::new(NodeData::Element(self.data.take().unwrap()));
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
