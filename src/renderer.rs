use vello::{
    peniko::Color,
    util::{RenderContext, RenderSurface},
    RenderParams, RendererOptions, Scene,
};
use winit::{event_loop::EventLoopWindowTarget, window::Window};

pub struct Renderer {
    window: Window,
    surface: Option<RenderSurface>,
    render_cx: RenderContext,
    renderers: Vec<Option<vello::Renderer>>,
}

impl Renderer {
    pub fn resume<T>(&mut self, event_loop: &EventLoopWindowTarget<T>) {
        let size = self.window.inner_size();
        let surface = pollster::block_on(self.render_cx.create_surface(
            &self.window,
            size.width,
            size.height,
        ))
        .unwrap();

        self.renderers
            .resize_with(self.render_cx.devices.len(), || None);

        let id = surface.dev_id;
        self.renderers[id].get_or_insert_with(|| {
            vello::Renderer::new(
                &self.render_cx.devices[id].device,
                RendererOptions {
                    surface_format: Some(surface.format),
                    use_cpu: false,
                    antialiasing_support: vello::AaSupport::all(),
                },
            )
            .unwrap()
        });

        self.surface = Some(surface);
    }

    pub fn render(&mut self, scene: &Scene) {
        let render_surface = self.surface.as_ref().unwrap();
        let surface_texture = render_surface.surface.get_current_texture().unwrap();

        let device_handle = &self.render_cx.devices[render_surface.dev_id];
        let size = self.window.inner_size();

        self.renderers[render_surface.dev_id]
            .as_mut()
            .unwrap()
            .render_to_surface(
                &device_handle.device,
                &device_handle.queue,
                scene,
                &surface_texture,
                &RenderParams {
                    base_color: Color::BLACK,
                    width: size.width,
                    height: size.height,
                    antialiasing_method: vello::AaConfig::Msaa16,
                },
            )
            .unwrap()
    }
}
