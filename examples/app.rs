use viewbuilder::{LocalTree, Text, UserInterface};

fn main() {
    let mut ui = UserInterface::new();

    let sub_tree = ui.insert(LocalTree::builder());
    let text = sub_tree.get_mut(&mut ui).insert(Text::new("A"));
    dbg!(text.get(sub_tree.get_mut(&mut ui)).content());

    let tree = ui.insert(LocalTree::builder());
    tree.get_mut(&mut ui).insert(sub_tree);

    Text::set_content(text, sub_tree.get_mut(&mut ui), "B");
    dbg!(text.get(sub_tree.get_mut(&mut ui)).content());
}
