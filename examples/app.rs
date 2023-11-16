use viewbuilder::UserInterface;

fn main() {
    let ui = UserInterface::new();

    ui.transaction(|tx| {
        let a = tx.insert(());

        a.get(tx);
    });
}
