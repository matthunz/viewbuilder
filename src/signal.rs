use crate::{Handle, Object, Runtime, Slot};
use slotmap::DefaultKey;
use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

pub struct Signal<T> {
    key: DefaultKey,
    id: u32,
    _marker: PhantomData<T>,
}

impl<T> Signal<T> {
    pub fn new(key: DefaultKey, id: u32) -> Self {
        Self {
            key,
            id,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub fn bind<O>(&self, handle: &Handle<O>, slot: impl Slot<O, T> + 'static)
    where
        O: Object + 'static,
    {
        let f = Rc::new(RefCell::new(slot));
        let handle = handle.clone();
        Runtime::current().inner.borrow_mut().nodes[self.key]
            .listeners
            .push((
                self.id,
                Rc::new(RefCell::new(move |any: &dyn Any| {
                    let data = any.downcast_ref::<T>().unwrap().clone();
                    let f = f.clone();
                    handle.update(move |object| f.borrow_mut().handle(object, data.clone()))
                })),
            ));
    }
}
