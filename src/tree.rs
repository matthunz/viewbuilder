use crate::element::Element;
use crate::factory::ViewFactory;
use crate::text_factory::TextElementFactory;
use crate::virtual_tree::DynAttribute;
use crate::Factory;
use crate::TextFactory;
use dioxus_native_core::node::OwnedAttributeDiscription;
use dioxus_native_core::node::OwnedAttributeValue;
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

    pub fn create_element(
        &mut self,
        tag: &str,
        attrs: &HashMap<
            OwnedAttributeDiscription,
            OwnedAttributeValue<DynAttribute>,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    ) -> Option<Box<dyn Element>> {
        self.factories
            .get_mut(tag)
            .map(|elem| elem.from_attrs(attrs))
    }

    pub fn create_text_element(&mut self, text: &str) -> Box<dyn Element> {
        self.text_factory.create_text(text)
    }

    pub fn insert_element(
        &mut self,
        id: EntityId,
        tag: &str,
        attrs: &HashMap<
            OwnedAttributeDiscription,
            OwnedAttributeValue<DynAttribute>,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    ) {
        let elem = self.create_element(tag, attrs).unwrap();
        self.elements.insert(id, elem);
    }

    pub fn insert_text_element(&mut self, id: EntityId, text: &str) {
        let elem = self.create_text_element(text);
        self.elements.insert(id, elem);
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Box<dyn Element>> {
        self.elements.remove(&id)
    }
}
