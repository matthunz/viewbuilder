use viewbuilder::{prelude::*, layout::FlexDirection, virtual_tree::VirtualTree};

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        view {
            flex_direction: FlexDirection::RowReverse
        }
    })
}

fn main() {
    //viewbuilder::run(App)

    let mut vtree = VirtualTree::new(App);
    vtree.rebuild();

    println!("{}", vtree);
}
