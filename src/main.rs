pub mod element;
pub use element::Element;

pub mod tree;
use taffy::prelude::Size;
pub use tree::Tree;

use crate::element::ElementData;

pub struct Click {}

fn main() {
    let mut tree = Tree::default();

    let a = tree.insert(Element::text("Hello World!"));

    let b = Element::builder()
        .size(Size::from_points(100., 100.))
        .child(a)
        .build(&mut tree);

    let root = Element::builder()
        .size(Size::from_points(100., 100.))
        .child(b)
        .build(&mut tree);

    println!("{}", tree.display(root));
}
