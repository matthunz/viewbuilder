use crate::{event, Context};
use skia_safe::Color4f;
use taffy::{
    prelude::Size,
    style::{AlignItems, Dimension, FlexDirection, JustifyContent},
};

#[derive(Clone, Copy, PartialEq, Eq)]
/// Element attribute kind.
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

/// Element attribute value.
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

/// Element attribute.
pub struct Attribute {
    pub(super) kind: AttributeKind,
    pub(super) value: AttributeValue,
}

impl Attribute {
    pub fn kind(&self) -> AttributeKind {
        self.kind
    }

    pub fn value(&self) -> &AttributeValue {
        &self.value
    }
}
