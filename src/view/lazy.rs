use crate::View;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

/// Create a lazy view that only renders when the given value changes.
pub fn lazy<T, M, V>(value: impl Hash, view: V) -> Lazy<V>
where
    V: View<T, M>,
{
    let mut hasher = FxHasher::default();
    value.hash(&mut hasher);
    let hash = hasher.finish();

    Lazy { hash, view }
}

/// View for the [`lazy`] function.
pub struct Lazy<V> {
    hash: u64,
    view: V,
}

impl<T, M, V> View<T, M> for Lazy<V>
where
    V: View<T, M>,
{
    type Element = (u64, V::Element);

    fn build(&mut self, cx: &mut crate::Context<M>, tree: &mut T) -> Self::Element {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Build", view = "Lazy");
        #[cfg(feature = "tracing")]
        let _g = span.enter();

        let element = self.view.build(cx, tree);
        (self.hash, element)
    }

    fn rebuild(&mut self, cx: &mut crate::Context<M>, tree: &mut T, element: &mut Self::Element) {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Rebuild", view = "Lazy");
        #[cfg(feature = "tracing")]
        let _g = span.enter();

        if self.hash != element.0 {
            #[cfg(feature = "tracing")]
            tracing::trace!(name: "Hash change", new = self.hash, old = element.0);

            element.0 = self.hash;
            self.view.rebuild(cx, tree, &mut element.1);
        }
    }

    fn remove(&mut self, cx: &mut crate::Context<M>, state: &mut T, element: Self::Element) {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Remove", view = "Lazy");
        #[cfg(feature = "tracing")]
        let _g = span.enter();

        self.view.remove(cx, state, element.1);
    }
}
