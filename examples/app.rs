use viewbuilder::{LocalTree, Text, UserInterface};

#[tokio::main]
async fn main() {
    let mut ui = UserInterface::new();

    let tree = ui.insert(LocalTree::builder());
    let a = tree.get_mut(&mut ui).insert(Text::new("Window A"));
    ui.insert_window(tree.key, a.key);

    let sub_tree = ui.insert(LocalTree::builder());
    tree.get_mut(&mut ui).insert(sub_tree);

    let b = tree.get_mut(&mut ui).insert(Text::new("Window B"));
    ui.insert_window(sub_tree.key, b.key);

    ui.run();
}
