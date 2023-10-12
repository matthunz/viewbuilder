#[cfg(feature = "layout")]
#[cfg_attr(docsrs, doc(cfg(feature = "layout")))]
pub mod layout;

#[cfg(feature = "semantics")]
#[cfg_attr(docsrs, doc(cfg(feature = "semantics")))]
pub mod semantics;

pub mod geometry;

pub mod tree;
use slotmap::DefaultKey;
pub use tree::{Element, Tree};

pub(crate) enum Operation {
    Push(DefaultKey),
    Pop,
}