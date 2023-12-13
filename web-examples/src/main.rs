use concoct::{Handle, Object, Slot};
use viewbuilder::web::{Element, Text};

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
    fn handle(&mut self, _cx: concoct::Handle<Self>, msg: Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        };
        self.text.send(self.value.to_string());
    }
}

fn counter_button(counter: &Handle<Counter>, label: &str, msg: Message) -> Handle<Element> {
    let button = Element::builder().child(Text::new(label)).build().start();
    button.map(&counter, move |_| msg);
    button
}

#[viewbuilder::main]
fn main() {
    let text = Text::new("0").start();

    let counter = Counter {
        value: 0,
        text: text.clone(),
    }
    .start();

    Element::builder()
        .child((
            text,
            counter_button(&counter, "Up high!", Message::Increment),
            counter_button(&counter, "Down low!", Message::Decrement),
        ))
        .build()
        .start();
}
