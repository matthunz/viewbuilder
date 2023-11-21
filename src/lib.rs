use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::{any::Any, marker::PhantomData};

pub trait Element {}

pub trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<E> AnyElement for E
where
    E: Element + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ElementRef<E> {
    key: DefaultKey,
    _marker: PhantomData<E>,
}

impl<E> ElementRef<E> {
    pub fn get(self, tree: &Tree) -> &E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any().downcast_ref().unwrap()
    }

    pub fn get_mut(self, tree: &mut Tree) -> &mut E
    where
        E: 'static,
    {
        tree.elements[self.key].as_any_mut().downcast_mut().unwrap()
    }

    pub fn push_child(self, tree: &mut Tree, key: DefaultKey) {
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

pub struct Tree {
    elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
    children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

impl Tree {
    pub fn insert<E: Element + 'static>(
        &mut self,
        element: E,
        _parent: Option<DefaultKey>,
    ) -> ElementRef<E> {
        let key = self.elements.insert(Box::new(element));

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}
