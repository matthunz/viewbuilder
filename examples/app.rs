use viewbuilder::{Element, Text, TextMessage, UserInterface};

fn main() {
    let text = Text::new("A").spawn();
    text.send(TextMessage::Set);

    UserInterface::current().run();
}
