use super::IntoView;

pub trait ViewGroup {}

impl<V: IntoView> ViewGroup for V {}

impl<A: IntoView, B: IntoView> ViewGroup for (A, B) {}
