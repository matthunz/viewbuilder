use super::Element;
use crate::virtual_tree::FlexDirection;

pub struct View {
    pub(crate) flex_direction: FlexDirection,
}

impl Element for View {}
