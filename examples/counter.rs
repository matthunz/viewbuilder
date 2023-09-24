use skia_safe::Color4f;
use std::rc::Rc;
use std::sync::atomic::{AtomicI64, Ordering};
use taffy::style::FlexDirection;
use viewbuilder::{node::Element, Tree};

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
                .child(
                    Element::builder()
                        .on_click(Box::new(move |tree, _event| {
                            inc_count.fetch_add(1, Ordering::SeqCst);
                            tree.set_text(text, inc_count.load(Ordering::SeqCst).to_string())
                        }))
                        .background_color(Color4f::new(1., 1., 0., 1.))
                        .child(tree.insert("More!"))
                        .build(&mut tree),
                )
                .child(
                    Element::builder()
                        .on_click(Box::new(move |tree, _event| {
                            dec_count.fetch_sub(1, Ordering::SeqCst);
                            tree.set_text(text, dec_count.load(Ordering::SeqCst).to_string())
                        }))
                        .background_color(Color4f::new(1., 1., 0., 1.))
                        .child(tree.insert("Less!"))
                        .build(&mut tree),
                )
                .build(&mut tree),
        )
        .build(&mut tree);

    viewbuilder::run(tree, root)
}
