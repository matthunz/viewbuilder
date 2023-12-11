extern crate self as viewbuilder;

use slotmap::{DefaultKey, SlotMap};
use std::{
    any::Any,
    cell::{self, RefCell},
    marker::PhantomData,
    mem,
    ops::Deref,
    rc::Rc,
};

pub use viewbuilder_macros::object;

pub struct HandleState<O: Object> {
    pub key: DefaultKey,
    _marker: PhantomData<O>,
}

impl<O: Object> Clone for HandleState<O> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            _marker: self._marker.clone(),
        }
    }
}

impl<O: Object> HandleState<O> {
    pub fn update(&self, mut f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        Runtime::current().inner.borrow_mut().updates.push((
            self.key,
            Box::new(move |element| f(element.downcast_mut().unwrap())),
        ))
    }

    pub fn borrow(&self) -> Ref<O> {
        let rc = Runtime::current().inner.borrow().nodes[self.key]
            .object
            .clone();
        let r = unsafe {
            mem::transmute(cell::Ref::map(rc.borrow(), |object| {
                object.as_any().downcast_ref::<O>().unwrap()
            }))
        };
        Ref { rc, r }
    }
}

pub struct Ref<O: 'static> {
    rc: Rc<RefCell<dyn AnyObject>>,
    r: cell::Ref<'static, O>,
}

impl<O: 'static> Deref for Ref<O> {
    type Target = O;

    fn deref(&self) -> &Self::Target {
        &*self.r
    }
}

pub struct Handle<O: Object> {
    state: HandleState<O>,
    sender: O::Sender,
}

impl<O: Object> Clone for Handle<O> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            sender: self.sender.clone(),
        }
    }
}

impl<O: Object> Handle<O> {
    pub fn update(&self, f: impl FnMut(&mut O) + 'static)
    where
        O: 'static,
    {
        self.state.update(f)
    }

    pub fn borrow(&self) -> Ref<O> {
        self.state.borrow()
    }
}

impl<O: Object> Deref for Handle<O> {
    type Target = O::Sender;

    fn deref(&self) -> &Self::Target {
        &self.sender
    }
}

pub trait Object: Sized {
    type Sender: From<HandleState<Self>> + Clone;

    fn spawn(self) -> Handle<Self>
    where
        Self: 'static,
    {
        let key = Runtime::current().inner.borrow_mut().nodes.insert(Node {
            object: Rc::new(RefCell::new(self)),
            listeners: Vec::new(),
        });

        Handle {
            state: HandleState {
                key,
                _marker: PhantomData,
            },
            sender: HandleState {
                key,
                _marker: PhantomData,
            }
            .into(),
        }
    }
}

pub trait AnyObject {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<O> AnyObject for O
where
    O: Object + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

struct Node {
    object: Rc<RefCell<dyn AnyObject>>,
    listeners: Vec<Rc<RefCell<dyn FnMut(&dyn Any)>>>,
}

#[derive(Default)]
struct Inner {
    nodes: SlotMap<DefaultKey, Node>,
    updates: Vec<(DefaultKey, Box<dyn FnMut(&mut dyn Any)>)>,
    message_queue: Vec<(DefaultKey, Box<dyn Any>)>,
    current: Option<DefaultKey>,
}

#[derive(Clone, Default)]
pub struct Runtime {
    inner: Rc<RefCell<Inner>>,
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

pub struct Signal<T> {
    key: DefaultKey,
    _marker: PhantomData<T>,
}

impl<T> Signal<T> {
    pub fn new(key: DefaultKey) -> Self {
        Self {
            key,
            _marker: PhantomData,
        }
    }
}

impl<T: Clone + 'static> Signal<T> {
    pub fn bind<O>(&self, handle: &Handle<O>, slot: impl FnMut(&mut O, T) + 'static)
    where
        O: Object + 'static,
    {
        let f = Rc::new(RefCell::new(slot));
        let handle = handle.clone();
        Runtime::current().inner.borrow_mut().nodes[self.key]
            .listeners
            .push(Rc::new(RefCell::new(move |any: &dyn Any| {
                let data = any.downcast_ref::<T>().unwrap().clone();
                let f = f.clone();
                handle.update(move |object| f.borrow_mut()(object, data.clone()))
            })));
    }
}
