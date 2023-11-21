use std::{any::Any, marker::PhantomData};

use slotmap::SlotMap;
use tokio::sync::mpsc;
use vello::SceneBuilder;

use crate::{tree::TreeBuilder, TreeKey, TreeRef};

pub struct UserInterface {
    pub(crate) trees: SlotMap<TreeKey, Box<dyn Any>>,
    tx: mpsc::UnboundedSender<TreeKey>,
    rx: mpsc::UnboundedReceiver<TreeKey>,
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

    pub fn insert<T: 'static>(&mut self, tree_builder: impl TreeBuilder<Tree = T>) -> TreeRef<T> {
        let key = self
            .trees
            .insert_with_key(|key| Box::new(tree_builder.insert_with_key(key, self.tx.clone())));
        TreeRef {
            key,
            _marker: PhantomData,
        }
    }

    pub async fn render(&mut self, _scene: SceneBuilder<'_>) {
        let _key = self.rx.recv().await.unwrap();
    }
}
