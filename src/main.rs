pub mod node;
pub use node::Node;

pub mod tree;
use taffy::prelude::Size;
pub use tree::Tree;

use crate::node::{Element, NodeData};

pub enum Event {
    Click(Click),
}

pub struct Click {}

fn main() {
    let mut tree = Tree::default();

    let a = tree.insert("Hello World!");

    let b = Element::builder()
        .size(Size::from_points(100., 100.))
        .on_click(Box::new(move |tree, _| tree.set_text(a, "New!")))
        .child(a)
        .build(&mut tree);

    let root = Element::builder()
        .size(Size::from_points(100., 100.))
        .child(b)
        .build(&mut tree);

    println!("{}", tree.display(root));

    tree.send(b, Event::Click(Click {}));
    println!("{}", tree.display(root));
}
