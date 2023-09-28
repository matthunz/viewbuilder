use std::marker::PhantomData;

use super::{Attribute, AttributeKind, AttributeValue};
use crate::{event, Context};
use skia_safe::Color4f;
use taffy::{
    prelude::{Rect, Size},
    style::{AlignItems, Dimension, FlexDirection, JustifyContent, LengthPercentage},
};

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
        pub fn $fn_name(&mut self) -> Option<Box<dyn FnMut(&mut Context<T>, event::$ty)>> {
            if let Some(attr) = self.remove(AttributeKind::$kind_ty) {
                match attr.value {
                    AttributeValue::$kind_ty(f) => Some(f),
                    _ => todo!(),
                }
            } else {
                None
            }
        }

        pub fn $set_fn_name(&mut self, handler: Box<dyn FnMut(&mut Context<T>, event::$ty)>) {
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
pub struct ElementData<T> {
    attributes: Vec<Attribute<T>>,
    _marker: PhantomData<T>,
}

impl<T> Default for ElementData<T> {
    fn default() -> Self {
        Self {
            attributes: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<T> ElementData<T> {
    /// Get a reference to the attribute of this kind if present.
    pub fn attr(&self, kind: AttributeKind) -> Option<&Attribute<T>> {
        self.attributes.iter().find(|attr| attr.kind() == kind)
    }

    /// Get a mutable reference to the attribute of this kind if present.
    pub fn attr_mut(&mut self, kind: AttributeKind) -> Option<&mut Attribute<T>> {
        self.attributes.iter_mut().find(|attr| attr.kind() == kind)
    }

    /// Remove an attribute by kind from this element.
    pub fn remove(&mut self, kind: AttributeKind) -> Option<Attribute<T>> {
        self.attributes
            .iter()
            .position(|attr| attr.kind() == kind)
            .map(|idx| self.attributes.remove(idx))
    }

    make_style_fn!(size, set_size, Size<Dimension>, Size, Size);
    make_style_fn!(padding, set_padding, Rect<LengthPercentage>, Padding, Rect);

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

    make_handler_fn!(on_click, set_on_click, MouseEvent, OnClick);
    make_handler_fn!(on_mouse_in, set_on_mouse_in, MouseEvent, OnMouseIn);
    make_handler_fn!(on_mouse_out, set_on_mouse_out, MouseEvent, OnMouseOut);
}
