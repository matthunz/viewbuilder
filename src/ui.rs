use slotmap::{DefaultKey, SlotMap};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    marker::PhantomData,
};
use tokio::sync::mpsc;
use vello::{Scene, SceneBuilder};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder, WindowId},
};

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
    windows: HashMap<WindowId, (Window, TreeKey, DefaultKey)>,
    tx: mpsc::UnboundedSender<(TreeKey, DefaultKey, Box<dyn Any>)>,
    rx: mpsc::UnboundedReceiver<(TreeKey, DefaultKey, Box<dyn Any>)>,
    event_loop: EventLoop<()>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self {
            trees: SlotMap::default(),
            windows: HashMap::new(),
            event_loop: EventLoop::new().unwrap(),
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

    pub fn insert_window(&mut self, tree_key: TreeKey, key: DefaultKey) {
        let window = WindowBuilder::new().build(&self.event_loop).unwrap();
        self.windows.insert(window.id(), (window, tree_key, key));
    }

    pub async fn render(&mut self) {
        let mut dirty = HashSet::new();

        // Await the first event
        let (tree_key, key, msg) = self.rx.recv().await.unwrap();
        self.trees[tree_key].handle_any(Box::new(TreeMessage::Handle { key, msg }));
        dirty.insert((tree_key, key));

        // Process any remaining events
        while let Ok((tree_key, key, msg)) = self.rx.try_recv() {
            self.trees[tree_key].handle_any(Box::new(TreeMessage::Handle { key, msg }));
            dirty.insert((tree_key, key));
        }

        let mut dirty_trees = HashSet::new();
        for (tree_key, key) in dirty {
            let tree = self.trees.get_mut(tree_key).unwrap();
            tree.handle_any(Box::new(TreeMessage::Render { key }));

            dirty_trees.insert(tree_key);
        }

        for tree_key in dirty_trees {
            let mut scene = Scene::new();
            let tree = self.trees.get_mut(tree_key).unwrap();
            tree.handle_any(Box::new(TreeMessage::Render { key }));
            tree.render_any(SceneBuilder::for_scene(&mut scene));
        }
    }

    pub fn render_all(&mut self) {
        for tree in self.trees.values_mut() {
            let mut scene = Scene::new();
            tree.render_any(SceneBuilder::for_scene(&mut scene));
        }
    }

    pub fn run(mut self) {
        self.render_all();
        self.event_loop.run(|_, _| {}).unwrap();
    }
}
