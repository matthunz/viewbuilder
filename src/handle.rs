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
    /// Send an update to the object.
    pub fn update(&self, f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        self.state.update(f)
    }

    /// Immutably borrow the object.
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
    /// Send an update to the object.
    pub fn update(&self, mut f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        Runtime::current().inner.borrow_mut().updates.push((
            self.key,
            Box::new(move |element| f(element.downcast_mut().unwrap())),
        ))
    }

    /// Immutably borrow the object.
    pub fn borrow(&self) -> Ref<O> {
        let guard = Runtime::current().inner.borrow().nodes[self.key]
            .object
            .clone();

        let object = cell::Ref::map(guard.borrow(), |object| {
            object.as_any().downcast_ref::<O>().unwrap()
        });

        // Safety: `guard` is held as long as `Ref`.
        let object = unsafe { mem::transmute(object) };

        Ref {
            _guard: guard,
            object,
        }
    }
}

pub struct Ref<O: 'static> {
    object: cell::Ref<'static, O>,
    _guard: Rc<RefCell<dyn AnyObject>>,
}

impl<O: 'static> Deref for Ref<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &*self.object
    }
}
