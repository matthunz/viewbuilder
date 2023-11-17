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

pub enum Attribute {
    Dynamic { id: usize },
}

pub enum VirtualNode {
    Text(String),
    Element {
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<Self>,
    },
}

impl VirtualNode {
    fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => VirtualNode::Text(text.to_string()),
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children,
            } => {
                let children = children.iter().map(Self::from_template).collect();
                let attrs = attrs
                    .into_iter()
                    .map(|attr| match attr {
                        TemplateAttribute::Dynamic { id } => Attribute::Dynamic { id: *id },
                        _ => todo!(),
                    })
                    .collect();
                VirtualNode::Element {
                    tag: tag.to_string(),
                    attrs,
                    children,
                }
            }
            TemplateNode::DynamicText { id: _ } => VirtualNode::Text(String::new()),
            _ => todo!(),
        }
    }
}

struct Template {
    roots: Vec<VirtualNode>,
}

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

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, (String, DefaultKey)>,
    pub(crate) virtual_elements: HashMap<&'static str, Arc<Mutex<Box<dyn VirtualElement>>>>,
    tx: mpsc::UnboundedSender<ElementId>,
    rx: mpsc::UnboundedReceiver<ElementId>,
    message_tx: mpsc::UnboundedSender<Message>,
    text_elements: HashMap<ElementId, ElementId>,
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
                templates: HashMap::new(),
                elements: HashMap::new(),
                virtual_elements,
                tx,
                rx,
                message_tx,
                text_elements: HashMap::new(),
            },
            message_rx,
        )
    }

    pub async fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);
        update(
            &mut self.templates,
            &mut self.elements,
            &self.tx,
            &self.virtual_elements,
            mutations,
            &self.message_tx,
            &mut self.text_elements,
        )
        .await
    }

    pub async fn wait(&mut self) {
        let id = self.rx.recv().await.unwrap();
        dbg!("event");
        self.vdom
            .handle_event("click", Rc::new(ClickEvent {}), id, true);
        self.vdom.process_events();
    }

    pub async fn run(&mut self) {
        let mutations = self.vdom.render_immediate();
        dbg!(&mutations);
        update(
            &mut self.templates,
            &mut self.elements,
            &self.tx,
            &self.virtual_elements,
            mutations,
            &self.message_tx,
            &mut self.text_elements,
        )
        .await
    }

    pub async fn update(&mut self, mutations: Mutations<'_>) {
        update(
            &mut self.templates,
            &mut self.elements,
            &self.tx,
            &self.virtual_elements,
            mutations,
            &self.message_tx,
            &mut self.text_elements,
        )
        .await
    }
}

async fn update(
    templates: &mut HashMap<String, Template>,
    elements: &mut HashMap<ElementId, (String, DefaultKey)>,
    tx: &mpsc::UnboundedSender<ElementId>,
    virtual_elements: &HashMap<&'static str, Arc<Mutex<Box<dyn VirtualElement>>>>,
    mutations: Mutations<'_>,
    message_tx: &mpsc::UnboundedSender<Message>,
    text_elements: &mut HashMap<ElementId, ElementId>,
) {
    for template in mutations.templates {
        let roots = template
            .roots
            .iter()
            .map(VirtualNode::from_template)
            .collect();
        templates.insert(template.name.to_string(), Template { roots });
    }

    let mut stack = Vec::new();
    for edit in mutations.edits {
        match edit {
            Mutation::LoadTemplate { name, index, id } => {
                let template = &templates[name];
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
                        message_tx
                            .send(Message::Insert {
                                element: Box::new(text.build()),
                                tx,
                            })
                            .unwrap();
                        let key = rx.await.ok().unwrap();

                        stack.push(id);
                        elements.insert(id, (tag.to_string(), key));
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
                let (tag, key) = &elements[&id];
                message_tx
                    .send(Message::SetAttribute {
                        key: *key,
                        name: name.to_string(),
                        tag: tag.clone(),
                        value: match value {
                            dioxus::core::BorrowedAttributeValue::Float(n) => Box::new(n),
                            _ => todo!(),
                        },
                        virtual_element: virtual_elements[&**tag].clone(),
                    })
                    .unwrap();
            }
            Mutation::NewEventListener { name, id } => {
                let tx = tx.clone();
                let handler = Box::new(move || {
                    tx.send(id).unwrap();
                });

                let (tag, key) = &elements[&id];
                message_tx
                    .send(Message::SetHandler {
                        name: name.to_string(),
                        handler,
                        key: *key,
                        virtual_element: virtual_elements[&**tag].clone(),
                    })
                    .unwrap();
            }
            Mutation::HydrateText { path: _, value, id } => {
                let parent_id = stack.last().unwrap();
                text_elements.insert(id, *parent_id);

                let (tag, key) = &elements[&parent_id];
                message_tx
                    .send(Message::HydrateText {
                        key: *key,
                        path: 0,
                        value: value.to_string(),
                        virtual_element: virtual_elements[&**tag].clone(),
                    })
                    .unwrap();
            }
            Mutation::SetText { value, id } => {
                let parent_id = text_elements[&id];
                let (tag, key) = &elements[&parent_id];
                message_tx
                    .send(Message::SetText {
                        key: *key,
                        value: value.to_string(),
                        virtual_element: virtual_elements[&**tag].clone(),
                    })
                    .unwrap();
            }
            _ => {}
        }
    }
}
