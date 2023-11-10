use std::{cell::RefCell, sync::Arc};

use dioxus::{
    core::{exports::bumpalo::boxed::Box as BumpBox, AnyValue, AttributeValue},
    prelude::IntoAttributeValue,
};

use crate::virtual_tree::DynAttribute;

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
    pub alpha: f64,
}

impl Color {
    pub fn from_rgb(red: u8, _blue: u8, _green: u8) -> Self {
        Self {
            red: red as f64 / 255.,
            blue: red as f64 / 255.,
            green: red as f64 / 255.,
            alpha: 1.,
        }
    }
}

impl<'a> IntoAttributeValue<'a> for Color {
    fn into_value(
        self,
        bump: &'a dioxus::core::exports::bumpalo::Bump,
    ) -> dioxus::core::AttributeValue<'a> {
        let value = DynAttribute(Some(Arc::new(self)));
        let boxed: BumpBox<'a, dyn AnyValue> = unsafe { BumpBox::from_raw(bump.alloc(value)) };
        AttributeValue::Any(RefCell::new(Some(boxed)))
    }
}
