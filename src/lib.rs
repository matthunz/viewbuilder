//! A high performance UI framework.
//!
//! Viewbuilder is a moduler GUI library that can be used as an entire framework,
//! or with individual parts.
//!
//! ## Features
//! - `full`: Enables all the features below.
//! - `tracing`: Enables structured logging and performance metrics with the `tracing` crate.
//! - `web`: Enables web support.

#![cfg_attr(docsrs, feature(doc_cfg))]

use std::rc::Rc;

mod rt;
pub use self::rt::Runtime;

pub mod view;
pub use self::view::View;

#[cfg(feature = "web")]
#[cfg_attr(docsrs, doc(cfg(feature = "web")))]
pub mod web;

pub struct Context<M> {
    send: Rc<dyn Fn(M)>,
}

impl<M> Context<M> {
    pub fn new(send: impl Fn(M) + 'static) -> Self {
        Self {
            send: Rc::new(send),
        }
    }

    pub fn send(&self, msg: M) {
        (self.send)(msg)
    }
}

impl<M> Clone for Context<M> {
    fn clone(&self) -> Self {
        Self {
            send: self.send.clone(),
        }
    }
}

/// Control flow returned from [`Model::handle`].
pub enum ControlFlow {
    /// This model is pending changes, do not rebuild the view.
    Pending,

    /// Rebuild the view with the updated model.
    Rebuild,
}

/// Model for a view builder.
pub trait Model<M> {
    fn handle(&mut self, msg: M) -> ControlFlow;
}

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc_cfg(feature = "tracing"))]
#[macro_export]
macro_rules! build_span {
    ($name:tt) => {
        $crate::span!("build", $name)
    };
}

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc_cfg(feature = "tracing"))]
#[macro_export]
macro_rules! rebuild_span {
    ($name:tt) => {
        $crate::span!("rebuild", $name)
    };
}

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc_cfg(feature = "tracing"))]
#[macro_export]
macro_rules! remove_span {
    ($name:tt) => {
        $crate::span!("remove", $name)
    };
}

#[cfg(feature = "tracing")]
#[cfg_attr(docsrs, doc_cfg(feature = "tracing"))]
#[macro_export]
macro_rules! span {
    ($method:tt, $name:tt) => {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!(concat!("View::", $method), view = $name);
        #[cfg(feature = "tracing")]
        let _g = span.enter();
    };
}
