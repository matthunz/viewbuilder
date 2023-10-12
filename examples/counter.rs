use viewbuilder::prelude::*;

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    cx.render(rsx! {
        view {
            "{count}"
            view { onclick: move |_| count += 1, "Up high!" }
            view { onclick: move |_| count -= 1, "Down low!" }
        }
    })
}

fn main() {
    viewbuilder::run(App)
}
