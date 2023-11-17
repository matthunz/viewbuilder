use viewbuilder::prelude::*;

fn app(cx: Scope) -> Element {
    cx.render(rsx! { text { "Hello World!" } })
}

#[tokio::main]
async fn main() {
    viewbuilder::launch(app)
}
