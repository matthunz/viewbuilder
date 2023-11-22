use vello::SceneBuilder;

pub trait Element {
    type Message;

    fn handle(&mut self, msg: Self::Message);

    fn render(&mut self, scene: SceneBuilder);
}
