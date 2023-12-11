use crate::{Element, Handle, Node};
use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, cell::RefCell, collections::HashSet, marker::PhantomData, mem, rc::Rc};

#[derive(Default)]
pub(crate) struct Inner {
    pub(crate) queue: Vec<(DefaultKey, Box<dyn Any>)>,
    pub(crate) nodes: SlotMap<DefaultKey, Node>,
    pub(crate) pending_layouts: HashSet<DefaultKey>,
}

#[derive(Clone, Default)]
pub struct UserInterface {
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl UserInterface {
    pub fn current() -> Self {
        thread_local! {
            static CURRENT: RefCell<Option<UserInterface>> = RefCell::default();
        }

        CURRENT
            .try_with(|cell| {
                let mut current = cell.borrow_mut();
                if let Some(ui) = &*current {
                    ui.clone()
                } else {
                    let ui = Self::default();
                    *current = Some(ui.clone());
                    ui
                }
            })
            .unwrap()
    }

    pub fn insert<E: Element + 'static>(&self, element: E) -> Handle<E> {
        let node = Node {
            element: Rc::new(RefCell::new(element)),
        };
        let key = self.inner.borrow_mut().nodes.insert(node);
        Handle {
            key,

            _marker: PhantomData,
        }
    }

    pub fn run(&self) {
        let mut queue = mem::take(&mut self.inner.borrow_mut().queue);
        while let Some((key, msg)) = queue.pop() {
            let element = self.inner.borrow().nodes[key].element.clone();
            element.borrow_mut().update_any(key, self.clone(), msg);
        }

        let pending_layouts = mem::take(&mut self.inner.borrow_mut().pending_layouts);
        for key in pending_layouts {
            let element = self.inner.borrow().nodes[key].element.clone();
            element.borrow_mut().layout_any(None, None);
        }
    }
}
