use crate::{tree::LocalTree, AnyElement};
use slotmap::DefaultKey;
use std::{
    any::Any,
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Clone)]
pub struct AnyElementRef {
    pub(crate) element: Rc<RefCell<Box<dyn AnyElement>>>,
    pub tree: LocalTree,
    pub key: DefaultKey,
}

impl AnyElementRef {
    pub fn get_mut(&self) -> RefMut<Box<dyn AnyElement>> {
        RefMut::map(self.element.borrow_mut(), |element| {
            element.as_any_mut().downcast_mut().unwrap()
        })
    }

    pub fn send(self, msg: Box<dyn Any>) {
        let ui = self.tree.ui.inner.borrow();
        ui.tx
            .send((self.tree.inner.borrow().key, self.key, msg))
            .unwrap();
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
