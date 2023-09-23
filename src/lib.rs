pub mod node;
pub use node::Node;

pub mod tree;

use slotmap::DefaultKey;
pub use tree::Tree;

mod render;
pub use render::Renderer;

pub mod event;
pub use event::Event;

pub fn run(tree: Tree, root: DefaultKey) {
    let renderer = Renderer::new();
    renderer.run(tree, root)
}
