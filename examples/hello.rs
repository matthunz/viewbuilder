use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{Context, Element, NodeKey};

fn app(cx: &mut Context) -> NodeKey {
    Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(cx.insert("Hello World!"))
        .build(cx)
}

fn main() {
    viewbuilder::run((), app)
}
