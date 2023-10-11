use viewbuilder::{
    geometry::Size,
    layout::{Layout, LayoutTree},
};

fn main() {
    let mut tree = LayoutTree::default();
    let root = Layout::builder()
        .size(Size::from_points(100., 100.))
        .build(&mut tree);

    tree.build_with_listener(root, |_, node| {
        dbg!(node);
    });
    dbg!(tree);
}
