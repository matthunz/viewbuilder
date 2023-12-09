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
use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    Renderer, RendererOptions, Scene, SceneBuilder,
};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy},
    window::{Window, WindowBuilder, WindowId},
};

struct RenderState {
    // TODO: We MUST drop the surface before the `window`, so the fields
    // must be in this order
    surface: RenderSurface,
    window: Window,
}

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
    pub(crate) trees: SlotMap<TreeKey, Rc<RefCell<dyn AnyElement>>>,
    windows: HashMap<WindowId, (TreeKey, DefaultKey)>,
    pub(crate) tx: mpsc::UnboundedSender<(TreeKey, DefaultKey, Box<dyn Any>)>,
    rx: mpsc::UnboundedReceiver<(TreeKey, DefaultKey, Box<dyn Any>)>,
    event_loop: Option<EventLoop<UserEvent>>,
    scene: Scene,
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
        let event_loop = EventLoopBuilder::with_user_event().build();
        let proxy = event_loop.create_proxy();
        Self {
            inner: Rc::new(RefCell::new(Inner {
                trees: SlotMap::default(),
                windows: HashMap::new(),
                event_loop: Some(event_loop),
                tx,
                rx,
                scene: Scene::new(),
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
            Rc::new(RefCell::new(tree))
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
        me.trees[tree_key]
            .borrow_mut()
            .handle_any(Box::new(TreeMessage::Handle { key, msg }));
        dirty.insert((tree_key, key));

        // Process any remaining events
        while let Ok((tree_key, key, msg)) = me.rx.try_recv() {
            me.trees[tree_key]
                .borrow_mut()
                .handle_any(Box::new(TreeMessage::Handle { key, msg }));
            dirty.insert((tree_key, key));
        }

        let mut dirty_trees = HashSet::new();
        for (tree_key, key) in dirty {
            let tree = me.trees.get_mut(tree_key).unwrap();
            tree.borrow_mut()
                .handle_any(Box::new(TreeMessage::Render { key }));

            dirty_trees.insert(tree_key);
        }

        drop(me);

        for tree_key in dirty_trees {
            let mut me = self.inner.borrow_mut();
            let tree = me.trees.get(tree_key).unwrap().clone();
            tree.borrow_mut()
                .handle_any(Box::new(TreeMessage::Render { key }));
            tree.borrow_mut()
                .render_any(SceneBuilder::for_scene(&mut me.scene));
        }
    }

    pub fn render_all(&self) {
        let me = self.inner.borrow_mut();
        let trees = me.trees.values().cloned().collect::<Vec<_>>();
        drop(me);

        for tree in trees {
            let mut me = self.inner.borrow_mut();
            tree.borrow_mut()
                .render_any(SceneBuilder::for_scene(&mut me.scene));
        }
    }

    pub fn run(self) {
        self.render_all();
        let event_loop = self.inner.borrow_mut().event_loop.take().unwrap();
        let mut render_cx = RenderContext::new().unwrap();
        let mut renderers: Vec<Option<Renderer>> = vec![];
        let mut render_states = HashMap::new();

        event_loop.run(move |event, event_loop, control_flow| {
            match event {
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
                            .insert(window.id(), (tree_key, key));

                        let size = window.inner_size();
                        let surface_future =
                            render_cx.create_surface(&window, size.width, size.height);
                        let surface =
                            pollster::block_on(surface_future).expect("Error creating surface");
                        render_states.insert(window.id(), {
                            let render_state = RenderState { surface, window };
                            renderers.resize_with(render_cx.devices.len(), || None);
                            let id = render_state.surface.dev_id;
                            renderers[id].get_or_insert_with(|| {
                                Renderer::new(
                                    &render_cx.devices[id].device,
                                    RendererOptions {
                                        surface_format: Some(render_state.surface.format),
                                        timestamp_period: render_cx.devices[id]
                                            .queue
                                            .get_timestamp_period(),
                                        antialiasing_support: vello::AaSupport::all(),
                                        use_cpu: false,
                                    },
                                )
                                .unwrap()
                            });
                            render_state
                        });

                        *control_flow = ControlFlow::Poll;
                    }
                },
                Event::RedrawRequested(_) => {
                    for render_state in render_states.values_mut() {
                        let width = render_state.surface.config.width;
                        let height = render_state.surface.config.height;

                        let device_handle = &render_cx.devices[render_state.surface.dev_id];
                        let surface_texture =
                            render_state.surface.surface.get_current_texture().unwrap();
                        let render_params = vello::RenderParams {
                            base_color: Color::PURPLE,
                            width,
                            height,
                            antialiasing_method: vello::AaConfig::Msaa16,
                        };

                        {
                            vello::block_on_wgpu(
                                &device_handle.device,
                                renderers[render_state.surface.dev_id]
                                    .as_mut()
                                    .unwrap()
                                    .render_to_surface_async(
                                        &device_handle.device,
                                        &device_handle.queue,
                                        &self.inner.borrow().scene,
                                        &surface_texture,
                                        &render_params,
                                    ),
                            )
                            .unwrap();
                        }

                        surface_texture.present();
                    }
                }
                _ => {}
            }

            *control_flow = ControlFlow::Poll;
        });
    }
}
