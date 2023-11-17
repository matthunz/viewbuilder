mod any_element;
pub use any_element::AnyElement;

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
    Runtime::current().transaction(f)
}

pub struct ClickEvent {}

macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident
        )*
    ) => {
        $(
            $( #[$attr] )*
            #[inline]
            pub fn $name<'a, E: dioxus::prelude::EventReturn<T>, T>(_cx: &'a ::dioxus::core::ScopeState, mut _f: impl FnMut(::dioxus::core::Event<$data>) -> E + 'a) -> ::dioxus::core::Attribute<'a> {
                ::dioxus::core::Attribute::new(
                    stringify!($name),
                    _cx.listener(move |e: ::dioxus::core::Event<$data>| {
                        _f(e).spawn(_cx);
                    }),
                    None,
                    false,
                )
            }
        )*
    };
}

#[cfg(feature = "dioxus")]
pub mod prelude {
    pub use crate::ClickEvent;

    pub use dioxus::prelude::{render, rsx, use_state, Element, Scope, ScopeState};
    pub use dioxus_signals::{use_signal, Signal};

    pub mod events {
        use crate::ClickEvent;

        impl_event!(ClickEvent; onclick);
    }

    pub mod dioxus_elements {
        pub use super::events;

        #[allow(non_camel_case_types)]
        pub struct text;

        impl text {
            pub const TAG_NAME: &'static str = "text";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const font_size: (&'static str, Option<&'static str>, bool) =
                ("font_size", None, false);
        }

        #[allow(non_camel_case_types)]
        pub struct view;

        impl view {
            pub const TAG_NAME: &'static str = "view";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

#[cfg(feature = "dioxus")]
pub fn launch(app: dioxus::prelude::Component) {
    use crate::virtual_tree::VirtualTree;
    use tokio::task::LocalSet;

    tokio::task::spawn_blocking(move || {
        let local_set = LocalSet::new();
        local_set.block_on(&tokio::runtime::Runtime::new().unwrap(), async move {
            let mut virtual_tree = VirtualTree::new(app);
            virtual_tree.run().await;
        })
    });

    run()
}
