use crate::Transaction;
use slotmap::DefaultKey;
use std::marker::PhantomData;

pub struct ElementRef<T> {
    pub key: DefaultKey,
    pub(crate) _marker: PhantomData<T>,
}

impl<T> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementRef<T> {}

impl<T: 'static> ElementRef<T> {
    pub fn get(self, tx: &Transaction) -> Option<&T> {
        tx.nodes
            .get(self.key)
            .map(|node| node.element.as_any().downcast_ref().unwrap())
    }

    pub fn get_mut(self, tx: &mut Transaction) -> Option<&mut T> {
        tx.nodes
            .get_mut(self.key)
            .map(|node| node.element.as_any_mut().downcast_mut().unwrap())
    }
}
