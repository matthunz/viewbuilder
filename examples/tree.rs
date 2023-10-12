use viewbuilder::{Tree, Element, layout::{self, Layout}};

struct Button {

}

impl Element for Button {
    fn children(&mut self) -> Option<Vec<slotmap::DefaultKey>> {
        None
    }

    fn layout(&mut self) -> layout::Builder {
        Layout::builder()
    }
}

fn main() {
    let mut tree = Tree::default();
    let root = tree.insert(Box::new(Button {}));
    tree.layout(root)
}
