use concoct::{Handler, Object};
use viewbuilder::{window, UserInterface, Window};

struct App;

impl Object for App {}

impl Handler<window::Resized> for App {
    fn handle(&mut self, _handle: concoct::Context<Self>, msg: window::Resized) {
        dbg!(msg);
    }
}

fn main() {
    let ui = UserInterface::default();
    let _guard = ui.enter();

    let window = Window::default().spawn();
    let app = App.spawn();
    window.bind(&app);

    ui.run()
}
