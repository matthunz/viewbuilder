use concoct::{Handle, Object, Slot};
use viewbuilder::native::{window, Window};

struct App;

impl Object for App {}

impl Slot<window::Resized> for App {
    fn handle(&mut self, _cx: Handle<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

#[viewbuilder::main]
fn main() {
    let app = App.start();

    let window = Window::new().start();
    window.bind(&app);
}
