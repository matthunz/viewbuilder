use crate::{any_element::AnyElement, virtual_element::VirtualElement};

use futures::channel::oneshot;
use slotmap::DefaultKey;
use std::{
    any::Any,
    sync::{Arc, Mutex},
};

pub enum Message {
    Insert {
        element: Box<dyn AnyElement>,
        tx: oneshot::Sender<DefaultKey>,
    },
    SetAttribute {
        tag: String,
        key: DefaultKey,
        name: String,
        value: Box<dyn Any + Send>,
        virtual_element: Arc<Mutex<Box<dyn VirtualElement>>>,
    },
    SetHandler {
        name: String,
        handler: Box<dyn FnMut() + Send>,
        key: DefaultKey,
        virtual_element: Arc<Mutex<Box<dyn VirtualElement>>>,
    },
    HydrateText {
        key: DefaultKey,
        path: usize,
        value: String,
        virtual_element: Arc<Mutex<Box<dyn VirtualElement>>>,
    },
    SetText {
        key: DefaultKey,
        value: String,
        virtual_element: Arc<Mutex<Box<dyn VirtualElement>>>,
    },
}
