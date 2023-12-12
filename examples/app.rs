use concoct::{Handler, Object};
use viewbuilder::{
    view::{LinearLayout, Text, View},
    window, UserInterface, Window,
};

fn app() -> impl View {
    let text = Text {}.spawn();
    LinearLayout::new((text, Text {}))
}

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
