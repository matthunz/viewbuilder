use concoct::Object;
use taffy::{geometry::Size, style_helpers::TaffyMaxContent};
use viewbuilder::native::layout::{self, LayoutNode, LayoutTree};

#[viewbuilder::main]
fn main() {
    let tree = LayoutTree::default().start();

    let node = LayoutNode::default().start();
    tree.borrow().insert(&node);

    let _listener = node.listen(|msg: &layout::LayoutEvent| {
        dbg!(msg);
    });

    tree.borrow().layout(&node, Size::MAX_CONTENT);
}
