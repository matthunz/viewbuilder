use std::any::Any;

use crate::{ui::UserInterfaceRef, Element, TreeKey};
use slotmap::DefaultKey;

mod local;
pub use local::{LocalTree, LocalTreeBuilder};

pub enum TreeMessage {
    Handle { key: DefaultKey, msg: Box<dyn Any> },
}

pub trait TreeBuilder {
    type Tree: Element<Message = TreeMessage> + 'static;

    fn insert_with_key(self, key: TreeKey, ui: UserInterfaceRef) -> Self::Tree;
}
