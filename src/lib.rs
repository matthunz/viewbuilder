pub mod virtual_tree;

pub mod element;

mod factory;
pub use self::factory::Factory;

mod tree;
pub use tree::Tree;

mod text_factory;
pub use self::text_factory::TextFactory;

pub mod prelude {
    pub use dioxus::prelude::{rsx, use_state, Element, Scope};

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::events;

        #[allow(non_camel_case_types)]
        pub struct view;

        impl view {
            pub const TAG_NAME: &'static str = "view";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const flex_direction: (&'static str, Option<&'static str>, bool) =
                ("flex_direction", None, false);
        }
    }
}
