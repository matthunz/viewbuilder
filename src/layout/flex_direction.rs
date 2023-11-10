use dioxus::{prelude::IntoAttributeValue, core::{AttributeValue, exports::bumpalo::Bump}};
use shipyard::Component;

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
}

impl TryFrom<u8> for FlexDirection {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Row),
            1 => Ok(Self::Column),
            _ => Err(()),
        }
    }
}

impl<'a> IntoAttributeValue<'a> for FlexDirection {
    fn into_value(self, bump: &'a Bump) -> AttributeValue<'a> {
        AttributeValue::Int(self as u8 as _)
    }
}