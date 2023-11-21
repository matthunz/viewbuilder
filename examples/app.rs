use viewbuilder::{Text, Tree};

fn main() {
    let mut tree = Tree::default();

    let text = tree.insert(Text::new("A"));
    dbg!(text.get(&tree).content());
    
    Text::set_content(text, &mut tree, "B");
    dbg!(text.get(&tree).content());
}
