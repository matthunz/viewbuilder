use viewbuilder::{
    element::{Text, Window},
    LocalTree, UserInterface,
};

#[tokio::main]
async fn main() {
    let ui = UserInterface::new();

    // Window A
    let tree = ui.insert(LocalTree::builder(
        Window::builder().title("Window A").build(),
    ));

    let a = tree.insert(Text::new("Window A"));
    tree.root().push_child(a.key);

    // Window B
    let sub_tree = ui.insert(LocalTree::builder(
        Window::builder().title("Window B").build(),
    ));
    tree.insert(sub_tree.tree.clone());

    let b = sub_tree.insert(Text::new("Window B"));
    sub_tree.root().push_child(b.key);

    // Window C
    let window_c = sub_tree.insert(Window::builder().title("Window C").build());
    sub_tree.root().push_child(window_c.key);

    let a = sub_tree.insert(Text::new("Window C"));
    window_c.push_child(a.key);

    ui.run();
}