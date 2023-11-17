use viewbuilder::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! { text { font_size: 100., "Hello World!" } })
}

#[tokio::main]
async fn main() {
    viewbuilder::launch(app)
}
