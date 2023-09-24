use crate::{event, node::NodeData, Context, Node, NodeKey};
use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};

mod data;
pub use self::data::ElementData;

mod attribute;
pub use self::attribute::{Attribute, AttributeKind, AttributeValue};

/// Element of a user interface.
pub struct Element {
    data: Option<ElementData>,
    children: Option<Vec<NodeKey>>,
}

impl Default for Element {
    fn default() -> Self {
        Self {
            data: Some(ElementData::default()),
            children: Default::default(),
        }
    }
}

macro_rules! make_builder_fn {
    ($name:literal, $fn_name:ident, $set_fn_name:ident, $ty:path) => {
        #[doc = concat!("Set the ", $name, " of this element.")]
        pub fn $fn_name(&mut self, $fn_name: $ty) -> &mut Self {
            self.data_mut().$set_fn_name($fn_name);
            self
        }
    };
}

impl Element {
    /// Create a new element.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a child to the element.
    pub fn child(&mut self, key: NodeKey) -> &mut Self {
        if let Some(ref mut children) = self.children {
            children.push(key);
        } else {
            self.children = Some(vec![key])
        }
        self
    }

    make_builder_fn!("size", size, set_size, Size<Dimension>);
    make_builder_fn!(
        "flex direction",
        flex_direction,
        set_flex_direction,
        FlexDirection
    );
    make_builder_fn!("item alignment", align_items, set_align_items, AlignItems);
    make_builder_fn!(
        "content justification",
        justify_content,
        set_justify_content,
        JustifyContent
    );

    make_builder_fn!(
        "click handler",
        on_click,
        set_on_click,
        Box<dyn FnMut(&mut Context, event::Click)>
    );

    make_builder_fn!(
        "background color",
        background_color,
        set_background_color,
        Color4f
    );

    pub fn data_mut(&mut self) -> &mut ElementData {
        self.data.as_mut().unwrap()
    }

    /// Build the element and insert it into the tree.
    pub fn build(&mut self, cx: &mut Context) -> NodeKey {
        let mut elem = Node::new(NodeData::Element(self.data.take().unwrap()));
        elem.children = self.children.take();

        let key = cx.insert(elem);
        for child in self.children.iter().flatten() {
            let node = &mut cx.tree[*child];
            node.parent = Some(key);
        }
        key
    }
}
