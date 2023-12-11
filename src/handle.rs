use crate::{AnyObject, Object, Runtime};
use slotmap::DefaultKey;
use std::{
    cell::{self, RefCell},
    marker::PhantomData,
    mem,
    ops::Deref,
    rc::Rc,
};

pub struct Handle<O: Object> {
    pub(crate) state: HandleState<O>,
    pub(crate) sender: O::Sender,
}

impl<O: Object> Clone for Handle<O> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<O: Object> Handle<O> {
    pub fn update(&self, f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        self.state.update(f)
    }

    pub fn borrow(&self) -> Ref<O> {
        self.state.borrow()
    }
}

impl<O: Object> Deref for Handle<O> {
    type Target = O::Sender;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

pub struct HandleState<O: Object> {
    pub key: DefaultKey,
    pub(crate) _marker: PhantomData<O>,
}

impl<O: Object> Clone for HandleState<O> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<O: Object> HandleState<O> {
    pub fn update(&self, mut f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        Runtime::current().inner.borrow_mut().updates.push((
            self.key,
            Box::new(move |element| f(element.downcast_mut().unwrap())),
        ))
    }

    pub fn borrow(&self) -> Ref<O> {
        let rc = Runtime::current().inner.borrow().nodes[self.key]
            .object
            .clone();
        let r = unsafe {
            mem::transmute(cell::Ref::map(rc.borrow(), |object| {
                object.as_any().downcast_ref::<O>().unwrap()
            }))
        };
        Ref { rc, r }
    }
}

pub struct Ref<O: 'static> {
    rc: Rc<RefCell<dyn AnyObject>>,
    r: cell::Ref<'static, O>,
}

impl<O: 'static> Deref for Ref<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &*self.r
    }
}
