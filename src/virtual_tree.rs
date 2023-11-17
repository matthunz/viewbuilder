use dioxus::{
    core::{ElementId, Mutation},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};

use slotmap::DefaultKey;
use std::collections::HashMap;

use crate::{element::Text, Element, UserInterface};

pub trait VirtualElement {
    fn from_vnode(&self, node: &Node) -> Box<dyn Element>;
}

pub struct VirtualText {}

impl VirtualElement for VirtualText {
    fn from_vnode(&self, node: &Node) -> Box<dyn Element> {
        if let Node::Element {
            tag: _,
            attrs: _,
            children,
        } = node
        {
            let mut text = Text::builder();
            for child in children {
                if let Node::Text(s) = child {
                    text.content(s.clone());
                }
            }
            Box::new(text.build())
        } else {
            todo!()
        }
    }
}

enum Attribute {
    Dynamic { id: usize },
}

pub enum Node {
    Text(String),
    Element {
        tag: String,
        attrs: Vec<Attribute>,
        children: Vec<Self>,
    },
}

impl Node {
    fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => Node::Text(text.to_string()),
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
                Node::Element {
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
    roots: Vec<Node>,
}

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, DefaultKey>,
    virtual_elements: HashMap<&'static str, Box<dyn VirtualElement>>,
}

impl VirtualTree {
    pub fn new(app: Component) -> Self {
        let mut virtual_elements: HashMap<&str, Box<dyn VirtualElement>> = HashMap::new();
        virtual_elements.insert("text", Box::new(VirtualText {}));

        Self {
            vdom: VirtualDom::new(app),
            templates: HashMap::new(),
            elements: HashMap::new(),
            virtual_elements,
        }
    }

    pub fn rebuild(&mut self, ui: &mut UserInterface) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);

        for template in mutations.templates {
            let roots = template.roots.iter().map(Node::from_template).collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

        for edit in mutations.edits {
            match edit {
                Mutation::LoadTemplate { name, index, id } => {
                    let template = &self.templates[name];
                    let root = &template.roots[index];
                    match root {
                        Node::Element {
                            tag: _,
                            attrs: _,
                            children,
                        } => {
                            let mut text = Text::builder();
                            for child in children {
                                if let Node::Text(s) = child {
                                    text.content(s.clone());
                                }
                            }

                            let elem_ref = ui.insert(text.build());
                            self.elements.insert(id, elem_ref.key);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
