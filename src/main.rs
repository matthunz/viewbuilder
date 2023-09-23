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

    let root = Element::builder().size(Size::from_points(100., 100.)).child(a).build(&mut tree);

    tree.display(root)
}
