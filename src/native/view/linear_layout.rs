use super::{View, ViewGroup};
use concoct::Object;

pub struct LinearLayout<V> {
    views: V,
}

impl<V> LinearLayout<V> {
    pub fn new(views: impl ViewGroup<Handles = V>) -> Self {
        Self {
            views: views.view_group(),
        }
    }
}

impl<V> Object for LinearLayout<V> {}

impl<V> View for LinearLayout<V> where V: ViewGroup {}
