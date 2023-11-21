use crate::{tree::LocalTree, Element};
use slotmap::DefaultKey;
use std::marker::PhantomData;

pub struct ElementRef<E> {
    pub(crate) key: DefaultKey,
    pub(crate) _marker: PhantomData<E>,
}

impl<E> Clone for ElementRef<E> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<E> Copy for ElementRef<E> {}

impl<E> ElementRef<E> {
    pub fn get(self, tree: &LocalTree) -> &E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any().downcast_ref().unwrap()
    }

    pub fn get_mut(self, tree: &mut LocalTree) -> &mut E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any_mut().downcast_mut().unwrap()
    }

    pub fn send(self, tree: &mut LocalTree, msg: E::Message)
    where
        E: Element + 'static,
    {
        tree.elements[self.key].handle_any(Box::new(msg));
        tree.tx.send(tree.key).unwrap();
    }

    pub fn push_child(self, tree: &mut LocalTree, key: DefaultKey) {
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
