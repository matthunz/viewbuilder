use skia_safe::Image;
use slotmap::DefaultKey;
use taffy::{prelude::Size, style::Style};

mod any_element;

mod element_ref;
pub use element_ref::ElementRef;

mod runtime;
pub use runtime::Runtime;

mod ui;
pub use ui::UserInterface;

mod app;
pub use app::App;

mod view;
pub use view::View;

pub trait Element: Send {
    fn children(&self) -> Option<Vec<DefaultKey>>;

    fn layout(&mut self) -> Style;

    fn render(&mut self, size: Size<f32>) -> Image;
}

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}
