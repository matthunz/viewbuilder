use viewbuilder::{
    view,
    web::{self, html, Web},
    ControlFlow, Model, View,
};

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

fn view(model: &App) -> impl View<Web, Message> {
    (
        format!("High five count: {}", model.count),
        view::once(html::div(html::on_click(|| Message::Increment), "Up high!")),
        view::once(html::div(
            html::on_click(|| Message::Decrement),
            "Down low!",
        )),
    )
}

fn main() {
    web::run(App::default(), view)
}
