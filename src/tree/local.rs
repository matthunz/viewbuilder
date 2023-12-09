use super::{TreeBuilder, TreeMessage};
use crate::{
    element::{Lifecycle, LifecycleContext},
    AnyElement, AnyElementRef, Element, LocalElementRef, TreeKey, UserInterface,
};
use slotmap::{DefaultKey, SlotMap, SparseSecondaryMap};
use std::{cell::RefCell, marker::PhantomData, rc::Rc};
use vello::{kurbo::Affine, Scene, SceneBuilder, SceneFragment};

pub(crate) struct Inner {
    pub(crate) key: TreeKey,
    pub(crate) elements: SlotMap<DefaultKey, Rc<RefCell<Box<dyn AnyElement>>>>,
    pub(crate) children: SparseSecondaryMap<DefaultKey, Vec<DefaultKey>>,
    pub(crate) parents: SparseSecondaryMap<DefaultKey, DefaultKey>,
}

#[derive(Clone)]
pub struct LocalTree {
    pub(crate) root: Option<Box<AnyElementRef>>,
    pub(crate) ui: UserInterface,
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl LocalTree {
    pub fn builder(root: impl Element + 'static) -> LocalTreeBuilder {
        LocalTreeBuilder {
            root: Box::new(root),
        }
    }

    pub fn root(&self) -> AnyElementRef {
        self.root.as_deref().unwrap().clone()
    }

    pub fn insert<E2: Element + 'static>(&self, element: E2) -> LocalElementRef<E2> {
        let element: Rc<RefCell<Box<dyn AnyElement>>> = Rc::new(RefCell::new(Box::new(element)));
        let key = self.inner.borrow_mut().elements.insert(element.clone());

        // TODO
        element.borrow_mut().lifecycle_any(
            LifecycleContext {
                ui: self.ui.clone(),
                tree_key: self.inner.borrow().key,
                key,
            },
            Lifecycle::Build,
        );

        LocalElementRef {
            element,
            tree: self.clone(),
            key,
            _marker: PhantomData,
        }
    }

    pub fn insert_any(&self, element: Box<dyn AnyElement>) -> AnyElementRef {
        let element: Rc<RefCell<Box<dyn AnyElement>>> = Rc::new(RefCell::new(element));
        let key = self.inner.borrow_mut().elements.insert(element.clone());

        // TODO
        element.borrow_mut().lifecycle_any(
            LifecycleContext {
                ui: self.ui.clone(),
                tree_key: self.inner.borrow().key,
                key,
            },
            Lifecycle::Build,
        );

        AnyElementRef {
            element,
            tree: self.clone(),
            key,
        }
    }
}

impl Element for LocalTree {
    type Message = TreeMessage;

    fn lifecycle(&mut self, _cx: LifecycleContext, _lifecycle: Lifecycle) {}

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

    fn render(&mut self, mut scene: vello::SceneBuilder) {
        for element in self.inner.borrow_mut().elements.values_mut() {
            let mut child_scene = SceneFragment::new();
            element
                .borrow_mut()
                .render_any(SceneBuilder::for_fragment(&mut child_scene));
            scene.append(&child_scene, Some(Affine::default()));
        }
    }
}

pub struct LocalTreeBuilder {
    pub(crate) root: Box<dyn AnyElement>,
}

impl TreeBuilder for LocalTreeBuilder {
    type Tree = LocalTree;

    fn insert_with_key(self, key: TreeKey, ui: UserInterface) -> Self::Tree {
        let mut me = LocalTree {
            root: None,
            ui: ui.clone(),
            inner: Rc::new(RefCell::new(Inner {
                key,
                elements: SlotMap::new(),
                children: Default::default(),
                parents: Default::default(),
            })),
        };
        let root = me.insert_any(self.root);
        me.root = Some(Box::new(root));
        me
    }
}
