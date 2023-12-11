use crate::{Element, UserInterface};
use slotmap::DefaultKey;
use std::marker::PhantomData;

pub struct Handle<E: ?Sized> {
    pub(crate) key: DefaultKey,
    pub(crate) ui: UserInterface,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Clone for Handle<E> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            ui: self.ui.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<E> Handle<E> {
    pub fn send(&self, msg: E::Message)
    where
        E: Element,
        E::Message: 'static,
    {
        self.ui
            .inner
            .borrow_mut()
            .queue
            .push((self.key, Box::new(msg)));
    }

    pub fn layout(&self) {
        self.ui.inner.borrow_mut().pending_layouts.insert(self.key);
    }
}
