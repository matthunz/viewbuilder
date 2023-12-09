use kurbo::Size;
use slotmap::{DefaultKey, SlotMap};
use std::{cell::RefCell, collections::HashMap, marker::PhantomData, rc::Rc};
use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    Renderer, Scene,
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
    scene: Scene,
    event_loop: Option<EventLoop<()>>,
    render_cx: RenderContext,
    renderers: Vec<Option<Renderer>>,
    render_states: HashMap<WindowId, RenderState>,
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
            let me = &mut *self.inner.borrow_mut();

            match event {
                Event::RedrawRequested(_) => {
                    for render_state in me.render_states.values_mut() {
                        let width = render_state.surface.config.width;
                        let height = render_state.surface.config.height;

                        let _device_handle = &me.render_cx.devices[render_state.surface.dev_id];
                        let surface_texture =
                            render_state.surface.surface.get_current_texture().unwrap();
                        let _render_params = vello::RenderParams {
                            base_color: Color::PURPLE,
                            width,
                            height,
                            antialiasing_method: vello::AaConfig::Msaa16,
                        };

                        let root = &mut me.nodes[render_state.root];

                        let window_size = render_state.window.inner_size();
                        root.element.borrow_mut().as_element_mut().layout(
                            None,
                            Some(Size::new(window_size.width as _, window_size.height as _)),
                        );

                        /* TODO
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

                         */

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
