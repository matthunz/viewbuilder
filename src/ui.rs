use slotmap::{DefaultKey, SlotMap};
use std::{any::Any, marker::PhantomData};
use tokio::sync::mpsc;

use crate::{
    tree::{TreeBuilder, TreeMessage},
    AnyElement, Element, TreeKey, TreeRef,
};

#[derive(Clone)]
pub struct UserInterfaceRef {
    tx: mpsc::UnboundedSender<(TreeKey, DefaultKey, Box<dyn Any>)>,
}
impl UserInterfaceRef {
    pub fn send(&self, tree_key: TreeKey, key: DefaultKey, msg: Box<dyn Any>) {
        self.tx.send((tree_key, key, msg)).unwrap();
    }
}

pub struct UserInterface {
    pub(crate) trees: SlotMap<TreeKey, Box<dyn AnyElement>>,
    tx: mpsc::UnboundedSender<(TreeKey, DefaultKey, Box<dyn Any>)>,
    rx: mpsc::UnboundedReceiver<(TreeKey, DefaultKey, Box<dyn Any>)>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            trees: SlotMap::default(),
            tx,
            rx,
        }
    }

    pub fn insert<T: Element + 'static>(
        &mut self,
        tree_builder: impl TreeBuilder<Tree = T>,
    ) -> TreeRef<T> {
        let ui_ref = UserInterfaceRef {
            tx: self.tx.clone(),
        };
        let key = self
            .trees
            .insert_with_key(|key| Box::new(tree_builder.insert_with_key(key, ui_ref)));
        TreeRef {
            key,
            _marker: PhantomData,
        }
    }

    pub async fn process_events(&mut self) {
        let (tree_key, key, msg) = self.rx.recv().await.unwrap();
        self.trees[tree_key].handle_any(Box::new(TreeMessage::Handle { key, msg }));
    }
}
