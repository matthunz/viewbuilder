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

mod message;
pub use message::Message;

mod virtual_node;
pub use virtual_node::{Attribute, VirtualNode};

struct Template {
    roots: Vec<VirtualNode>,
}

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    inner: Inner,
}

impl VirtualTree {
    pub fn new(app: Component) -> (Self, mpsc::UnboundedReceiver<Message>) {
        let mut virtual_elements: HashMap<&str, Arc<Mutex<Box<dyn VirtualElement>>>> =
            HashMap::new();
        virtual_elements.insert("text", Arc::new(Mutex::new(Box::new(VirtualText {}))));

        let (tx, rx) = mpsc::unbounded_channel();
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        (
            Self {
                vdom: VirtualDom::new(app),
                inner: Inner {
                    templates: HashMap::new(),
                    elements: HashMap::new(),
                    virtual_elements,
                    tx,
                    rx,
                    message_tx,
                    text_elements: HashMap::new(),
                },
            },
            message_rx,
        )
    }

    pub async fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);
        self.inner.update(mutations).await;
    }

    pub async fn wait(&mut self) {
        let id = self.inner.rx.recv().await.unwrap();
        dbg!("event");
        self.vdom
            .handle_event("click", Rc::new(ClickEvent {}), id, true);
        self.vdom.process_events();
    }

    pub async fn run(&mut self) {
        let mutations = self.vdom.render_immediate();
        dbg!(&mutations);
        self.inner.update(mutations).await;
    }

    pub async fn update(&mut self, mutations: Mutations<'_>) {
        self.inner.update(mutations).await;
    }
}

struct Inner {
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, (String, DefaultKey)>,
    pub(crate) virtual_elements: HashMap<&'static str, Arc<Mutex<Box<dyn VirtualElement>>>>,
    tx: mpsc::UnboundedSender<ElementId>,
    rx: mpsc::UnboundedReceiver<ElementId>,
    message_tx: mpsc::UnboundedSender<Message>,
    text_elements: HashMap<ElementId, ElementId>,
}

impl Inner {
    async fn update(&mut self, mutations: Mutations<'_>) {
        for template in mutations.templates {
            let roots = template
                .roots
                .iter()
                .map(VirtualNode::from_template)
                .collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

        let mut stack = Vec::new();
        for edit in mutations.edits {
            match edit {
                Mutation::LoadTemplate { name, index, id } => {
                    let template = &self.templates[name];
                    let root = &template.roots[index];
                    match root {
                        VirtualNode::Element {
                            tag,
                            attrs: _,
                            children,
                        } => {
                            let mut text = Text::builder();
                            for child in children {
                                if let VirtualNode::Text(s) = child {
                                    text.content(s.clone());
                                }
                            }

                            let (tx, rx) = oneshot::channel();
                            self.message_tx
                                .send(Message::Insert {
                                    element: Box::new(text.build()),
                                    tx,
                                })
                                .unwrap();
                            let key = rx.await.ok().unwrap();

                            stack.push(id);
                            self.elements.insert(id, (tag.to_string(), key));
                        }
                        _ => {}
                    }
                }
                Mutation::SetAttribute {
                    name,
                    value,
                    id,
                    ns: _,
                } => {
                    let (tag, key) = &self.elements[&id];
                    self.message_tx
                        .send(Message::SetAttribute {
                            key: *key,
                            name: name.to_string(),
                            tag: tag.clone(),
                            value: match value {
                                dioxus::core::BorrowedAttributeValue::Float(n) => Box::new(n),
                                _ => todo!(),
                            },
                            virtual_element: self.virtual_elements[&**tag].clone(),
                        })
                        .unwrap();
                }
                Mutation::NewEventListener { name, id } => {
                    let tx = self.tx.clone();
                    let handler = Box::new(move || {
                        tx.send(id).unwrap();
                    });

                    let (tag, key) = &self.elements[&id];
                    self.message_tx
                        .send(Message::SetHandler {
                            name: name.to_string(),
                            handler,
                            key: *key,
                            virtual_element: self.virtual_elements[&**tag].clone(),
                        })
                        .unwrap();
                }
                Mutation::HydrateText { path: _, value, id } => {
                    let parent_id = stack.last().unwrap();
                    self.text_elements.insert(id, *parent_id);

                    let (tag, key) = &self.elements[&parent_id];
                    self.message_tx
                        .send(Message::HydrateText {
                            key: *key,
                            path: 0,
                            value: value.to_string(),
                            virtual_element: self.virtual_elements[&**tag].clone(),
                        })
                        .unwrap();
                }
                Mutation::SetText { value, id } => {
                    let parent_id = self.text_elements[&id];
                    let (tag, key) = &self.elements[&parent_id];
                    self.message_tx
                        .send(Message::SetText {
                            key: *key,
                            value: value.to_string(),
                            virtual_element: self.virtual_elements[&**tag].clone(),
                        })
                        .unwrap();
                }
                _ => {}
            }
        }
    }
}
