use taffy::style::{AlignItems, JustifyContent};
use viewbuilder::{window::Error, Context, Element, NodeKey};

fn app(cx: &mut Context) -> NodeKey {
    Element::new()
        .align_items(AlignItems::Center)
        .justify_content(JustifyContent::Center)
        .child(cx.insert("Hello World!"))
        .build(cx)
}

fn main() -> Result<(), Error> {
    viewbuilder::run((), app)
}
