#![allow(clippy::module_inception)]
use slotmap::DefaultKey;

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

pub(crate) enum Operation {
    Push(DefaultKey),
    Pop,
}
