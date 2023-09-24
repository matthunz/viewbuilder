//! # Viewbuilder
//!
//! Cross-platform user interface framework for Rust.
//!
//! This crate provides an HTML-like render API for the backend of a UI.
//! It supports layout, drawing, and accessability.

use slotmap::DefaultKey;

pub mod node;
pub use node::Node;

pub mod tree;
pub use tree::{NodeRef, Tree};

pub mod element;
pub use element::Element;

mod render;
pub use render::Renderer;

pub mod event;
pub use event::Event;

pub type ElementKey = slotmap::DefaultKey;

/// Run the user interface tree.
///
/// This will create a new window and render the tree,
/// propagating events and re-rendering as they occuring.
pub fn run(tree: Tree, root: DefaultKey) {
    let renderer = Renderer::new();
    renderer.run(tree, root)
}
