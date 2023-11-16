use std::{
    sync::{Arc, Mutex},
    thread,
};
use viewbuilder::{UserInterface, View};

#[tokio::main]
async fn main() {
    let ui = UserInterface::new();

    let root_cell = Arc::new(Mutex::new(None));
    let root_cell_clone = root_cell.clone();

    ui.transaction(move |tx| {
        let child_ref = tx.insert(View::default());

        let mut root = View::default();
        root.with_child(child_ref.key);
        let root_ref = tx.insert(root);

        *root_cell_clone.lock().unwrap() = Some((root_ref, child_ref));
    });

    thread::spawn(move || {
        ui.transaction(move |tx| {
            let (root_ref, child_ref) = root_cell.lock().unwrap().unwrap();
            let root = root_ref.get_mut(tx).unwrap();
            root.remove_child(child_ref.key);
        });
    })
    .join()
    .unwrap();
}
