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
