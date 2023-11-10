use crate::element::Element;
use crate::factory::ViewFactory;
use crate::text_factory::TextElementFactory;
use crate::virtual_tree::DynAttribute;
use crate::Factory;
use crate::TextFactory;
use dioxus_native_core::node::OwnedAttributeDiscription;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::prelude::NodeType;
use dioxus_native_core::real_dom::NodeImmutable;
use dioxus_native_core::real_dom::NodeRef;
use dioxus_native_core::tree::Node;
use shipyard::EntityId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

pub struct Tree {
    factories: HashMap<Cow<'static, str>, Box<dyn Factory>>,
    text_factory: Box<dyn TextFactory>,
    elements: HashMap<EntityId, Box<dyn Element>>,
}

impl Default for Tree {
    fn default() -> Self {
        let mut me = Self {
            factories: Default::default(),
            text_factory: Box::new(TextElementFactory {}),
            elements: HashMap::new(),
        };
        me.insert_factory("view", ViewFactory {});
        me.insert_factory("Root", ViewFactory {});
        me
    }
}

impl Tree {
    pub fn insert_factory(
        &mut self,
        tag: impl Into<Cow<'static, str>>,
        element: impl Factory + 'static,
    ) {
        self.factories.insert(tag.into(), Box::new(element));
    }

    pub fn create_element(&mut self, node: NodeRef<DynAttribute>) -> Option<Box<dyn Element>> {
        match &*node.node_type() {
            NodeType::Text(text_node) => Some(self.text_factory.create_text(&text_node.text)),
            NodeType::Element(elem) => self
                .factories
                .get_mut(&*elem.tag)
                .map(|factory| factory.create_element(node, elem)),
            NodeType::Placeholder => todo!(),
        }
    }

    pub fn insert(&mut self, node: NodeRef<DynAttribute>) {
        let elem = self.create_element(node).unwrap();
        self.elements.insert(node.id(), elem);
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Box<dyn Element>> {
        self.elements.remove(&id)
    }
}
