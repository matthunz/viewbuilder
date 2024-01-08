use std::{any::Any, cell::RefCell, mem};

use viewbuilder::{
    class, div,
    view::{self, once},
    Application, ControlFlow, Model, View, Web,
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

thread_local! {
    static APP: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
}

fn main() {
    let mut app = Application::new(AppModel, app, Web::default());
    app.build();

    APP.try_with(|cell| *cell.borrow_mut() = Some(Box::new(app)))
        .unwrap();
}
