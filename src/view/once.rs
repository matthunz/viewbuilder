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
        let span = tracing::trace_span!("View::Build", view = "Once");
        let _g = span.enter();

        self.view.build(cx, tree)
    }

    fn rebuild(&mut self, _cx: &mut Context<M>, _tree: &mut T, _element: &mut Self::Element) {
        let span = tracing::trace_span!("View::Rebuild", view = "Once");
        let _g = span.enter();
    }
}
