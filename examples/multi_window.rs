use std::error::Error;
use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{Context, Element, NodeKey, Renderer, Window};

fn app(cx: &mut Context) -> NodeKey {
    Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(cx.insert("Hello World!"))
        .build(cx)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut renderer = Renderer::new();
    let mut cx = Context::new(());

    let a = Window::builder()
        .title("Window 1")
        .build(&mut renderer, app(&mut cx))?;
    let b = Window::builder()
        .title("Window 2")
        .build(&mut renderer, app(&mut cx))?;

    let cx_key = renderer.context(cx);
    renderer.insert_window(a, cx_key);
    renderer.insert_window(b, cx_key);

    renderer.run();
    Ok(())
}
