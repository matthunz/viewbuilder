use taffy::prelude::Size;
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();

    Element::builder()
        .size(Size::from_points(100., 100.))
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .on_click(Box::new(move |tree, click| {
                    tree.set_text(click.target, "New!")
                }))
                .child(tree.insert("Hello World!"))
                .build(&mut tree),
        )
        .build(&mut tree);

    dbg!(tree.semantics());
}
