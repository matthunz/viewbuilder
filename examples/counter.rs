use skia_safe::Color4f;
use viewbuilder::ElementKey;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use taffy::style::FlexDirection;
use viewbuilder::{node::Element, Tree};

fn button(tree: &mut Tree, mut f: impl FnMut(&mut Tree) + 'static) -> ElementKey {
    Element::builder()
        .on_click(Box::new(move |tree, _event| f(tree)))
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(tree.insert("More!"))
        .build(tree)
}

fn main() {
    let mut tree = Tree::default();

    let inc_count = Rc::new(AtomicI64::new(0));
    let dec_count = inc_count.clone();

    let text = tree.insert("0");
    let root = Element::builder()
        .flex_direction(FlexDirection::Column)
        .child(Element::builder().child(text).build(&mut tree))
        .child(
            Element::builder()
                .flex_direction(FlexDirection::Row)
                .child(button(&mut tree, move |tree| {
                    inc_count.fetch_add(1, Ordering::SeqCst);
                    tree.set_text(text, inc_count.load(Ordering::SeqCst).to_string())
                }))
                .child(button(&mut tree, move |tree| {
                    dec_count.fetch_sub(1, Ordering::SeqCst);
                    tree.set_text(text, dec_count.load(Ordering::SeqCst).to_string())
                }))
                .build(&mut tree),
        )
        .build(&mut tree);

    viewbuilder::run(tree, root)
}
