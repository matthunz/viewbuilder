use crate::AnyObject;
use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, cell::RefCell, mem, rc::Rc};

pub(crate) struct Node {
    pub(crate) object: Rc<RefCell<dyn AnyObject>>,
    pub(crate) listeners: Vec<Rc<RefCell<dyn FnMut(&dyn Any)>>>,
}

#[derive(Default)]
pub(crate) struct Inner {
    pub(crate) nodes: SlotMap<DefaultKey, Node>,
    pub(crate) updates: Vec<(DefaultKey, Box<dyn FnMut(&mut dyn Any)>)>,
    pub(crate) message_queue: Vec<(DefaultKey, Box<dyn Any>)>,
    pub(crate) current: Option<DefaultKey>,
}

#[derive(Clone, Default)]
pub struct Runtime {
    pub(crate) inner: Rc<RefCell<Inner>>,
}

impl Runtime {
    pub fn current() -> Self {
        thread_local! {
            static CURRENT: RefCell<Option<Runtime>> = RefCell::default();
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

    pub fn emit(&self, msg: Box<dyn Any>) {
        let mut me = self.inner.borrow_mut();
        let key = me.current.unwrap();
        me.message_queue.push((key, msg));
    }

    pub fn send(&self, key: DefaultKey, msg: Box<dyn Any>) {
        let mut me = self.inner.borrow_mut();
        me.message_queue.push((key, msg));
    }

    pub fn run(&self) {
        let mut updates = mem::take(&mut self.inner.borrow_mut().updates);
        for (key, f) in &mut updates {
            let object = self.inner.borrow().nodes[*key].object.clone();
            self.inner.borrow_mut().current = Some(*key);
            f(object.borrow_mut().as_any_mut());
            self.inner.borrow_mut().current = None;
        }

        let mut message_queue = mem::take(&mut self.inner.borrow_mut().message_queue);
        for (key, msg) in &mut message_queue {
            let listeners = self.inner.borrow().nodes[*key].listeners.clone();
            for listener in &listeners {
                listener.borrow_mut()(&**msg);
            }
        }
    }
}
