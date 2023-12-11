use std::{any::Any, marker::PhantomData};

use kurbo::Size;
use slotmap::DefaultKey;

use crate::{Element, Handle, UserInterface};

pub trait AnyElement {
    fn update_any(&mut self, key: DefaultKey, ui: UserInterface, msg: Box<dyn Any>);

    fn layout_any(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size;
}

impl<E: Element> AnyElement for E
where
    E::Message: 'static,
{
    fn update_any(&mut self, key: DefaultKey, ui: UserInterface, msg: Box<dyn Any>) {
        let cx = Handle {
            key,
            ui,
            _marker: PhantomData,
        };
        self.update(cx, *msg.downcast().unwrap())
    }

    fn layout_any(&mut self, min_size: Option<Size>, max_size: Option<Size>) -> Size {
        self.layout(min_size, max_size)
    }
}
