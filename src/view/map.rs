use crate::{Context, View};
use std::{marker::PhantomData, sync::Arc};

/// View for the `View::map` method.
pub struct Map<V, F, M> {
    pub(super) view: V,
    pub(super) f: Arc<F>,
    pub(super) _marker: PhantomData<M>,
}

impl<V, F, M1, M2> View<M1> for Map<V, F, M2>
where
    V: View<M2>,
    F: Fn(M2) -> M1 + Send + Sync + 'static,
    M1: Send + 'static,
{
    type Element = V::Element;

    fn build(&mut self, cx: &mut Context<M1>) -> Self::Element {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.build(&mut cx)
    }

    fn rebuild(&mut self, cx: &mut Context<M1>, state: &mut Self::Element) {
        let f = self.f.clone();
        let send = cx.send.clone();
        let mut cx = Context::new(Arc::new(move |msg| send(f(msg))));

        self.view.rebuild(&mut cx, state)
    }
}
