use skia_safe::Color4f;
use taffy::prelude::Size;
use taffy::style::FlexDirection;
use viewbuilder::render::Renderer;
use viewbuilder::{node::Element, Tree};

fn main() {
    let mut tree = Tree::default();

    let root = Element::builder()
        .child(
            Element::builder()
                .size(Size::from_points(100., 100.))
                .background_color(Color4f::new(1., 0., 0., 1.))
                .build(&mut tree),
        )
        .child(
            Element::builder()
                .flex_direction(FlexDirection::Column)
                .child(
                    Element::builder()
                        .size(Size::from_points(100., 100.))
                        .background_color(Color4f::new(1., 0., 0., 1.))
                        .build(&mut tree),
                )
                .child(
                    Element::builder()
                        .size(Size::from_points(100., 100.))
                        .background_color(Color4f::new(0., 1., 0., 1.))
                        .build(&mut tree),
                )
                .build(&mut tree),
        )
        .build(&mut tree);

    let renderer = Renderer::new();
    renderer.run(tree, root)
}
