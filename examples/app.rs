use std::{
    sync::{Arc, Mutex},
    thread,
};
use viewbuilder::{UserInterface, View};

#[tokio::main]
async fn main() {
    let ui = UserInterface::new();

    ui.transaction(move |tx| {
        let child_ref = tx.insert(View::default());

        let mut root = View::default();
        root.with_child(child_ref.key);
        tx.insert(root);
    });

    viewbuilder::run(ui);
}
