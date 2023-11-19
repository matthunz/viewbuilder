use dyn_hash::DynHash;
use lazy_static::lazy_static;
use slotmap::{DefaultKey, SlotMap};
use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc;

pub struct ObjectRef<T> {
    id: u64,
    _marker: PhantomData<T>,
}

impl<T> Clone for ObjectRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}


impl<T> Copy for ObjectRef<T> {

} 

impl<T> ObjectRef<T> {
    pub fn update(self, msg: T::Message)
    where
        T: Object,
    {
        let rt = Runtime::current();
        rt.tx
            .send(Message::Update {
                id: self.id,
                msg: Arc::new(msg),
            })
            .unwrap();
    }

    pub fn listen(self, mut f: impl FnMut(&T::Message) + Send + 'static)
    where
        T: Object,
    {
        let rt = Runtime::current();
        rt.tx.send(Message::Listen {
            id: self.id,
            f: Box::new(move |msg| f(msg.downcast_ref().unwrap())),
        }).unwrap();
    }
}

pub fn spawn<T>(object: T) -> ObjectRef<T>
where
    T: Object + 'static,
{
    let rt = Runtime::current();
    let id = rt.next_id.fetch_add(1, Ordering::SeqCst);
    rt.tx
        .send(Message::Insert {
            id,
            object: Box::new(object),
        })
        .unwrap();

    ObjectRef {
        id,
        _marker: PhantomData,
    }
}

pub trait Object: Send {
    type Message: Send + Sync + 'static;

    fn update(&mut self, msg: &Self::Message);
}

pub trait AnyObject: Send {
    fn any_update(&mut self, msg: Arc<dyn Any>);
}

impl<T> AnyObject for T
where
    T: Object,
{
    fn any_update(&mut self, msg: Arc<dyn Any>) {
        Object::update(self, msg.downcast_ref().unwrap())
    }
}

pub enum Message {
    Insert {
        id: u64,
        object: Box<dyn AnyObject>,
    },
    Update {
        id: u64,
        msg: Arc<dyn Any + Send + Sync>,
    },
    Listen {
        id: u64,
        f: Box<dyn FnMut(Arc<dyn Any + Send + Sync>) + Send>,
    },
}

#[derive(Clone)]
pub struct Runtime {
    tx: mpsc::UnboundedSender<Message>,
    next_id: Arc<AtomicU64>,
}

impl Runtime {
    fn new() -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut objects = HashMap::new();
            let mut listeners: HashMap<
                u64,
                Vec<Box<dyn FnMut(Arc<dyn Any + Send + Sync>) + Send>>,
            > = HashMap::new();

            while let Some(msg) = rx.recv().await {
                match msg {
                    Message::Insert { id, object } => {
                        objects.insert(id, object);
                    }
                    Message::Update { id, msg } => {
                        objects.get_mut(&id).unwrap().any_update(msg.clone());
                        if let Some(listeners) = listeners.get_mut(&id) {
                            for listener in listeners {
                                listener(msg.clone());
                            }
                        }
                    }
                    Message::Listen { id, f } => {
                        if let Some(listeners) = listeners.get_mut(&id) {
                            listeners.push(f);
                        } else {
                            listeners.insert(id, vec![f]);
                        }
                    }
                }
            }
        });

        Self {
            tx,
            next_id: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn current() -> Self {
        lazy_static! {
            static ref RUNTIME: Runtime = Runtime::new();
        }
        RUNTIME.clone()
    }
}
