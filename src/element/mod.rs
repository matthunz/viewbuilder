use vello::SceneBuilder;

pub trait Element {
    type Message;

    fn handle(&mut self, msg: Self::Message);

    fn render(&mut self, scene: SceneBuilder);
}

#[derive(Default)]
pub struct Window {}

impl Element for Window {
    type Message = ();

    fn handle(&mut self, _msg: Self::Message) {}

    fn render(&mut self, _scene: SceneBuilder) {}
}
