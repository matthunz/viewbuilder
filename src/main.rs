pub mod element;
pub use element::Element;

pub mod tree;
pub use tree::Tree;

use crate::element::ElementData;

pub struct Click {}

fn main() {
    let mut tree = Tree::default();

    let a = tree.insert(Element::text("Hello World!"));

    let root = Element::builder().child(a).build(&mut tree);

    dbg!(tree.display(root));
}
