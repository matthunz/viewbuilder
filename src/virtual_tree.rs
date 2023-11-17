use crate::{
    element::Text,
    virtual_element::{VirtualElement, VirtualText},
    UserInterface,
};
use dioxus::{
    core::{ElementId, Mutation, Mutations},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};
use slotmap::DefaultKey;
use std::{collections::HashMap, rc::Rc};
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
            _ => todo!(),
        }
    }
}

struct Template {
    roots: Vec<VirtualNode>,
}

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, (String, DefaultKey)>,
    virtual_elements: HashMap<&'static str, Box<dyn VirtualElement>>,
    tx: mpsc::UnboundedSender<ElementId>,
    rx: mpsc::UnboundedReceiver<ElementId>,
}

impl VirtualTree {
    pub fn new(app: Component) -> Self {
        let mut virtual_elements: HashMap<&str, Box<dyn VirtualElement>> = HashMap::new();
        virtual_elements.insert("text", Box::new(VirtualText {}));

        let (tx, rx) = mpsc::unbounded_channel();

        Self {
  vdom: VirtualDom::new(app),
            templates: HashMap::new(),
            elements: HashMap::new(),
            virtual_elements,
            tx,
            rx,
        }
    }

    pub fn rebuild(&mut self, ui: &mut UserInterface) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);
        update(
            &mut self.templates,
            &mut self.elements,
            &mut self.virtual_elements,
            &self.tx,
            mutations,
            ui,
        )
    }

    pub async fn wait(&mut self) {
        let id = self.rx.recv().await.unwrap();
        self.vdom.handle_event("click", Rc::new(()), id, false);
    }

    pub fn run(&mut self, ui: &mut UserInterface) {
        let mutations = self.vdom.render_immediate();

        update(
            &mut self.templates,
            &mut self.elements,
            &mut self.virtual_elements,
            &self.tx,
            mutations,
            ui,
        )
    }

    pub fn update(&mut self, mutations: Mutations, ui: &mut UserInterface) {
        update(
            &mut self.templates,
            &mut self.elements,
            &mut self.virtual_elements,
            &self.tx,
            mutations,
            ui,
        )
    }
}

fn update(
    templates: &mut HashMap<String, Template>,
    elements: &mut HashMap<ElementId, (String, DefaultKey)>,
    virtual_elements: &mut HashMap<&'static str, Box<dyn VirtualElement>>,
    tx: &mpsc::UnboundedSender<ElementId>,
    mutations: Mutations,
    ui: &mut UserInterface,
) {
    for template in mutations.templates {
        let roots = template
            .roots
            .iter()
            .map(VirtualNode::from_template)
            .collect();
        templates.insert(template.name.to_string(), Template { roots });
    }

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

                        let elem_ref = ui.insert(text.build());
                        elements.insert(id, (tag.to_string(), elem_ref.key));
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
                let element = &mut *ui.nodes[*key].element;
                virtual_elements[&**tag].set_attribute(name, value, element);
            }
            Mutation::NewEventListener { name, id } => {
                let tx = tx.clone();
                let handler = Box::new(move || {
                    tx.send(id).unwrap();
                });

                let (tag, key) = &elements[&id];
                let element = &mut *ui.nodes[*key].element;

                virtual_elements[&**tag].set_handler(name, handler, element);
            }
            _ => {}
        }
    }
}
