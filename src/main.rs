use skia_safe::Color4f;
use taffy::prelude::Size;
use viewbuilder::render::Renderer;
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();

    let root = Element::builder()
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .background_color(Color4f::new(1., 0., 0., 1.))
                .on_mouse_in(Box::new(|_tree, _event| {
                    dbg!("in 1");
                }))
                .on_mouse_out(Box::new(|_tree, _event| {
                    dbg!("out 1");
                }))
                .build(&mut tree),
        )
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .background_color(Color4f::new(0., 1., 0., 1.))
                .on_mouse_in(Box::new(|_tree, _event| {
                    dbg!("in 2");
                }))
                .on_mouse_out(Box::new(|_tree, _event| {
                    dbg!("out 2");
                }))
                .build(&mut tree),
        )
        .build(&mut tree);

    let renderer = Renderer::new();
    renderer.run(tree, root)
}
