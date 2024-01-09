use viewbuilder::{
    native::{self, Native, Window},
    Model, View,
};

struct App;

impl Model<()> for App {
    fn handle(&mut self, _msg: ()) -> viewbuilder::ControlFlow {
        todo!()
    }
}

fn app(_model: &App) -> impl View<Native<()>, ()> {
    (Window::new(), Window::new())
}

fn main() {
    native::run(App, app)
}
