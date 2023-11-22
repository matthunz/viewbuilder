use super::{TreeBuilder, TreeMessage};
use crate::{ui::UserInterfaceRef, AnyElement, Element, LocalElementRef, TreeKey};
use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use vello::{Scene, SceneBuilder};

pub(crate) struct Inner {
    pub(crate) key: TreeKey,
    pub(crate) ui: UserInterfaceRef,
    pub(crate) elements: SlotMap<DefaultKey, Rc<RefCell<Box<dyn AnyElement>>>>,
    pub(crate) children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    pub(crate) parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

#[derive(Clone)]
pub struct LocalTree {
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl LocalTree {
    pub fn builder() -> LocalTreeBuilder {
        LocalTreeBuilder {}
    }

    pub fn insert<E: Element + 'static>(&mut self, element: E) -> LocalElementRef<E> {
        let element: Rc<RefCell<Box<dyn AnyElement>>> = Rc::new(RefCell::new(Box::new(element)));
        let key = self.inner.borrow_mut().elements.insert(element.clone());

        LocalElementRef {
            element,
            tree: self.clone(),
            key,
            _marker: PhantomData,
        }
    }
}

impl Element for LocalTree {
    type Message = TreeMessage;

    fn handle(&mut self, msg: Self::Message) {
        match msg {
            TreeMessage::Handle { key, msg } => self.inner.borrow_mut().elements[key]
                .borrow_mut()
                .handle_any(msg),
            TreeMessage::Render { key } => {
                let mut scene = Scene::new();
                self.inner.borrow_mut().elements[key]
                    .borrow_mut()
                    .render_any(SceneBuilder::for_scene(&mut scene))
            }
        }
    }

    fn render(&mut self, _scene: vello::SceneBuilder) {
        for element in self.inner.borrow_mut().elements.values_mut() {
            let mut scene = Scene::new();
            element
                .borrow_mut()
                .render_any(SceneBuilder::for_scene(&mut scene))
        }
    }
}

pub struct LocalTreeBuilder {}

impl TreeBuilder for LocalTreeBuilder {
    type Tree = LocalTree;

    fn insert_with_key(self, key: TreeKey, ui: UserInterfaceRef) -> Self::Tree {
        LocalTree {
            inner: Rc::new(RefCell::new(Inner {
                key,
                ui,
                elements: Default::default(),
                children: Default::default(),
                parents: Default::default(),
            })),
        }
    }
}
