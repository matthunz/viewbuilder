mod any_element;

mod app;
pub use app::App;

mod element;
pub use self::element::Element;

mod element_ref;
pub use element_ref::ElementRef;

mod runtime;
pub use runtime::Runtime;

mod text;
pub use self::text::Text;

mod ui;
pub use ui::UserInterface;

mod view;
pub use view::View;

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}
