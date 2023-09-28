use skia_safe::Color4f;
use taffy::prelude::Rect;
use taffy::style::{FlexDirection, LengthPercentage};
use viewbuilder::{Context, Element, Error, NodeKey};

fn button(
    cx: &mut Context<i32>,
    label: &'static str,
    text_key: NodeKey,
    mut handler: impl FnMut(&mut i32) + 'static,
) -> NodeKey {
    Element::new()
        .on_click(Box::new(move |cx, _event| {
            handler(&mut cx.state);
            let content = cx.state.to_string();
            cx.node(text_key).set_text(content);
        }))
        .padding(Rect {
            left: LengthPercentage::Points(100.),
            right: LengthPercentage::Points(100.),
            top: LengthPercentage::Points(50.),
            bottom: LengthPercentage::Points(50.),
        })
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(cx.insert(label))
        .build(cx)
}

fn app(cx: &mut Context<i32>) -> NodeKey {
    let text = cx.insert("0");

    Element::new()
        .flex_direction(FlexDirection::Column)
        .child(Element::new().child(text).build(cx))
        .child(
            Element::new()
                .flex_direction(FlexDirection::Row)
                .child(button(cx, "More!", text, move |count| *count += 1))
                .child(button(cx, "Less!", text, move |count| *count -= 1))
                .build(cx),
        )
        .build(cx)
}

fn main() -> Result<(), Error> {
    viewbuilder::run(0, app)
}
