use crate::{View, ViewGroup};

pub struct LinearLayout<V> {
    view: V,
}

impl<V> LinearLayout<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }
}

impl<'a, M, V: ViewGroup<'a, M>> View<'a, M> for LinearLayout<V> {
    type Element = ();

    fn build(&'a mut self) -> Self::Element {
        todo!()
    }

    fn rebuild(&'a mut self, _element: &mut Self::Element) {
        todo!()
    }

    fn handle(&'a mut self, _msg: M) {
        todo!()
    }
}
