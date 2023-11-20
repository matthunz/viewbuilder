use bumpalo::Bump;
use viewbuilder::View;

fn app<'a>(_bump: &'a Bump, _count: &mut ()) -> impl View<'a, ()> {
    "Hello World!"
}

fn main() {
    viewbuilder::run((), app, |_count: &mut (), _| {});
}
