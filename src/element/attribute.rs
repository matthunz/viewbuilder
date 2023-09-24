use crate::{event, Context};
use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};

#[derive(Clone, Copy, PartialEq, Eq)]
/// Element attribute kind.
pub enum AttributeKind {
    /// Size attribute.
    Size,

    /// Flex direction attribute.
    FlexDirection,

    /// Item alignment attribute.
    AlignItems,

    /// Content justification attribute.
    JustifyContent,

    /// Click handler attribute.
    OnClick,

    /// Mouse in handler attribute.
    OnMouseIn,

    /// Mouse out handler attribute.
    OnMouseOut,

    /// Background color attribute.
    BackgroundColor,
}

/// Element attribute value.
pub enum AttributeValue {
    /// Size attribute value.
    Size(Size<Dimension>),

    /// Flex direction attribute value.
    FlexDirection(FlexDirection),

    /// Item alignment attribute value.
    AlignItems(AlignItems),

    /// Content justification attribute value.
    JustifyContent(JustifyContent),

    /// Click handler attribute value.
    OnClick(Box<dyn FnMut(&mut Context, event::Click)>),

    /// Mouse in handler attribute value.
    OnMouseIn(Box<dyn FnMut(&mut Context, event::MouseIn)>),

    // Mouse out handler attribute value.
    OnMouseOut(Box<dyn FnMut(&mut Context, event::MouseOut)>),

    /// Color attribute value.
    Color(Color4f),
}

/// Element attribute.
pub struct Attribute {
    /// Attribute kind.
    pub(super) kind: AttributeKind,

    /// Attribute value.
    pub(super) value: AttributeValue,
}

impl Attribute {
    /// Get the kind of this attribute.
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    /// Get the value of this attribute.
    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}
