//! A cross-platform user interface framework for Rust.
//!  
//! Viewbuilder is a moduler GUI library that can be used as an entire framework,
//! or with individual parts.

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate self as viewbuilder;

pub use viewbuilder_macros::object;

mod any_object;
pub use self::any_object::AnyObject;

mod handle;
pub use self::handle::{Handle, HandleState, Ref};

mod object;
pub use self::object::Object;

mod rt;
pub(crate) use self::rt::Node;
pub use rt::Runtime;

mod signal;
pub use self::signal::Signal;

mod slot;
pub use self::slot::Slot;

#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub mod ui;
#[cfg(feature = "ui")]
#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
pub use ui::UserInterface;
