use slotmap::DefaultKey;

pub mod node;
pub use node::Node;

pub mod tree;
pub use tree::{NodeRef, Tree};

pub mod element;
pub use element::Element;

mod render;
pub use render::Renderer;

pub mod event;
pub use event::Event;

pub type ElementKey = slotmap::DefaultKey;

pub fn run(tree: Tree, root: DefaultKey) {
    let renderer = Renderer::new();
    renderer.run(tree, root)
}
