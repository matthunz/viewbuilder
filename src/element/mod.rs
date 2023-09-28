use crate::{
    event,
    node::{NodeData, Overflow},
    Context, Node, NodeKey,
};
use skia_safe::Color4f;
use std::marker::PhantomData;
use taffy::{
    prelude::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent, LengthPercentage},
};

mod data;
pub use self::data::ElementData;

mod attribute;
pub use self::attribute::{Attribute, AttributeKind, AttributeValue};

/// Element of a user interface.
pub struct Element<T> {
    data: Option<ElementData<T>>,
    children: Option<Vec<NodeKey>>,
    _marker: PhantomData<T>,
}

impl<T> Default for Element<T> {
    fn default() -> Self {
        Self {
            data: Some(ElementData::default()),
            children: Default::default(),

            _marker: PhantomData,
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

impl<T> Element<T> {
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
    make_builder_fn!("padding", padding, set_padding, Rect<LengthPercentage>);

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
        Box<dyn FnMut(&mut Context<T>, event::MouseEvent)>
    );

    make_builder_fn!(
        "background color",
        background_color,
        set_background_color,
        Color4f
    );

    make_builder_fn!("x overflow", overflow_x, set_overflow_x, Overflow);

    make_builder_fn!("y overflow", overflow_y, set_overflow_y, Overflow);

    pub fn data_mut(&mut self) -> &mut ElementData<T> {
        self.data.as_mut().unwrap()
    }

    /// Build the element and insert it into the tree.
    pub fn build(&mut self, cx: &mut Context<T>) -> NodeKey {
        let mut node = Node::new(NodeData::Element(self.data.take().unwrap()));
        node.children = self.children.take();

        let key = cx.insert(node);
        for child_key in self.children.iter().flatten() {
            let child = &mut cx.tree[*child_key];
            child.parent = Some(key);
        }
        key
    }
}

impl<T> Extend<NodeKey> for Element<T> {
    fn extend<I: IntoIterator<Item = NodeKey>>(&mut self, iter: I) {
        for key in iter {
            self.child(key);
        }
    }
}
