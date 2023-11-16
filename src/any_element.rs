use crate::Element;
use std::any::Any;

pub trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_element(&self) -> &dyn Element;

    fn as_element_mut(&mut self) -> &mut dyn Element;
}

impl<T> AnyElement for T
where
    T: Element + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self
    }
}
