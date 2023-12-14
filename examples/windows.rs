use concoct::{Handle, Object, Slot};
use viewbuilder::native::{window, Window};

struct App;

impl Object for App {}

impl Slot<window::Resized> for App {
    fn update(&mut self, _cx: Handle<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

#[viewbuilder::main]
fn main() {
    let app = App.start();

    let window_a = Window::builder().title("Window A").build().start();
    window_a.bind(&app);

    let window_b = Window::builder().title("Window B").build().start();
    window_b.bind(&app);
}
