use super::{TreeBuilder, TreeMessage};
use crate::{element::Window, AnyElement, Element, LocalElementRef, TreeKey, UserInterface};
use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use vello::{Scene, SceneBuilder};

pub(crate) struct Inner {
    pub(crate) key: TreeKey,
    pub(crate) elements: SlotMap<DefaultKey, Rc<RefCell<Box<dyn AnyElement>>>>,
    pub(crate) children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    pub(crate) parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

pub struct LocalTree<E> {
    pub(crate) root: Option<Box<LocalElementRef<E, E>>>,
    pub(crate) ui: UserInterface,
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl<E> Clone for LocalTree<E> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            ui: self.ui.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<E> LocalTree<E> {
    pub fn builder(root: E) -> LocalTreeBuilder<E> {
        LocalTreeBuilder { root }
    }

    pub fn root(&self) -> LocalElementRef<E, E> {
        self.root.as_deref().unwrap().clone()
    }

    pub fn insert<E2: Element + 'static>(&self, element: E2) -> LocalElementRef<E, E2> {
        let element: Rc<RefCell<Box<dyn AnyElement>>> = Rc::new(RefCell::new(Box::new(element)));
        let key = self.inner.borrow_mut().elements.insert(element.clone());

        LocalElementRef {
            element,
            tree: self.clone(),
            key,
            _marker: PhantomData,
        }
    }

    pub fn insert_window(&self) -> LocalElementRef<E, Window> {
        let window = self.insert(Window {});
        self.ui.insert_window(self.inner.borrow().key, window.key);
        window
    }
}

impl<E: Element> Element for LocalTree<E> {
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

pub struct LocalTreeBuilder<E> {
    pub(crate) root: E,
}

impl<E: Element + 'static> TreeBuilder for LocalTreeBuilder<E> {
    type Tree = LocalTree<E>;

    fn insert_with_key(self, key: TreeKey, ui: UserInterface) -> Self::Tree {
        let mut me = LocalTree {
            root: None,
            ui,
            inner: Rc::new(RefCell::new(Inner {
                key,

                elements: SlotMap::new(),
                children: Default::default(),
                parents: Default::default(),
            })),
        };
        let root = me.insert(self.root);
        me.root = Some(Box::new(root));
        me
    }
}
