use slotmap::DefaultKey;
use std::{any::Any, cell::RefCell, marker::PhantomData, rc::Rc};

use crate::{Handle, Object, Runtime};

pub struct Signal<T> {
    key: DefaultKey,
    _marker: PhantomData<T>,
}

impl<T> Signal<T> {
    pub fn new(key: DefaultKey) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub fn bind<O>(&self, handle: &Handle<O>, slot: impl FnMut(&mut O, T) + 'static)
    where
        O: Object + 'static,
    {
        let f = Rc::new(RefCell::new(slot));
        let handle = handle.clone();
        Runtime::current().inner.borrow_mut().nodes[self.key]
            .listeners
            .push(Rc::new(RefCell::new(move |any: &dyn Any| {
                let data = any.downcast_ref::<T>().unwrap().clone();
                let f = f.clone();
                handle.update(move |object| f.borrow_mut()(object, data.clone()))
            })));
    }
}
