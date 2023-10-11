use viewbuilder::{
    geometry::Size,
    layout::{LayoutNode, LayoutTree},
};

fn main() {
    let mut tree = LayoutTree::default();
    let mut layout_node = LayoutNode::default();
    layout_node.size(Size::from_points(100., 100.));
    let root = tree.insert(layout_node);

    tree.build_with_listener(root, |_, node| {
        dbg!(node);
    });
    dbg!(tree);
}
