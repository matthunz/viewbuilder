use skia_safe::Color4f;
use taffy::prelude::Size;
use viewbuilder::{Context, Element, Error, Renderer, Window};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut cx = Context::new(());
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

    let mut renderer = Renderer::default();
    let cx_key = renderer.insert_context(cx);
    let window = Window::builder().build(&renderer, root).unwrap();
    renderer.insert_window(window, cx_key);

    tokio::spawn(renderer.animation(0., 100., move |cx, size| {
        cx.node(animated)
            .set_size(Size::from_points(size as f32, size as f32))
    }));

    renderer.run()
}
