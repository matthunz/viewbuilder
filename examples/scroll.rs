use taffy::style::FlexDirection;
use viewbuilder::{node::Overflow, Context, Element, NodeKey};

fn app(cx: &mut Context) -> NodeKey {
    let mut count = 0;
    let children = std::iter::repeat_with(|| {
        let key = cx.insert(count.to_string());
        count += 1;
        key
    })
    .take(100);

    let mut elem = Element::new();
    elem.overflow_y(Overflow::Scroll)
        .flex_direction(FlexDirection::Column)
        .extend(children);
    elem.build(cx)
}

fn main() {
    viewbuilder::run(app)
}
