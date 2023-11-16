use crate::Element;
use slotmap::DefaultKey;

#[derive(Default)]
pub struct View {
    children: Vec<DefaultKey>,
}

impl View {
    pub fn with_child(&mut self, key: DefaultKey) -> &mut Self {
        self.children.push(key);
        self
    }

    pub fn remove_child(&mut self, key: DefaultKey) {
        let idx = self
            .children
            .iter()
            .position(|child_key| key == *child_key)
            .unwrap();
        self.children.remove(idx);
    }
}

impl Element for View {}
