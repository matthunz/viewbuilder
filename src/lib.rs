use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, marker::PhantomData, sync::mpsc};
use tokio::task;

pub trait Element {}

impl Element for () {}

pub struct ElementRef<T> {
    key: DefaultKey,
    _marker: PhantomData<T>,
}

impl<T> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for ElementRef<T> {}

impl<T: 'static> ElementRef<T> {
    pub fn get(self, tx: &Transaction) -> Option<&T> {
        tx.elements
            .get(self.key)
            .map(|any| any.as_any().downcast_ref().unwrap())
    }

    pub fn get_mut(self, tx: &mut Transaction) -> Option<&mut T> {
        tx.elements
            .get_mut(self.key)
            .map(|any| any.as_any_mut().downcast_mut().unwrap())
    }
}

#[derive(Default)]
pub struct Transaction {
    elements: SlotMap<DefaultKey, Box<dyn AnyElement>>,
}

impl Transaction {
    pub fn insert<T>(&mut self, element: T) -> ElementRef<T>
    where
        T: Element + 'static,
    {
        let key = self.elements.insert(Box::new(element));
        ElementRef {
            key,
            _marker: PhantomData,
        }
    }
}

trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_element(&self) -> &dyn Element;

    fn as_element_mut(&mut self) -> &mut dyn Element;
}

impl<T> AnyElement for T
where
    T: Element + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_element_mut(&mut self) -> &mut dyn Element {
        self
    }
}

pub struct UserInterface {
    tx: mpsc::Sender<Box<dyn FnOnce(&mut Transaction) + Send>>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Box<dyn FnOnce(&mut Transaction) + Send>>();

        task::spawn_blocking(move || {
            let mut transaction = Transaction::default();
            while let Ok(f) = rx.recv() {
                f(&mut transaction)
            }
        });

        Self { tx }
    }

    pub fn transaction(&self, f: impl FnOnce(&mut Transaction) + Send + 'static) {
        self.tx.send(Box::new(f)).unwrap();
    }
}
