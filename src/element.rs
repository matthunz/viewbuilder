use crate::{event, node::NodeData, Context, Node, NodeKey};
use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};

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
    ($fn_name:ident, $set_fn_name:ident, $ty:path) => {
        /// Set the size of this element.
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

    make_builder_fn!(size, set_size, Size<Dimension>);
    make_builder_fn!(flex_direction, set_flex_direction, FlexDirection);
    make_builder_fn!(align_items, set_align_items, AlignItems);
    make_builder_fn!(justify_content, set_justify_content, JustifyContent);

    make_builder_fn!(
        on_click,
        set_on_click,
        Box<dyn FnMut(&mut Context, event::Click)>
    );

    make_builder_fn!(background_color, set_background_color, Color4f);

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

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttributeKind {
    Size,
    FlexDirection,
    AlignItems,
    JustifyContent,
    OnClick,
    OnMouseIn,
    OnMouseOut,
    BackgroundColor,
}

pub enum AttributeValue {
    Size(Size<Dimension>),
    FlexDirection(FlexDirection),
    AlignItems(AlignItems),
    JustifyContent(JustifyContent),
    OnClick(Box<dyn FnMut(&mut Context, event::Click)>),
    OnMouseIn(Box<dyn FnMut(&mut Context, event::MouseIn)>),
    OnMouseOut(Box<dyn FnMut(&mut Context, event::MouseOut)>),
    Color(Color4f),
}

pub struct Attribute {
    kind: AttributeKind,
    value: AttributeValue,
}

impl Attribute {
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}

macro_rules! make_style_fn {
    ($fn_name:ident, $set_fn_name:ident, $ty:path, $kind_ty:ident, $value_ty:ident) => {
        pub fn $fn_name(&self) -> Option<$ty> {
            self.attr(AttributeKind::$kind_ty)
                .map(|attr| match attr.value {
                    AttributeValue::$value_ty(size) => size,
                    _ => todo!(),
                })
        }

        pub fn $set_fn_name(&mut self, $fn_name: $ty) {
            if let Some(attr_val) =
                self.attr_mut(AttributeKind::$kind_ty)
                    .map(|attr| match attr.value {
                        AttributeValue::$value_ty(ref mut val) => val,
                        _ => todo!(),
                    })
            {
                *attr_val = $fn_name;
            } else {
                self.attributes.push(Attribute {
                    kind: AttributeKind::$kind_ty,
                    value: AttributeValue::$value_ty($fn_name),
                })
            }
        }
    };

    ($fn_name:ident, $set_fn_name:ident, $ty:path, $kind_ty:ident) => {
        make_style_fn!($fn_name, $set_fn_name, $ty, $kind_ty, $kind_ty);
    };
}

macro_rules! make_handler_fn {
    ($fn_name:ident, $set_fn_name:ident, $ty:ident, $kind_ty:ident) => {
        pub fn $fn_name(&mut self) -> Option<Box<dyn FnMut(&mut Context, event::$ty)>> {
            if let Some(attr) = self.remove(AttributeKind::$kind_ty) {
                match attr.value {
                    AttributeValue::$kind_ty(f) => Some(f),
                    _ => todo!(),
                }
            } else {
                None
            }
        }

        pub fn $set_fn_name(&mut self, handler: Box<dyn FnMut(&mut Context, event::$ty)>) {
            if let Some(attr_val) =
                self.attr_mut(AttributeKind::$kind_ty)
                    .map(|attr| match attr.value {
                        AttributeValue::$kind_ty(ref mut val) => val,
                        _ => todo!(),
                    })
            {
                *attr_val = handler;
            } else {
                self.attributes.push(Attribute {
                    kind: AttributeKind::$kind_ty,
                    value: AttributeValue::$kind_ty(handler),
                })
            }
        }
    };
}

/// Data of an element.
#[derive(Default)]
pub struct ElementData {
    attributes: Vec<Attribute>,
}

impl ElementData {
    pub fn attr(&self, kind: AttributeKind) -> Option<&Attribute> {
        self.attributes.iter().find(|attr| attr.kind == kind)
    }

    pub fn attr_mut(&mut self, kind: AttributeKind) -> Option<&mut Attribute> {
        self.attributes.iter_mut().find(|attr| attr.kind == kind)
    }

    pub fn remove(&mut self, kind: AttributeKind) -> Option<Attribute> {
        self.attributes
            .iter()
            .position(|attr| attr.kind == kind)
            .map(|idx| self.attributes.remove(idx))
    }

    make_style_fn!(size, set_size, Size<Dimension>, Size, Size);

    make_style_fn!(
        flex_direction,
        set_flex_direction,
        FlexDirection,
        FlexDirection
    );

    make_style_fn!(align_items, set_align_items, AlignItems, AlignItems);

    make_style_fn!(
        justify_content,
        set_justify_content,
        JustifyContent,
        JustifyContent
    );

    make_style_fn!(
        background_color,
        set_background_color,
        Color4f,
        BackgroundColor,
        Color
    );

    make_handler_fn!(on_click, set_on_click, Click, OnClick);
    make_handler_fn!(on_mouse_in, set_on_mouse_in, MouseIn, OnMouseIn);
    make_handler_fn!(on_mouse_out, set_on_mouse_out, MouseOut, OnMouseOut);
}
