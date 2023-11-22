use crate::{Element, UserInterface};
use vello::SceneBuilder;

use super::LifecycleContext;

pub struct Window {
    ui: UserInterface,
}

impl Window {
    pub fn new(ui: &UserInterface) -> Self {
        Self { ui: ui.clone() }
    }
}

impl Element for Window {
    type Message = ();

    fn lifecycle(&mut self, cx: LifecycleContext, _lifecycle: super::Lifecycle) {
        self.ui.insert_window(cx.tree_key, cx.key)
    }

    fn handle(&mut self, _msg: Self::Message) {}

    fn render(&mut self, _scene: SceneBuilder) {}
}
