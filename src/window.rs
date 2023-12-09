use crate::{RenderState, UserInterface, View};
use std::borrow::Cow;
use vello::{Renderer, RendererOptions};

#[derive(Default)]
pub struct WindowBuilder {
    title: Option<Cow<'static, str>>,
}

impl WindowBuilder {
    pub fn title(&mut self, title: impl Into<Cow<'static, str>>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn build(&mut self, view: impl View) -> Window {
        let root = view.view();

        let current_ui = UserInterface::current();
        let ui = &mut *current_ui.inner.borrow_mut();
        if let Some(event_loop) = &ui.event_loop {
            let mut builder = winit::window::WindowBuilder::new();

            if let Some(title) = &self.title {
                builder = builder.with_title(&**title);
            }

            let window = builder.build(&event_loop).unwrap();

            let size = window.inner_size();
            let surface_future = ui
                .render_cx
                .create_surface(&window, size.width, size.height);
            let surface = pollster::block_on(surface_future).expect("Error creating surface");
            ui.render_states.borrow_mut().insert(window.id(), {
                let render_state = RenderState {
                    surface,
                    window,
                    root,
                };
                ui.renderers
                    .resize_with(ui.render_cx.devices.len(), || None);
                let id = render_state.surface.dev_id;
                ui.renderers[id].get_or_insert_with(|| {
                    Renderer::new(
                        &ui.render_cx.devices[id].device,
                        RendererOptions {
                            surface_format: Some(render_state.surface.format),
                            timestamp_period: ui.render_cx.devices[id].queue.get_timestamp_period(),
                            antialiasing_support: vello::AaSupport::all(),
                            use_cpu: false,
                        },
                    )
                    .unwrap()
                });
                render_state
            });
        } else {
            todo!()
        }

        Window {}
    }
}

pub struct Window {}

impl Window {
    pub fn new(view: impl View) -> Self {
        Self::builder().build(view)
    }

    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }
}
