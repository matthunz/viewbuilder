use crate::{
    element,
    tree::{TreeBuilder, TreeMessage},
    AnyElement, Element, TreeKey,
};
use slotmap::{DefaultKey, SlotMap};
use std::{
    any::Any,
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    rc::Rc,
};
use tokio::sync::mpsc;
use vello::{Scene, SceneBuilder};
use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder, WindowId},
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

pub(crate) struct Inner {
    pub(crate) trees: SlotMap<TreeKey, Box<dyn AnyElement>>,
    windows: HashMap<WindowId, (Window, TreeKey, DefaultKey)>,
    pub(crate) tx: mpsc::UnboundedSender<(TreeKey, DefaultKey, Box<dyn Any>)>,
    rx: mpsc::UnboundedReceiver<(TreeKey, DefaultKey, Box<dyn Any>)>,
    event_loop: Option<EventLoop<UserEvent>>,
}

#[derive(Clone)]
pub struct TreeRef<T> {
    pub key: TreeKey,
    pub tree: T,
}

impl<T> Deref for TreeRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<T> DerefMut for TreeRef<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

pub enum UserEvent {
    CreateWindow {
        ui: UserInterface,
        tree_key: TreeKey,
        key: DefaultKey,
        window: element::Window,
    },
}

#[derive(Clone)]
pub struct UserInterface {
    pub(crate) inner: Rc<RefCell<Inner>>,
    proxy: EventLoopProxy<UserEvent>,
}

impl UserInterface {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let event_loop = EventLoopBuilder::with_user_event().build().unwrap();
        let proxy = event_loop.create_proxy();
        Self {
            inner: Rc::new(RefCell::new(Inner {
                trees: SlotMap::default(),
                windows: HashMap::new(),
                event_loop: Some(event_loop),
                tx,
                rx,
            })),
            proxy,
        }
    }

    pub fn insert<T: Element + Clone + 'static>(
        &self,
        tree_builder: impl TreeBuilder<Tree = T>,
    ) -> TreeRef<T> {
        let mut me = self.inner.borrow_mut();
        let ui_ref = self.clone();
        let mut tree_cell = None;
        let key = me.trees.insert_with_key(|key| {
            let tree = tree_builder.insert_with_key(key, ui_ref);
            tree_cell = Some(tree.clone());
            Box::new(tree)
        });
        TreeRef {
            key,
            tree: tree_cell.unwrap(),
        }
    }

    pub(crate) fn insert_window(
        &self,
        tree_key: TreeKey,
        key: DefaultKey,
        window: element::Window,
    ) {
        self.proxy
            .send_event(UserEvent::CreateWindow {
                ui: self.clone(),
                tree_key,
                key,
                window,
            })
            .ok()
            .unwrap();
    }

    pub async fn render(&self) {
        let mut me = self.inner.borrow_mut();

        let mut dirty = HashSet::new();

        // Await the first event
        let (tree_key, key, msg) = me.rx.recv().await.unwrap();
        me.trees[tree_key].handle_any(Box::new(TreeMessage::Handle { key, msg }));
        dirty.insert((tree_key, key));

        // Process any remaining events
        while let Ok((tree_key, key, msg)) = me.rx.try_recv() {
            me.trees[tree_key].handle_any(Box::new(TreeMessage::Handle { key, msg }));
            dirty.insert((tree_key, key));
        }

        let mut dirty_trees = HashSet::new();
        for (tree_key, key) in dirty {
            let tree = me.trees.get_mut(tree_key).unwrap();
            tree.handle_any(Box::new(TreeMessage::Render { key }));

            dirty_trees.insert(tree_key);
        }

        for tree_key in dirty_trees {
            let mut scene = Scene::new();
            let tree = me.trees.get_mut(tree_key).unwrap();
            tree.handle_any(Box::new(TreeMessage::Render { key }));
            tree.render_any(SceneBuilder::for_scene(&mut scene));
        }
    }

    pub fn render_all(&self) {
        let mut me = self.inner.borrow_mut();
        for tree in me.trees.values_mut() {
            let mut scene = Scene::new();
            tree.render_any(SceneBuilder::for_scene(&mut scene));
        }
    }

    pub fn run(self) {
        self.render_all();
        let event_loop = self.inner.borrow_mut().event_loop.take().unwrap();

        event_loop
            .run(|event, event_loop| match event {
                Event::UserEvent(user_event) => match user_event {
                    UserEvent::CreateWindow {
                        ui,
                        tree_key,
                        key,
                        window,
                    } => {
                        let mut builder = WindowBuilder::new();
                        if let Some(title) = window.title() {
                            builder = builder.with_title(title);
                        }
                        let window = builder.build(event_loop).unwrap();
                        ui.inner
                            .borrow_mut()
                            .windows
                            .insert(window.id(), (window, tree_key, key));
                    }
                },
                _ => {}
            })
            .unwrap();
    }
}
