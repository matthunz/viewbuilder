use viewbuilder::{prelude::*, virtual_tree::VirtualTree};

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        view { "Hello World!" }
    })
}

fn main() {
    let mut vtree = VirtualTree::new(App);
    vtree.rebuild();

    println!("{}", vtree);
}
