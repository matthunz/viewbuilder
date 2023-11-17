use crate::{
    transaction,
    virtual_element::{VirtualElement, VirtualText, VirtualView},
    ClickEvent,
};
use dioxus::{
    core::{ElementId, Mutation, Mutations},
    prelude::{Component, VirtualDom},
};
use futures::channel::oneshot;
use slotmap::DefaultKey;
use std::{collections::HashMap, rc::Rc, sync::Arc};
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
    pub fn new(app: Component) -> Self {
        let mut virtual_elements: HashMap<&str, Arc<dyn VirtualElement>> = HashMap::new();
        virtual_elements.insert("text", Arc::new(VirtualText {}));
        virtual_elements.insert("view", Arc::new(VirtualView {}));

        let (tx, rx) = mpsc::unbounded_channel();
        let (message_tx, mut message_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            while let Some(msg) = message_rx.recv().await {
                match msg {
                    Message::Insert { element, tx } => transaction(move |ui| {
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
                        virtual_element.set_attribute(&name, value, element);
                    }),
                    Message::SetHandler {
                        name,
                        handler,
                        key,
                        virtual_element,
                    } => transaction(move |ui| {
                        let element = &mut *ui.nodes[key].element;
                        virtual_element.set_handler(&name, handler, element);
                    }),
                    Message::HydrateText {
                        key,
                        path,
                        value,
                        virtual_element,
                    } => transaction(move |ui| {
                        let element = &mut *ui.nodes[key].element;
                        virtual_element.hydrate_text(path, value, element);
                    }),
                    Message::SetText {
                        key,
                        value,
                        virtual_element,
                    } => transaction(move |ui| {
                        let element = &mut *ui.nodes[key].element;
                        virtual_element.set_text(value, element);
                    }),
                }
            }
        });

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
        }
    }

    pub async fn run(&mut self) {
        self.rebuild().await;

        loop {
            self.wait().await;
            self.step().await;
        }
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

    pub async fn step(&mut self) {
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
    pub(crate) virtual_elements: HashMap<&'static str, Arc<dyn VirtualElement>>,
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
                    if let VirtualNode::Element {
                        tag,
                        attrs: _,
                        children: _,
                    } = root
                    {
                        let element = self.virtual_elements[&**tag].from_vnode(root);

                        let (tx, rx) = oneshot::channel();
                        self.message_tx
                            .send(Message::Insert { element, tx })
                            .unwrap();
                        let key = rx.await.ok().unwrap();

                        stack.push(id);
                        self.elements.insert(id, (tag.to_string(), key));
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
