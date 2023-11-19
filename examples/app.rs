use viewbuilder::Object;

enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Counter {
    value: i32,
}

impl Object for Counter {
    type Message = Message;

    fn update(&mut self, msg: &Self::Message) {
        match msg {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
        dbg!(self.value);
    }
}

#[tokio::main]
async fn main() {
    let a = viewbuilder::spawn(Counter::default());
   
    a.listen(|msg | {
        dbg!("msg");
    });

    a.update(Message::Increment);

}
