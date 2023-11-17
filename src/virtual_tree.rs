use dioxus::{
    core::{BorrowedAttributeValue, ElementId, Mutation},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};
use skia_safe::{Font, Typeface};
use slotmap::DefaultKey;
use std::{collections::HashMap, fmt};

enum Attribute {
    Dynamic { id: usize },
}

enum Node {
    Text(String),
    Element {
        attrs: Vec<Attribute>,
        children: Vec<Self>,
    },
}

impl Node {
    fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => Node::Text(text.to_string()),
            TemplateNode::Element {
                tag: _,
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
                Node::Element { attrs, children }
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
}

impl VirtualTree {
    pub fn new(app: Component) -> Self {
        Self {
            vdom: VirtualDom::new(app),
            templates: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);

        for template in mutations.templates {
            let roots = template.roots.iter().map(Node::from_template).collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

        for edit in mutations.edits {
            match edit {
                Mutation::LoadTemplate { name, index, id } => {}
                _ => {}
            }
        }
    }
}
