use bumpalo::Bump;
use viewbuilder::{App, View};

fn app<'a>(_bump: &'a Bump, _count: &mut ()) -> impl View<'a, ()> {
    "Hello World!"
}

fn main() {
    let mut app = App::new((), app, |_count: &mut (), _| {});
    app.run();
}
