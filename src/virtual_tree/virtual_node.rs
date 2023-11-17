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
    pub fn from_template(template_node: &TemplateNode) -> Self {
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
