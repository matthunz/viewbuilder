use slotmap::{DefaultKey, SlotMap};
use std::sync::{mpsc, Arc, Mutex};
use tokio::task;

pub trait Element {}


#[derive(Default)]
pub struct Transaction {
    elements: SlotMap<DefaultKey, Box<dyn Element>>,
}

impl Transaction {
    pub fn insert(&mut self, element: impl Element + 'static) -> DefaultKey {
        self.elements.insert(Box::new(element))
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
