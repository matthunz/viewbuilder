#[cfg(feature = "native")]
pub mod native;

#[cfg(feature = "web")]
pub mod web;

pub use viewbuilder_macros::main;
