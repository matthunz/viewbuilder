use crate::element::Element;
use crate::factory::ViewFactory;
use crate::text_factory::TextElementFactory;
use crate::virtual_tree::DynAttribute;
use crate::{Factory, LayoutTree, TextFactory};
use dioxus_native_core::node_ref::NodeMask;
use dioxus_native_core::prelude::NodeType;
use dioxus_native_core::real_dom::{NodeImmutable, NodeRef};
use shipyard::EntityId;
use skia_safe::Canvas;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use taffy::prelude::Layout;
use taffy::Taffy;

pub(crate) struct Slot {
    element: Box<dyn Element>,
    pub(crate) layout: Layout,
}

pub struct Tree {
    factories: HashMap<Cow<'static, str>, Box<dyn Factory>>,
    text_factory: Box<dyn TextFactory>,
    pub(crate) slots: HashMap<EntityId, Slot>,
    pub(crate) taffy: Arc<Mutex<Taffy>>,
    pub(crate) layout: LayoutTree,
}

impl Default for Tree {
    fn default() -> Self {
        let mut me = Self {
            factories: Default::default(),
            text_factory: Box::new(TextElementFactory {}),
            slots: HashMap::new(),
            taffy: Arc::new(Mutex::new(Taffy::new())),
            layout:LayoutTree::new(16)
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
        node: NodeRef<DynAttribute>,
        taffy: &Arc<Mutex<Taffy>>,
    ) -> Option<Box<dyn Element>> {
        match &*node.node_type() {
            NodeType::Text(text_node) => Some(self.text_factory.create_text(&text_node.text)),
            NodeType::Element(elem) => self
                .factories
                .get_mut(&*elem.tag)
                .map(|factory| factory.create_element(node, elem, taffy)),
            NodeType::Placeholder => todo!(),
        }
    }

    pub fn insert(&mut self, node: NodeRef<DynAttribute>, taffy: &Arc<Mutex<Taffy>>) {
        let element = self.create_element(node, taffy).unwrap();
        self.slots.insert(
            node.id(),
            Slot {
                element,
                layout: Layout::new(),
            },
        );
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Box<dyn Element>> {
        self.slots.remove(&id).map(|slot| slot.element)
    }

    pub fn update(
        &mut self,
        id: EntityId,
        node: NodeRef<DynAttribute>,
        mask: NodeMask,
        taffy: &Arc<Mutex<Taffy>>,
    ) {
        self.slots
            .get_mut(&id)
            .unwrap()
            .element
            .update(node, mask, taffy)
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        for slot in self.slots.values_mut() {
            slot.element.render(slot.layout, canvas)
        }
    }

    pub fn target(&self, x: f64, y: f64) -> impl Iterator<Item = EntityId> + '_ {
        self.layout.query([x, y])
    }
}
