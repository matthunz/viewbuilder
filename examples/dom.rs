use viewbuilder::{layout::FlexDirection, prelude::*, virtual_tree::VirtualTree};

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    use_future(cx, count, |count| async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count.set(*count + 1);
        }
    });

    render!( view { flex_direction: FlexDirection::Column, "{count}" } )
}

#[tokio::main]
async fn main() {
    let mut tree = VirtualTree::new(app);
    loop {
        tree.run().await.unwrap();
    }
}
