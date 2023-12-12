use super::{View, ViewGroup};

pub struct LinearLayout<V> {
    views: V,
}

impl<V> LinearLayout<V> {
    pub fn new(views: V) -> Self {
        Self { views }
    }
}

impl<V> View for LinearLayout<V>
where
    V: ViewGroup,
{
    fn view(self) {
        todo!()
    }
}
