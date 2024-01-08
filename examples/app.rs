use viewbuilder::{
    class, div,
    view::{self, once},
    App, ControlFlow, Model, View, Web,
};

struct AppModel;

impl Model<()> for AppModel {
    fn handle(&mut self, _msg: ()) -> ControlFlow {
        ControlFlow::Pending
    }
}

fn app(_model: &AppModel) -> impl View<Web, ()> {
    div(
        view::once(class("parent")),
        div(view::once(class("child")), ()),
    )
}

fn main() {
    let mut app = App::new(AppModel, app, Web::default());
    app.build();
}
