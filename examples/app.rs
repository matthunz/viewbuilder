use bumpalo::Bump;
use viewbuilder::{format_in, LinearLayout, Text, View};

enum Message {
    Increment,
    Decrement,
}

fn app<'a>(bump: &'a Bump, count: &mut i32) -> impl View<'a, Message> {
    LinearLayout::new((
        format_in!(bump, "High five count: {}", *count),
        LinearLayout::new((
            Text::new("Up high!").on_click(Message::Increment),
            Text::new("Down low!").on_click(Message::Decrement),
        )),
    ))
}

fn main() {
    viewbuilder::run(0, app, |count: &mut i32, msg| match msg {
        Message::Increment => *count += 1,
        Message::Decrement => *count -= 1,
    })
}
