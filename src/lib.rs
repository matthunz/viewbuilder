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
pub use layout::LayoutTree;

mod render;
pub use render::Renderer;

pub mod prelude {
    pub use crate::layout::{Dimension, FlexDirection, IntoDimension};
    pub use crate::Color;

    pub use dioxus::prelude::{render, rsx, use_future, use_state, Element, Scope};

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::events;

        macro_rules! make_attr {
            ($name:ident) => {
                #[allow(non_upper_case_globals)]
                pub const $name: (&'static str, Option<&'static str>, bool) =
                    (stringify!($name), None, false);
            };
        }

        #[allow(non_camel_case_types)]
        pub struct view;

        impl view {
            pub const TAG_NAME: &'static str = "view";
            pub const NAME_SPACE: Option<&'static str> = None;

            make_attr!(flex_direction);
            make_attr!(width);
            make_attr!(height);
            make_attr!(background_color);
        }
    }
}
