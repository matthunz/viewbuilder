use kurbo::{Point, Size};
use slotmap::{DefaultKey, SlotMap};
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    Renderer, Scene, SceneBuilder,
};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::WindowId,
};

pub mod element;
pub use self::element::{AnyElement, Element};

mod element_ref;
pub use element_ref::{ElementRef, Entry};

mod view;
pub use self::view::View;

mod window;
pub use self::window::Window;

pub struct Node {
    element: Rc<RefCell<dyn AnyElement>>,
}

struct RenderState {
    // TODO: We MUST drop the surface before the `window`, so the fields
    // must be in this order
    surface: RenderSurface,
    window: winit::window::Window,
    root: DefaultKey,
}

struct Inner {
    nodes: SlotMap<DefaultKey, Node>,
    scene: Rc<RefCell<Scene>>,
    event_loop: Option<EventLoop<()>>,
    render_cx: RenderContext,
    renderers: Vec<Option<Renderer>>,
    render_states: Rc<RefCell<HashMap<WindowId, RenderState>>>,
}

impl Default for Inner {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
            scene: Default::default(),
            event_loop: Some(EventLoop::new()),
            render_cx: RenderContext::new().unwrap(),
            renderers: Default::default(),
            render_states: Default::default(),
        }
    }
}

#[derive(Clone, Default)]
pub struct UserInterface {
    inner: Rc<RefCell<Inner>>,
}

impl UserInterface {
    pub fn current() -> Self {
        thread_local! {
            static CURRENT: UserInterface = UserInterface::default()
        }
        CURRENT.try_with(|ui| ui.clone()).unwrap()
    }

    pub fn view<E: Element + 'static>(&self, element: E) -> ElementRef<E> {
        let node = Node {
            element: Rc::new(RefCell::new(element)),
        };
        let key = self.inner.borrow_mut().nodes.insert(node);
        self.inner.borrow_mut().nodes[key]
            .element
            .borrow_mut()
            .as_element_mut()
            .build(key);

        ElementRef {
            key,
            _marker: PhantomData,
        }
    }

    pub fn get(&self, key: DefaultKey) -> Rc<RefCell<dyn AnyElement>> {
        self.inner.borrow_mut().nodes[key].element.clone()
    }

    pub fn run(self) {
        let event_loop = self.inner.borrow_mut().event_loop.take().unwrap();

        event_loop.run(move |event, _event_loop, control_flow| {
            match event {
                Event::RedrawRequested(_) => {
                    let render_states = self.inner.borrow().render_states.clone();
                    for render_state in render_states.borrow_mut().values_mut() {
                        let width = render_state.surface.config.width;
                        let height = render_state.surface.config.height;

                        let root = self.inner.borrow().nodes[render_state.root].element.clone();
                        let window_size = render_state.window.inner_size();
                        root.borrow_mut().as_element_mut().layout(
                            None,
                            Some(Size::new(window_size.width as _, window_size.height as _)),
                        );

                        let scene = self.inner.borrow().scene.clone();
                        root.borrow_mut().as_element_mut().render(
                            Point::ZERO,
                            Size::new(window_size.width as _, window_size.height as _),
                            &mut SceneBuilder::for_scene(&mut *scene.borrow_mut()),
                        );

                        let me = &mut *self.inner.borrow_mut();
                        let device_handle = &me.render_cx.devices[render_state.surface.dev_id];
                        let surface_texture =
                            render_state.surface.surface.get_current_texture().unwrap();
                        let render_params = vello::RenderParams {
                            base_color: Color::PURPLE,
                            width,
                            height,
                            antialiasing_method: vello::AaConfig::Msaa16,
                        };

                        vello::block_on_wgpu(
                            &device_handle.device,
                            me.renderers[render_state.surface.dev_id]
                                .as_mut()
                                .unwrap()
                                .render_to_surface_async(
                                    &device_handle.device,
                                    &device_handle.queue,
                                    &me.scene.borrow(),
                                    &surface_texture,
                                    &render_params,
                                ),
                        )
                        .unwrap();
                        surface_texture.present();
                    }
                }
                _ => {}
            }

            *control_flow = ControlFlow::Poll;
        });
    }
}

pub fn view<E: Element + 'static>(element: E) -> ElementRef<E> {
    UserInterface::current().view(element)
}

pub fn run() {
    UserInterface::current().run()
}

pub fn launch<E: Element + 'static>(element: E) {
    let ui = UserInterface::current();

    Window::new(element);

    ui.run()
}

pub struct Update {
    key: DefaultKey,
    layout: bool,
    render: bool,
}

impl Update {
    pub fn new(key: DefaultKey) -> Self {
        Self {
            key,
            layout: false,
            render: false,
        }
    }

    pub fn layout(mut self) -> Self {
        self.layout = true;
        self
    }

    pub fn render(mut self) -> Self {
        self.render = true;
        self
    }
}

impl Drop for Update {
    fn drop(&mut self) {
        todo!()
    }
}
