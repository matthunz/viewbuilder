use crate::{rt::RuntimeMessage, AnyObject, Object, Runtime};
use slotmap::DefaultKey;
use std::{
    cell::{self, RefCell},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub struct Handle<O: Object> {
    pub(crate) state: HandleState<O>,
    pub(crate) handle: O::Handle,
}

impl<O: Object> Clone for Handle<O> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            handle: self.handle.clone(),
        }
    }
}

impl<O: Object> Handle<O> {
    pub fn key(&self) -> DefaultKey {
        self.state.key()
    }

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

    /// Mutably borrow the object.
    pub fn borrow_mut(&self) -> RefMut<O> {
        self.state.borrow_mut()
    }
}

impl<O: Object> Deref for Handle<O> {
    type Target = O::Handle;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

struct Dropper {
    key: DefaultKey,
}

impl Drop for Dropper {
    fn drop(&mut self) {
        if let Some(rt) = Runtime::try_current() {
            rt.tx.send(RuntimeMessage::Remove { key: self.key }).ok();
        }
    }
}

pub struct HandleState<O: Object> {
    dropper: Rc<Dropper>,
    _marker: PhantomData<O>,
}

impl<O: Object> Clone for HandleState<O> {
    fn clone(&self) -> Self {
        Self {
            dropper: self.dropper.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<O: Object> HandleState<O> {
    pub(crate) fn new(key: DefaultKey) -> Self {
        Self {
            dropper: Rc::new(Dropper { key }),
            _marker: PhantomData,
        }
    }

    pub fn key(&self) -> DefaultKey {
        self.dropper.key
    }

    /// Send an update to the object.
    pub fn update(&self, mut f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        Runtime::current()
            .tx
            .send(RuntimeMessage::Update {
                key: self.key(),
                update: Box::new(move |element| f(element.downcast_mut().unwrap())),
            })
            .unwrap();
    }

    /// Immutably borrow the object.
    pub fn borrow(&self) -> Ref<O> {
        let guard = Runtime::current().inner.borrow().nodes[self.key()]
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

    /// Mutably borrow the object.
    pub fn borrow_mut(&self) -> RefMut<O> {
        let guard = Runtime::current().inner.borrow().nodes[self.key()]
            .object
            .clone();

        let object = cell::RefMut::map(guard.borrow_mut(), |object| {
            object.as_any_mut().downcast_mut::<O>().unwrap()
        });

        // Safety: `guard` is held as long as `Ref`.
        let object = unsafe { mem::transmute(object) };

        RefMut {
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

pub struct RefMut<O: 'static> {
    object: cell::RefMut<'static, O>,
    _guard: Rc<RefCell<dyn AnyObject>>,
}

impl<O: 'static> Deref for RefMut<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &*self.object
    }
}

impl<O: 'static> DerefMut for RefMut<O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.object
    }
}
