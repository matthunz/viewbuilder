use crate::TreeKey;
use tokio::sync::mpsc::UnboundedSender;

mod local;
pub use local::{LocalTree, LocalTreeBuilder};

pub trait TreeBuilder {
    type Tree: 'static;

    fn insert_with_key(self, key: TreeKey, tx: UnboundedSender<TreeKey>) -> Self::Tree;
}
