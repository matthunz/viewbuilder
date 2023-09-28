use crate::{
    event::{self},
    node::{NodeData, Overflow},
    Context, Error, NodeKey,
};
use glutin::{
    context::PossiblyCurrentContext,
    prelude::PossiblyCurrentContextGlSurfaceAccessor,
    surface::{GlSurface, WindowSurface},
};
use kurbo::{Point, Size};
use skia_safe::{
    gpu::{self, gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
    Color, ColorType, Surface,
};
use std::num::NonZeroU32;
use winit::{
    event::{ElementState, KeyboardInput, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

mod builder;
use self::builder::Builder;

// Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
// `DirectContext`.
//
// https://github.com/rust-skia/rust-skia/issues/476
/// Window renderer.
pub struct Window {
    surface: Surface,
    gl_surface: glutin::surface::Surface<WindowSurface>,
    gr_context: skia_safe::gpu::DirectContext,
    gl_context: PossiblyCurrentContext,
    pub(crate) window: winit::window::Window,
    num_samples: usize,
    stencil_size: usize,
    fb_info: FramebufferInfo,
    pub(crate) root: NodeKey,
    cursor_pos: Option<Point>,
    hover_target: Option<NodeKey>,
    clicked: Option<NodeKey>,
}

impl Window {
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Paint the user interface on to the window.
    pub fn paint<T>(&mut self, cx: &mut Context<T>) {
        self.gl_context.make_current(&self.gl_surface).unwrap();

        let canvas = self.surface.canvas();
        canvas.clear(Color::WHITE);

        // PAINT
        let window_size = self.window.inner_size();
        cx.layout(
            self.root,
            Size::new(window_size.width as _, window_size.height as _),
        );
        cx.paint(self.root, canvas);

        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }

    /// Handle a window event.
    pub fn handle<T>(
        &mut self,
        tree: &mut Context<T>,
        root: NodeKey,
        event: WindowEvent,
    ) -> Result<Option<ControlFlow>, Error> {
        // TODO
        #[allow(deprecated)]
        match event {
            WindowEvent::CloseRequested => {
                return Ok(Some(ControlFlow::Exit));
            }
            WindowEvent::Resized(physical_size) => {
                // Create a new render surface
                self.surface = create_surface(
                    &self.window,
                    self.fb_info,
                    &mut self.gr_context,
                    self.num_samples,
                    self.stencil_size,
                )?;

                // Resize the gl surface
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
                        return Ok(Some(ControlFlow::Exit));
                    }
                }

                self.window.request_redraw();
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
                ..
            } => {
                let pos = Point::new(position.x, position.y);
                self.cursor_pos = Some(pos);

                if let Some(target) = tree.tree.target(root, pos) {
                    if let Some(last_target) = self.hover_target {
                        if target != last_target {
                            self.hover_target = Some(target);

                            tree.send(
                                last_target,
                                crate::Event::MouseOut(event::MouseEvent {
                                    target: last_target,
                                    location: pos,
                                }),
                            );

                            tree.send(
                                target,
                                crate::Event::MouseIn(event::MouseEvent {
                                    target,
                                    location: pos,
                                }),
                            );
                        }
                    } else {
                        self.hover_target = Some(target);
                        tree.send(
                            target,
                            crate::Event::MouseIn(event::MouseEvent {
                                target,
                                location: pos,
                            }),
                        );
                    }
                } else if let Some(last_target) = self.hover_target {
                    self.hover_target = None;

                    tree.send(
                        last_target,
                        crate::Event::MouseOut(event::MouseEvent {
                            target: last_target,
                            location: pos,
                        }),
                    );
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button: _,
                ..
            } => match state {
                ElementState::Pressed => {
                    if let Some(pos) = self.cursor_pos {
                        if let Some(target) = tree.tree.target(root, Point::new(pos.x, pos.y)) {
                            self.clicked = Some(target);
                        }
                    }
                }
                ElementState::Released => {
                    if let (Some(pos), Some(clicked)) = (self.cursor_pos, self.clicked.take()) {
                        let pos = Point::new(pos.x, pos.y);
                        if let Some(target) = tree.tree.target(root, pos) {
                            if target == clicked {
                                tree.send(
                                    clicked,
                                    crate::Event::Click(event::MouseEvent {
                                        target: clicked,
                                        location: pos,
                                    }),
                                )
                            }
                        }
                    }
                }
            },
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase: _,
                ..
            } => {
                if let Some(pos) = self.cursor_pos {
                    if let MouseScrollDelta::PixelDelta(px_delta) = delta {
                        let pos = Point::new(pos.x, pos.y);
                        if let Some(target) =
                            tree.tree
                                .target_with_filter(root, Point::new(pos.x, pos.y), |node| {
                                    if let NodeData::Element(ref elem) = node.data {
                                        elem.overflow_y() == Some(Overflow::Scroll)
                                    } else {
                                        false
                                    }
                                })
                        {
                            let mut node = tree.node(target);
                            node.scroll(Size::new(px_delta.x, px_delta.y));
                        }
                    }
                }
            }
            _ => (),
        }

        Ok(None)
    }
}

fn create_surface(
    window: &winit::window::Window,
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Result<Surface, Error> {
    let size = window.inner_size();
    let size = (
        size.width.try_into().unwrap(),
        size.height.try_into().unwrap(),
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
    .ok_or_else(|| Error::Surface)
}
