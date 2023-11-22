use crate::{tree::LocalTree, AnyElement, Element};
use slotmap::DefaultKey;
use std::{
    cell::{RefCell, RefMut},
    marker::PhantomData,
    rc::Rc,
};

pub struct LocalElementRef<E> {
    pub(crate) element: Rc<RefCell<Box<dyn AnyElement>>>,
    pub tree: LocalTree,
    pub key: DefaultKey,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Clone for LocalElementRef<E> {
    fn clone(&self) -> Self {
        Self {
            element: self.element.clone(),
            tree: self.tree.clone(),
            key: self.key.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<E> LocalElementRef<E> {
    pub fn get_mut(&self) -> RefMut<E>
    where
        E: 'static,
    {
        RefMut::map(self.element.borrow_mut(), |element| {
            element.as_any_mut().downcast_mut().unwrap()
        })
    }

    pub fn send(self, msg: E::Message)
    where
        E: Element + 'static,
    {
        let tree = self.tree.inner.borrow_mut();
        tree.ui.send(tree.key, self.key, Box::new(msg));
    }

    pub fn push_child(&self, key: DefaultKey) {
        let mut tree = self.tree.inner.borrow_mut();
        let tree = &mut *tree;

        if let Some(children) = tree.children.get_mut(self.key) {
            children.push(key);
        } else {
            tree.children.insert(self.key, vec![key]);
        }

        if let Some(parent) = tree.parents.get_mut(key) {
            // Remove this key's previous parent (if it exists).
            if let Some(children) = tree.children.get_mut(*parent) {
                if let Some(idx) = children.iter().position(|child_key| key == *child_key) {
                    children.remove(idx);
                }
            }

            *parent = self.key;
        } else {
            tree.parents.insert(key, self.key);
        }
    }
}
