use bumpalo::Bump;
use viewbuilder::{Tree, View};

fn app(bump: &Bump) -> impl View {
    &**bump.alloc("")
}

fn main() {
    let mut tree = Tree::new(app);
    tree.view();
}
