use dioxus::prelude::{render, use_future};
use viewbuilder::{layout::FlexDirection, prelude::*, virtual_dom};

fn app(cx: Scope) -> Element {
    let count = use_state(cx, || 0);

    use_future(cx, (count,), |(count,)| async move {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            count.set(*count + 1);
        }
    });

    render!(
        view {
            flex_direction: FlexDirection::Column,
            view {
                flex_direction: FlexDirection::Row,
                "Hello World!"
            }
        }
    )
}

fn main() {
    virtual_dom::run(app).unwrap();
}
