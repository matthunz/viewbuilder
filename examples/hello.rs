use viewbuilder::Tree;

fn main() {
    let mut tree = Tree::default();
    let text = tree.insert("Hello World!");

    viewbuilder::run(tree, text)
}
