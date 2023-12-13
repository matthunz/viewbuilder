use concoct::{Context, Handle, Object, Slot};
use viewbuilder::{view::Text, window, UserInterface, Window};

struct App {
    text: Handle<Text>,
}

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _handle: Context<Self>, msg: window::Resized) {
        self.text.send(msg.width.to_string().into())
    }
}

#[viewbuilder::main]
fn main() {
    let text = Text::default().spawn();

    let app = App { text: text.clone() }.spawn();

    let window = Window::new(text).spawn();
    window.bind(&app);
}
