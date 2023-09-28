use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{Context, Element, NodeKey, Renderer};

fn app(cx: &mut Context) -> NodeKey {
    Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(cx.insert("Hello World!"))
        .build(cx)
}

fn main() {
    let mut renderer = Renderer::new();

    {
        let mut cx = Context::new(());
        let node_key = app(&mut cx);
        let cx_key = renderer.context(cx);
        let window = renderer.window(node_key);
        renderer.insert_window(window, cx_key);
    }

    {
        let mut cx = Context::new(());
        let node_key = app(&mut cx);
        let cx_key = renderer.context(cx);
        let window = renderer.window(node_key);
        renderer.insert_window(window, cx_key);
    }

    renderer.run();
}
