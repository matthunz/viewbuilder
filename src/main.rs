pub mod node;
pub use node::Node;

pub mod tree;
use taffy::prelude::Size;
pub use tree::Tree;

use crate::node::NodeData;

pub struct Click {}

fn main() {
    let mut tree = Tree::default();

    let a = tree.insert(Node::text("Hello World!"));

    let b = Node::builder()
        .size(Size::from_points(100., 100.))
        .child(a)
        .build(&mut tree);

    let root = Node::builder()
        .size(Size::from_points(100., 100.))
        .child(b)
        .build(&mut tree);

    println!("{}", tree.display(root));
}
