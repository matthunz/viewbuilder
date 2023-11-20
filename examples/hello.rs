use bumpalo::Bump;
use viewbuilder::{format_in, App, LinearLayout, Text, View};

fn app<'a>(bump: &'a Bump, count: &mut ()) -> impl View<'a, ()> {
    "Hello World!"
}

fn main() {
    let mut app = App::new((), app, |count: &mut (), _| {});
    app.run();
}
