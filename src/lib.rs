//! # Viewbuilder
//!
//! Cross-platform user interface framework for Rust.
//!
//! This crate provides an HTML-like render API for the backend of a UI.
//! It supports layout, drawing, and accessability.
//!
//! ```
//! use viewbuilder::{Context, Element, NodeKey};
//! use taffy::prelude::{AlignItems, JustifyContent};
//!
//! fn app(cx: &mut Context) -> NodeKey {
//!     Element::new()
//!         .align_items(AlignItems::Center)
//!         .justify_content(JustifyContent::Center)
//!         .child(cx.insert("Hello World!"))
//!         .build(cx)
//! }
//! ```
//!

pub mod node;
pub use node::Node;

pub mod tree;
pub use tree::{NodeRef, Tree};

mod context;
pub use context::Context;

pub mod element;
pub use element::Element;

pub mod render;
pub use render::Renderer;

pub mod event;
pub use event::Event;

mod window;
pub use window::Window;

slotmap::new_key_type! {
    /// Key to access a node in a tree.
    pub struct NodeKey;
}

/// Run the user interface tree.
///
/// This will create a new window and render the tree,
/// propagating events and re-rendering as they occuring.
pub fn run<T: 'static>(state: T, f: impl FnOnce(&mut Context<T>) -> NodeKey) {
    let mut renderer = Renderer::new();
    let mut cx = Context::new(state);
    let root = f(&mut cx);

    let cx_key = renderer.context(cx);

    let window = Window::builder().build(&renderer, root);
    renderer.insert_window(window, cx_key);
    renderer.run();
}
