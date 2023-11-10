use viewbuilder::{layout::FlexDirection, prelude::*, Renderer};

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    use_future(cx, count, |count| async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count.set(*count + 1);
        }
    });

    render!(view { flex_direction: FlexDirection::Column, width: 100., height: 100., "{count}" })
}

#[tokio::main]
async fn main() {
    Renderer.run(app);
}
