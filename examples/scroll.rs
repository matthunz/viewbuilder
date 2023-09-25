use taffy::style::FlexDirection;
use viewbuilder::{node::Overflow, Context, Element, NodeKey};

fn app(cx: &mut Context) -> NodeKey {
    let mut elem = Element::new();
    elem.overflow_y(Overflow::Scroll)
        .flex_direction(FlexDirection::Column)
        .extend((0..100).map(|count| cx.insert(count.to_string())));
    elem.build(cx)
}

fn main() {
    viewbuilder::run(app)
}
