use crate::{Element, View};
use slotmap::DefaultKey;

pub struct LinearLayoutBuilder {
    cell: Option<LinearLayout>,
}

impl LinearLayoutBuilder {
    pub fn child(&mut self, view: impl View) -> &mut Self {
        self.cell.as_mut().unwrap().children.push(view.view());
        self
    }

    pub fn build(&mut self) -> LinearLayout {
        self.cell.take().unwrap()
    }
}

#[derive(Default)]
pub struct LinearLayout {
    children: Vec<DefaultKey>,
}

impl LinearLayout {
    pub fn builder() -> LinearLayoutBuilder {
        LinearLayoutBuilder {
            cell: Some(Self::default()),
        }
    }
}

impl Element for LinearLayout {
    fn children(&self) -> Option<Box<[DefaultKey]>> {
        Some(self.children.clone().into_boxed_slice())
    }
}
