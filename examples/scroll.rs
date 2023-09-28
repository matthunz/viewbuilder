use taffy::{
    prelude::Rect,
    style::{FlexDirection, LengthPercentage},
};
use viewbuilder::{element::Overflow, Context, Element, Error, NodeKey};

fn app(cx: &mut Context) -> NodeKey {
    let mut elem = Element::new();
    elem.overflow_y(Overflow::Scroll)
        .flex_direction(FlexDirection::Column)
        .extend((0..100).map(|count| {
            Element::new()
                .padding(Rect {
                    left: LengthPercentage::Points(100.),
                    right: LengthPercentage::Points(100.),
                    top: LengthPercentage::Points(50.),
                    bottom: LengthPercentage::Points(50.),
                })
                .child(cx.insert(count.to_string()))
                .build(cx)
        }));
    elem.build(cx)
}

fn main() -> Result<(), Error> {
    viewbuilder::run((), app)
}
