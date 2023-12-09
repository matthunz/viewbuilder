use crate::{Element, UserInterface, View};
use kurbo::{Point, Size};
use slotmap::DefaultKey;

pub struct LinearLayoutBuilder {
    cell: Option<LinearLayout>,
}

impl LinearLayoutBuilder {
    pub fn child(&mut self, view: impl View) -> &mut Self {
        self.cell.as_mut().unwrap().children.push(Child {
            key: view.view(),
            pos: None,
        });
        self
    }

    pub fn build(&mut self) -> LinearLayout {
        self.cell.take().unwrap()
    }
}

struct Child {
    key: DefaultKey,
    pos: Option<f64>,
}

#[derive(Default)]
pub struct LinearLayout {
    children: Vec<Child>,
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
        Some(self.children.iter().map(|child| child.key).collect())
    }

    fn layout(&mut self, min: Option<kurbo::Size>, max: Option<kurbo::Size>) -> kurbo::Size {
        let mut pos = 0.;
        let mut max_bound = 0f64;
        for child in &mut self.children {
            let child_elem = UserInterface::current().get(child.key);
            let child_size = child_elem.borrow_mut().as_element_mut().layout(min, max);
            child.pos = Some(pos);
            pos += child_size.width;
            max_bound = max_bound.max(child_size.height);
        }
        Size::new(pos, max_bound)
    }

    fn render(&mut self, point: kurbo::Point, size: Size, scene: &mut vello::SceneBuilder) {
        for child in &self.children {
            let child_elem = UserInterface::current().get(child.key);
            let point = Point::new(point.x + child.pos.unwrap(), point.y);
            child_elem
                .borrow_mut()
                .as_element_mut()
                .render(point, size, scene);
        }
    }
}
