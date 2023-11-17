mod any_element;

mod app;
pub use app::App;

pub mod element;
pub use self::element::Element;

mod element_ref;
pub use element_ref::ElementRef;

mod runtime;
pub use runtime::Runtime;

mod ui;
pub use ui::UserInterface;

#[cfg(feature = "dioxus")]
mod virtual_tree;
#[cfg(feature = "dioxus")]
pub use virtual_tree::VirtualTree;

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}

#[cfg(feature = "dioxus")]
pub mod prelude {
    pub use dioxus::prelude::{render, rsx, use_state, Element, Scope};

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::events;

        #[allow(non_camel_case_types)]
        pub struct text;

        impl text {
            pub const TAG_NAME: &'static str = "text";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

#[cfg(feature = "dioxus")]
pub fn launch(app: dioxus::prelude::Component) {
    transaction(move |ui| {
        let mut virtual_tree = VirtualTree::new(app);
        virtual_tree.rebuild(ui);
    });

    run()
}
