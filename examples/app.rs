use viewbuilder::{Element, Tree};

struct Text;

impl Element for Text {}

fn main() {
    let mut tree = Tree::default();
    let root = tree.insert(Text);

    dbg!(tree.slice(root));
}
