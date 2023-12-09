use slotmap::new_key_type;

mod any_element;
pub use any_element::AnyElement;

mod any_element_ref;
pub use any_element_ref::AnyElementRef;

pub mod element;
pub use element::Element;

mod element_ref;
pub use element_ref::LocalElementRef;

pub mod tree;
pub use tree::LocalTree;

mod ui;
pub use ui::UserInterface;

new_key_type! {
    pub struct TreeKey;
}
