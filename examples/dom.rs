use dioxus::prelude::{render, use_future};
use viewbuilder::{prelude::*, virtual_tree::VirtualTree};

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    use_future(cx, (count,), |(count,)| async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count.set(*count + 1);
        }
    });

    render!( view { "Hello World!" } )
}

fn main() {
    let tree = VirtualTree::default();
    tree.run(app).unwrap();
}
