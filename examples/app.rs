use viewbuilder::{Text, Tree};

fn main() {
    let mut tree = Tree::default();

    let mut sub_tree = Tree::default();
    let text = sub_tree.insert(Text::new("A"));

    let sub_tree_ref = tree.insert(sub_tree);
    dbg!(text.get(&sub_tree_ref.get(&tree)).content());
}
