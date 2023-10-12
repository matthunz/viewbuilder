#![allow(clippy::module_inception)]

#[cfg(feature = "layout")]
#[cfg_attr(docsrs, doc(cfg(feature = "layout")))]
pub mod layout;

#[cfg(feature = "semantics")]
#[cfg_attr(docsrs, doc(cfg(feature = "semantics")))]
pub mod semantics;

#[cfg(feature = "element")]
#[cfg_attr(docsrs, doc(cfg(feature = "element")))]
pub mod element;

pub mod geometry;

#[cfg(feature = "element")]
#[cfg_attr(docsrs, doc(cfg(feature = "element")))]
pub mod tree;

#[cfg(feature = "gl")]
#[cfg_attr(docsrs, doc(cfg(feature = "gl")))]
pub mod render;

#[cfg(feature = "element")]
#[cfg_attr(docsrs, doc(cfg(feature = "element")))]
pub mod virtual_tree;

pub mod prelude {
    pub use dioxus::prelude::{rsx, use_state, Element, Scope};

    pub mod dioxus_elements {
        pub use dioxus::prelude::dioxus_elements::events;

        #[allow(non_camel_case_types)]
        pub struct view;

        impl view {
            pub const TAG_NAME: &'static str = "view";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
}

#[cfg(feature = "element")]
#[cfg_attr(docsrs, doc(cfg(feature = "element")))]
pub fn run(app: dioxus::prelude::Component) {
    use render::Renderer;
    use virtual_tree::VirtualTree;

    let mut vtree = VirtualTree::new(app);
    vtree.rebuild();

    Renderer.run(vtree.tree, vtree.root)
}
