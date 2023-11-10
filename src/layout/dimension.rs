use crate::virtual_tree::DynAttribute;
use dioxus::{
    core::{
        exports::bumpalo::{boxed::Box as BumpBox, Bump},
        AnyValue, AttributeValue,
    },
    prelude::IntoAttributeValue,
};
use std::{cell::RefCell, sync::Arc};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Dimension {
    #[default]
    Auto,
    Px(f64),
    Dp(f64),
    Percent(f64),
}

impl Dimension {
    pub fn into_taffy(self, scale: f64) -> taffy::prelude::Dimension {
        match self {
            Dimension::Auto => taffy::prelude::Dimension::Auto,
            Dimension::Px(px) => taffy::prelude::Dimension::Points(px as _),
            Dimension::Dp(dp) => taffy::prelude::Dimension::Points((dp * scale) as _),
            Dimension::Percent(percent) => taffy::prelude::Dimension::Percent(percent as _),
        }
    }
}

impl<'a> IntoAttributeValue<'a> for Dimension {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        let value = DynAttribute(Some(Arc::new(self)));
        let boxed: BumpBox<'a, dyn AnyValue> = unsafe { BumpBox::from_raw(bump.alloc(value)) };
        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}

pub trait IntoDimension: Sized {
    fn dp(self) -> Dimension;

    fn px(self) -> Dimension;

    fn percent(self) -> Dimension;
}

impl IntoDimension for f64 {
    fn dp(self) -> Dimension {
        Dimension::Dp(self)
    }

    fn px(self) -> Dimension {
        Dimension::Px(self)
    }

    fn percent(self) -> Dimension {
        Dimension::Percent(self)
    }
}

impl IntoDimension for i32 {
    fn dp(self) -> Dimension {
        Dimension::Dp(self as _)
    }

    fn px(self) -> Dimension {
        Dimension::Px(self as _)
    }

    fn percent(self) -> Dimension {
        Dimension::Percent(self as _)
    }
}
