use viewbuilder::{layout::FlexDirection, prelude::*, virtual_tree::VirtualTree};

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        view {
            flex_direction: FlexDirection::Column,
            "A",
            "B"
        }
    })
}

fn main() {
    viewbuilder::run(App);

    let mut vtree = VirtualTree::new(App);
    vtree.rebuild();

    println!("{}", vtree);
}
