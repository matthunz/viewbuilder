use std::any::Any;

use crate::{Element, TreeKey, UserInterface};
use slotmap::DefaultKey;

mod local;
pub use local::{LocalTree, LocalTreeBuilder};

pub enum TreeMessage {
    Handle { key: DefaultKey, msg: Box<dyn Any> },
    Render { key: DefaultKey },
}

pub trait TreeBuilder {
    type Tree: Element<Message = TreeMessage> + 'static;

    fn insert_with_key(self, key: TreeKey, ui: UserInterface) -> Self::Tree;
}
