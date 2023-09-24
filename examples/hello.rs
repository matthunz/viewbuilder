use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{Context, Element};

fn main() {
    let mut tree = Context::default();
    let root = Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(tree.insert("Hello World!"))
        .build(&mut tree);

    viewbuilder::run(tree, root)
}
