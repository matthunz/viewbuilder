use crate::{Element, UserInterface};
use slotmap::DefaultKey;
use std::marker::PhantomData;

pub struct Handle<E: ?Sized> {
    pub(crate) key: DefaultKey,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Clone for Handle<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for Handle<E> {}

impl<E> Handle<E> {
    pub fn send(&self, msg: E::Message)
    where
        E: Element,
        E::Message: 'static,
    {
        UserInterface::current()
            .inner
            .borrow_mut()
            .queue
            .push((self.key, Box::new(msg)));
    }

    pub fn layout(&self) {
        UserInterface::current()
            .inner
            .borrow_mut()
            .pending_layouts
            .insert(self.key);
    }
}
