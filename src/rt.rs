use crate::AnyObject;
use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, cell::RefCell, rc::Rc};
use tokio::sync::mpsc;

pub(crate) enum RuntimeMessage {
    Update {
        key: DefaultKey,
        update: Box<dyn FnMut(&mut dyn Any)>,
    },
    Message {
        key: DefaultKey,
        msg: Box<dyn Any>,
    },
    Remove {
        key: DefaultKey,
    },
}

pub(crate) struct Node {
    pub(crate) object: Rc<RefCell<dyn AnyObject>>,
    pub(crate) listeners: Vec<Rc<RefCell<dyn FnMut(&dyn Any)>>>,
}

pub(crate) struct Inner {
    pub(crate) nodes: SlotMap<DefaultKey, Node>,
    pub(crate) rx: mpsc::UnboundedReceiver<RuntimeMessage>,
    pub(crate) current: Option<DefaultKey>,
}

thread_local! {
    static CURRENT: RefCell<Option<Runtime>> = RefCell::default();
}

#[derive(Clone)]
pub struct Runtime {
    pub(crate) inner: Rc<RefCell<Inner>>,
    pub(crate) tx: mpsc::UnboundedSender<RuntimeMessage>,
}

impl Default for Runtime {
    fn default() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            inner: Rc::new(RefCell::new(Inner {
                nodes: Default::default(),
                rx,
                current: Default::default(),
            })),
            tx,
        }
    }
}

impl Runtime {
    pub fn enter(&self) -> RuntimeGuard {
        CURRENT
            .try_with(|cell| {
                let mut current = cell.borrow_mut();
                if current.is_some() {
                    panic!("A Viewbuilder runtime is already running in this thread.");
                }
                *current = Some(self.clone());
            })
            .unwrap();

        RuntimeGuard { _priv: () }
    }

    pub fn current() -> Self {
        Self::try_current().expect("There is no Viewbuilder runtime running on this thread.")
    }

    pub fn try_current() -> Option<Self> {
        CURRENT.try_with(|cell| cell.borrow().clone()).unwrap()
    }

    pub fn emit(&self, msg: Box<dyn Any>) {
        let me = self.inner.borrow();
        let key = me.current.unwrap();
        self.send(key, msg);
    }

    pub fn send(&self, key: DefaultKey, msg: Box<dyn Any>) {
        self.tx.send(RuntimeMessage::Message { key, msg }).unwrap();
    }

    pub async fn run(&self) {
        let mut me = self.inner.borrow_mut();
        if let Some(msg) = me.rx.recv().await {
            drop(me);
            self.handle(msg);

            loop {
                let mut me = self.inner.borrow_mut();
                if let Ok(msg) = me.rx.try_recv() {
                    drop(me);
                    self.handle(msg);
                } else {
                    break;
                }
            }
        }
    }

    fn handle(&self, msg: RuntimeMessage) {
        match msg {
            RuntimeMessage::Update { key, mut update } => {
                let object = self.inner.borrow().nodes[key].object.clone();
                self.inner.borrow_mut().current = Some(key);
                update(object.borrow_mut().as_any_mut());
                self.inner.borrow_mut().current = None;
            }
            RuntimeMessage::Message { key, msg } => {
                let listeners = self.inner.borrow().nodes[key].listeners.clone();
                for listener in &listeners {
                    listener.borrow_mut()(&*msg);
                }
            }
            RuntimeMessage::Remove { key: _ } => todo!(),
        }
    }
}

pub struct RuntimeGuard {
    _priv: (),
}

impl Drop for RuntimeGuard {
    fn drop(&mut self) {
        CURRENT.try_with(|cell| cell.borrow_mut().take()).unwrap();
    }
}
