use crate::{event, Context, NodeKey};
use gl::types::*;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{
        ContextApi, ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor,
        PossiblyCurrentContext,
    },
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use kurbo::Point;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    Color, ColorType, Surface,
};
use std::{
    ffi::CString,
    future::Future,
    num::NonZeroU32,
    sync::{mpsc, Arc},
    time::{Duration, Instant},
};
use tokio::sync::Notify;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopBuilder},
    window::{Window, WindowBuilder},
};

pub struct UserEvent(pub Box<dyn FnOnce(&mut Context) + Send>);

// Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
// `DirectContext`.
//
// https://github.com/rust-skia/rust-skia/issues/476
pub struct Renderer {
    surface: Surface,
    gl_surface: GlutinSurface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    window: Window,
    event_loop: EventLoop<UserEvent>,
    num_samples: usize,
    stencil_size: usize,
    fb_info: FramebufferInfo,
    pub tx: mpsc::Sender<UserEvent>,
    rx: mpsc::Receiver<UserEvent>,
    pub notify: Arc<Notify>,
}

impl Renderer {
    pub fn new() -> Self {
        let el = EventLoopBuilder::with_user_event().build();
        let winit_window_builder = WindowBuilder::new().with_title("Viewbuilder");

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));
        let (window, gl_config) = display_builder
            .build(&el, template, |configs| {
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
            .unwrap();
        let window = window.expect("Could not create window with OpenGL context");
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
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
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
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

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
        .expect("Could not create interface");

        let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");

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
            1024.0, 1024.0,
        )));

        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;

        let surface = create_surface(&window, fb_info, &mut gr_context, num_samples, stencil_size);

        let (tx, rx) = mpsc::channel();

        Self {
            surface,
            gl_surface,
            gl_context,
            gr_context,
            window,
            event_loop: el,
            num_samples,
            stencil_size,
            fb_info,
            tx,
            rx,
            notify: Arc::new(Notify::default()),
        }
    }

    pub fn animation(
        &self,
        _key: NodeKey,
        min: f32,
        max: f32,
        f: impl Fn(&mut Context, f32) + Send + Sync + 'static,
    ) -> impl Future<Output = ()> {
        let tx = self.tx.clone();
        let notify = self.notify.clone();

        let mut is_forward = true;
        let mut start = Instant::now();

        async move {
            let f = Arc::new(f);
            loop {
                let elapsed = Instant::now() - start;
                let millis = elapsed.as_millis() as f32;

                let (begin, end) = if is_forward { (min, max) } else { (max, min) };
                let interpolated: f32 = interpolation::lerp(&begin, &end, &(millis / max));
                let size = interpolated.min(max).max(min);

                if elapsed >= Duration::from_secs(1) {
                    start = Instant::now();
                    is_forward = !is_forward;
                }

                let f2 = f.clone();
                tx.send(UserEvent(Box::new(move |cx| f2(cx, size))))
                    .unwrap();

                notify.notified().await;
            }
        }
    }

    pub fn run(mut self, mut tree: Context, root: NodeKey) {
        let mut previous_frame_start = Instant::now();

        let mut hover_target = None;
        let mut cursor_pos = None;
        let mut clicked = None;

        let proxy = self.event_loop.create_proxy();

        self.event_loop.run(move |event, _, control_flow| {
            let frame_start = Instant::now();
            let mut draw_frame = false;

            if let Ok(event) = self.rx.try_recv() {
                proxy.send_event(event).ok().unwrap();
            }

            #[allow(deprecated)]
            match event {
                Event::LoopDestroyed => {}
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.surface = create_surface(
                            &self.window,
                            self.fb_info,
                            &mut self.gr_context,
                            self.num_samples,
                            self.stencil_size,
                        );
                        /* First resize the opengl drawable */
                        let (width, height): (u32, u32) = physical_size.into();

                        self.gl_surface.resize(
                            &self.gl_context,
                            NonZeroU32::new(width.max(1)).unwrap(),
                            NonZeroU32::new(height.max(1)).unwrap(),
                        );
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode,
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        if modifiers.logo() {
                            if let Some(VirtualKeyCode::Q) = virtual_keycode {
                                *control_flow = ControlFlow::Exit;
                            }
                        }

                        self.window.request_redraw();
                    }
                    WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        modifiers: _,
                    } => {
                        cursor_pos = Some(position);

                        if let Some(target) =
                            tree.tree.target(root, Point::new(position.x, position.y))
                        {
                            if let Some(last_target) = hover_target {
                                if target != last_target {
                                    hover_target = Some(target);

                                    tree.send(
                                        last_target,
                                        crate::Event::MouseOut(event::MouseOut {
                                            target: last_target,
                                        }),
                                    );

                                    tree.send(
                                        target,
                                        crate::Event::MouseIn(event::MouseIn { target }),
                                    );
                                }
                            } else {
                                hover_target = Some(target);
                                tree.send(target, crate::Event::MouseIn(event::MouseIn { target }));
                            }
                        } else if let Some(last_target) = hover_target {
                            hover_target = None;

                            tree.send(
                                last_target,
                                crate::Event::MouseOut(event::MouseOut {
                                    target: last_target,
                                }),
                            );
                        }
                    }
                    WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button: _,
                        modifiers: _,
                    } => match state {
                        ElementState::Pressed => {
                            if let Some(pos) = cursor_pos {
                                if let Some(target) =
                                    tree.tree.target(root, Point::new(pos.x, pos.y))
                                {
                                    clicked = Some(target);
                                }
                            }
                        }
                        ElementState::Released => {
                            if let Some(pos) = cursor_pos {
                                if let Some(clicked) = clicked.take() {
                                    if let Some(target) =
                                        tree.tree.target(root, Point::new(pos.x, pos.y))
                                    {
                                        if target == clicked {
                                            tree.send(
                                                clicked,
                                                crate::Event::Click(event::Click {
                                                    target: clicked,
                                                }),
                                            )
                                        }
                                    }
                                }
                            }
                        }
                    },
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    draw_frame = true;
                }
                Event::UserEvent(UserEvent(update)) => update(&mut tree),
                _ => (),
            }

            let frame_duration = Duration::from_millis(10);
            if frame_start - previous_frame_start > frame_duration {
                draw_frame = true;
                previous_frame_start = frame_start;
            }
            if draw_frame {
                self.notify.notify_waiters();

                let canvas = self.surface.canvas();
                canvas.clear(Color::WHITE);

                // PAINT
                tree.layout(root);
                tree.paint(root, canvas);

                self.gr_context.flush_and_submit();
                self.gl_surface.swap_buffers(&self.gl_context).unwrap();
            }

            *control_flow = ControlFlow::WaitUntil(previous_frame_start + frame_duration)
        });
    }
}

fn create_surface(
    window: &Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let size = window.inner_size();
    let size = (
        size.width.try_into().expect("Could not convert width"),
        size.height.try_into().expect("Could not convert height"),
    );
    let backend_render_target =
        BackendRenderTarget::new_gl(size, num_samples, stencil_size, fb_info);

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
    .expect("Could not create skia surface")
}
