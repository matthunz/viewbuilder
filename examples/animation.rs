use skia_safe::Color4f;
use taffy::prelude::Size;
use viewbuilder::{Context, Element, Renderer};

#[tokio::main]
async fn main() {
    let mut cx = Context::default();
    let animated = Element::new()
        .size(Size::from_points(100., 100.))
        .background_color(Color4f::new(0., 0., 1., 1.))
        .build(&mut cx);

    let root = Element::new()
        .size(Size::from_points(1000., 200.))
        .child(cx.insert("Hello"))
        .child(animated)
        .child(cx.insert("World"))
        .build(&mut cx);

    let renderer = Renderer::new();
    tokio::spawn(renderer.animation(animated, 0., 100., move |cx, size| {
        cx.node(animated)
            .set_size(Size::from_points(size as f32, size as f32))
    }));

    renderer.run(cx, root)
}
