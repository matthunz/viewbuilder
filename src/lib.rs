mod any_element;

mod app;
pub use app::App;

pub mod element;
pub use self::element::Element;

mod element_ref;
pub use element_ref::ElementRef;

mod runtime;
pub use runtime::Runtime;

mod ui;
pub use ui::UserInterface;

#[cfg(feature = "dioxus")]
pub mod virtual_element;

#[cfg(feature = "dioxus")]
pub mod virtual_tree;

pub fn run() {
    Runtime::current().run()
}

pub fn transaction(f: impl FnOnce(&mut UserInterface) + Send + 'static) {
    Runtime::current().ui().tx.send(Box::new(f)).unwrap();
}

pub struct ClickEvent {}

macro_rules! impl_event {
    (
        $data:ty;
        $(
            $( #[$attr:meta] )*
            $name:ident
        )*
    ) => {
        $(
            $( #[$attr] )*
            #[inline]
            pub fn $name<'a, E: dioxus::prelude::EventReturn<T>, T>(_cx: &'a ::dioxus::core::ScopeState, mut _f: impl FnMut(::dioxus::core::Event<$data>) -> E + 'a) -> ::dioxus::core::Attribute<'a> {
                ::dioxus::core::Attribute::new(
                    stringify!($name),
                    _cx.listener(move |e: ::dioxus::core::Event<$data>| {
                        _f(e).spawn(_cx);
                    }),
                    None,
                    false,
                )
            }
        )*
    };
}

#[cfg(feature = "dioxus")]
pub mod prelude {
    pub use crate::ClickEvent;

    pub use dioxus::prelude::{render, rsx, use_state, Element, Scope, ScopeState};
    pub use dioxus_signals::{use_signal, Signal};

    pub mod events {
        use dioxus::prelude::EventReturn;

        use crate::ClickEvent;

        impl_event!(ClickEvent; onclick);
    }

    pub mod dioxus_elements {
        pub use super::events;

        #[allow(non_camel_case_types)]
        pub struct text;

        impl text {
            pub const TAG_NAME: &'static str = "text";
            pub const NAME_SPACE: Option<&'static str> = None;

            #[allow(non_upper_case_globals)]
            pub const font_size: (&'static str, Option<&'static str>, bool) =
                ("font_size", None, false);
        }
    }
}

#[cfg(feature = "dioxus")]
pub fn launch(app: dioxus::prelude::Component) {
    use tokio::task::LocalSet;
    use virtual_tree::Message;

    use crate::{any_element::AnyElement, element::Text};

    tokio::task::spawn_blocking(move || {
        let local_set = LocalSet::new();
        local_set.block_on(&tokio::runtime::Runtime::new().unwrap(), async move {
            let (mut virtual_tree, mut rx) = virtual_tree::VirtualTree::new(app);

            tokio::spawn(async move {
                while let Some(msg) = rx.recv().await {
                    match msg {
                        Message::Insert { element, tx } => transaction(move |ui| {
                            element.as_any().downcast_ref::<Text>().unwrap();

                            let key = ui.insert_boxed(element);
                            tx.send(key).unwrap();
                        }),
                        Message::SetAttribute {
                            tag: _,
                            key,
                            name,
                            value,
                            virtual_element,
                        } => transaction(move |ui| {
                            let element = &mut *ui.nodes[key].element;
                            virtual_element
                                .lock()
                                .unwrap()
                                .set_attribute(&name, value, element);
                        }),
                        Message::SetHandler {
                            name,
                            handler,
                            key,
                            virtual_element,
                        } => transaction(move |ui| {
                            let element = &mut *ui.nodes[key].element;
                            virtual_element
                                .lock()
                                .unwrap()
                                .set_handler(&name, handler, element);
                        }),
                        Message::HydrateText {
                            key,
                            path,
                            value,
                            virtual_element,
                        } => transaction(move |ui| {
                            let element = &mut *ui.nodes[key].element;
                            virtual_element
                                .lock()
                                .unwrap()
                                .hydrate_text(path, value, element);
                        }),
                        Message::SetText {
                            key,
                            value,
                            virtual_element,
                        } => transaction(move |ui| {
                            let element = &mut *ui.nodes[key].element;
                            virtual_element.lock().unwrap().set_text(value, element);
                        }),
                    }
                }
            });

            virtual_tree.rebuild().await;

            dbg!("start");
            loop {
                dbg!("loop");
                virtual_tree.wait().await;
                dbg!("run");
                virtual_tree.run().await;
            }
        })
    });

    run()
}
