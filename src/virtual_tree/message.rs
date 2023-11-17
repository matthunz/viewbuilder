use crate::{
    any_element::AnyElement,
    element::Text,
    virtual_element::{VirtualElement, VirtualText},
    ClickEvent,
};
use dioxus::{
    core::{ElementId, Mutation, Mutations},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};
use futures::channel::oneshot;
use slotmap::DefaultKey;
use std::{
    any::Any,
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;


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
