use skia_safe::Color4f;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use taffy::style::FlexDirection;
use viewbuilder::NodeKey;
use viewbuilder::{Element, Tree};

fn button(
    tree: &mut Tree,
    label: &'static str,
    mut handler: impl FnMut(&mut Tree) + 'static,
) -> NodeKey {
    Element::new()
        .on_click(Box::new(move |tree, _event| handler(tree)))
        .background_color(Color4f::new(1., 1., 0., 1.))
        .child(tree.insert(label))
        .build(tree)
}

fn main() {
    let mut tree = Tree::default();

    let inc_count = Rc::new(AtomicI64::new(0));
    let dec_count = inc_count.clone();

    let text = tree.insert("0");
    let root = Element::new()
        .flex_direction(FlexDirection::Column)
        .child(Element::new().child(text).build(&mut tree))
        .child(
            Element::new()
                .flex_direction(FlexDirection::Row)
                .child(button(&mut tree, "More!", move |tree| {
                    inc_count.fetch_add(1, Ordering::SeqCst);
                    tree.set_text(text, inc_count.load(Ordering::SeqCst).to_string())
                }))
                .child(button(&mut tree, "Less!", move |tree| {
                    dec_count.fetch_sub(1, Ordering::SeqCst);
                    tree.set_text(text, dec_count.load(Ordering::SeqCst).to_string())
                }))
                .build(&mut tree),
        )
        .build(&mut tree);

    viewbuilder::run(tree, root)
}
