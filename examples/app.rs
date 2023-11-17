use viewbuilder::prelude::*;

fn app(cx: Scope) -> Element {
    let count = use_signal(cx, || 0);

    cx.render(rsx! { text { font_size: 100., onclick: move |_| *count.write() += 1, "{count}" } })
}

#[tokio::main]
async fn main() {
    viewbuilder::launch(app)
}
