use concoct::Object;

use super::{View, ViewGroup};

pub struct LinearLayout<V> {
    views: V,
}

impl<V> LinearLayout<V> {
    pub fn new(views: V) -> Self {
        Self { views }
    }
}

impl<V> Object for LinearLayout<V> {}

impl<V> View for LinearLayout<V> where V: ViewGroup {}
