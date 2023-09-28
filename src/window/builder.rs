use super::{create_surface, Error};
use crate::{NodeKey, Renderer, Window};
use gl::types::GLint;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::{GlConfig, GlDisplay, NotCurrentGlContextSurfaceAccessor},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use kurbo::Size;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::gpu::gl::FramebufferInfo;
use std::{borrow::Cow, ffi::CString, num::NonZeroU32};
use winit::window::WindowBuilder;

/// Builder for a window.
pub struct Builder {
    size: Size,
    title: Cow<'static, str>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            size: Size::new(500., 500.),
            title: Cow::default(),
        }
    }
}

impl Builder {
    /// Set the size of the window.
    pub fn size(&mut self, size: Size) -> &mut Self {
        self.size = size;
        self
    }

    /// Set the title of the window.
    pub fn title(&mut self, title: impl Into<Cow<'static, str>>) -> &mut Self {
        self.title = title.into();
        self
    }

    /// Build the window with a renderer.
    pub fn build<T>(&self, renderer: &Renderer<T>, root: NodeKey) -> Result<Window, Error> {
        let winit_window_builder = WindowBuilder::new().with_title(self.title.as_ref());
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));
        let (window, gl_config) = display_builder
            .build(&renderer.event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .map_err(Error::Display)?;
        let window = window.ok_or(Error::Window)?;
        let raw_window_handle = window.raw_window_handle();

        // The context creation part. It can be created before surface and that's how
        // it's expected in multithreaded + multiwindow operation mode, since you
        // can send NotCurrentContext, but not Surface.
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                })?
        };

        let (width, height): (u32, u32) = window.inner_size().into();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)?
        };
        let gl_context = not_current_gl_context.make_current(&gl_surface)?;

        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });
        let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
        .ok_or(Error::Surface)?;

        let mut gr_context =
            skia_safe::gpu::DirectContext::new_gl(Some(interface), None).ok_or(Error::Surface)?;

        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: skia_safe::gpu::gl::Format::RGBA8.into(),
                ..Default::default()
            }
        };

        window.set_inner_size(winit::dpi::Size::new(winit::dpi::LogicalSize::new(
            self.size.width,
            self.size.height,
        )));

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;
        let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size)?;

        Ok(Window {
            surface,
            gl_surface,
            gr_context,
            gl_context,
            window,
            num_samples,
            stencil_size,
            fb_info,
            root,
            cursor_pos: None,
            hover_target: None,
            clicked: None,
        })
    }
}
