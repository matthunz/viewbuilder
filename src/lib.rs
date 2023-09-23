pub mod node;
pub use node::Node;

pub mod tree;
use slotmap::DefaultKey;

pub use tree::Tree;

pub mod render;

pub enum Event {
    Click(Click),
    MouseIn(MouseIn),
    MouseOut(MouseOut),
}

pub struct Click {
    pub target: DefaultKey,
}

pub struct MouseIn {
    pub target: DefaultKey,
}

pub struct MouseOut {
    pub target: DefaultKey,
}
