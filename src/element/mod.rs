pub trait Element {
    type Message;

    fn handle(&mut self, msg: Self::Message);
}
