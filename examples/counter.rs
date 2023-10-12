use viewbuilder::prelude::*;
use viewbuilder::virtual_tree::VirtualTree;

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    cx.render(rsx!("Hello World!"))
}

fn main() {
    viewbuilder::run(App)
}
