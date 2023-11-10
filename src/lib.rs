pub mod virtual_tree;

mod color;
pub use color::Color;

pub mod element;

mod factory;
pub use self::factory::Factory;

mod tree;
pub use tree::Tree;

mod text_factory;
pub use self::text_factory::TextFactory;

pub mod layout;
pub use layout::Layout;

mod render;
pub use render::Renderer;

pub mod prelude {
    pub use crate::Color;
    pub use crate::layout::{FlexDirection, Dimension, IntoDimension};

    pub use dioxus::prelude::{render, rsx, use_future, use_state, Element, Scope};

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

            #[allow(non_upper_case_globals)]
            pub const width: (&'static str, Option<&'static str>, bool) = ("width", None, false);

            #[allow(non_upper_case_globals)]
            pub const height: (&'static str, Option<&'static str>, bool) = ("height", None, false);

            #[allow(non_upper_case_globals)]
            pub const background_color: (&'static str, Option<&'static str>, bool) =
                ("background_color", None, false);
        }
    }
}
