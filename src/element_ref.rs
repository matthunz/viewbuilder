use crate::{AnyElement, UserInterface};
use slotmap::DefaultKey;
use std::{
    cell::{Ref, RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

pub struct Entry<E> {
    element: Rc<RefCell<dyn AnyElement>>,
    _marker: PhantomData<E>,
}

impl<E: 'static> Entry<E> {
    pub fn borrow(&self) -> Ref<E> {
        Ref::map(self.element.borrow(), |element| {
            element.as_any().downcast_ref().unwrap()
        })
    }

    pub fn borrow_mut(&self) -> RefMut<E> {
        RefMut::map(self.element.borrow_mut(), |element| {
            element.as_any_mut().downcast_mut().unwrap()
        })
    }
}

pub struct ElementRef<E> {
    pub key: DefaultKey,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Clone for ElementRef<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for ElementRef<E> {}

impl<E> ElementRef<E> {
    pub fn get(self) -> Entry<E> {
        let element = UserInterface::current().get(self.key);
        Entry {
            element,
            _marker: PhantomData,
        }
    }
}
