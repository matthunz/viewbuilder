use taffy::prelude::Size;
use viewbuilder::render::Renderer;
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();

    let root = Element::builder()
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .build(&mut tree),
        )
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .build(&mut tree),
        )
        .build(&mut tree);

    let renderer = Renderer::new();
    renderer.run(tree, root)
}
