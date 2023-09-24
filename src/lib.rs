//! # Viewbuilder
//!
//! Cross-platform user interface framework for Rust.
//!
//! This crate provides an HTML-like render API for the backend of a UI.
//! It supports layout, drawing, and accessability.
//!
//! ```
//! fn app(cx: &mut Context) -> NodeKey {
//!     Element::new()
//!         .align_items(AlignItems::Center)
//!         .justify_content(JustifyContent::Center)
//!         .child(cx.insert("Hello World!"))
//!         .build(cx)
//! }
//! 
//! fn main() {
//!     viewbuilder::run(app)
//! }
//! ```
//!

pub mod node;
pub use node::Node;

pub mod tree;

mod context;
pub use context::Context;

pub mod element;
pub use element::Element;

pub mod render;
pub use render::Renderer;

pub mod event;
pub use event::Event;

slotmap::new_key_type! {
    /// Key to access a node in a tree.
    pub struct NodeKey;
}

/// Run the user interface tree.
///
/// This will create a new window and render the tree,
/// propagating events and re-rendering as they occuring.
pub fn run(f: impl FnOnce(&mut Context) -> NodeKey) {
    let renderer = Renderer::default();

    let mut cx = Context::default();
    let root = f(&mut cx);

    renderer.run(cx, root)
}
