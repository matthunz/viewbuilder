use crate::{Element, ElementData};
use slotmap::{DefaultKey, SlotMap};
use std::borrow::Cow;

mod iter;
pub use iter::Iter;

#[derive(Default)]
pub struct Tree {
    elements: SlotMap<DefaultKey, Element>,
}

impl Tree {
    pub fn iter(&self, root: DefaultKey) -> Iter {
        Iter::new(self, root)
    }

    pub fn display(&self, root: DefaultKey) {
        for (level, elem) in self.iter(root) {
            let mut indent = String::new();
            for _ in 0..level {
                indent.push_str("  ")
            }

            let elem_str = match elem.data {
                ElementData::Text(ref content) => format!("\"{content}\""),
                ElementData::Container { size, .. } => {
                    let mut s = String::from("Container");
                    if let Some(size) = size {
                        s.push('\n');
                        for _ in 0..level + 1 {
                            s.push_str("  ")
                        }
                        s.push_str(&format!("size: ({:?}, {:?})", size.width, size.height));
                    }
                    s
                }
            };
            println!("{indent}{elem_str}");
        }
    }

    pub fn insert(&mut self, element: Element) -> DefaultKey {
        self.elements.insert(element)
    }
}
