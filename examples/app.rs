use dioxus::prelude::*;
use skia_safe::Color4f;
use viewbuilder::element::View;

fn app(cx: Scope) -> Element {
    render!("")
}

#[tokio::main]
async fn main() {
    viewbuilder::launch(app)
}
