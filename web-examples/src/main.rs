use concoct::{Handle, Object, Slot};
use viewbuilder::web::{self, Element, Text};

#[derive(Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

struct Counter {
    value: i32,
    text: Handle<Text>,
}

impl Object for Counter {}

impl Slot<Message> for Counter {
    fn handle(&mut self, _cx: concoct::Context<Self>, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        };
        self.text.send(self.value.to_string());
    }
}

struct CounterButton {
    msg: Message,
    counter: Handle<Counter>,
}

impl Object for CounterButton {}

impl Slot<web::MouseEvent> for CounterButton {
    fn handle(&mut self, _cx: concoct::Context<Self>, _msg: web::MouseEvent) {
        self.counter.send(self.msg);
    }
}

#[viewbuilder::main]
fn main() {
    let text = Text::new("0").spawn();

    let counter = Counter {
        value: 0,
        text: text.clone(),
    }
    .spawn();

    let increment_button = CounterButton {
        msg: Message::Increment,
        counter: counter.clone(),
    }
    .spawn();
    let decrement_button = CounterButton {
        msg: Message::Decrement,
        counter,
    }
    .spawn();

    Element::builder()
        .child((
            text,
            Element::builder()
                .on_click(increment_button)
                .child(Text::new("Up High!"))
                .build(),
            Element::builder()
                .on_click(decrement_button)
                .child(Text::new("Down Low!"))
                .build(),
        ))
        .build()
        .spawn();
}
