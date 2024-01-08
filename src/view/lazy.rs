use crate::View;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Create a lazy view that only renders when the given value changes.
pub fn lazy<M, V>(value: impl Hash, view: V) -> Lazy<V>
where
    V: View<M>,
{
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    Lazy { hash, view }
}

/// View for the [`lazy`] function.
pub struct Lazy<V> {
    hash: u64,
    view: V,
}

impl<M, V> View<M> for Lazy<V>
where
    V: View<M>,
{
    type Element = (u64, V::Element);

    fn build(&mut self, cx: &mut crate::Context<M>) -> Self::Element {
        let state = self.view.build(cx);
        (self.hash, state)
    }

    fn rebuild(&mut self, cx: &mut crate::Context<M>, state: &mut Self::Element) {
        if self.hash != state.0 {
            state.0 = self.hash;
            self.view.rebuild(cx, &mut state.1);
        }
    }
}
