use crate::{Element, UserInterface, View};
use kurbo::Size;
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

    fn layout(&mut self, min: Option<kurbo::Size>, max: Option<kurbo::Size>) -> kurbo::Size {
        let mut pos = 0.;
        let mut max_bound = 0f64;
        for child_key in &self.children {
            let child = UserInterface::current().get(*child_key);
            let child_size = child.borrow_mut().as_element_mut().layout(min, max);
            pos += child_size.width;
            max_bound = max_bound.max(child_size.height);
        }
        Size::new(pos, max_bound)
    }
}
