use crate::{element::ElementKind, Element, ElementData};
use slotmap::{DefaultKey, SlotMap};

mod iter;
pub use iter::Iter;

use self::iter::Item;

#[derive(Default)]
pub struct Tree {
    elements: SlotMap<DefaultKey, Element>,
}

impl Tree {
    pub fn iter(&self, root: DefaultKey) -> Iter {
        Iter::new(self, root)
    }

    pub fn display(&self, root: DefaultKey) -> String {
        let mut s = String::new();

        for item in self.iter(root) {
            match item {
                Item::Element { element, level } => {
                    for _ in 0..level {
                        s.push_str("  ");
                    }

                    match &element.data {
                        ElementData::Text(content) => s.push_str(&format!("\"{}\",", content)),
                        ElementData::Container { size } => {
                            s.push_str("{\n");
                            if let Some(size) = size {
                                for _ in 0..level + 1 {
                                    s.push_str("  ");
                                }

                                s.push_str(&format!(
                                    "size: ({:?}, {:?}),\n",
                                    size.width, size.height
                                ));
                            }
                        }
                    }
                }
                Item::Pop { kind, level } => {
                    if kind == ElementKind::Container {
                        s.push('\n');

                        for _ in 0..level {
                            s.push_str("  ");
                        }

                        s.push_str("},");
                    }
                }
            }
        }
        s
    }

    pub fn insert(&mut self, element: Element) -> DefaultKey {
        self.elements.insert(element)
    }
}
