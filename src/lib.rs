pub mod node;
pub use node::Node;

pub mod tree;
use slotmap::DefaultKey;

pub use tree::Tree;

pub enum Event {
    Click(Click),
}

pub struct Click {
    pub target: DefaultKey,
}
