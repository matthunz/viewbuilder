use super::Context;
use crate::object;
use kurbo::Point;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase},
    window::Theme,
};

pub struct WindowBuilder {
    window: Option<winit::window::WindowBuilder>,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            window: Some(Default::default()),
        }
    }
}

impl WindowBuilder {
    pub fn build(&mut self) -> Window {
        Window {
            window: self.window.take(),
        }
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.window = Some(self.window.take().unwrap().with_title(title));
        self
    }

    pub fn is_resizable(&mut self, is_resizable: bool) -> &mut Self {
        self.attr(|builder| builder.with_resizable(is_resizable))
    }

    pub fn is_active(&mut self, is_active: bool) -> &mut Self {
        self.attr(|builder| builder.with_active(is_active))
    }

    pub fn decorations(&mut self, decorations: bool) -> &mut Self {
        self.attr(|builder| builder.with_decorations(decorations))
    }

    pub fn theme(&mut self, theme: Option<Theme>) -> &mut Self {
        self.attr(|builder| builder.with_theme(theme))
    }

    fn attr(
        &mut self,
        f: impl FnOnce(winit::window::WindowBuilder) -> winit::window::WindowBuilder,
    ) -> &mut Self {
        self.window = Some(f(self.window.take().unwrap()));
        self
    }
}

/// Window on the native platform.
pub struct Window {
    window: Option<winit::window::WindowBuilder>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            window: Some(Default::default()),
        }
    }
}

#[object]
impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }

    fn start(&mut self, handle: Handle<Self>) {
        Context::current()
            .inner
            .borrow_mut()
            .pending_windows
            .push((self.window.take().unwrap(), handle.clone()));
    }

    /// Signal for the cursor movement event.
    fn cursor_moved(&self, point: Point);

    /// Signal for the mouse event.
    fn mouse_event(&self, state: ElementState, button: MouseButton);

    /// Signal for the mouse wheel event.
    fn mouse_wheel(&self, delta: MouseScrollDelta, phase: TouchPhase);

    /// Signal for the window resize event.
    fn resized(&self, size: PhysicalSize<u32>);

    /// Signal for the window focus event.
    fn focused(&self, is_focused: bool);

    /// Signal for the cursor enter event.
    fn cursor_entered(&self);

    /// Signal for the cursor leave event.
    fn cursor_left(&self);
}
