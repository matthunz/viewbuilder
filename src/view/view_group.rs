use super::View;

pub trait ViewGroup {}

impl<V: View> ViewGroup for V {}

impl<A: View, B: View> ViewGroup for (A, B) {}
