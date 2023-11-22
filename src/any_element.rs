use std::any::Any;

use vello::SceneBuilder;

use crate::Element;

pub trait AnyElement {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn handle_any(&mut self, msg: Box<dyn Any>);

    fn render_any(&mut self, scene: SceneBuilder);
}

impl<E> AnyElement for E
where
    E: Element + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn handle_any(&mut self, msg: Box<dyn Any>) {
        self.handle(*msg.downcast().unwrap())
    }

    fn render_any(&mut self, scene: SceneBuilder) {
        self.render(scene)
    }
}
