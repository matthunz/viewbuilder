use crate::{event, node::Overflow, Context};
use skia_safe::Color4f;
use taffy::{
    prelude::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent, LengthPercentage},
};

type Handler<T> = Box<dyn FnMut(&mut Context<T>, event::MouseEvent)>;

#[derive(Clone, Copy, PartialEq, Eq)]
/// Element attribute kind.
pub enum AttributeKind {
    /// Size attribute.
    Size,

    /// Padding attribute.
    Padding,

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

    OverflowX,
    OverflowY,
}

/// Element attribute value.
pub enum AttributeValue<T> {
    /// Size attribute value.
    Size(Size<Dimension>),

    Rect(Rect<LengthPercentage>),

    /// Flex direction attribute value.
    FlexDirection(FlexDirection),

    /// Item alignment attribute value.
    AlignItems(AlignItems),

    /// Content justification attribute value.
    JustifyContent(JustifyContent),

    /// Click handler attribute value.
    OnClick(Handler<T>),

    /// Mouse in handler attribute value.
    OnMouseIn(Handler<T>),

    // Mouse out handler attribute value.
    OnMouseOut(Handler<T>),

    /// Color attribute value.
    Color(Color4f),
    Overflow(Overflow),
}

/// Element attribute.
pub struct Attribute<T> {
    /// Attribute kind.
    pub(super) kind: AttributeKind,

    /// Attribute value.
    pub(super) value: AttributeValue<T>,
}

impl<T> Attribute<T> {
    /// Get the kind of this attribute.
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    /// Get the value of this attribute.
    pub fn value(&self) -> &AttributeValue<T> {
        &self.value
    }
}
