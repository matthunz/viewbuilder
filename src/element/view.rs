use crate::layout::FlexDirection;
use super::Element;

pub struct View {
    pub(crate) flex_direction: FlexDirection,
}

impl Element for View {}
