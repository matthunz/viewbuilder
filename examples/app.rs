use viewbuilder::{Text, TextMessage, UserInterface};

fn main() {
    let ui = UserInterface::default();
    let text = ui.insert(Text {});
    text.send(TextMessage::Set);

    ui.run();
}
