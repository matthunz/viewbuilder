use crate::{ui::UserInterfaceRef, AnyElement, Element, ElementRef, TreeKey};
use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::marker::PhantomData;

use super::{TreeBuilder, TreeMessage};

pub struct LocalTree {
    pub(crate) key: TreeKey,
    pub(crate) ui: UserInterfaceRef,
    pub(crate) elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
    pub(crate) children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    pub(crate) parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

impl LocalTree {
    pub fn builder() -> LocalTreeBuilder {
        LocalTreeBuilder {}
    }

    pub fn insert<E: Element + 'static>(&mut self, element: E) -> ElementRef<E> {
        let key = self.elements.insert(Box::new(element));

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}

impl Element for LocalTree {
    type Message = TreeMessage;

    fn handle(&mut self, msg: Self::Message) {
        match msg {
            TreeMessage::Handle { key, msg } => self.elements[key].handle_any(msg),
        }
    }
}

pub struct LocalTreeBuilder {}

impl TreeBuilder for LocalTreeBuilder {
    type Tree = LocalTree;

    fn insert_with_key(self, key: TreeKey, ui: UserInterfaceRef) -> Self::Tree {
        LocalTree {
            key,
            ui,
            elements: Default::default(),
            children: Default::default(),
            parents: Default::default(),
        }
    }
}
