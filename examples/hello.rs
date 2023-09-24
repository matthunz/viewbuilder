use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();
    let root = Element::builder()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(tree.insert("Hello World!"))
        .build(&mut tree);

    viewbuilder::run(tree, root)
}
