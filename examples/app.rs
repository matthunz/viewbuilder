use viewbuilder::{
    element::{LinearLayout, Text, TextMessage, Window},
    Element, UserInterface,
};

fn main() {
    let label = Text::new("0").spawn();

    let _window = Window::new(LinearLayout::new((
        label,
        Text::new("Up High!"),
        Text::new("Down Low!"),
    )))
    .spawn();

    label.send(TextMessage::Set);

    UserInterface::current().run();
}
