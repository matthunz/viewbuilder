use viewbuilder::{
    element::{Text, Window},
    LocalTree, UserInterface,
};

#[tokio::main]
async fn main() {
    let ui = UserInterface::new();
    let tree = ui.insert(LocalTree::builder(Window::default()));

    let text = tree.insert(Text::new("Hello World!"));
    tree.root().push_child(text.key);

    ui.run();
}
