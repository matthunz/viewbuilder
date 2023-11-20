use bumpalo::Bump;
use viewbuilder::{fmt, Tree, View};

enum Message {
    Increment,
    Decrement,
}

fn app<'a>(bump: &'a Bump, count: &mut i32) -> impl View<'a, Message> {
    fmt!(bump, "High five count: {}", *count)
}

fn main() {
    let mut tree = Tree::new(0, app, |count: &mut i32, msg| match msg {
        Message::Increment => *count += 1,
        Message::Decrement => *count -= 1,
    });
    tree.view();
    tree.handle(Message::Increment);
    tree.view();
}
