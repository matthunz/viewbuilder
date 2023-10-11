//! Semantics and accessibility

use accesskit::{NodeBuilder, NodeClassSet, NodeId, TreeUpdate};
use std::{
    collections::{HashMap, HashSet},
    mem,
    num::NonZeroU128,
};

pub trait NodeFactory {
    fn semantics(&mut self) -> NodeBuilder;
}

/// Semantics and accessibility tree.
pub struct SemanticsTree {
    next_id: NonZeroU128,
    unused_ids: Vec<NodeId>,
    nodes: HashMap<NodeId, Box<dyn NodeFactory>>,
    changed: HashSet<NodeId>,
}

impl Default for SemanticsTree {
    fn default() -> Self {
        Self {
            next_id: NonZeroU128::MIN,
            unused_ids: Default::default(),
            nodes: Default::default(),
            changed: HashSet::new(),
        }
    }
}

impl SemanticsTree {
    pub fn insert(&mut self, factory: Box<dyn NodeFactory>) {
        let id = self.unused_ids.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id = self.next_id.checked_add(1).unwrap();
            NodeId(id)
        });
        self.nodes.insert(id, factory);
    }

    pub fn update(&mut self) -> TreeUpdate {
        let mut nodes = Vec::new();
        for id in mem::take(&mut self.changed) {
            let node_factory = self.nodes.get_mut(&id).unwrap();
            let node_builder = node_factory.semantics();
            let node = node_builder.build(&mut NodeClassSet::lock_global());
            nodes.push((id, node))
        }

        TreeUpdate {
            nodes,
            tree: None,
            focus: None,
        }
    }
}
