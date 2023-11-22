use slotmap::DefaultKey;
use vello::SceneBuilder;

mod text;
pub use text::{Text, TextMessage};

mod window;
pub use window::Window;

use crate::TreeKey;

pub struct LifecycleContext {
    pub tree_key: TreeKey,
    pub key: DefaultKey,
}

pub enum Lifecycle {
    Build,
}

pub trait Element {
    type Message;

    fn lifecycle(&mut self, _cx: LifecycleContext, _lifecycle: Lifecycle) {}

    fn handle(&mut self, msg: Self::Message);

    fn render(&mut self, scene: SceneBuilder);
}
