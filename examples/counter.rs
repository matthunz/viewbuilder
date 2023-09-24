use skia_safe::Color4f;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use taffy::style::FlexDirection;
use viewbuilder::NodeKey;
use viewbuilder::{Context, Element};

fn button(
    cx: &mut Context,
    label: &'static str,
    mut handler: impl FnMut(&mut Context) + 'static,
) -> NodeKey {
    Element::new()
        .on_click(Box::new(move |cx, _event| handler(cx)))
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(cx.insert(label))
        .build(cx)
}

fn main() {
    let mut cx = Context::default();

    let inc_count = Rc::new(AtomicI64::new(0));
    let dec_count = inc_count.clone();

    let text = cx.insert("0");
    let root = Element::new()
        .flex_direction(FlexDirection::Column)
        .child(Element::new().child(text).build(&mut cx))
        .child(
            Element::new()
                .flex_direction(FlexDirection::Row)
                .child(button(&mut cx, "More!", move |cx| {
                    inc_count.fetch_add(1, Ordering::SeqCst);
                    cx.node(text)
                        .set_text(inc_count.load(Ordering::SeqCst).to_string())
                }))
                .child(button(&mut cx, "Less!", move |cx| {
                    dec_count.fetch_sub(1, Ordering::SeqCst);
                    cx.node(text)
                        .set_text(dec_count.load(Ordering::SeqCst).to_string())
                }))
                .build(&mut cx),
        )
        .build(&mut cx);

    viewbuilder::run(cx, root)
}
