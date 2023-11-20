use crate::{element::LinearLayoutElement, View, ViewGroup};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

pub struct LinearLayout<V> {
    view: V,
    direction: Direction,
}

impl<V> LinearLayout<V> {
    pub fn new(direction: Direction, view: V) -> Self {
        Self { view, direction }
    }

    pub fn row(view: V) -> Self {
        Self::new(Direction::Row, view)
    }

    pub fn column(view: V) -> Self {
        Self::new(Direction::Column, view)
    }
}

impl<'a, M, V: ViewGroup<'a, M>> View<'a, M> for LinearLayout<V> {
    type Element = LinearLayoutElement;

    fn build(&'a mut self) -> Self::Element {
        let mut nodes = Vec::new();
        self.view.build(&mut nodes);
        LinearLayoutElement {
            nodes,
            points: Vec::new(),
            direction: self.direction,
        }
    }

    fn rebuild(&'a mut self, element: &mut Self::Element) {
        self.view.rebuild(&mut element.nodes)
    }
}
