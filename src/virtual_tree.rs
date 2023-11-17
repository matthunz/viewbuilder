use crate::{
    element::Text,
    virtual_element::{VirtualElement, VirtualText},
    UserInterface,
};
use dioxus::{
    core::{ElementId, Mutation},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};
use slotmap::DefaultKey;
use std::collections::HashMap;

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
            let roots = template
                .roots
                .iter()
                .map(VirtualNode::from_template)
                .collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

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

                            let elem_ref = ui.insert(text.build());
                            self.elements.insert(id, (tag.to_string(), elem_ref.key));
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
                    let element = &mut *ui.nodes[*key].element;
                    self.virtual_elements[&**tag].set_attribute(name, value, element);
                }
                _ => {}
            }
        }
    }
}
