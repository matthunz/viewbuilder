use super::LifecycleContext;
use crate::Element;
use std::borrow::Cow;
use vello::SceneBuilder;

pub struct WindowBuilder {
    window: Option<Window>,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            window: Some(Window::default()),
        }
    }
}

impl WindowBuilder {
    pub fn title(&mut self, title: impl Into<Cow<'static, str>>) -> &mut Self {
        self.window.as_mut().unwrap().title = Some(title.into());
        self
    }

    pub fn build(&mut self) -> Window {
        self.window.take().unwrap()
    }
}

#[derive(Clone, Default)]
pub struct Window {
    title: Option<Cow<'static, str>>,
}

impl Window {
    pub fn builder() -> WindowBuilder {
        WindowBuilder::default()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}

impl Element for Window {
    type Message = ();

    fn lifecycle(&mut self, cx: LifecycleContext, _lifecycle: super::Lifecycle) {
        cx.ui.insert_window(cx.tree_key, cx.key, self.clone())
    }

    fn handle(&mut self, _msg: Self::Message) {}

    fn render(&mut self, _scene: SceneBuilder) {}
}
