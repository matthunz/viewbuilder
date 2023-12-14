#[cfg(feature = "native")]
#[cfg_attr(docsrs, doc(cfg(feature = "native")))]
pub mod native;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use viewbuilder_macros::main;
