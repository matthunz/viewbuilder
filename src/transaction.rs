use std::marker::PhantomData;

use slotmap::{DefaultKey, SlotMap};

use crate::{any_element::AnyElement, Element, ElementRef};

#[derive(Default)]
pub struct Transaction {
    pub(crate) elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
}

impl Transaction {
    pub fn insert<T>(&mut self, element: T) -> ElementRef<T>
    where
        T: Element + 'static,
    {
        let key = self.elements.insert(Box::new(element));
        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}
