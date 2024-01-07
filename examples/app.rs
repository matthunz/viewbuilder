use viewbuilder::{view, View};

fn app() -> impl View<()> {
    view::from_fn(|_| ())
}

fn main() {}
