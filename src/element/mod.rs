use crate::{TreeKey, UserInterface};
use slotmap::DefaultKey;
use vello::SceneBuilder;

mod text;
pub use text::{Text, TextMessage};

mod window;
pub use window::Window;

pub struct LifecycleContext {
    pub ui: UserInterface,
    pub tree_key: TreeKey,
    pub key: DefaultKey,
}

pub enum Lifecycle {
    Build,
}

pub trait Element {
    type Message;

    fn lifecycle(&mut self, cx: LifecycleContext, lifecycle: Lifecycle);

    fn handle(&mut self, msg: Self::Message);

    fn render(&mut self, scene: SceneBuilder);
}
