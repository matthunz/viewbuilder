use crate::Window;
use winit::window::WindowBuilder;

pub struct Builder {
    raw: Option<WindowBuilder>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            raw: Some(WindowBuilder::new()),
        }
    }
}

impl Builder {
    pub fn active(&mut self, active: bool) -> &mut Self {
        self.raw = Some(self.raw.take().unwrap().with_active(active));
        self
    }

    pub fn build(&mut self) -> Window {
        Window::from_builder(self.raw.take().unwrap())
    }
}
