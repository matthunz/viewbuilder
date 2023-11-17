mod any_element;

mod app;
pub use app::App;
use dioxus::prelude::Component;

pub mod element;
pub use self::element::Element;

mod element_ref;
pub use element_ref::ElementRef;

mod runtime;
pub use runtime::Runtime;

mod ui;
pub use ui::UserInterface;

mod virtual_tree;
pub use virtual_tree::VirtualTree;

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}

pub fn launch(app: Component) {
    let mut virtual_tree = VirtualTree::new(app);
    virtual_tree.rebuild();

    run()
}
