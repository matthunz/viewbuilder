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
pub mod virtual_element;

#[cfg(feature = "dioxus")]
pub mod virtual_tree;

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}

pub struct ClickEvent {}

#[cfg(feature = "dioxus")]
pub mod prelude {
    pub use crate::ClickEvent;
    use dioxus::core::Attribute;
    pub use dioxus::prelude::{render, rsx, use_state, Element, Scope, ScopeState};

    pub fn onclick<'a, E, T>(
        _cx: &'a ScopeState,
        _f: impl FnMut(ClickEvent) -> E + 'a,
    ) -> Attribute<'a> {
        todo!()
    }

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::events;

        #[allow(non_camel_case_types)]
        pub struct text;

        impl text {
            pub const TAG_NAME: &'static str = "text";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const font_size: (&'static str, Option<&'static str>, bool) =
                ("font_size", None, false);
        }
    }
}

#[cfg(feature = "dioxus")]
pub fn launch(app: dioxus::prelude::Component) {
    tokio::spawn(async move {
        transaction(move |ui| {
            let mut virtual_tree = virtual_tree::VirtualTree::new(app);
            virtual_tree.rebuild(ui);
        })
    });

    run()
}
