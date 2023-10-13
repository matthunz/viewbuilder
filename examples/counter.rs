use viewbuilder::{layout::FlexDirection, prelude::*, virtual_tree::VirtualTree};

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        view {
            flex_direction: FlexDirection::Column,
            "High five count: {count}",
            view {
                flex_direction: FlexDirection::Row,
                view { onclick: move |_| count += 1, "Up high!" },
                view { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    })
}

fn main() {
    let vtree = VirtualTree::new(App);

    println!("{}", vtree);
}
