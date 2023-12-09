use slotmap::DefaultKey;
use std::any::Any;

mod linear_layout;
pub use self::linear_layout::{LinearLayout, LinearLayoutBuilder};

mod text;
pub use self::text::Text;

pub trait Element {
    fn children(&self) -> Option<Box<[DefaultKey]>>;
}

pub trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_element(&self) -> &dyn Element;
}

impl<E: Element + 'static> AnyElement for E {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> &dyn Element {
        self
    }
}
