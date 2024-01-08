use crate::{Context, View};

pub fn once<V, T, M>(view: V) -> Once<V>
where
    V: View<T, M>,
{
    Once { view }
}

pub struct Once<V> {
    view: V,
}

impl<V, T, M> View<T, M> for Once<V>
where
    V: View<T, M>,
{
    type Element = V::Element;

    fn build(&mut self, cx: &mut Context<M>, tree: &mut T) -> Self::Element {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Build", view = "Once");
        #[cfg(feature = "tracing")]
        let _g = span.enter();

        self.view.build(cx, tree)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut T, _element: &mut Self::Element) {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Rebuild", view = "Once");
        #[cfg(feature = "tracing")]
        let _g = span.enter();
    }

    fn remove(&mut self, _cx: &mut Context<M>, _state: &mut T, _element: Self::Element) {
        #[cfg(feature = "tracing")]
        let span = tracing::trace_span!("View::Remove", view = "Once");
        #[cfg(feature = "tracing")]
        let _g = span.enter();
    }
}
