use bumpalo::Bump;
use viewbuilder::{format_in, Component, LinearLayout, Text, View};

enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    count: i32,
}

impl Component for Counter {
    type Message = Message;

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Message::Increment => self.count += 1,
            Message::Decrement => self.count -= 1,
        }
    }

    fn view<'a>(&mut self, bump: &'a Bump) -> impl View<'a, Self::Message> {
        dbg!(self.count);

        LinearLayout::new((
            format_in!(bump, "High five count: {}", self.count),
            LinearLayout::new((
                Text::new("Up high!").on_click(|_| Message::Increment),
                Text::new("Down low!"),
            )),
        ))
    }
}

fn main() {
    viewbuilder::run(Counter::default())
}
