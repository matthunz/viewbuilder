use viewbuilder::prelude::*;

fn app(cx: Scope) -> Element {
    let count = use_signal(cx, || 0);

    cx.render(rsx! {
        view {
            text { font_size: 100., "{count}" }
            view {
                text { font_size: 50., onclick: move |_| *count.write() += 1, "More" }
                text { font_size: 50., onclick: move |_| *count.write() -= 1, "Less" }
            }
        }
    })
}

#[tokio::main]
async fn main() {
    viewbuilder::launch(app)
}
