use concoct::{Handle, Object, Signal};
use slotmap::{DefaultKey, SparseSecondaryMap};
use std::{cell::RefCell, rc::Rc};
use taffy::{
    geometry::Size,
    layout::Layout,
    style::{AvailableSpace, Style},
    Taffy,
};

#[derive(Default)]
pub struct LayoutNode {
    key: RefCell<Option<DefaultKey>>,
    style: Style,
    tree: RefCell<Option<Handle<LayoutTree>>>,
}

impl Object for LayoutNode {}

#[derive(Debug, Clone, Copy)]
pub struct LayoutEvent(pub Layout);

impl Signal<LayoutEvent> for LayoutNode {}

struct NodeHandle {
    node: Handle<LayoutNode>,
    layout: Option<Layout>,
}

#[derive(Default)]
struct Inner {
    taffy: Taffy,
    nodes: SparseSecondaryMap<DefaultKey, NodeHandle>,
}

#[derive(Default)]
pub struct LayoutTree {
    inner: Rc<RefCell<Inner>>,
}

impl LayoutTree {
    pub fn insert(&self, node: &Handle<LayoutNode>) {
        let mut me = self.inner.borrow_mut();
        let key = me.taffy.new_leaf(node.borrow().style.clone()).unwrap();
        *node.borrow().key.borrow_mut() = Some(key);

        me.nodes.insert(
            key,
            NodeHandle {
                node: node.clone(),
                layout: None,
            },
        );
    }

    pub fn layout(&self, node: &Handle<LayoutNode>, available_space: Size<AvailableSpace>) {
        let mut me = self.inner.borrow_mut();
        let key = node.borrow().key.borrow().unwrap();

        me.taffy.compute_layout(key, available_space).unwrap();

        let layout = me.taffy.layout(key).unwrap();
        me.nodes[key].node.emit(LayoutEvent(*layout));
    }
}

impl Object for LayoutTree {}
