use super::Tree;
use crate::Element;
use slotmap::DefaultKey;

enum Item {
    Key(DefaultKey),
    Pop,
}

pub struct Iter<'a> {
    tree: &'a Tree,
    stack: Vec<Item>,
    count: usize,
}

impl<'a> Iter<'a> {
    pub(crate) fn new(tree: &'a Tree, root: DefaultKey) -> Self {
        Iter {
            tree,
            stack: vec![Item::Key(root)],
            count: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (usize, &'a Element);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.stack.pop() {
            match item {
                Item::Key(key) => {
                    let elem = &self.tree.elements[key];

                    self.stack.push(Item::Pop);
                    for child in elem.children.iter().flatten().copied().map(Item::Key) {
                        self.stack.push(child);
                    }

                    let count = self.count;
                    self.count += 1;

                    return Some((count, elem));
                }
                Item::Pop => self.count -= 1,
            }
        }

        None
    }
}
