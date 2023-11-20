use crate::{element::LinearLayoutElement, View, ViewGroup};

pub struct LinearLayout<V> {
    view: V,
}

impl<V> LinearLayout<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }
}

impl<'a, M, V: ViewGroup<'a, M>> View<'a, M> for LinearLayout<V> {
    type Element = LinearLayoutElement;

    fn build(&'a mut self) -> Self::Element {
        let mut nodes = Vec::new();
        self.view.build(&mut nodes);
        LinearLayoutElement { nodes }
    }

    fn rebuild(&'a mut self, element: &mut Self::Element) {
        self.view.rebuild(&mut element.nodes)
    }

    fn handle(&'a mut self, _msg: M) {}
}
