use accesskit::{NodeBuilder, Role};
use viewbuilder::semantics::{node_factory, SemanticsTree};

fn main() {
    let mut tree = SemanticsTree::default();

    let button = node_factory::from_fn(|| NodeBuilder::new(Role::Button));
    tree.insert(Box::new(button));

    dbg!(tree.update());
}
