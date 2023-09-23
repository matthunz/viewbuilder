use std::sync::atomic::{AtomicI64, Ordering};

use skia_safe::Color4f;
use taffy::prelude::Size;
use viewbuilder::render::Renderer;
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();

    let mut count = AtomicI64::new(0);

    let text = tree.insert("0");

    let mut root = Element::builder()
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .child(text)
                .build(&mut tree),
        )
        .child(
            Element::builder()
                .on_mouse_in(Box::new(move |tree, event| {
                    count.fetch_add(1, Ordering::SeqCst);
                    tree.set_text(text, count.load(Ordering::SeqCst).to_string())
                }))
                .size(Size::from_points(100., 100.))
                .background_color(Color4f::new(1., 1., 0., 1.))
                .child(tree.insert("More!"))
                .build(&mut tree),
        )
        .build(&mut tree);

    let renderer = Renderer::new();
    renderer.run(tree, root)
}
