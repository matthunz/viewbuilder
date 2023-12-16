//! A cross-platform user interface framework for Rust.
//! Viewbuilder is a moduler GUI library that can be used as an entire framework,
//! or with individual parts. This crate provides reactive objects for UI using the
//! [`concoct`](https://docs.rs/concoct/latest/concoct/) runtime.
#![cfg_attr(docsrs, feature(doc_cfg))]

macro_rules! cfg_flag {
    ($flag:tt; $($i:item)*) => {
        $(
            #[cfg(feature = $flag)]
            #[cfg_attr(docsrs, doc(cfg(feature = $flag)))]
            $i
        )*
    };
}

cfg_flag!(
    "EventLoop";
    pub mod event_loop;
    pub use event_loop::EventLoop;
);

cfg_flag!(
    "Window";
    pub mod window;
    pub use window::Window;
);
