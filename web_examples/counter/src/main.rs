use std::{any::Any, cell::RefCell, rc::Rc, sync::Arc};
use viewbuilder::{view, web::html, Application, ControlFlow, Model, View, Web};

enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct App {
    count: i32,
}

impl Model<Message> for App {
    fn handle(&mut self, msg: Message) -> ControlFlow {
        match msg {
            Message::Decrement => self.count -= 1,
            Message::Increment => self.count += 1,
        }
        ControlFlow::Rebuild
    }
}

fn app(model: &App) -> impl View<Web, Message> {
    (
        format!("High five count: {}", model.count),
        view::once(html::div(html::on_click(|| Message::Increment), "Up high!")),
        view::once(html::div(
            html::on_click(|| Message::Decrement),
            "Down low!",
        )),
    )
}

thread_local! {
    static APP: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
}

fn main() {
    let cell = Rc::new(RefCell::new(None::<Application<_, _, _, _, _>>));
    let cell_clone = cell.clone();
    let mut app = Application::new(
        Arc::new(move |msg| {
            let mut g = cell_clone.borrow_mut();
            let app = g.as_mut().unwrap();
            if let ControlFlow::Rebuild = app.handle(msg) {
                app.rebuild();
            }
        }),
        App::default(),
        app,
        Web::default(),
    );
    app.build();

    *cell.borrow_mut() = Some(app);
}
